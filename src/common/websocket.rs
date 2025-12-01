use async_trait::async_trait;
use flate2::read::ZlibDecoder;
use futures::{SinkExt, StreamExt, stream::FuturesUnordered};
use http::header::USER_AGENT;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    io::Read,
    marker::PhantomData,
    mem::take,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio::{
    net::TcpStream,
    select, spawn,
    sync::{
        Mutex, Notify,
        mpsc::{Receiver, Sender, UnboundedSender, channel, unbounded_channel},
        oneshot,
    },
    task::JoinHandle,
    time::{sleep, timeout},
};
use tokio_tungstenite::{
    Connector, MaybeTlsStream, WebSocketStream, connect_async_tls_with_config,
    tungstenite::{
        Message,
        client::IntoClientRequest,
        protocol::{CloseFrame, WebSocketConfig, frame::coding::CloseCode},
    },
};
use tokio_util::time::DelayQueue;
use tracing::{debug, error, info, warn};

use super::{
    config::{AgentConnector, ConfigurationWebsocketApi, ConfigurationWebsocketStreams},
    errors::{WebsocketConnectionFailureReason, WebsocketError},
    models::{WebsocketApiResponse, WebsocketEvent, WebsocketMode},
    utils::{ID_REGEX, build_websocket_api_message, random_string, validate_time_unit},
};

pub type WebSocketClient = WebSocketStream<MaybeTlsStream<TcpStream>>;

const MAX_CONN_DURATION: Duration = Duration::from_secs(23 * 60 * 60);

pub struct Subscription {
    handle: JoinHandle<()>,
}

impl Subscription {
    /// Cancels the ongoing WebSocket event subscription and stops the event processing task.
    ///
    /// This method aborts the background task responsible for receiving and processing
    /// WebSocket events, effectively unsubscribing from further event notifications.
    ///
    /// # Examples
    ///
    ///
    /// let emitter = `WebsocketEventEmitter::new()`;
    /// let subscription = emitter.subscribe(|event| {
    ///     // Handle WebSocket event
    /// });
    /// `subscription.unsubscribe()`; // Stop receiving events
    ///
    pub fn unsubscribe(self) {
        self.handle.abort();
    }
}

#[derive(Clone)]
pub enum WebsocketBase {
    WebsocketApi(Arc<WebsocketApi>),
    WebsocketStreams(Arc<WebsocketStreams>),
}

pub struct WebsocketEventEmitter {
    subscribers: Arc<std::sync::Mutex<Vec<UnboundedSender<WebsocketEvent>>>>,
}

impl Default for WebsocketEventEmitter {
    fn default() -> Self {
        Self::new()
    }
}

impl WebsocketEventEmitter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    /// Subscribes to WebSocket events and returns a `Subscription` that allows event processing.
    ///
    /// This method creates an unbounded channel for receiving WebSocket events and
    /// spawns an asynchronous task to process these events using the provided callback function.
    ///
    /// # Arguments
    ///
    /// * `callback` - A mutable function that will be called for each received WebSocket event.
    ///   The callback must be thread-safe and have a static lifetime.
    ///
    /// # Returns
    ///
    /// A `Subscription` that can be used to unsubscribe and stop event processing.
    ///
    /// # Examples
    ///
    ///
    /// let emitter = `WebsocketEventEmitter::new()`;
    /// let subscription = emitter.subscribe(|event| {
    ///     // Handle WebSocket event
    ///     println!("Received event: {:?}", event);
    /// });
    ///
    /// // Later, when no longer needed
    /// `subscription.unsubscribe()`;
    ///
    pub fn subscribe<F>(&self, mut callback: F) -> Subscription
    where
        F: FnMut(WebsocketEvent) + Send + 'static,
    {
        let (tx, mut rx) = unbounded_channel();
        let mut guard = match self.subscribers.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        guard.push(tx);
        drop(guard);

        let handle = spawn(async move {
            while let Some(event) = rx.recv().await {
                callback(event);
            }
        });
        Subscription { handle }
    }

    /// Emits a WebSocket event to all registered subscribers.
    ///
    /// This method sends the given event to all active subscribers. If a subscriber
    /// has been dropped without unsubscribing, a warning is logged and the subscriber
    /// is removed from the list.
    ///
    /// # Arguments
    ///
    /// * `event` - The WebSocket event to be emitted to all subscribers.
    pub fn emit(&self, event: &WebsocketEvent) {
        let mut guard = match self.subscribers.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        guard.retain(|tx| {
            if tx.send(event.clone()).is_ok() {
                true
            } else {
                warn!("subscriber dropped without unsubscribing");
                false
            }
        });
    }
}

/// A trait defining the lifecycle and behavior of a WebSocket connection.
///
/// This trait provides methods for handling WebSocket connection events,
/// including connection opening, message handling, and reconnection URL retrieval.
///
/// # Methods
///
/// * `on_open`: Called when a WebSocket connection is established
/// * `on_message`: Called when a message is received over the WebSocket
/// * `get_reconnect_url`: Determines the URL to use for reconnecting
///
/// # Thread Safety
///
/// Implementors must be safely shareable across threads, as indicated by the `Send + Sync + 'static` bounds.
#[async_trait]
pub trait WebsocketHandler: Send + Sync + 'static {
    async fn on_open(&self, url: String, connection: Arc<WebsocketConnection>);
    async fn on_message(&self, data: String, connection: Arc<WebsocketConnection>);
    async fn get_reconnect_url(
        &self,
        default_url: String,
        connection: Arc<WebsocketConnection>,
    ) -> String;
}

pub struct PendingRequest {
    pub completion: oneshot::Sender<Result<Value, WebsocketError>>,
}

#[derive(Clone)]
pub struct WebsocketSessionLogonReq {
    pub method: String,
    pub payload: BTreeMap<String, Value>,
    pub options: WebsocketMessageSendOptions,
}

pub struct WebsocketConnectionState {
    pub reconnection_pending: bool,
    pub renewal_pending: bool,
    pub close_initiated: bool,
    pub pending_requests: HashMap<String, PendingRequest>,
    pub pending_subscriptions: VecDeque<String>,
    pub stream_callbacks: HashMap<String, Vec<Arc<dyn Fn(&Value) + Send + Sync + 'static>>>,
    pub is_session_logged_on: bool,
    pub session_logon_req: Option<WebsocketSessionLogonReq>,
    pub handler: Option<Arc<dyn WebsocketHandler>>,
    pub ws_write_tx: Option<UnboundedSender<Message>>,
}

impl Default for WebsocketConnectionState {
    fn default() -> Self {
        Self::new()
    }
}

impl WebsocketConnectionState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            reconnection_pending: false,
            renewal_pending: false,
            close_initiated: false,
            pending_requests: HashMap::new(),
            pending_subscriptions: VecDeque::new(),
            stream_callbacks: HashMap::new(),
            is_session_logged_on: false,
            session_logon_req: None,
            handler: None,
            ws_write_tx: None,
        }
    }
}

pub struct WebsocketConnection {
    pub id: String,
    pub drain_notify: Notify,
    pub state: Mutex<WebsocketConnectionState>,
}

impl WebsocketConnection {
    pub fn new(id: impl Into<String>) -> Arc<Self> {
        Arc::new(Self {
            id: id.into(),
            drain_notify: Notify::new(),
            state: Mutex::new(WebsocketConnectionState::new()),
        })
    }

    pub async fn set_handler(&self, handler: Arc<dyn WebsocketHandler>) {
        let mut conn_state = self.state.lock().await;
        conn_state.handler = Some(handler);
    }
}

struct ReconnectEntry {
    connection_id: String,
    url: String,
    is_renewal: bool,
}

pub struct WebsocketCommon {
    pub events: WebsocketEventEmitter,
    mode: WebsocketMode,
    round_robin_index: AtomicUsize,
    connection_pool: Vec<Arc<WebsocketConnection>>,
    reconnect_tx: Sender<ReconnectEntry>,
    renewal_tx: Sender<(String, String)>,
    reconnect_delay: usize,
    agent: Option<AgentConnector>,
    user_agent: Option<String>,
}

impl WebsocketCommon {
    #[must_use]
    pub fn new(
        mut initial_pool: Vec<Arc<WebsocketConnection>>,
        mode: WebsocketMode,
        reconnect_delay: usize,
        agent: Option<AgentConnector>,
        user_agent: Option<String>,
    ) -> Arc<Self> {
        if initial_pool.is_empty() {
            for _ in 0..mode.pool_size() {
                let id = random_string();
                initial_pool.push(WebsocketConnection::new(id));
            }
        }

        let (reconnect_tx, reconnect_rx) = channel::<ReconnectEntry>(mode.pool_size());
        let (renewal_tx, renewal_rx) = channel::<(String, String)>(mode.pool_size());

        let common = Arc::new(Self {
            events: WebsocketEventEmitter::new(),
            mode,
            round_robin_index: AtomicUsize::new(0),
            connection_pool: initial_pool,
            reconnect_tx,
            renewal_tx,
            reconnect_delay,
            agent,
            user_agent,
        });

        Self::spawn_reconnect_loop(Arc::clone(&common), reconnect_rx);
        Self::spawn_renewal_loop(&Arc::clone(&common), renewal_rx);

        common
    }

    /// Spawns an asynchronous loop to handle websocket reconnection attempts
    ///
    /// This method manages reconnection logic for websocket connections, including:
    /// - Scheduling reconnects with a configurable delay
    /// - Finding the appropriate connection in the connection pool
    /// - Attempting to reinitialize the connection
    /// - Logging reconnection failures or warnings
    ///
    /// # Arguments
    /// * `common` - A shared reference to the `WebsocketCommon` instance
    /// * `reconnect_rx` - A receiver channel for reconnection entries
    ///
    /// # Behavior
    /// - Waits for reconnection entries from the channel
    /// - Applies a configurable delay before attempting reconnection
    /// - Attempts to reinitialize the connection with the provided URL
    /// - Handles and logs any reconnection errors
    fn spawn_reconnect_loop(common: Arc<Self>, mut reconnect_rx: Receiver<ReconnectEntry>) {
        spawn(async move {
            while let Some(entry) = reconnect_rx.recv().await {
                info!("Scheduling reconnect for id {}", entry.connection_id);

                if !entry.is_renewal {
                    sleep(Duration::from_millis(common.reconnect_delay as u64)).await;
                }

                if let Some(conn_arc) = common
                    .connection_pool
                    .iter()
                    .find(|c| c.id == entry.connection_id)
                    .cloned()
                {
                    let common_clone = Arc::clone(&common);
                    if let Err(err) = common_clone
                        .init_connect(&entry.url, entry.is_renewal, Some(conn_arc.clone()))
                        .await
                    {
                        error!(
                            "Reconnect failed for {} → {}: {:?}",
                            entry.connection_id, entry.url, err
                        );
                    }

                    sleep(Duration::from_secs(1)).await;
                } else {
                    warn!("No connection {} found for reconnect", entry.connection_id);
                }
            }
        });
    }

    /// Spawns an asynchronous loop to manage connection renewals
    ///
    /// This method handles the periodic renewal of websocket connections by:
    /// - Maintaining a delay queue for connection expiration
    /// - Receiving renewal requests for specific connections
    /// - Triggering reconnection when a connection reaches its maximum duration
    /// - Attempting to find and renew connections in the connection pool
    ///
    /// # Behavior
    /// - Listens for renewal requests on a channel
    /// - Tracks connection expiration using a delay queue
    /// - Initiates reconnection process when a connection expires
    /// - Handles and logs any renewal failures
    fn spawn_renewal_loop(common: &Arc<Self>, renewal_rx: Receiver<(String, String)>) {
        let common = Arc::clone(common);
        spawn(async move {
            let mut dq = DelayQueue::new();
            let mut renewal_rx = renewal_rx;

            loop {
                select! {
                    Some((conn_id, url)) = renewal_rx.recv() => {
                        debug!("Scheduling renewal for {}", conn_id);
                        dq.insert((conn_id, url), MAX_CONN_DURATION);
                    }

                    Some(expired) = dq.next() => {
                        let (conn_id, default_url) = expired.into_inner();

                        if let Some(conn_arc) = common
                            .connection_pool
                            .iter()
                            .find(|c| c.id == conn_id)
                            .cloned()
                        {
                            debug!("Renewing connection {}", conn_id);
                            let url = common
                                .get_reconnect_url(&default_url, Arc::clone(&conn_arc))
                                .await;
                            if let Err(e) = common.reconnect_tx.send(ReconnectEntry {
                                connection_id: conn_id.clone(),
                                url,
                                is_renewal: true,
                            }).await {
                                error!(
                                    "Failed to enqueue renewal for {}: {:?}",
                                    conn_id, e
                                );
                            }
                        } else {
                            warn!("No connection {} found for renewal", conn_id);
                        }
                    }
                }
            }
        });
    }

    /// Checks if a WebSocket connection is ready for use.
    ///
    /// # Arguments
    ///
    /// * `connection` - The WebSocket connection to check
    /// * `allow_non_established` - If true, allows connections that are not fully established
    ///
    /// # Returns
    ///
    /// `true` if the connection is ready, `false` otherwise
    ///
    /// # Behavior
    ///
    /// A connection is considered ready if:
    /// - It has a write channel (unless `allow_non_established` is true)
    /// - No reconnection is pending
    /// - No close has been initiated
    pub async fn is_connection_ready(
        &self,
        connection: &WebsocketConnection,
        allow_non_established: bool,
    ) -> bool {
        let conn_state = connection.state.lock().await;
        (allow_non_established || conn_state.ws_write_tx.is_some())
            && !conn_state.reconnection_pending
            && !conn_state.close_initiated
    }

    /// Checks if a WebSocket connection is established.
    ///
    /// # Arguments
    ///
    /// * `connection` - Optional specific WebSocket connection to check
    ///
    /// # Returns
    ///
    /// `true` if a connection is ready and established, `false` otherwise
    ///
    /// # Behavior
    ///
    /// - If a specific connection is provided, checks only that connection
    /// - If no connection is provided, checks all connections in the pool
    /// - A connection is considered established if it is ready and not in a non-established state
    async fn is_connected(&self, connection: Option<&Arc<WebsocketConnection>>) -> bool {
        if let Some(conn_arc) = connection {
            return self.is_connection_ready(conn_arc, false).await;
        }

        for conn_arc in &self.connection_pool {
            if self.is_connection_ready(conn_arc, false).await {
                return true;
            }
        }

        false
    }

    /// Retrieves available WebSocket connections from the connection pool.
    ///
    /// # Arguments
    ///
    /// * `allow_non_established` - If `true`, includes connections that are not fully established
    ///
    /// # Returns
    ///
    /// A vector of `Arc<WebsocketConnection>` that are ready based on the `allow_non_established` flag
    ///
    /// # Behavior
    ///
    /// - For single connection mode, returns the first connection
    /// - For multi-connection mode, filters connections based on readiness
    /// - Uses `is_connection_ready` to determine connection availability
    async fn get_available_connections(
        &self,
        allow_non_established: bool,
    ) -> Vec<Arc<WebsocketConnection>> {
        if let WebsocketMode::Single = self.mode {
            return vec![Arc::clone(&self.connection_pool[0])];
        }

        let mut ready = Vec::new();
        for conn in &self.connection_pool {
            if self.is_connection_ready(conn, allow_non_established).await {
                ready.push(Arc::clone(conn));
            }
        }

        ready
    }

    /// Retrieves a WebSocket connection from the connection pool.
    ///
    /// # Arguments
    ///
    /// * `allow_non_established` - If `true`, allows selecting a connection that is not fully established
    ///
    /// # Returns
    ///
    /// An `Arc` to a `WebsocketConnection` from the pool, selected using round-robin strategy
    ///
    /// # Errors
    ///
    /// Returns `WebsocketError::NotConnected` if no suitable connection is available
    ///
    /// # Behavior
    ///
    /// - For single connection mode, returns the first connection
    /// - For multi-connection mode, selects a ready connection using round-robin
    /// - Filters connections based on `allow_non_established` parameter
    async fn get_connection(
        &self,
        allow_non_established: bool,
    ) -> Result<Arc<WebsocketConnection>, WebsocketError> {
        let ready = self.get_available_connections(allow_non_established).await;

        if ready.is_empty() {
            return Err(WebsocketError::NotConnected);
        }

        let idx = self.round_robin_index.fetch_add(1, Ordering::Relaxed) % ready.len();

        Ok(Arc::clone(&ready[idx]))
    }

    /// Gracefully closes a WebSocket connection by waiting for pending requests to complete.
    ///
    /// # Arguments
    ///
    /// * `ws_write_tx_to_close` - Sender channel for sending close message
    /// * `connection` - Shared reference to the WebSocket connection
    ///
    /// # Behavior
    ///
    /// - Waits up to 30 seconds for all pending requests to complete
    /// - Logs debug and warning messages during the closing process
    /// - Sends a normal close frame to the WebSocket
    ///
    /// # Returns
    ///
    /// `Ok(())` if connection closes successfully, otherwise a `WebsocketError`
    async fn close_connection_gracefully(
        &self,
        ws_write_tx_to_close: UnboundedSender<Message>,
        connection: Arc<WebsocketConnection>,
    ) -> Result<(), WebsocketError> {
        debug!("Waiting for pending requests to complete before disconnecting.");

        let drain = async {
            loop {
                {
                    let conn_state = connection.state.lock().await;
                    if conn_state.pending_requests.is_empty() {
                        debug!("All pending requests completed, proceeding to close.");
                        break;
                    }
                }
                connection.drain_notify.notified().await;
            }
        };

        if timeout(Duration::from_secs(30), drain).await.is_err() {
            warn!("Timeout waiting for pending requests; forcing close.");
        }

        info!("Closing WebSocket connection for {}", connection.id);
        let _ = ws_write_tx_to_close.send(Message::Close(Some(CloseFrame {
            code: CloseCode::Normal,
            reason: "".into(),
        })));

        Ok(())
    }

    /// Retrieves the URL to use for reconnecting to the WebSocket.
    ///
    /// # Arguments
    ///
    /// * `default_url` - The default URL to use if no custom reconnect URL is provided
    /// * `connection` - A shared reference to the WebSocket connection
    ///
    /// # Returns
    ///
    /// The URL to use for reconnecting, either from a custom handler or the default URL
    ///
    /// # Behavior
    ///
    /// - Checks if a connection handler is available
    /// - If a handler exists, calls its `get_reconnect_url` method
    /// - Otherwise, returns the default URL
    async fn get_reconnect_url(
        &self,
        default_url: &str,
        connection: Arc<WebsocketConnection>,
    ) -> String {
        if let Some(handler) = {
            let conn_state = connection.state.lock().await;
            conn_state.handler.clone()
        } {
            return handler
                .get_reconnect_url(default_url.to_string(), Arc::clone(&connection))
                .await;
        }

        default_url.to_string()
    }

    /// Handles the WebSocket connection opening event.
    ///
    /// This method is called when a WebSocket connection is successfully established. It performs
    /// the following key actions:
    /// - Invokes the connection handler's `on_open` method if a handler is present
    /// - Logs connection information
    /// - Handles connection renewal and close scenarios
    /// - Emits a WebSocket open event
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the WebSocket server
    /// * `connection` - A shared reference to the WebSocket connection
    /// * `old_ws_writer` - Optional previous WebSocket writer for graceful connection handling
    ///
    /// # Behavior
    ///
    /// - If a connection handler exists, calls its `on_open` method
    /// - Checks for pending renewal or close states
    /// - Closes the previous connection if renewal is in progress
    /// - Emits an open event if the connection is successfully established
    async fn on_open(
        &self,
        url: String,
        connection: Arc<WebsocketConnection>,
        old_ws_writer: Option<UnboundedSender<Message>>,
    ) {
        if let Some(handler) = {
            let conn_state = connection.state.lock().await;
            conn_state.handler.clone()
        } {
            handler.on_open(url.clone(), Arc::clone(&connection)).await;
        }

        let conn_id = &connection.id;
        info!("Connected to WebSocket Server with id {}: {}", conn_id, url);

        {
            let mut conn_state = connection.state.lock().await;

            if conn_state.renewal_pending {
                conn_state.renewal_pending = false;
                drop(conn_state);
                if let Some(tx) = old_ws_writer {
                    info!("Connection renewal in progress; closing previous connection.");
                    let _ = self
                        .close_connection_gracefully(tx, Arc::clone(&connection))
                        .await;
                }
                return;
            }

            if conn_state.close_initiated {
                drop(conn_state);
                if let Some(tx) = connection.state.lock().await.ws_write_tx.clone() {
                    info!("Close initiated; closing connection.");
                    let _ = self
                        .close_connection_gracefully(tx, Arc::clone(&connection))
                        .await;
                }
                return;
            }

            self.events.emit(&WebsocketEvent::Open);
        }
    }

    /// Handles an incoming WebSocket message
    ///
    /// # Arguments
    ///
    /// * `msg` - The received message as a string
    /// * `connection` - A shared reference to the WebSocket connection
    ///
    /// # Behavior
    ///
    /// - If a connection handler exists, spawns an async task to call its `on_message` method
    /// - Emits a `WebsocketEvent::Message` event with the received message
    async fn on_message(&self, msg: String, connection: Arc<WebsocketConnection>) {
        if let Some(handler) = connection.state.lock().await.handler.clone() {
            let handler_clone = handler.clone();
            let data = msg.clone();
            let conn_clone = connection.clone();
            spawn(async move {
                handler_clone.on_message(data, conn_clone).await;
            });
        }
        self.events.emit(&WebsocketEvent::Message(msg));
    }

    /// Creates a WebSocket connection with optional configuration and agent
    ///
    /// # Arguments
    ///
    /// * `url` - The WebSocket server URL to connect to
    /// * `agent` - Optional agent connector for configuring the connection
    /// * `user_agent` - Optional custom user agent string
    ///
    /// # Returns
    ///
    /// A `Result` containing the established WebSocket stream or a `WebsocketError`
    ///
    /// # Errors
    ///
    /// Returns a `WebsocketError` if:
    /// - The WebSocket handshake fails
    /// - The connection times out after 10 seconds
    ///
    /// # Behavior
    ///
    /// Attempts to establish a WebSocket connection with a configurable timeout,
    /// supporting optional TLS, custom user agent, and connection connectors
    async fn create_websocket(
        url: &str,
        agent: Option<AgentConnector>,
        user_agent: Option<String>,
    ) -> Result<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>, WebsocketError> {
        let mut req = url
            .into_client_request()
            .map_err(|e| WebsocketError::Handshake(e.to_string()))?;

        if let Some(ua) = user_agent {
            req.headers_mut().insert(USER_AGENT, ua.parse().unwrap());
        }

        let ws_config: Option<WebSocketConfig> = None;
        let disable_nagle = false;
        let connector: Option<Connector> = agent.map(|dbg| dbg.0);

        let timeout_duration = Duration::from_secs(10);
        let handshake = connect_async_tls_with_config(req, ws_config, disable_nagle, connector);
        match timeout(timeout_duration, handshake).await {
            Ok(Ok((ws_stream, response))) => {
                debug!("WebSocket connected: {:?}", response);
                Ok(ws_stream)
            }
            Ok(Err(e)) => {
                let msg = e.to_string();
                error!("WebSocket handshake failed: {}", msg);
                Err(WebsocketError::Handshake(msg))
            }
            Err(_) => {
                error!(
                    "WebSocket connection timed out after {}s",
                    timeout_duration.as_secs()
                );
                Err(WebsocketError::Timeout)
            }
        }
    }

    /// Connects to a WebSocket URL for all connections in the connection pool concurrently
    ///
    /// # Arguments
    ///
    /// * `url` - The WebSocket server URL to connect to
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether all connections were successfully established
    ///
    /// # Errors
    ///
    /// Returns a `WebsocketError` if any connection in the pool fails to establish
    ///
    /// # Behavior
    ///
    /// Attempts to initialize a WebSocket connection for each connection in the pool
    /// concurrently, logging successes and failures for each connection attempt
    async fn connect_pool(self: Arc<Self>, url: &str) -> Result<(), WebsocketError> {
        let mut tasks = FuturesUnordered::new();

        for conn in &self.connection_pool {
            let common = Arc::clone(&self);
            let url = url.to_owned();
            let conn_clone = Arc::clone(conn);

            tasks.push(async move {
                match common.init_connect(&url, false, Some(conn_clone)).await {
                    Ok(()) => {
                        info!("Successfully connected to {}", url);
                        Ok(())
                    }
                    Err(err) => {
                        error!("Failed to connect to {}: {:?}", url, err);
                        Err(err)
                    }
                }
            });
        }

        while let Some(result) = tasks.next().await {
            result?;
        }

        Ok(())
    }

    /// Initializes a WebSocket connection for a specific connection in the pool
    ///
    /// # Arguments
    ///
    /// * `url` - The WebSocket server URL to connect to
    /// * `is_renewal` - Flag indicating whether this is a connection renewal attempt
    /// * `connection` - Optional specific WebSocket connection to use, otherwise selects from the pool
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the connection was successfully established
    ///
    /// # Errors
    ///
    /// Returns a `WebsocketError` if the connection fails to initialize or establish
    ///
    /// # Behavior
    ///
    /// Handles connection establishment, splitting read/write streams, spawning reader/writer tasks,
    /// and managing connection state including renewal, reconnection, and error handling
    async fn init_connect(
        self: Arc<Self>,
        url: &str,
        is_renewal: bool,
        connection: Option<Arc<WebsocketConnection>>,
    ) -> Result<(), WebsocketError> {
        let conn = connection.unwrap_or(self.get_connection(true).await?);

        {
            let mut conn_state = conn.state.lock().await;
            if conn_state.renewal_pending && is_renewal {
                info!("Renewal in progress {}→{}", conn.id, url);
                return Ok(());
            }
            if conn_state.ws_write_tx.is_some() && !is_renewal {
                info!("Exists {}; skipping {}", conn.id, url);
                return Ok(());
            }
            if is_renewal {
                conn_state.renewal_pending = true;
            }

            conn_state.is_session_logged_on = false;
        }

        let ws = Self::create_websocket(url, self.agent.clone(), self.user_agent.clone())
            .await
            .map_err(|e| {
                error!("Handshake failed {}: {:?}", url, e);
                e
            })?;

        info!("Established {} → {}", conn.id, url);

        if let Err(e) = self.renewal_tx.try_send((conn.id.clone(), url.to_string())) {
            error!("Failed to schedule renewal for {}: {:?}", conn.id, e);
        }

        let (write_half, mut read_half) = ws.split();
        let (tx, mut rx) = unbounded_channel::<Message>();

        let old_writer = {
            let mut conn_state = conn.state.lock().await;
            conn_state.ws_write_tx.replace(tx.clone())
        };

        {
            let wconn = conn.clone();
            let common_clone = self.clone();
            let writer_url = url.to_string();

            spawn(async move {
                let mut sink = write_half;
                while let Some(msg) = rx.recv().await {
                    if let Err(e) = sink.send(msg).await {
                        let failure_reason =
                            WebsocketConnectionFailureReason::from_tungstenite_error(&e);

                        error!(
                            "Write error on {}: {:?}, classified as {:?}",
                            wconn.id, e, failure_reason
                        );

                        // Apply same reconnection logic as reader errors
                        let mut conn_state = wconn.state.lock().await;
                        if !conn_state.close_initiated
                            && !is_renewal
                            && failure_reason.should_reconnect()
                        {
                            info!(
                                "Writer connection {} has recoverable error, attempting reconnection: {:?}",
                                wconn.id, failure_reason
                            );
                            conn_state.reconnection_pending = true;
                            conn_state.is_session_logged_on = false;
                            drop(conn_state);
                            let reconnect_url = common_clone
                                .get_reconnect_url(&writer_url, Arc::clone(&wconn))
                                .await;

                            let _ = common_clone
                                .reconnect_tx
                                .send(ReconnectEntry {
                                    connection_id: wconn.id.clone(),
                                    url: reconnect_url,
                                    is_renewal: false,
                                })
                                .await;
                        } else {
                            warn!(
                                "Writer connection {} has permanent error, will not reconnect: {:?}",
                                wconn.id, failure_reason
                            );
                        }

                        break;
                    }
                }
                debug!("Writer {} exit", wconn.id);
            });
        }

        {
            let common = self.clone();
            let conn = conn.clone();
            let url = url.to_string();
            spawn(async move {
                common.on_open(url, conn, old_writer).await;
            });
        }

        {
            let common = self.clone();
            let reader_conn = conn.clone();
            let read_url = url.to_string();

            spawn(async move {
                while let Some(item) = read_half.next().await {
                    match item {
                        Ok(Message::Text(msg)) => {
                            common
                                .on_message(msg.to_string(), Arc::clone(&reader_conn))
                                .await;
                        }
                        Ok(Message::Binary(bin)) => {
                            let mut decoder = ZlibDecoder::new(&bin[..]);
                            let mut decompressed = String::new();
                            if let Err(err) = decoder.read_to_string(&mut decompressed) {
                                error!("Binary message decompress failed: {:?}", err);
                                continue;
                            }
                            common
                                .on_message(decompressed, Arc::clone(&reader_conn))
                                .await;
                        }
                        Ok(Message::Ping(payload)) => {
                            info!("PING received from server on {}", reader_conn.id);
                            common.events.emit(&WebsocketEvent::Ping);
                            if let Some(tx) = reader_conn.state.lock().await.ws_write_tx.clone() {
                                let _ = tx.send(Message::Pong(payload));
                                info!(
                                    "Responded PONG to server's PING message on {}",
                                    reader_conn.id
                                );
                            }
                        }
                        Ok(Message::Pong(_)) => {
                            info!("Received PONG from server on {}", reader_conn.id);
                            common.events.emit(&WebsocketEvent::Pong);
                        }
                        Ok(Message::Close(frame)) => {
                            let (code, reason) = frame
                                .map_or((1000, String::new()), |CloseFrame { code, reason }| {
                                    (code.into(), reason.to_string())
                                });
                            common
                                .events
                                .emit(&WebsocketEvent::Close(code, reason.clone()));

                            // Classify the close reason
                            let user_initiated = {
                                let conn_state = reader_conn.state.lock().await;
                                conn_state.close_initiated
                            };

                            let failure_reason = WebsocketConnectionFailureReason::from_close_code(
                                code,
                                user_initiated,
                            );

                            info!(
                                "Connection {} received close frame: code={}, reason='{}', classified as {:?}",
                                reader_conn.id, code, reason, failure_reason
                            );

                            let mut conn_state = reader_conn.state.lock().await;
                            if !conn_state.close_initiated
                                && !is_renewal
                                && failure_reason.should_reconnect()
                            {
                                info!(
                                    "Connection {} received close frame with reconnectable failure: {:?}",
                                    reader_conn.id, failure_reason
                                );
                                conn_state.reconnection_pending = true;
                                conn_state.is_session_logged_on = false;
                                drop(conn_state);
                                let reconnect_url = common
                                    .get_reconnect_url(&read_url, Arc::clone(&reader_conn))
                                    .await;

                                let _ = common
                                    .reconnect_tx
                                    .send(ReconnectEntry {
                                        connection_id: reader_conn.id.clone(),
                                        url: reconnect_url,
                                        is_renewal: false,
                                    })
                                    .await;
                            } else {
                                warn!(
                                    "Connection {} received close frame with non-reconnectable failure: {:?}",
                                    reader_conn.id, failure_reason
                                );

                                // Emit detailed error for permanent failures
                                common.events.emit(&WebsocketEvent::Error(format!(
                                    "[CRITICAL] Connection {} permanently failed: {:?}",
                                    reader_conn.id, failure_reason
                                )));
                            }

                            break;
                        }
                        Err(e) => {
                            // Classify the error type for reconnection decision
                            let failure_reason =
                                WebsocketConnectionFailureReason::from_tungstenite_error(&e);

                            error!(
                                "WebSocket error on {}: {:?}, classified as {:?}",
                                reader_conn.id, e, failure_reason
                            );

                            common.events.emit(&WebsocketEvent::Error(e.to_string()));

                            // Apply the same reconnection logic as Close frames
                            let mut conn_state = reader_conn.state.lock().await;
                            if !conn_state.close_initiated
                                && !is_renewal
                                && failure_reason.should_reconnect()
                            {
                                info!(
                                    "Connection {} has recoverable error, attempting reconnection: {:?}",
                                    reader_conn.id, failure_reason
                                );
                                conn_state.reconnection_pending = true;
                                conn_state.is_session_logged_on = false;
                                drop(conn_state);
                                let reconnect_url = common
                                    .get_reconnect_url(&read_url, Arc::clone(&reader_conn))
                                    .await;

                                let _ = common
                                    .reconnect_tx
                                    .send(ReconnectEntry {
                                        connection_id: reader_conn.id.clone(),
                                        url: reconnect_url,
                                        is_renewal: false,
                                    })
                                    .await;
                            } else {
                                warn!(
                                    "Connection {} has permanent error, will not reconnect: {:?}",
                                    reader_conn.id, failure_reason
                                );

                                // Emit critical error for non-reconnectable failures
                                common.events.emit(&WebsocketEvent::Error(format!(
                                    "[CRITICAL] Connection {} permanently failed: {:?}",
                                    reader_conn.id, failure_reason
                                )));
                            }

                            break;
                        }
                        _ => {}
                    }
                }

                // Handle case where stream ends unexpectedly (e.g., network disconnection)
                info!("WebSocket stream ended for connection {}", reader_conn.id);

                // Handle unexpected stream end with same logic as other errors
                let failure_reason = WebsocketConnectionFailureReason::StreamEnded;

                info!(
                    "WebSocket stream ended for connection {}, classified as {:?}",
                    reader_conn.id, failure_reason
                );

                let mut conn_state = reader_conn.state.lock().await;
                if !conn_state.close_initiated && !is_renewal && failure_reason.should_reconnect() {
                    info!(
                        "Connection {} stream ended unexpectedly, attempting reconnection",
                        reader_conn.id
                    );
                    conn_state.reconnection_pending = true;
                    conn_state.is_session_logged_on = false;
                    drop(conn_state);
                    let reconnect_url = common
                        .get_reconnect_url(&read_url, Arc::clone(&reader_conn))
                        .await;

                    let _ = common
                        .reconnect_tx
                        .send(ReconnectEntry {
                            connection_id: reader_conn.id.clone(),
                            url: reconnect_url,
                            is_renewal: false,
                        })
                        .await;
                } else {
                    debug!(
                        "Connection {} stream ended normally (close_initiated={}, is_renewal={})",
                        reader_conn.id, conn_state.close_initiated, is_renewal
                    );
                }

                debug!("Reader actor for {} exiting", reader_conn.id);
            });
        }

        Ok(())
    }
    /// Gracefully disconnects all active WebSocket connections.
    ///
    /// This method attempts to close all connections in the connection pool within a 30-second timeout.
    /// It marks each connection as close-initiated and attempts to close them gracefully.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if all connections are successfully closed
    /// - `Err(WebsocketError)` if there are errors during disconnection or a timeout occurs
    ///
    /// # Errors
    ///
    /// Returns `WebsocketError::Timeout` if disconnection takes longer than 30 seconds
    ///
    async fn disconnect(&self) -> Result<(), WebsocketError> {
        if !self.is_connected(None).await {
            warn!("No active connection to close.");
            return Ok(());
        }

        let mut shutdowns = FuturesUnordered::new();
        for conn in &self.connection_pool {
            {
                let mut conn_state = conn.state.lock().await;
                conn_state.close_initiated = true;
                if let Some(tx) = &conn_state.ws_write_tx {
                    shutdowns.push(self.close_connection_gracefully(tx.clone(), Arc::clone(conn)));
                }
            }
        }

        let close_all = async {
            while let Some(result) = shutdowns.next().await {
                result?;
            }
            Ok::<(), WebsocketError>(())
        };

        match timeout(Duration::from_secs(30), close_all).await {
            Ok(Ok(())) => {
                info!("Disconnected all WebSocket connections successfully.");
                for conn in &self.connection_pool {
                    let mut st = conn.state.lock().await;
                    st.is_session_logged_on = false;
                    st.session_logon_req = None;
                }
                Ok(())
            }
            Ok(Err(err)) => {
                error!("Error while disconnecting: {:?}", err);
                Err(err)
            }
            Err(_) => {
                error!("Timed out while disconnecting WebSocket connections.");
                Err(WebsocketError::Timeout)
            }
        }
    }

    /// Sends a PING message to all ready WebSocket connections.
    ///
    /// This method iterates through the connection pool, identifies ready connections,
    /// and sends a PING message to each of them. It logs the number of connections
    /// being pinged and handles any send errors individually.
    ///
    /// # Behavior
    ///
    /// - Skips connections that are not ready
    /// - Logs a warning if no connections are ready
    /// - Sends PING messages concurrently
    /// - Logs debug/error messages for each PING attempt
    async fn ping_server(&self) {
        let mut ready = Vec::new();
        for conn in &self.connection_pool {
            if self.is_connection_ready(conn, false).await {
                let id = conn.id.clone();
                let ws_write_tx = {
                    let conn_state = conn.state.lock().await;
                    conn_state.ws_write_tx.clone()
                };
                ready.push((id, ws_write_tx));
            }
        }

        if ready.is_empty() {
            warn!("No ready connections for PING.");
            return;
        }
        info!("Sending PING to {} WebSocket connections.", ready.len());

        let mut tasks = FuturesUnordered::new();
        for (id, ws_write_tx_opt) in ready {
            if let Some(tx) = ws_write_tx_opt {
                tasks.push(async move {
                    if let Err(e) = tx.send(Message::Ping(Vec::new().into())) {
                        error!("Failed to send PING to {}: {:?}", id, e);
                    } else {
                        debug!("Sent PING to connection {}", id);
                    }
                });
            } else {
                error!("Connection {} was ready but has no write channel", id);
            }
        }

        while tasks.next().await.is_some() {}
    }

    /// Sends a WebSocket message and optionally waits for a reply.
    ///
    /// # Arguments
    ///
    /// * `payload` - The message payload to send
    /// * `id` - Optional request identifier, required when waiting for a reply
    /// * `wait_for_reply` - Whether to wait for a response to the message
    /// * `timeout` - Maximum duration to wait for a reply
    /// * `connection` - Optional specific WebSocket connection to use
    ///
    /// # Returns
    ///
    /// A receiver for the response if `wait_for_reply` is true, otherwise `None`
    ///
    /// # Errors
    ///
    /// Returns a `WebsocketError` if the connection is not ready or the send fails
    async fn send(
        &self,
        payload: String,
        id: Option<String>,
        wait_for_reply: bool,
        timeout: Duration,
        connection: Option<Arc<WebsocketConnection>>,
    ) -> Result<Option<oneshot::Receiver<Result<Value, WebsocketError>>>, WebsocketError> {
        let conn = if let Some(c) = connection {
            c
        } else {
            self.get_connection(false).await?
        };

        if !self.is_connected(Some(&conn)).await {
            warn!("Send attempted on a non-connected socket");
            return Err(WebsocketError::NotConnected);
        }

        let ws_write_tx = {
            let conn_state = conn.state.lock().await;
            conn_state
                .ws_write_tx
                .clone()
                .ok_or(WebsocketError::NotConnected)?
        };

        debug!("Sending message to WebSocket on connection {}", conn.id);

        ws_write_tx
            .send(Message::Text(payload.clone().into()))
            .map_err(|_| WebsocketError::NotConnected)?;

        if !wait_for_reply {
            return Ok(None);
        }

        let request_id = id.ok_or_else(|| {
            error!("id is required when waiting for a reply");
            WebsocketError::NotConnected
        })?;

        let (tx, rx) = oneshot::channel();
        {
            let mut conn_state = conn.state.lock().await;
            conn_state
                .pending_requests
                .insert(request_id.clone(), PendingRequest { completion: tx });
        }

        let conn_clone = Arc::clone(&conn);
        spawn(async move {
            sleep(timeout).await;
            let mut conn_state = conn_clone.state.lock().await;
            if let Some(pending_req) = conn_state.pending_requests.remove(&request_id) {
                let _ = pending_req.completion.send(Err(WebsocketError::Timeout));
            }
        });

        Ok(Some(rx))
    }
}

#[derive(Debug, Default, Clone)]
pub struct WebsocketMessageSendOptions {
    pub with_api_key: bool,
    pub is_signed: bool,
    pub is_session_logon: Option<bool>,
    pub is_session_logout: Option<bool>,
}

impl WebsocketMessageSendOptions {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_api_key(mut self) -> Self {
        self.with_api_key = true;
        self
    }

    #[must_use]
    pub fn signed(mut self) -> Self {
        self.is_signed = true;
        self
    }

    #[must_use]
    pub fn session_logon(mut self) -> Self {
        self.is_session_logon = Some(true);
        self
    }

    #[must_use]
    pub fn session_logout(mut self) -> Self {
        self.is_session_logout = Some(true);
        self
    }
}

#[derive(Debug)]
pub enum SendWebsocketMessageResult<R> {
    Single(WebsocketApiResponse<R>),
    Multiple(Vec<WebsocketApiResponse<R>>),
}

impl<R> IntoIterator for SendWebsocketMessageResult<R> {
    type Item = WebsocketApiResponse<R>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            SendWebsocketMessageResult::Single(resp) => vec![resp].into_iter(),
            SendWebsocketMessageResult::Multiple(v) => v.into_iter(),
        }
    }
}

pub struct WebsocketApi {
    pub common: Arc<WebsocketCommon>,
    configuration: ConfigurationWebsocketApi,
    is_connecting: Arc<Mutex<bool>>,
    stream_callbacks: Mutex<HashMap<String, Vec<Arc<dyn Fn(&Value) + Send + Sync + 'static>>>>,
}

impl WebsocketApi {
    #[must_use]
    /// Creates a new WebSocket API instance with the given configuration and connection pool.
    ///
    /// # Arguments
    ///
    /// * `configuration` - Configuration settings for the WebSocket API
    /// * `connection_pool` - A vector of WebSocket connections to be used
    ///
    /// # Returns
    ///
    /// An `Arc`-wrapped `WebsocketApi` instance ready for use
    ///
    /// # Panics
    ///
    /// This function will panic if the configuration is not valid.
    ///
    /// # Examples
    ///
    ///
    /// let api = `WebsocketApi::new(config`, `connection_pool`);
    ///
    pub fn new(
        configuration: ConfigurationWebsocketApi,
        connection_pool: Vec<Arc<WebsocketConnection>>,
    ) -> Arc<Self> {
        let agent_clone = configuration.agent.clone();
        let user_agent_clone = configuration.user_agent.clone();
        let common = WebsocketCommon::new(
            connection_pool,
            configuration.mode.clone(),
            usize::try_from(configuration.reconnect_delay)
                .expect("reconnect_delay should fit in usize"),
            agent_clone,
            Some(user_agent_clone),
        );

        Arc::new(Self {
            common: Arc::clone(&common),
            configuration,
            is_connecting: Arc::new(Mutex::new(false)),
            stream_callbacks: Mutex::new(HashMap::new()),
        })
    }

    /// Connects to a WebSocket server with a configurable timeout and connection handling.
    ///
    /// This method attempts to establish a WebSocket connection if not already connected.
    /// It prevents multiple simultaneous connection attempts and supports a connection pool.
    ///
    /// # Errors
    ///
    /// Returns a `WebsocketError` if:
    /// - Connection fails
    /// - Connection times out after 10 seconds
    ///
    /// # Behavior
    ///
    /// - Checks if already connected and returns early if so
    /// - Prevents multiple concurrent connection attempts
    /// - Sets a WebSocket handler for the connection pool
    /// - Attempts to connect with a 10-second timeout
    ///
    /// # Returns
    ///
    /// `Ok(())` if connection is successful, otherwise a `WebsocketError`
    pub async fn connect(self: Arc<Self>) -> Result<(), WebsocketError> {
        if self.common.is_connected(None).await {
            info!("WebSocket connection already established");
            return Ok(());
        }

        {
            let mut flag = self.is_connecting.lock().await;
            if *flag {
                info!("Already connecting...");
                return Ok(());
            }
            *flag = true;
        }

        let url = self.prepare_url(self.configuration.ws_url.as_deref().unwrap_or_default());

        let handler: Arc<dyn WebsocketHandler> = self.clone();
        for slot in &self.common.connection_pool {
            slot.set_handler(handler.clone()).await;
        }

        let result = select! {
            () = sleep(Duration::from_secs(10)) => Err(WebsocketError::Timeout),
            r = self.common.clone().connect_pool(&url) => r,
        };

        {
            let mut flag = self.is_connecting.lock().await;
            *flag = false;
        }

        result
    }

    /// Disconnects the WebSocket connection.
    ///
    /// # Returns
    ///
    /// `Ok(())` if disconnection is successful, otherwise a `WebsocketError`
    ///
    /// # Errors
    ///
    /// Returns a `WebsocketError` if:
    /// - Disconnection fails
    /// - Connection is not established
    ///
    pub async fn disconnect(&self) -> Result<(), WebsocketError> {
        self.common.disconnect().await
    }

    /// Checks if the WebSocket connection is currently established.
    ///
    /// # Returns
    ///
    /// `true` if the connection is active, `false` otherwise.
    pub async fn is_connected(&self) -> bool {
        self.common.is_connected(None).await
    }

    /// Sends a ping to the WebSocket server to maintain the connection.
    ///
    /// This method calls the underlying connection's ping mechanism to check
    /// and keep the WebSocket connection alive.
    pub async fn ping_server(&self) {
        self.common.ping_server().await;
    }

    /// Sends a WebSocket message with the specified method and payload.
    ///
    /// This method prepares and sends a WebSocket request with optional API key and signature.
    /// It handles connection status, generates a unique request ID, and processes the response.
    ///
    /// # Arguments
    ///
    /// * `method` - The WebSocket API method to be called
    /// * `payload` - A map of parameters to be sent with the request
    /// * `options` - Configuration options for message sending (API key, signing)
    ///
    /// # Returns
    ///
    /// A deserialized response of type `R` or a `WebsocketError` if the request fails
    ///
    /// # Panics
    ///
    /// Panics if:
    ///
    /// - The WebSocket is not connected
    /// - The request cannot be processed
    /// - No response is received within the timeout
    ///
    /// # Errors
    ///
    /// Returns `WebsocketError` if:
    /// - The WebSocket is not connected
    /// - The request cannot be processed
    /// - No response is received within the timeout
    pub async fn send_message<R>(
        &self,
        method: &str,
        payload: BTreeMap<String, Value>,
        options: WebsocketMessageSendOptions,
    ) -> Result<SendWebsocketMessageResult<R>, WebsocketError>
    where
        R: DeserializeOwned + Send + Sync + 'static,
    {
        if !self.common.is_connected(None).await {
            return Err(WebsocketError::NotConnected);
        }

        let do_multi =
            options.is_session_logon.unwrap_or(false) || options.is_session_logout.unwrap_or(false);

        let connections = if do_multi {
            self.common.get_available_connections(false).await
        } else {
            vec![self.common.get_connection(false).await?]
        };

        let skip_auth = if do_multi {
            false
        } else {
            let connection = &connections[0];
            let conn_state = connection.state.lock().await;
            self.configuration.auto_session_relogon && conn_state.is_session_logged_on
        };

        let payload_clone = payload.clone();

        let (id, request) =
            build_websocket_api_message(&self.configuration, method, payload, &options, skip_auth);
        let raw_payload = serde_json::to_string(&request).unwrap();
        debug!("Sending message to WebSocket API: {:?}", request);

        let timeout = Duration::from_millis(self.configuration.timeout);

        let mut receivers = Vec::with_capacity(connections.len());
        for connection in &connections {
            let opt_rx = self
                .common
                .send(
                    raw_payload.clone(),
                    Some(id.clone()),
                    true,
                    timeout,
                    Some(connection.clone()),
                )
                .await?;
            receivers.push((connection.clone(), opt_rx));
        }

        let mut raw_msgs = Vec::with_capacity(receivers.len());
        for (_conn, opt_rx) in receivers {
            let rx = opt_rx.ok_or(WebsocketError::NoResponse)?;
            let msg = rx.await.unwrap_or(Err(WebsocketError::Timeout))?;
            raw_msgs.push(msg);
        }

        let mut responses = Vec::with_capacity(raw_msgs.len());
        for msg in raw_msgs {
            let raw = msg
                .get("result")
                .or_else(|| msg.get("response"))
                .cloned()
                .unwrap_or(Value::Null);

            let rate_limits = msg
                .get("rateLimits")
                .and_then(Value::as_array)
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| serde_json::from_value(v.clone()).ok())
                        .collect()
                })
                .unwrap_or_default();

            responses.push(WebsocketApiResponse {
                raw,
                rate_limits,
                _marker: PhantomData,
            });
        }

        if do_multi && self.configuration.auto_session_relogon {
            for connection in &connections {
                let mut state = connection.state.lock().await;
                if options.is_session_logon.unwrap_or(false) {
                    state.is_session_logged_on = true;
                    state.session_logon_req = Some(WebsocketSessionLogonReq {
                        method: method.to_string(),
                        payload: payload_clone.clone(),
                        options: options.clone(),
                    });
                } else {
                    state.is_session_logged_on = false;
                    state.session_logon_req = None;
                }
            }
        }

        Ok(if responses.len() == 1 && !do_multi {
            SendWebsocketMessageResult::Single(responses.into_iter().next().unwrap())
        } else {
            SendWebsocketMessageResult::Multiple(responses)
        })
    }

    /// Prepares a WebSocket URL by appending a validated time unit parameter.
    ///
    /// This method checks if a time unit is configured and validates it. If valid,
    /// the time unit is appended to the URL as a query parameter. If no time unit
    /// is specified or the validation fails, the original URL is returned.
    ///
    /// # Arguments
    ///
    /// * `ws_url` - The base WebSocket URL to be modified
    ///
    /// # Returns
    ///
    /// A modified URL with the time unit parameter, or the original URL if no
    /// modification is possible
    fn prepare_url(&self, ws_url: &str) -> String {
        let mut url = ws_url.to_string();

        let time_unit = match &self.configuration.time_unit {
            Some(u) => u.to_string(),
            None => return url,
        };

        match validate_time_unit(&time_unit) {
            Ok(Some(validated)) => {
                let sep = if url.contains('?') { '&' } else { '?' };
                url.push(sep);
                url.push_str("timeUnit=");
                url.push_str(validated);
            }
            Ok(None) => {}
            Err(e) => {
                error!("Invalid time unit provided: {:?}", e);
            }
        }

        url
    }
}

#[async_trait]
impl WebsocketHandler for WebsocketApi {
    /// Handles the WebSocket connection opening event, attempting to re-establish a session logon if needed.
    ///
    /// This method checks if a session logon request exists and has not already been logged on.
    /// If conditions are met, it attempts to send a session re-logon message and update the connection state.
    ///
    /// # Arguments
    ///
    /// * `_url` - The WebSocket connection URL (unused)
    /// * `connection` - The WebSocket connection context
    ///
    /// # Behavior
    ///
    /// - Checks for an existing session logon request
    /// - Verifies the session is not already logged on
    /// - Attempts to send a re-logon message
    /// - Updates connection state upon successful re-logon
    /// - Logs errors if re-logon dispatch fails
    async fn on_open(&self, _url: String, connection: Arc<WebsocketConnection>) {
        let session_req = {
            let conn_state = connection.state.lock().await;
            conn_state.session_logon_req.clone()
        };

        let Some(req) = session_req else {
            return;
        };

        let already_logged_on = {
            let conn_state = connection.state.lock().await;
            conn_state.is_session_logged_on
        };

        if already_logged_on {
            debug!(
                "Connection {} already logged on, skipping re-logon",
                connection.id
            );
            return;
        }

        let conn = connection.clone();
        let common = Arc::clone(&self.common);
        let configuration = self.configuration.clone();
        let method = req.method.clone();
        let payload = req.payload.clone();
        let options = req.options.clone();

        spawn(async move {
            let (id, json_msg) =
                build_websocket_api_message(&configuration, &method, payload, &options, false);

            let raw_message = match serde_json::to_string(&json_msg) {
                Ok(msg) => msg,
                Err(e) => {
                    warn!(
                        "Failed to serialize session logon message for connection {}: {}",
                        conn.id, e
                    );
                    return;
                }
            };

            debug!(
                "Session re-logon on connection {}: {}",
                conn.id, raw_message
            );

            let rx = match common
                .send(
                    raw_message,
                    Some(id.clone()),
                    true,
                    Duration::from_millis(configuration.timeout),
                    Some(conn.clone()),
                )
                .await
            {
                Ok(Some(rx)) => rx,
                Ok(None) => {
                    warn!(
                        "Session re-logon dispatch returned None for connection {}",
                        conn.id
                    );
                    return;
                }
                Err(e) => {
                    warn!(
                        "Session re-logon dispatch failed on connection {}: {}",
                        conn.id, e
                    );
                    return;
                }
            };

            let Ok(result) = timeout(Duration::from_millis(configuration.timeout), rx).await else {
                warn!("Session re-logon timed out on connection {}", conn.id);
                return;
            };

            let final_result = match result {
                Ok(final_result) => final_result,
                Err(e) => {
                    warn!(
                        "Session re-logon receiver error on connection {}: {}",
                        conn.id, e
                    );
                    return;
                }
            };

            let payload = match final_result {
                Ok(payload) => payload,
                Err(e) => {
                    warn!(
                        "Session re-logon payload error on connection {}: {}",
                        conn.id, e
                    );
                    return;
                }
            };

            debug!(
                "Session re-logon succeeded on connection {}: {}",
                conn.id, payload
            );
            let mut conn_state = conn.state.lock().await;
            conn_state.is_session_logged_on = true;
        });
    }

    /// Handles incoming WebSocket messages by parsing the JSON payload and processing pending requests.
    ///
    /// This method is responsible for:
    /// - Parsing the received WebSocket message as JSON
    /// - Matching the message to a pending request by its ID
    /// - Sending the response back to the original request's completion channel
    /// - Handling both successful and error responses
    ///
    /// # Arguments
    ///
    /// * `data` - The raw WebSocket message as a string
    /// * `connection` - The WebSocket connection context associated with the message
    ///
    /// # Behavior
    ///
    /// - If message parsing fails, logs an error and returns
    /// - For known request IDs, sends the response to the corresponding completion channel
    /// - Warns about responses for unknown or timed-out requests
    /// - Differentiates between successful (status < 400) and error responses
    async fn on_message(&self, data: String, connection: Arc<WebsocketConnection>) {
        let msg: Value = match serde_json::from_str(&data) {
            Ok(v) => v,
            Err(err) => {
                error!("Failed to parse WebSocket message {} – {}", data, err);
                return;
            }
        };

        if let Some(id) = msg.get("id").and_then(Value::as_str) {
            let maybe_sender = {
                let mut conn_state = connection.state.lock().await;
                conn_state.pending_requests.remove(id)
            };

            if let Some(PendingRequest { completion }) = maybe_sender {
                connection.drain_notify.notify_one();
                let status = msg.get("status").and_then(Value::as_u64).unwrap_or(200);
                if status >= 400 {
                    let error_map = msg
                        .get("error")
                        .and_then(Value::as_object)
                        .unwrap_or(&serde_json::Map::new())
                        .clone();

                    let code = error_map
                        .get("code")
                        .and_then(Value::as_i64)
                        .unwrap_or(status.try_into().unwrap());

                    let message = error_map
                        .get("msg")
                        .and_then(Value::as_str)
                        .unwrap_or("Unknown error")
                        .to_string();

                    let _ = completion.send(Err(WebsocketError::ResponseError { code, message }));
                } else {
                    let _ = completion.send(Ok(msg.clone()));
                }
            }

            return;
        }

        if let Some(event) = msg.get("event") {
            if event.get("e").is_some() {
                for callbacks in self.stream_callbacks.lock().await.values() {
                    for callback in callbacks {
                        callback(event);
                    }
                }

                return;
            }
        }

        warn!(
            "Received response for unknown or timed-out request: {}",
            data
        );
    }

    /// Generates the URL to use for reconnecting to a WebSocket connection.
    ///
    /// # Arguments
    ///
    /// * `default_url` - The original URL to potentially modify for reconnection
    /// * `_connection` - The WebSocket connection context (currently unused)
    ///
    /// # Returns
    ///
    /// A `String` representing the URL to use for reconnecting
    async fn get_reconnect_url(
        &self,
        default_url: String,
        _connection: Arc<WebsocketConnection>,
    ) -> String {
        default_url
    }
}

pub struct WebsocketStreams {
    pub common: Arc<WebsocketCommon>,
    is_connecting: Mutex<bool>,
    connection_streams: Mutex<HashMap<String, Arc<WebsocketConnection>>>,
    configuration: ConfigurationWebsocketStreams,
}

impl WebsocketStreams {
    /// Creates a new `WebsocketStreams` instance with the given configuration and connection pool.
    ///
    /// # Arguments
    ///
    /// * `configuration` - Configuration settings for the WebSocket streams
    /// * `connection_pool` - A vector of WebSocket connections to use
    ///
    /// # Returns
    ///
    /// An `Arc`-wrapped `WebsocketStreams` instance
    ///
    /// # Panics
    ///
    /// Panics if the `reconnect_delay` cannot be converted to `usize`
    #[must_use]
    pub fn new(
        configuration: ConfigurationWebsocketStreams,
        connection_pool: Vec<Arc<WebsocketConnection>>,
    ) -> Arc<Self> {
        let agent_clone = configuration.agent.clone();
        let user_agent_clone = configuration.user_agent.clone();
        let common = WebsocketCommon::new(
            connection_pool,
            configuration.mode.clone(),
            usize::try_from(configuration.reconnect_delay)
                .expect("reconnect_delay should fit in usize"),
            agent_clone,
            Some(user_agent_clone),
        );
        Arc::new(Self {
            common,
            is_connecting: Mutex::new(false),
            connection_streams: Mutex::new(HashMap::new()),
            configuration,
        })
    }

    /// Establishes a WebSocket connection for the given streams.
    ///
    /// This method attempts to connect to a WebSocket server using the connection pool.
    /// If a connection is already established or in progress, it returns immediately.
    ///
    /// # Arguments
    ///
    /// * `streams` - A vector of stream identifiers to connect to
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the connection was successful or an error occurred
    ///
    /// # Errors
    ///
    /// Returns a `WebsocketError` if the connection fails or times out after 10 seconds
    pub async fn connect(self: Arc<Self>, streams: Vec<String>) -> Result<(), WebsocketError> {
        if self.common.is_connected(None).await {
            info!("WebSocket connection already established");
            return Ok(());
        }

        {
            let mut flag = self.is_connecting.lock().await;
            if *flag {
                info!("Already connecting...");
                return Ok(());
            }
            *flag = true;
        }

        let url = self.prepare_url(&streams);

        let handler: Arc<dyn WebsocketHandler> = self.clone();
        for conn in &self.common.connection_pool {
            conn.set_handler(handler.clone()).await;
        }

        let connect_res = select! {
            () = sleep(Duration::from_secs(10)) => Err(WebsocketError::Timeout),
            r = self.common.clone().connect_pool(&url) => r,
        };

        {
            let mut flag = self.is_connecting.lock().await;
            *flag = false;
        }

        connect_res
    }

    /// Disconnects all WebSocket connections and clears associated state.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the disconnection was successful or an error occurred
    ///
    /// # Errors
    ///
    /// Returns a `WebsocketError` if there are issues during the disconnection process
    ///
    /// # Side Effects
    ///
    /// - Clears stream callbacks for all connections
    /// - Clears pending subscriptions for all connections
    /// - Removes all connection stream mappings
    pub async fn disconnect(&self) -> Result<(), WebsocketError> {
        for connection in &self.common.connection_pool {
            let mut conn_state = connection.state.lock().await;
            conn_state.stream_callbacks.clear();
            conn_state.pending_subscriptions.clear();
        }
        self.connection_streams.lock().await.clear();
        self.common.disconnect().await
    }

    /// Checks if the WebSocket connection is currently active.
    ///
    /// # Returns
    ///
    /// `true` if the WebSocket connection is established, `false` otherwise.
    pub async fn is_connected(&self) -> bool {
        self.common.is_connected(None).await
    }

    /// Sends a ping to the WebSocket server to maintain the connection.
    ///
    /// This method delegates the ping operation to the underlying common WebSocket connection.
    /// It is typically used to keep the connection alive and check its status.
    ///
    /// # Side Effects
    ///
    /// Sends a ping request to the WebSocket server through the common connection.
    pub async fn ping_server(&self) {
        self.common.ping_server().await;
    }

    /// Subscribes to multiple WebSocket streams, handling connection and queuing logic.
    ///
    /// # Arguments
    ///
    /// * `streams` - A vector of stream names to subscribe to
    /// * `id` - An optional request identifier for the subscription
    ///
    /// # Behavior
    ///
    /// - Filters out streams already subscribed
    /// - Assigns streams to appropriate connections
    /// - Handles subscription for active connections
    /// - Queues subscriptions for inactive connections
    ///
    /// # Side Effects
    ///
    /// - Sends subscription payloads for active connections
    /// - Adds pending subscriptions for inactive connections
    pub async fn subscribe(self: Arc<Self>, streams: Vec<String>, id: Option<String>) {
        let streams: Vec<String> = {
            let map = self.connection_streams.lock().await;
            streams
                .into_iter()
                .filter(|s| !map.contains_key(s))
                .collect()
        };
        let connection_streams = self.handle_stream_assignment(streams).await;

        for (conn, streams) in connection_streams {
            if !self.common.is_connected(Some(&conn)).await {
                info!(
                    "Connection is not ready. Queuing subscription for streams: {:?}",
                    streams
                );
                let mut conn_state = conn.state.lock().await;
                conn_state.pending_subscriptions.extend(streams.clone());
                continue;
            }
            self.send_subscription_payload(&conn, &streams, id.clone());
        }
    }

    /// Unsubscribes from specified WebSocket streams.
    ///
    /// # Arguments
    ///
    /// * `streams` - A vector of stream names to unsubscribe from
    /// * `id` - An optional request identifier for the unsubscription
    ///
    /// # Behavior
    ///
    /// - Validates the request identifier or generates a random one
    /// - Checks for active connections and subscribed streams
    /// - Sends unsubscribe payload for streams with active callbacks
    /// - Removes stream from connection streams and callbacks
    ///
    /// # Side Effects
    ///
    /// - Sends unsubscribe request to WebSocket server
    /// - Removes stream tracking from internal state
    ///
    /// # Async
    ///
    /// This method is asynchronous and requires `.await` when called
    ///
    /// # Panics
    ///
    /// This method may panic if the request identifier is not valid.
    ///
    pub async fn unsubscribe(&self, streams: Vec<String>, id: Option<String>) {
        let request_id = id
            .filter(|s| ID_REGEX.is_match(s))
            .unwrap_or_else(random_string);

        for stream in streams {
            let maybe_conn = { self.connection_streams.lock().await.get(&stream).cloned() };

            let conn = if let Some(c) = maybe_conn {
                if !self.common.is_connected(Some(&c)).await {
                    warn!(
                        "Stream {} not associated with an active connection.",
                        stream
                    );
                    continue;
                }
                c
            } else {
                warn!("Stream {} was not subscribed.", stream);
                continue;
            };

            let callbacks = {
                let conn_state = conn.state.lock().await;
                conn_state
                    .stream_callbacks
                    .get(&stream)
                    .is_none_or(std::vec::Vec::is_empty)
            };

            if !callbacks {
                continue;
            }

            let payload = json!({
                "method": "UNSUBSCRIBE",
                "params": [stream.clone()],
                "id": request_id,
            });

            info!("UNSUBSCRIBE → {:?}", payload);

            let common = Arc::clone(&self.common);
            let conn_clone = Arc::clone(&conn);
            let msg = serde_json::to_string(&payload).unwrap();
            spawn(async move {
                let _ = common
                    .send(msg, None, false, Duration::ZERO, Some(conn_clone))
                    .await;
            });

            {
                let mut connection_streams = self.connection_streams.lock().await;
                connection_streams.remove(&stream);
            }
            {
                let mut conn_state = conn.state.lock().await;
                conn_state.stream_callbacks.remove(&stream);
            }
        }
    }

    /// Checks if a specific stream is currently subscribed.
    ///
    /// # Arguments
    ///
    /// * `stream` - The stream identifier to check for subscription status
    ///
    /// # Returns
    ///
    /// `true` if the stream is subscribed, `false` otherwise
    ///
    /// # Async
    ///
    /// This method is asynchronous and requires `.await` when called
    pub async fn is_subscribed(&self, stream: &str) -> bool {
        self.connection_streams.lock().await.contains_key(stream)
    }

    /// Prepares a WebSocket URL for streaming with optional stream names and time unit configuration.
    ///
    /// # Arguments
    ///
    /// * `streams` - A slice of stream names to be included in the URL
    ///
    /// # Returns
    ///
    /// A fully constructed WebSocket URL with optional stream and time unit parameters
    ///
    /// # Notes
    ///
    /// - If no time unit is specified, the base URL is returned
    /// - Validates and appends the time unit parameter if provided and valid
    /// - Handles URL parameter separator based on existing query parameters
    fn prepare_url(&self, streams: &[String]) -> String {
        let mut url = format!(
            "{}/stream?streams={}",
            self.configuration.ws_url.as_deref().unwrap_or(""),
            streams.join("/")
        );

        let time_unit = match &self.configuration.time_unit {
            Some(u) => u.to_string(),
            None => return url,
        };

        match validate_time_unit(&time_unit) {
            Ok(Some(validated)) => {
                let sep = if url.contains('?') { '&' } else { '?' };
                url.push(sep);
                url.push_str("timeUnit=");
                url.push_str(validated);
            }
            Ok(None) => {}
            Err(e) => {
                error!("Invalid time unit provided: {:?}", e);
            }
        }

        url
    }

    /// Handles stream assignment by finding or creating WebSocket connections for a list of streams.
    ///
    /// This method attempts to assign streams to existing WebSocket connections or creates new
    /// connections if needed. It groups streams by their assigned connections and handles scenarios
    /// such as closed or pending reconnection connections.
    ///
    /// # Arguments
    ///
    /// * `streams` - A vector of stream names to be assigned
    ///
    /// # Returns
    ///
    /// A vector of tuples containing WebSocket connections and their associated streams
    ///
    /// # Errors
    ///
    /// Returns an empty result if no connections can be established for the streams
    async fn handle_stream_assignment(
        &self,
        streams: Vec<String>,
    ) -> Vec<(Arc<WebsocketConnection>, Vec<String>)> {
        let mut connection_streams: Vec<(String, Arc<WebsocketConnection>)> = Vec::new();

        for stream in streams {
            let mut conn_opt = {
                let map = self.connection_streams.lock().await;
                map.get(&stream).cloned()
            };

            let need_new = if let Some(conn) = &conn_opt {
                let state = conn.state.lock().await;
                state.close_initiated || state.reconnection_pending
            } else {
                true
            };

            if need_new {
                match self.common.get_connection(true).await {
                    Ok(new_conn) => {
                        let mut map = self.connection_streams.lock().await;
                        map.insert(stream.clone(), new_conn.clone());
                        conn_opt = Some(new_conn);
                    }
                    Err(err) => {
                        warn!(
                            "No available WebSocket connection to subscribe stream `{}`: {:?}",
                            stream, err
                        );
                        continue;
                    }
                }
            }

            if let Some(conn) = conn_opt {
                {
                    let mut conn_state = conn.state.lock().await;
                    conn_state
                        .stream_callbacks
                        .entry(stream.clone())
                        .or_default();
                }
                connection_streams.push((stream.clone(), conn));
            }
        }

        let mut groups: Vec<(Arc<WebsocketConnection>, Vec<String>)> = Vec::new();
        for (stream, conn) in connection_streams {
            if let Some((_, vec)) = groups.iter_mut().find(|(c, _)| Arc::ptr_eq(c, &conn)) {
                vec.push(stream);
            } else {
                groups.push((conn, vec![stream]));
            }
        }

        groups
    }

    /// Sends a WebSocket subscription payload for the specified streams.
    ///
    /// # Arguments
    ///
    /// * `connection` - The WebSocket connection to send the subscription on
    /// * `streams` - A vector of stream names to subscribe to
    /// * `id` - An optional request ID for the subscription (will be randomly generated if not provided)
    ///
    /// # Remarks
    ///
    /// This method constructs a SUBSCRIBE payload, logs it, and sends it asynchronously using the WebSocket connection.
    /// If serialization fails, an error is logged and the method returns without sending.
    fn send_subscription_payload(
        &self,
        connection: &Arc<WebsocketConnection>,
        streams: &Vec<String>,
        id: Option<String>,
    ) {
        let request_id = id
            .filter(|s| ID_REGEX.is_match(s))
            .unwrap_or_else(random_string);

        let payload = json!({
            "method": "SUBSCRIBE",
            "params": streams,
            "id": request_id,
        });

        info!("SUBSCRIBE → {:?}", payload);

        let common = Arc::clone(&self.common);
        let msg = match serde_json::to_string(&payload) {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to serialize SUBSCRIBE payload: {}", e);
                return;
            }
        };
        let conn_clone = Arc::clone(connection);

        spawn(async move {
            let _ = common
                .send(msg, None, false, Duration::ZERO, Some(conn_clone))
                .await;
        });
    }
}

#[async_trait]
impl WebsocketHandler for WebsocketStreams {
    /// Handles the WebSocket connection opening by processing any pending subscriptions.
    ///
    /// This method is called when a WebSocket connection is established. It retrieves
    /// any pending stream subscriptions from the connection state and sends them
    /// immediately using the `send_subscription_payload` method.
    ///
    /// # Arguments
    ///
    /// * `_url` - The URL of the WebSocket connection (unused)
    /// * `connection` - The WebSocket connection that has just been opened
    ///
    /// # Remarks
    ///
    /// If there are any pending subscriptions, they are sent as a batch subscription
    /// payload. The method uses a lock to safely access and clear the pending subscriptions
    /// from the connection state.
    async fn on_open(&self, _url: String, connection: Arc<WebsocketConnection>) {
        let pending_subs: Vec<String> = {
            let mut conn_state = connection.state.lock().await;
            take(&mut conn_state.pending_subscriptions)
                .into_iter()
                .collect()
        };

        if !pending_subs.is_empty() {
            info!("Processing queued subscriptions for connection");
            self.send_subscription_payload(&connection, &pending_subs, None);
        }
    }

    /// Handles incoming WebSocket stream messages by parsing the JSON payload and invoking registered stream callbacks.
    ///
    /// This method processes WebSocket messages with a specific structure, extracting the stream name and data.
    /// It retrieves and executes any registered callbacks associated with the stream name.
    ///
    /// # Arguments
    ///
    /// * `data` - The raw WebSocket message as a JSON-formatted string
    /// * `connection` - The WebSocket connection through which the message was received
    ///
    /// # Behavior
    ///
    /// - Parses the JSON message
    /// - Extracts the stream name and data payload
    /// - Looks up and invokes any registered callbacks for the stream
    /// - Silently returns if message parsing or stream extraction fails
    async fn on_message(&self, data: String, connection: Arc<WebsocketConnection>) {
        let msg: Value = match serde_json::from_str(&data) {
            Ok(v) => v,
            Err(err) => {
                error!(
                    "Failed to parse WebSocket stream message {} – {}",
                    data, err
                );
                return;
            }
        };

        let (stream_name, payload) = match (
            msg.get("stream").and_then(Value::as_str),
            msg.get("data").cloned(),
        ) {
            (Some(name), Some(data)) => (name.to_string(), data),
            _ => return,
        };

        let callbacks = {
            let conn_state = connection.state.lock().await;
            conn_state
                .stream_callbacks
                .get(&stream_name)
                .cloned()
                .unwrap_or_else(Vec::new)
        };

        for callback in callbacks {
            callback(&payload);
        }
    }

    /// Retrieves the reconnection URL for a specific WebSocket connection by identifying all streams associated with that connection.
    ///
    /// # Arguments
    ///
    /// * `_default_url` - A default URL that can be used if no specific reconnection URL is determined
    /// * `connection` - The WebSocket connection for which to generate a reconnection URL
    ///
    /// # Returns
    ///
    /// A URL string that can be used to reconnect to the WebSocket, based on the streams associated with the given connection
    async fn get_reconnect_url(
        &self,
        _default_url: String,
        connection: Arc<WebsocketConnection>,
    ) -> String {
        let connection_streams = self.connection_streams.lock().await;
        let reconnect_streams = connection_streams
            .iter()
            .filter_map(|(stream, conn_arc)| {
                if Arc::ptr_eq(conn_arc, &connection) {
                    Some(stream.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        self.prepare_url(&reconnect_streams)
    }
}

pub struct WebsocketStream<T> {
    websocket_base: WebsocketBase,
    stream_or_id: String,
    callback: Mutex<Option<Arc<dyn Fn(&Value) + Send + Sync>>>,
    pub id: Option<String>,
    _phantom: PhantomData<T>,
}

impl<T> WebsocketStream<T>
where
    T: DeserializeOwned + Send + 'static,
{
    /// Registers a callback function for a specific event on the WebSocket stream.
    ///
    /// This method currently only supports the "message" event. When a message is received,
    /// the provided callback function will be invoked with the deserialized payload.
    ///
    /// # Arguments
    ///
    /// * `event` - The event type to listen for (currently only "message" is supported)
    /// * `callback_fn` - A function that will be called with the deserialized message payload
    ///
    /// # Errors
    ///
    /// Logs an error if the payload cannot be deserialized into the expected type
    ///
    /// # Examples
    ///
    ///
    /// stream.on("message", |data: `MyType`| {
    ///     // Handle the deserialized message
    /// });
    async fn on<F>(&self, event: &str, callback_fn: F)
    where
        F: Fn(T) + Send + Sync + 'static,
    {
        if event != "message" {
            return;
        }

        let cb_wrapper: Arc<dyn Fn(&Value) + Send + Sync> =
            Arc::new(
                move |v: &Value| match serde_json::from_value::<T>(v.clone()) {
                    Ok(data) => callback_fn(data),
                    Err(e) => error!("Failed to deserialize stream payload: {:?}", e),
                },
            );

        {
            let mut guard = self.callback.lock().await;
            *guard = Some(cb_wrapper.clone());
        }

        match &self.websocket_base {
            WebsocketBase::WebsocketStreams(ws_streams) => {
                let conn = {
                    let map = ws_streams.connection_streams.lock().await;
                    map.get(&self.stream_or_id)
                        .cloned()
                        .expect("stream must be subscribed")
                };

                {
                    let mut conn_state = conn.state.lock().await;
                    let entry = conn_state
                        .stream_callbacks
                        .entry(self.stream_or_id.clone())
                        .or_default();

                    if !entry
                        .iter()
                        .any(|existing| Arc::ptr_eq(existing, &cb_wrapper))
                    {
                        entry.push(cb_wrapper);
                    }
                }
            }
            WebsocketBase::WebsocketApi(ws_api) => {
                let mut stream_callbacks = ws_api.stream_callbacks.lock().await;
                let entry = stream_callbacks
                    .entry(self.stream_or_id.clone())
                    .or_default();

                if !entry
                    .iter()
                    .any(|existing| Arc::ptr_eq(existing, &cb_wrapper))
                {
                    entry.push(cb_wrapper);
                }
            }
        }
    }

    /// Synchronously sets a message callback for the WebSocket stream on the current thread.
    ///
    /// # Arguments
    ///
    /// * `callback_fn` - A function that will be called with the deserialized message payload
    ///
    /// # Panics
    ///
    /// Panics if the thread runtime fails to be created or if the thread join fails
    ///
    /// # Examples
    ///
    ///
    /// let stream = `Arc::new(WebsocketStream::new())`;
    /// `stream.on_message(|data`: `MyType`| {
    ///     // Handle the deserialized message
    /// });
    ///
    pub fn on_message<F>(self: &Arc<Self>, callback_fn: F)
    where
        T: Send + Sync,
        F: Fn(T) + Send + Sync + 'static,
    {
        let handler: Arc<Self> = Arc::clone(self);

        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("failed to build Tokio runtime");

            rt.block_on(handler.on("message", callback_fn));
        })
        .join()
        .expect("on_message thread panicked");
    }

    /// Unsubscribes from the current WebSocket stream and removes the associated callback.
    ///
    /// This method performs the following actions:
    /// - Removes the current callback associated with the stream
    /// - Removes the callback from the connection's stream callbacks
    /// - Asynchronously unsubscribes from the stream using the WebSocket streams base
    ///
    /// # Panics
    ///
    /// Panics if the stream is not subscribed to
    ///
    /// # Notes
    /// - If no callback is present, no action is taken
    /// - Spawns an asynchronous task to handle the unsubscription process
    pub async fn unsubscribe(&self) {
        let maybe_cb = {
            let mut guard = self.callback.lock().await;
            guard.take()
        };

        if let Some(cb) = maybe_cb {
            match &self.websocket_base {
                WebsocketBase::WebsocketStreams(ws_streams) => {
                    let conn = {
                        let map = ws_streams.connection_streams.lock().await;
                        map.get(&self.stream_or_id)
                            .cloned()
                            .expect("stream must have been subscribed")
                    };

                    {
                        let mut conn_state = conn.state.lock().await;
                        if let Some(list) = conn_state.stream_callbacks.get_mut(&self.stream_or_id)
                        {
                            list.retain(|existing| !Arc::ptr_eq(existing, &cb));
                        }
                    }

                    let stream = self.stream_or_id.clone();
                    let id = self.id.clone();
                    let websocket_streams_base = Arc::clone(ws_streams);
                    spawn(async move {
                        websocket_streams_base.unsubscribe(vec![stream], id).await;
                    });
                }
                WebsocketBase::WebsocketApi(ws_api) => {
                    let mut stream_callbacks = ws_api.stream_callbacks.lock().await;
                    if let Some(list) = stream_callbacks.get_mut(&self.stream_or_id) {
                        list.retain(|existing| !Arc::ptr_eq(existing, &cb));
                    }
                }
            }
        }
    }
}

pub async fn create_stream_handler<T>(
    websocket_base: WebsocketBase,
    stream_or_id: String,
    id: Option<String>,
) -> Arc<WebsocketStream<T>>
where
    T: DeserializeOwned + Send + 'static,
{
    match &websocket_base {
        WebsocketBase::WebsocketStreams(ws_streams) => {
            ws_streams
                .clone()
                .subscribe(vec![stream_or_id.clone()], id.clone())
                .await;
        }
        WebsocketBase::WebsocketApi(_) => {}
    }

    Arc::new(WebsocketStream {
        websocket_base,
        stream_or_id,
        id,
        callback: Mutex::new(None),
        _phantom: PhantomData,
    })
}

#[cfg(test)]
mod tests {
    use crate::TOKIO_SHARED_RT;
    use crate::common::utils::{SignatureGenerator, build_user_agent};
    use crate::common::websocket::{
        PendingRequest, ReconnectEntry, SendWebsocketMessageResult, WebsocketApi, WebsocketBase,
        WebsocketCommon, WebsocketConnection, WebsocketEvent, WebsocketEventEmitter,
        WebsocketHandler, WebsocketMessageSendOptions, WebsocketMode, WebsocketSessionLogonReq,
        WebsocketStream, WebsocketStreams, create_stream_handler,
    };
    use crate::config::{ConfigurationWebsocketApi, ConfigurationWebsocketStreams, PrivateKey};
    use crate::errors::{WebsocketConnectionFailureReason, WebsocketError};
    use crate::models::TimeUnit;
    use async_trait::async_trait;
    use futures::{SinkExt, StreamExt};
    use http::header::USER_AGENT;
    use regex::Regex;
    use serde_json::{Value, json};
    use std::collections::{BTreeMap, HashSet};
    use std::marker::PhantomData;
    use std::net::SocketAddr;
    use std::sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    };
    use tokio::net::TcpListener;
    use tokio::sync::{
        Mutex,
        mpsc::{Receiver, unbounded_channel},
        oneshot,
    };
    use tokio::time::{Duration, advance, pause, resume, sleep, timeout};
    use tokio_tungstenite::{accept_async, accept_hdr_async, tungstenite, tungstenite::Message};
    use tungstenite::handshake::server::Request;

    fn subscribe_events(common: &WebsocketCommon) -> Arc<Mutex<Vec<WebsocketEvent>>> {
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events.clone();
        common.events.subscribe(move |event| {
            let events_clone = events_clone.clone();
            tokio::spawn(async move {
                events_clone.lock().await.push(event);
            });
        });
        events
    }

    async fn create_connection(
        id: &str,
        has_writer: bool,
        reconnection_pending: bool,
        renewal_pending: bool,
        close_initiated: bool,
    ) -> Arc<WebsocketConnection> {
        let conn = WebsocketConnection::new(id);
        let mut st = conn.state.lock().await;
        st.reconnection_pending = reconnection_pending;
        st.renewal_pending = renewal_pending;
        st.close_initiated = close_initiated;
        if has_writer {
            let (tx, _) = unbounded_channel::<Message>();
            st.ws_write_tx = Some(tx);
        } else {
            st.ws_write_tx = None;
        }
        drop(st);
        conn
    }

    fn create_websocket_api(
        time_unit: Option<TimeUnit>,
        mode: Option<WebsocketMode>,
        auto_session_relogon: Option<bool>,
    ) -> Arc<WebsocketApi> {
        let mode = mode.unwrap_or(WebsocketMode::Single);
        let auto_session_relogon = auto_session_relogon.unwrap_or(true);
        let sig_gen = SignatureGenerator::new(
            Some("api_secret".into()),
            None::<PrivateKey>,
            None::<String>,
        );
        let config = ConfigurationWebsocketApi {
            api_key: Some("api_key".into()),
            api_secret: Some("api_secret".into()),
            private_key: None,
            private_key_passphrase: None,
            ws_url: Some("wss://example.com".into()),
            mode,
            reconnect_delay: 1000,
            signature_gen: sig_gen,
            timeout: 500,
            time_unit,
            auto_session_relogon,
            agent: None,
            user_agent: build_user_agent("product"),
        };
        let conn1 = WebsocketConnection::new("c1");
        let conn2 = WebsocketConnection::new("c2");
        WebsocketApi::new(config, vec![conn1, conn2])
    }

    fn create_websocket_streams(
        ws_url: Option<&str>,
        conns: Option<Vec<Arc<WebsocketConnection>>>,
    ) -> Arc<WebsocketStreams> {
        let mut connections: Vec<Arc<WebsocketConnection>> = vec![];
        if conns.is_none() {
            connections.push(WebsocketConnection::new("c1"));
            connections.push(WebsocketConnection::new("c2"));
        } else {
            connections = conns.expect("Expected connections to be set");
        }
        let config = ConfigurationWebsocketStreams {
            ws_url: Some(ws_url.unwrap_or("example.com").to_string()),
            mode: WebsocketMode::Single,
            reconnect_delay: 500,
            time_unit: None,
            agent: None,
            user_agent: build_user_agent("product"),
        };
        WebsocketStreams::new(config, connections)
    }

    fn subscribe_to_emitter(emitter: &WebsocketEventEmitter) -> Receiver<WebsocketEvent> {
        let (test_tx, test_rx) = tokio::sync::mpsc::channel(16);
        let _sub = emitter.subscribe(move |evt| {
            let _ = test_tx.try_send(evt);
        });
        test_rx
    }

    async fn expect_websocket_event(rx: &mut Receiver<WebsocketEvent>) -> WebsocketEvent {
        timeout(Duration::from_millis(200), rx.recv())
            .await
            .expect("timed out waiting for event")
            .expect("subscriber channel closed")
    }

    mod event_emitter {
        use super::*;

        #[test]
        fn event_emitter_subscribe_and_emit() {
            TOKIO_SHARED_RT.block_on(async {
                let emitter = WebsocketEventEmitter::new();
                let (tx, rx) = oneshot::channel();
                let tx = Arc::new(std::sync::Mutex::new(Some(tx)));
                let tx_clone = tx.clone();
                let _sub = emitter.subscribe(move |event| {
                    if let Some(sender) = tx_clone.lock().unwrap().take() {
                        let _ = sender.send(event);
                    }
                });
                emitter.emit(&WebsocketEvent::Open);
                let received = timeout(Duration::from_millis(100), rx)
                    .await
                    .expect("timed out");
                assert_eq!(received, Ok(WebsocketEvent::Open));
            });
        }

        #[test]
        fn single_subscriber_gets_event() {
            TOKIO_SHARED_RT.block_on(async {
                let emitter = WebsocketEventEmitter::new();
                let mut rx = subscribe_to_emitter(&emitter);

                let e1 = WebsocketEvent::Open;
                emitter.emit(&e1);

                let got = expect_websocket_event(&mut rx).await;
                assert_eq!(got, e1);
            });
        }

        #[test]
        fn multiple_subscribers_get_event() {
            TOKIO_SHARED_RT.block_on(async {
                let emitter = WebsocketEventEmitter::new();
                let mut rx1 = subscribe_to_emitter(&emitter);
                let mut rx2 = subscribe_to_emitter(&emitter);

                let e = WebsocketEvent::Message("hello".into());
                emitter.emit(&e);

                assert_eq!(expect_websocket_event(&mut rx1).await, e.clone());
                assert_eq!(expect_websocket_event(&mut rx2).await, e);
            });
        }

        #[test]
        fn closed_subscribers_are_pruned() {
            TOKIO_SHARED_RT.block_on(async {
                let emitter = WebsocketEventEmitter::new();
                let rx1 = subscribe_to_emitter(&emitter);
                let mut rx2 = subscribe_to_emitter(&emitter);
                drop(rx1);

                let e = WebsocketEvent::Pong;
                emitter.emit(&e);

                assert_eq!(expect_websocket_event(&mut rx2).await, e);
            });
        }

        #[test]
        fn prune_on_error_does_not_hang() {
            TOKIO_SHARED_RT.block_on(async {
                let emitter = WebsocketEventEmitter::new();
                let rx = subscribe_to_emitter(&emitter);
                drop(rx);

                let e = WebsocketEvent::Close(1000, "bye".into());
                emitter.emit(&e);
            });
        }
    }

    mod websocket_common {
        use super::*;

        mod initialisation {
            use super::*;

            #[test]
            fn single_mode() {
                TOKIO_SHARED_RT.block_on(async {
                    let common = WebsocketCommon::new(vec![], WebsocketMode::Single, 0, None, None);
                    assert_eq!(common.connection_pool.len(), 1);
                });
            }

            #[test]
            fn pool_mode() {
                TOKIO_SHARED_RT.block_on(async {
                    let common =
                        WebsocketCommon::new(vec![], WebsocketMode::Pool(3), 0, None, None);
                    assert_eq!(common.connection_pool.len(), 3);
                });
            }
        }

        mod spawn_reconnect_loop {
            use super::*;

            #[test]
            fn successful_reconnect_entry_triggers_init_connect() {
                TOKIO_SHARED_RT.block_on(async {
                    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
                    let addr = listener.local_addr().unwrap();
                    tokio::spawn(async move {
                        if let Ok((stream, _)) = listener.accept().await {
                            let mut ws = accept_async(stream).await.unwrap();
                            let _ = ws.close(None).await;
                        }
                    });

                    let conn = WebsocketConnection::new("c1");
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        10,
                        None,
                        None,
                    );
                    let url = format!("ws://{addr}");
                    common
                        .reconnect_tx
                        .send(ReconnectEntry {
                            connection_id: "c1".into(),
                            url: url.clone(),
                            is_renewal: false,
                        })
                        .await
                        .unwrap();

                    sleep(Duration::from_secs(2)).await;

                    let st = conn.state.lock().await;
                    assert!(st.ws_write_tx.is_some());
                });
            }

            #[test]
            fn reconnect_entry_with_unknown_id_is_ignored() {
                TOKIO_SHARED_RT.block_on(async {
                    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
                    let addr = listener.local_addr().unwrap();
                    tokio::spawn(async move {
                        if let Ok((stream, _)) = listener.accept().await {
                            let mut ws = accept_async(stream).await.unwrap();
                            let _ = ws.close(None).await;
                        }
                    });

                    let conn = WebsocketConnection::new("c1");
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        5,
                        None,
                        None,
                    );
                    let url = format!("ws://{addr}");
                    common
                        .reconnect_tx
                        .send(ReconnectEntry {
                            connection_id: "other".into(),
                            url,
                            is_renewal: false,
                        })
                        .await
                        .unwrap();

                    sleep(Duration::from_secs(1)).await;

                    let st = conn.state.lock().await;
                    assert!(st.ws_write_tx.is_none());
                });
            }

            #[test]
            fn renewal_entries_bypass_initial_delay() {
                TOKIO_SHARED_RT.block_on(async {
                    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
                    let addr = listener.local_addr().unwrap();
                    tokio::spawn(async move {
                        if let Ok((stream, _)) = listener.accept().await {
                            let mut ws = accept_async(stream).await.unwrap();
                            let _ = ws.close(None).await;
                        }
                    });

                    let conn = WebsocketConnection::new("renew");
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        200,
                        None,
                        None,
                    );
                    let url = format!("ws://{addr}");
                    common
                        .reconnect_tx
                        .send(ReconnectEntry {
                            connection_id: "renew".into(),
                            url: url.clone(),
                            is_renewal: true,
                        })
                        .await
                        .unwrap();

                    sleep(Duration::from_secs(2)).await;

                    let st = conn.state.lock().await;

                    assert!(st.ws_write_tx.is_some());
                });
            }

            #[test]
            fn non_renewal_entries_respect_initial_delay() {
                TOKIO_SHARED_RT.block_on(async {
                    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
                    let addr = listener.local_addr().unwrap();
                    tokio::spawn(async move {
                        if let Ok((stream, _)) = listener.accept().await {
                            let mut ws = accept_async(stream).await.unwrap();
                            let _ = ws.close(None).await;
                        }
                    });

                    let conn = WebsocketConnection::new("nonrenew");
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        200,
                        None,
                        None,
                    );
                    let url = format!("ws://{addr}");
                    common
                        .reconnect_tx
                        .send(ReconnectEntry {
                            connection_id: "nonrenew".into(),
                            url: url.clone(),
                            is_renewal: false,
                        })
                        .await
                        .unwrap();

                    sleep(Duration::from_millis(100)).await;
                    assert!(conn.state.lock().await.ws_write_tx.is_none());

                    sleep(Duration::from_secs(2)).await;

                    assert!(conn.state.lock().await.ws_write_tx.is_some());
                });
            }
        }

        mod spawn_renewal_loop {
            use super::*;

            #[tokio::test]
            async fn scheduling_renewal_does_not_panic_for_known_connection() {
                pause();

                let conn = WebsocketConnection::new("known");
                let common =
                    WebsocketCommon::new(vec![conn.clone()], WebsocketMode::Single, 0, None, None);
                let url = "wss://example".to_string();
                common
                    .renewal_tx
                    .send((conn.id.clone(), url))
                    .await
                    .unwrap();
                advance(Duration::from_secs(23 * 60 * 60 + 1)).await;
                resume();
            }

            #[tokio::test]
            async fn scheduling_renewal_ignored_for_unknown_connection() {
                pause();

                let conn = WebsocketConnection::new("c1");
                let common =
                    WebsocketCommon::new(vec![conn.clone()], WebsocketMode::Single, 0, None, None);
                common
                    .renewal_tx
                    .send(("other".into(), "u".into()))
                    .await
                    .unwrap();
                advance(Duration::from_secs(23 * 60 * 60 + 1)).await;

                resume();
            }
        }

        mod is_connection_ready {
            use super::*;

            #[test]
            fn is_connection_ready() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("c1");
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    assert!(!common.is_connection_ready(&conn, false).await);
                    assert!(common.is_connection_ready(&conn, true).await);
                });
            }

            #[test]
            fn connection_ready_basic() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = create_connection("c1", true, false, false, false).await;
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    assert!(common.is_connection_ready(&conn, false).await);
                });
            }

            #[test]
            fn connection_not_ready_without_writer() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = create_connection("c1", false, false, false, false).await;
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    assert!(!common.is_connection_ready(&conn, false).await);
                    assert!(common.is_connection_ready(&conn, true).await);
                });
            }

            #[test]
            fn connection_not_ready_when_flagged() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn1 = create_connection("c1", true, true, false, false).await;
                    let conn2 = create_connection("c2", true, false, true, false).await;
                    let conn3 = create_connection("c3", true, false, false, true).await;

                    let common = WebsocketCommon::new(
                        vec![conn1.clone(), conn2.clone(), conn3.clone()],
                        WebsocketMode::Pool(3),
                        0,
                        None,
                        None,
                    );

                    assert!(!common.is_connection_ready(&conn1, false).await);
                    assert!(common.is_connection_ready(&conn2, false).await);
                    assert!(!common.is_connection_ready(&conn3, false).await);
                });
            }
        }

        mod is_connected {
            use super::*;

            #[test]
            fn with_pool_various_connections() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn_a = create_connection("a", true, false, false, false).await;
                    let conn_b = create_connection("b", false, false, false, false).await;
                    let conn_c = create_connection("c", true, true, false, false).await;
                    let pool = vec![conn_a.clone(), conn_b.clone(), conn_c.clone()];
                    let common = WebsocketCommon::new(pool, WebsocketMode::Pool(3), 0, None, None);

                    assert!(common.is_connected(None).await);
                    assert!(common.is_connected(Some(&conn_a)).await);
                    assert!(!common.is_connected(Some(&conn_b)).await);
                    assert!(!common.is_connected(Some(&conn_c)).await);
                });
            }

            #[test]
            fn with_pool_all_bad_connections() {
                TOKIO_SHARED_RT.block_on(async {
                    let bad1 = create_connection("c1", false, false, false, false).await;
                    let bad2 = create_connection("c2", true, true, false, false).await;
                    let bad3 = create_connection("c3", true, false, false, true).await;
                    let common = WebsocketCommon::new(
                        vec![bad1, bad2, bad3],
                        WebsocketMode::Pool(3),
                        0,
                        None,
                        None,
                    );

                    assert!(!common.is_connected(None).await);
                });
            }

            #[test]
            fn with_pool_ignore_close_initiated() {
                TOKIO_SHARED_RT.block_on(async {
                    let good = create_connection("c1", true, false, false, false).await;
                    let closed = create_connection("c2", true, false, false, true).await;
                    let bad = create_connection("c3", false, false, false, false).await;
                    let common = WebsocketCommon::new(
                        vec![closed.clone(), good.clone(), bad.clone()],
                        WebsocketMode::Pool(3),
                        0,
                        None,
                        None,
                    );

                    assert!(common.is_connected(None).await);
                    assert!(!common.is_connected(Some(&closed)).await);
                });
            }
        }

        mod get_available_connections_tests {
            use super::*;

            #[test]
            fn single_mode() {
                TOKIO_SHARED_RT.block_on(async {
                    let common = WebsocketCommon::new(vec![], WebsocketMode::Single, 0, None, None);
                    let connections = common.get_available_connections(false).await;
                    assert_eq!(connections[0].id, common.connection_pool[0].id);
                });
            }

            #[test]
            fn pool_mode_not_ready() {
                TOKIO_SHARED_RT.block_on(async {
                    let common =
                        WebsocketCommon::new(vec![], WebsocketMode::Pool(2), 0, None, None);
                    let connections = common.get_available_connections(false).await;
                    assert!(connections.is_empty());
                });
            }

            #[test]
            fn pool_mode_with_ready() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn1 = WebsocketConnection::new("c1");
                    let conn2 = WebsocketConnection::new("c2");
                    let (tx1, _rx1) = unbounded_channel();
                    {
                        let mut s1 = conn1.state.lock().await;
                        s1.ws_write_tx = Some(tx1);
                    }
                    let pool = vec![conn1.clone(), conn2.clone()];
                    let common = WebsocketCommon::new(pool, WebsocketMode::Pool(2), 0, None, None);
                    let connections = common.get_available_connections(false).await;
                    assert!(connections.len() == 1);
                });
            }
        }

        mod get_connection {
            use super::*;

            #[test]
            fn single_mode() {
                TOKIO_SHARED_RT.block_on(async {
                    let common = WebsocketCommon::new(vec![], WebsocketMode::Single, 0, None, None);
                    let conn = common
                        .get_connection(false)
                        .await
                        .expect("should get connection");
                    assert_eq!(conn.id, common.connection_pool[0].id);
                });
            }

            #[test]
            fn pool_mode_not_ready() {
                TOKIO_SHARED_RT.block_on(async {
                    let common =
                        WebsocketCommon::new(vec![], WebsocketMode::Pool(2), 0, None, None);
                    let result = common.get_connection(false).await;
                    assert!(matches!(
                        result,
                        Err(crate::errors::WebsocketError::NotConnected)
                    ));
                });
            }

            #[test]
            fn pool_mode_with_ready() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn1 = WebsocketConnection::new("c1");
                    let conn2 = WebsocketConnection::new("c2");
                    let (tx1, _rx1) = unbounded_channel();
                    {
                        let mut s1 = conn1.state.lock().await;
                        s1.ws_write_tx = Some(tx1);
                    }
                    let pool = vec![conn1.clone(), conn2.clone()];
                    let common = WebsocketCommon::new(pool, WebsocketMode::Pool(2), 0, None, None);
                    let result = common.get_connection(false).await;
                    assert!(result.is_ok());
                    let chosen = result.unwrap();
                    assert_eq!(chosen.id, conn1.id);
                });
            }
        }

        mod close_connection_gracefully {
            use super::*;

            #[tokio::test]
            async fn waits_for_pending_requests_then_closes() {
                pause();

                let conn = WebsocketConnection::new("c1");
                let (tx, mut rx) = unbounded_channel::<Message>();
                let (req_tx, _req_rx) = oneshot::channel();
                {
                    let mut st = conn.state.lock().await;
                    st.pending_requests
                        .insert("r".to_string(), PendingRequest { completion: req_tx });
                }
                let common =
                    WebsocketCommon::new(vec![conn.clone()], WebsocketMode::Single, 0, None, None);
                let close_fut = common.close_connection_gracefully(tx.clone(), conn.clone());
                advance(Duration::from_secs(1)).await;
                {
                    let mut st = conn.state.lock().await;
                    st.pending_requests.clear();
                }
                conn.drain_notify.notify_waiters();
                advance(Duration::from_secs(1)).await;
                close_fut.await.unwrap();
                match rx.try_recv() {
                    Ok(Message::Close(_)) => {}
                    other => panic!("expected Close, got {other:?}"),
                }

                resume();
            }

            #[tokio::test]
            async fn force_closes_after_timeout() {
                pause();

                let conn = WebsocketConnection::new("c2");
                let (tx, mut rx) = unbounded_channel::<Message>();
                let (req_tx, _req_rx) = oneshot::channel();
                {
                    let mut st = conn.state.lock().await;
                    st.pending_requests.insert(
                        "request_id".to_string(),
                        PendingRequest { completion: req_tx },
                    );
                }
                let common =
                    WebsocketCommon::new(vec![conn.clone()], WebsocketMode::Single, 0, None, None);
                let close_fut = common.close_connection_gracefully(tx.clone(), conn.clone());
                advance(Duration::from_secs(30)).await;
                close_fut.await.unwrap();
                match rx.try_recv() {
                    Ok(Message::Close(_)) => {}
                    other => panic!("expected Close on timeout, got {other:?}"),
                }

                resume();
            }
        }

        mod get_reconnect_url {
            use super::*;

            struct DummyHandler {
                url: String,
            }

            #[async_trait::async_trait]
            impl WebsocketHandler for DummyHandler {
                async fn on_open(&self, _url: String, _connection: Arc<WebsocketConnection>) {}
                async fn on_message(&self, _data: String, _connection: Arc<WebsocketConnection>) {}
                async fn get_reconnect_url(
                    &self,
                    _default_url: String,
                    _connection: Arc<WebsocketConnection>,
                ) -> String {
                    self.url.clone()
                }
            }

            #[test]
            fn returns_default_when_no_handler() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("c1");
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    let default = "wss://default".to_string();
                    let result = common.get_reconnect_url(&default, conn.clone()).await;
                    assert_eq!(result, default);
                });
            }

            #[test]
            fn returns_handler_url_when_set() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("c2");
                    let handler = Arc::new(DummyHandler {
                        url: "wss://custom".into(),
                    });
                    conn.set_handler(handler).await;
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    let default = "wss://default".to_string();
                    let result = common.get_reconnect_url(&default, conn.clone()).await;
                    assert_eq!(result, "wss://custom");
                });
            }
        }

        mod on_open {
            use super::*;

            struct DummyHandler {
                called: Arc<Mutex<bool>>,
                opened_url: Arc<Mutex<Option<String>>>,
            }

            #[async_trait]
            impl WebsocketHandler for DummyHandler {
                async fn on_open(&self, url: String, _connection: Arc<WebsocketConnection>) {
                    let mut flag = self.called.lock().await;
                    *flag = true;
                    let mut store = self.opened_url.lock().await;
                    *store = Some(url);
                }
                async fn on_message(&self, _data: String, _connection: Arc<WebsocketConnection>) {}
                async fn get_reconnect_url(
                    &self,
                    default_url: String,
                    _connection: Arc<WebsocketConnection>,
                ) -> String {
                    default_url
                }
            }

            #[test]
            fn emits_open_and_calls_handler() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("c1");
                    let called = Arc::new(Mutex::new(false));
                    let opened_url = Arc::new(Mutex::new(None));
                    let handler = Arc::new(DummyHandler {
                        called: called.clone(),
                        opened_url: opened_url.clone(),
                    });

                    conn.set_handler(handler.clone()).await;
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    let events = subscribe_events(&common);
                    common
                        .on_open("wss://example.com".into(), conn.clone(), None)
                        .await;

                    sleep(std::time::Duration::from_millis(10)).await;

                    let evs = events.lock().await;
                    assert!(evs.iter().any(|e| matches!(e, WebsocketEvent::Open)));
                    assert!(*called.lock().await);
                    assert_eq!(
                        opened_url.lock().await.as_deref(),
                        Some("wss://example.com")
                    );
                });
            }

            #[test]
            fn handles_renewal_pending_and_closes_old_writer() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("c2");
                    let (old_tx, mut old_rx) = unbounded_channel::<Message>();
                    {
                        let mut st = conn.state.lock().await;
                        st.renewal_pending = true;
                    }
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    common
                        .on_open("url".into(), conn.clone(), Some(old_tx.clone()))
                        .await;
                    assert!(!conn.state.lock().await.renewal_pending);
                    match old_rx.try_recv() {
                        Ok(Message::Close(_)) => {}
                        other => panic!("expected Close, got {other:?}"),
                    }
                });
            }
        }

        mod on_message {
            use super::*;

            struct DummyHandler {
                called_with: Arc<Mutex<Vec<String>>>,
            }

            #[async_trait]
            impl WebsocketHandler for DummyHandler {
                async fn on_open(&self, _url: String, _connection: Arc<WebsocketConnection>) {}
                async fn on_message(&self, data: String, _connection: Arc<WebsocketConnection>) {
                    self.called_with.lock().await.push(data);
                }
                async fn get_reconnect_url(
                    &self,
                    default_url: String,
                    _connection: Arc<WebsocketConnection>,
                ) -> String {
                    default_url
                }
            }

            #[test]
            fn emits_message_event_without_handler() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("c1");
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    let events = subscribe_events(&common);
                    common.on_message("msg".into(), conn.clone()).await;

                    sleep(Duration::from_millis(10)).await;

                    let locked = events.lock().await;
                    assert!(
                        locked
                            .iter()
                            .any(|e| matches!(e, WebsocketEvent::Message(m) if m == "msg"))
                    );
                });
            }

            #[test]
            fn calls_handler_and_emits_message() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("c2");
                    let called = Arc::new(Mutex::new(Vec::new()));
                    let handler = Arc::new(DummyHandler {
                        called_with: called.clone(),
                    });
                    conn.set_handler(handler.clone()).await;

                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    let events = subscribe_events(&common);
                    common.on_message("msg".into(), conn.clone()).await;

                    sleep(Duration::from_millis(10)).await;

                    let evs = events.lock().await;
                    assert!(
                        evs.iter()
                            .any(|e| matches!(e, WebsocketEvent::Message(m) if m == "msg"))
                    );
                    let msgs = called.lock().await;
                    assert_eq!(msgs.as_slice(), &["msg".to_string()]);
                });
            }
        }

        mod create_websocket {
            use super::*;

            #[test]
            fn successful_connection() {
                TOKIO_SHARED_RT.block_on(async {
                    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
                    let addr: SocketAddr = listener.local_addr().unwrap();

                    let expected_ua = build_user_agent("product");
                    let expected_ua_clone = expected_ua.clone();

                    tokio::spawn(async move {
                        if let Ok((stream, _)) = listener.accept().await {
                            let callback = |req: &Request, resp| {
                                let got = req
                                    .headers()
                                    .get(USER_AGENT)
                                    .expect("no USER_AGENT header in WS handshake")
                                    .to_str()
                                    .expect("invalid USER_AGENT header");
                                assert_eq!(got, expected_ua_clone, "User-Agent mismatch");
                                Ok(resp)
                            };
                            let _ = accept_hdr_async(stream, callback).await.unwrap();
                        }
                    });

                    let url = format!("ws://{addr}");
                    let res =
                        WebsocketCommon::create_websocket(&url, None, Some(expected_ua)).await;
                    assert!(res.is_ok(), "handshake failed: {res:?}");
                });
            }

            #[test]
            fn invalid_url_returns_handshake_error() {
                TOKIO_SHARED_RT.block_on(async {
                    let res =
                        WebsocketCommon::create_websocket("not-a-valid-url", None, None).await;
                    assert!(matches!(res, Err(WebsocketError::Handshake(_))));
                });
            }

            #[test]
            fn unreachable_host_returns_handshake_error() {
                TOKIO_SHARED_RT.block_on(async {
                    let res =
                        WebsocketCommon::create_websocket("ws://127.0.0.1:1", None, None).await;
                    assert!(matches!(res, Err(WebsocketError::Handshake(_))));
                });
            }
        }

        mod connect_pool {
            use super::*;

            #[test]
            fn connects_all_in_pool() {
                TOKIO_SHARED_RT.block_on(async {
                    let pool_size = 3;
                    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
                    let addr = listener.local_addr().unwrap();
                    tokio::spawn(async move {
                        for _ in 0..pool_size {
                            if let Ok((stream, _)) = listener.accept().await {
                                let mut ws = accept_async(stream).await.unwrap();
                                let _ = ws.close(None).await;
                            }
                        }
                    });
                    let conns: Vec<Arc<WebsocketConnection>> = (0..pool_size)
                        .map(|i| WebsocketConnection::new(format!("c{i}")))
                        .collect();
                    let common = WebsocketCommon::new(
                        conns.clone(),
                        WebsocketMode::Pool(pool_size),
                        0,
                        None,
                        None,
                    );
                    let url = format!("ws://{addr}");
                    common.clone().connect_pool(&url).await.unwrap();
                    for conn in conns {
                        let st = conn.state.lock().await;
                        assert!(st.ws_write_tx.is_some());
                    }
                });
            }

            #[test]
            fn fails_if_any_refused() {
                TOKIO_SHARED_RT.block_on(async {
                    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
                    let addr = listener.local_addr().unwrap();
                    let pool_size = 3;
                    tokio::spawn(async move {
                        for _ in 0..2 {
                            if let Ok((stream, _)) = listener.accept().await {
                                let mut ws = accept_async(stream).await.unwrap();
                                let _ = ws.close(None).await;
                            }
                        }
                    });
                    let mut conns = Vec::new();
                    let valid_url = format!("ws://{addr}");
                    for i in 0..2 {
                        conns.push(WebsocketConnection::new(format!("c{i}")));
                    }
                    conns.push(WebsocketConnection::new("bad"));
                    let common = WebsocketCommon::new(
                        conns.clone(),
                        WebsocketMode::Pool(pool_size),
                        0,
                        None,
                        None,
                    );
                    let res = common.clone().connect_pool(&valid_url).await;
                    assert!(matches!(res, Err(WebsocketError::Handshake(_))));
                });
            }

            #[test]
            fn fails_on_invalid_url() {
                TOKIO_SHARED_RT.block_on(async {
                    let conns = vec![WebsocketConnection::new("c1")];
                    let common = WebsocketCommon::new(conns, WebsocketMode::Pool(1), 0, None, None);
                    let res = common.connect_pool("not-a-url").await;
                    assert!(matches!(res, Err(WebsocketError::Handshake(_))));
                });
            }

            #[test]
            fn fails_if_mixed_success_and_invalid_url() {
                TOKIO_SHARED_RT.block_on(async {
                    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
                    let addr = listener.local_addr().unwrap();
                    tokio::spawn(async move {
                        if let Ok((stream, _)) = listener.accept().await {
                            let mut ws = accept_async(stream).await.unwrap();
                            let _ = ws.close(None).await;
                        }
                    });
                    let good = WebsocketConnection::new("good");
                    let bad = WebsocketConnection::new("bad");
                    let common = WebsocketCommon::new(
                        vec![good, bad],
                        WebsocketMode::Pool(2),
                        0,
                        None,
                        None,
                    );
                    let url = format!("ws://{addr}");
                    let res = common.connect_pool(&url).await;
                    assert!(matches!(res, Err(WebsocketError::Handshake(_))));
                });
            }

            #[test]
            fn init_connect_invoked_for_each() {
                TOKIO_SHARED_RT.block_on(async {
                    let pool_size = 2;
                    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
                    let addr = listener.local_addr().unwrap();
                    tokio::spawn(async move {
                        for _ in 0..pool_size {
                            if let Ok((stream, _)) = listener.accept().await {
                                let mut ws = accept_async(stream).await.unwrap();
                                let _ = ws.close(None).await;
                            }
                        }
                    });
                    let conns: Vec<Arc<WebsocketConnection>> = (0..pool_size)
                        .map(|i| WebsocketConnection::new(format!("c{i}")))
                        .collect();
                    let common = WebsocketCommon::new(
                        conns.clone(),
                        WebsocketMode::Pool(pool_size),
                        0,
                        None,
                        None,
                    );
                    let url = format!("ws://{addr}");
                    common.clone().connect_pool(&url).await.unwrap();
                    for conn in conns {
                        let st = conn.state.lock().await;
                        assert!(st.ws_write_tx.is_some());
                    }
                });
            }

            #[test]
            fn single_mode_uses_first_connection() {
                TOKIO_SHARED_RT.block_on(async {
                    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
                    let addr = listener.local_addr().unwrap();
                    tokio::spawn(async move {
                        if let Ok((stream, _)) = listener.accept().await {
                            let mut ws = accept_async(stream).await.unwrap();
                            let _ = ws.close(None).await;
                        }
                    });
                    let conn = WebsocketConnection::new("c1");
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    let url = format!("ws://{addr}");
                    common.connect_pool(&url).await.unwrap();
                    let st = conn.state.lock().await;
                    assert!(st.ws_write_tx.is_some());
                });
            }
        }

        mod init_connect {
            use super::*;

            #[test]
            fn pool_mode_none_connection_uses_first() {
                TOKIO_SHARED_RT.block_on(async {
                    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
                    let addr = listener.local_addr().unwrap();
                    tokio::spawn(async move {
                        for _ in 0..2 {
                            if let Ok((stream, _)) = listener.accept().await {
                                let mut ws = accept_async(stream).await.unwrap();
                                ws.close(None).await.ok();
                            }
                        }
                    });

                    let c1 = WebsocketConnection::new("c1");
                    let c2 = WebsocketConnection::new("c2");
                    let common = WebsocketCommon::new(
                        vec![c1.clone(), c2.clone()],
                        WebsocketMode::Pool(2),
                        0,
                        None,
                        None,
                    );
                    let url = format!("ws://{addr}");

                    common
                        .clone()
                        .init_connect(&url, false, None)
                        .await
                        .unwrap();
                    let st1 = c1.state.lock().await;
                    let st2 = c2.state.lock().await;

                    assert!(st1.ws_write_tx.is_some());
                    assert!(st2.ws_write_tx.is_none());
                });
            }

            #[test]
            fn writer_channel_can_send_text() {
                TOKIO_SHARED_RT.block_on(async {
                    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
                    let addr = listener.local_addr().unwrap();
                    let received = Arc::new(Mutex::new(None::<String>));
                    let received_clone = received.clone();

                    tokio::spawn(async move {
                        if let Ok((stream, _)) = listener.accept().await {
                            let mut ws = accept_async(stream).await.unwrap();
                            if let Some(Ok(Message::Text(txt))) = ws.next().await {
                                *received_clone.lock().await = Some(txt.to_string());
                            }
                            ws.close(None).await.ok();
                        }
                    });

                    let conn = WebsocketConnection::new("cw");
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    let url = format!("ws://{addr}");
                    common
                        .clone()
                        .init_connect(&url, false, Some(conn.clone()))
                        .await
                        .unwrap();

                    let tx = conn.state.lock().await.ws_write_tx.clone().unwrap();
                    tx.send(Message::Text("ping".into())).unwrap();

                    sleep(Duration::from_millis(50)).await;

                    let lock = received.lock().await;
                    assert_eq!(lock.as_deref(), Some("ping"));
                });
            }

            #[test]
            fn responds_to_ping_with_pong() {
                TOKIO_SHARED_RT.block_on(async {
                    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
                    let addr = listener.local_addr().unwrap();

                    let saw_pong = Arc::new(Mutex::new(false));
                    let saw_pong2 = saw_pong.clone();

                    tokio::spawn(async move {
                        if let Ok((stream, _)) = listener.accept().await {
                            let mut ws = accept_async(stream).await.unwrap();
                            ws.send(Message::Ping(vec![1, 2, 3].into())).await.unwrap();
                            if let Some(Ok(Message::Pong(payload))) = ws.next().await {
                                if payload[..] == [1, 2, 3] {
                                    *saw_pong2.lock().await = true;
                                }
                            }
                            let _ = ws.close(None).await;
                        }
                    });

                    let conn = WebsocketConnection::new("c-ping");
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    let url = format!("ws://{addr}");
                    common
                        .clone()
                        .init_connect(&url, false, Some(conn))
                        .await
                        .unwrap();

                    sleep(Duration::from_millis(50)).await;

                    assert!(*saw_pong.lock().await, "server should have seen a Pong");
                });
            }

            #[test]
            fn handshake_error_on_invalid_url() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("c-invalid");
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    let res = common
                        .clone()
                        .init_connect("not-a-url", false, Some(conn.clone()))
                        .await;
                    assert!(matches!(res, Err(WebsocketError::Handshake(_))));
                });
            }

            #[test]
            fn skip_if_writer_exists_and_not_renewal() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("c-writer");
                    let (tx, mut rx) = unbounded_channel::<Message>();
                    {
                        let mut st = conn.state.lock().await;
                        st.ws_write_tx = Some(tx.clone());
                    }
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    let res = common
                        .clone()
                        .init_connect("ws://127.0.0.1:1", false, Some(conn.clone()))
                        .await;

                    assert!(res.is_ok());
                    assert!(rx.try_recv().is_err());
                });
            }

            #[test]
            fn short_circuit_on_already_renewing() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("c-renew");
                    {
                        let mut st = conn.state.lock().await;
                        st.renewal_pending = true;
                    }
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    let res = common
                        .clone()
                        .init_connect("ws://127.0.0.1:1", true, Some(conn.clone()))
                        .await;

                    assert!(res.is_ok());
                    assert!(conn.state.lock().await.ws_write_tx.is_none());
                });
            }

            #[test]
            fn is_renewal_true_sets_and_clears_flag() {
                TOKIO_SHARED_RT.block_on(async {
                    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
                    let addr = listener.local_addr().unwrap();
                    tokio::spawn(async move {
                        if let Ok((stream, _)) = listener.accept().await {
                            let mut ws = accept_async(stream).await.unwrap();
                            let _ = ws.close(None).await;
                        }
                    });

                    let conn = WebsocketConnection::new("c-new-renew");
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    let url = format!("ws://{addr}");
                    let res = common
                        .clone()
                        .init_connect(&url, true, Some(conn.clone()))
                        .await;

                    assert!(res.is_ok());

                    {
                        let st = conn.state.lock().await;
                        assert!(st.ws_write_tx.is_some(), "writer should be set");
                        assert!(
                            st.renewal_pending,
                            "renewal_pending must be true until on_open"
                        );
                    }

                    common.on_open(url.clone(), conn.clone(), None).await;

                    let st = conn.state.lock().await;
                    assert!(
                        !st.renewal_pending,
                        "renewal_pending should be cleared in on_open"
                    );
                });
            }

            #[test]
            fn default_connection_selected_when_none_passed() {
                TOKIO_SHARED_RT.block_on(async {
                    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
                    let addr = listener.local_addr().unwrap();
                    tokio::spawn(async move {
                        if let Ok((stream, _)) = listener.accept().await {
                            let mut ws = accept_async(stream).await.unwrap();
                            let _ = ws.close(None).await;
                        }
                    });
                    let conn = WebsocketConnection::new("c-default");
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    let url = format!("ws://{addr}");
                    let res = common.clone().init_connect(&url, false, None).await;

                    assert!(res.is_ok());
                    assert!(conn.state.lock().await.ws_write_tx.is_some());
                });
            }

            #[test]
            fn schedules_reconnect_on_abnormal_close() {
                TOKIO_SHARED_RT.block_on(async {
                    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
                    let addr = listener.local_addr().unwrap();
                    tokio::spawn(async move {
                        if let Ok((stream, _)) = listener.accept().await {
                            let mut ws = accept_async(stream).await.unwrap();
                            ws.close(Some(tungstenite::protocol::CloseFrame {
                                code: tungstenite::protocol::frame::coding::CloseCode::Abnormal,
                                reason: "oops".into(),
                            }))
                            .await
                            .ok();
                        }
                    });
                    let conn = WebsocketConnection::new("c-close");
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        10,
                        None,
                        None,
                    );
                    let url = format!("ws://{addr}");
                    common
                        .clone()
                        .init_connect(&url, false, Some(conn.clone()))
                        .await
                        .unwrap();

                    sleep(Duration::from_millis(50)).await;

                    let st = conn.state.lock().await;
                    assert!(
                        st.reconnection_pending,
                        "expected reconnection_pending to be true after abnormal close"
                    );
                });
            }
        }

        mod disconnect {
            use super::*;

            #[test]
            fn returns_ok_when_no_connections_are_ready() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("c1");
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    let res = common.disconnect().await;

                    assert!(res.is_ok());
                    assert!(!conn.state.lock().await.close_initiated);
                });
            }

            #[test]
            fn closes_all_ready_connections() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn1 = WebsocketConnection::new("c1");
                    let conn2 = WebsocketConnection::new("c2");
                    let (tx1, mut rx1) = unbounded_channel::<Message>();
                    let (tx2, mut rx2) = unbounded_channel::<Message>();
                    {
                        let mut s1 = conn1.state.lock().await;
                        s1.ws_write_tx = Some(tx1);
                    }
                    {
                        let mut s2 = conn2.state.lock().await;
                        s2.ws_write_tx = Some(tx2);
                    }
                    let common = WebsocketCommon::new(
                        vec![conn1.clone(), conn2.clone()],
                        WebsocketMode::Pool(2),
                        0,
                        None,
                        None,
                    );
                    let fut = common.disconnect();

                    sleep(Duration::from_millis(50)).await;

                    fut.await.unwrap();

                    assert!(conn1.state.lock().await.close_initiated);
                    assert!(conn2.state.lock().await.close_initiated);

                    {
                        let st = conn1.state.lock().await;
                        assert!(!st.is_session_logged_on, "conn1 should be logged out");
                        assert!(st.session_logon_req.is_none(), "conn1 req cleared");
                    }
                    {
                        let st = conn2.state.lock().await;
                        assert!(!st.is_session_logged_on, "conn2 should be logged out");
                        assert!(st.session_logon_req.is_none(), "conn2 req cleared");
                    }

                    match (rx1.try_recv(), rx2.try_recv()) {
                        (Ok(Message::Close(_)), Ok(Message::Close(_))) => {}
                        other => panic!("expected two Close frames, got {other:?}"),
                    }
                });
            }

            #[test]
            fn does_not_mark_close_initiated_if_no_writer() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("c-new");
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    common.disconnect().await.unwrap();

                    assert!(!conn.state.lock().await.close_initiated);
                });
            }

            #[test]
            fn mixed_pool_marks_all_and_closes_only_writers() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn_w = WebsocketConnection::new("with");
                    let conn_wo = WebsocketConnection::new("without");
                    let (tx, mut rx) = unbounded_channel::<Message>();
                    {
                        let mut st = conn_w.state.lock().await;
                        st.ws_write_tx = Some(tx);
                    }
                    let common = WebsocketCommon::new(
                        vec![conn_w.clone(), conn_wo.clone()],
                        WebsocketMode::Pool(2),
                        0,
                        None,
                        None,
                    );
                    let fut = common.disconnect();

                    sleep(Duration::from_millis(50)).await;

                    fut.await.unwrap();

                    assert!(conn_w.state.lock().await.close_initiated);
                    assert!(conn_wo.state.lock().await.close_initiated);
                    assert!(matches!(rx.try_recv(), Ok(Message::Close(_))));
                });
            }

            #[test]
            fn after_disconnect_not_connected() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("c1");
                    let (tx, mut _rx) = unbounded_channel::<Message>();
                    {
                        let mut st = conn.state.lock().await;
                        st.ws_write_tx = Some(tx);
                    }
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    common.disconnect().await.unwrap();
                    assert!(!common.is_connected(Some(&conn)).await);
                });
            }
        }

        mod ping_server {
            use super::*;

            #[test]
            fn sends_ping_to_all_ready_connections() {
                TOKIO_SHARED_RT.block_on(async {
                    let mut conns = Vec::new();
                    for i in 0..3 {
                        let conn = WebsocketConnection::new(format!("c{i}"));
                        let (tx, rx) = unbounded_channel::<Message>();
                        {
                            let mut st = conn.state.lock().await;
                            st.ws_write_tx = Some(tx);
                        }
                        conns.push((conn, rx));
                    }
                    let common = WebsocketCommon::new(
                        conns.iter().map(|(c, _)| c.clone()).collect(),
                        WebsocketMode::Pool(3),
                        0,
                        None,
                        None,
                    );
                    common.ping_server().await;
                    for (_, mut rx) in conns {
                        match rx.try_recv() {
                            Ok(Message::Ping(payload)) if payload.is_empty() => {}
                            other => panic!("expected empty-payload Ping, got {other:?}"),
                        }
                    }
                });
            }

            #[test]
            fn skips_not_ready_and_partial() {
                TOKIO_SHARED_RT.block_on(async {
                    let ready = WebsocketConnection::new("ready");
                    let not_ready = WebsocketConnection::new("not-ready");
                    let (tx_r, mut rx_r) = unbounded_channel::<Message>();
                    {
                        let mut st = ready.state.lock().await;
                        st.ws_write_tx = Some(tx_r);
                    }
                    {
                        let mut st = not_ready.state.lock().await;
                        st.ws_write_tx = None;
                    }
                    let common = WebsocketCommon::new(
                        vec![ready.clone(), not_ready.clone()],
                        WebsocketMode::Pool(2),
                        0,
                        None,
                        None,
                    );
                    common.ping_server().await;
                    match rx_r.try_recv() {
                        Ok(Message::Ping(payload)) if payload.is_empty() => {}
                        other => panic!("expected Ping on ready, got {other:?}"),
                    }
                });
            }

            #[test]
            fn no_ping_when_flags_block() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("c1");
                    let (tx, mut rx) = unbounded_channel::<Message>();
                    {
                        let mut st = conn.state.lock().await;
                        st.ws_write_tx = Some(tx);
                        st.reconnection_pending = true;
                    }
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    common.ping_server().await;
                    assert!(rx.try_recv().is_err());
                });
            }
        }

        mod send {
            use super::*;

            #[test]
            fn round_robin_send_without_specific() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn1 = WebsocketConnection::new("c1");
                    let conn2 = WebsocketConnection::new("c2");
                    let (tx1, mut rx1) = unbounded_channel::<Message>();
                    let (tx2, mut rx2) = unbounded_channel::<Message>();
                    {
                        let mut s1 = conn1.state.lock().await;
                        s1.ws_write_tx = Some(tx1);
                    }
                    {
                        let mut s2 = conn2.state.lock().await;
                        s2.ws_write_tx = Some(tx2);
                    }
                    let common = WebsocketCommon::new(
                        vec![conn1.clone(), conn2.clone()],
                        WebsocketMode::Pool(2),
                        0,
                        None,
                        None,
                    );

                    let res1 = common
                        .send("a".into(), None, false, Duration::from_secs(1), None)
                        .await
                        .unwrap();
                    assert!(res1.is_none());

                    let res2 = common
                        .send("b".into(), None, false, Duration::from_secs(1), None)
                        .await
                        .unwrap();
                    assert!(res2.is_none());

                    assert_eq!(
                        if let Message::Text(t) = rx1.try_recv().unwrap() {
                            t
                        } else {
                            panic!()
                        },
                        "a"
                    );
                    assert_eq!(
                        if let Message::Text(t) = rx2.try_recv().unwrap() {
                            t
                        } else {
                            panic!()
                        },
                        "b"
                    );
                });
            }

            #[test]
            fn round_robin_skips_not_ready() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn1 = WebsocketConnection::new("c1");
                    let conn2 = WebsocketConnection::new("c2");
                    let (tx2, mut rx2) = unbounded_channel::<Message>();
                    {
                        let mut s1 = conn1.state.lock().await;
                        s1.ws_write_tx = None;
                    }
                    {
                        let mut s2 = conn2.state.lock().await;
                        s2.ws_write_tx = Some(tx2);
                    }
                    let common = WebsocketCommon::new(
                        vec![conn1.clone(), conn2.clone()],
                        WebsocketMode::Pool(2),
                        0,
                        None,
                        None,
                    );
                    let res = common
                        .send("bar".into(), None, false, Duration::from_secs(1), None)
                        .await
                        .unwrap();
                    assert!(res.is_none());
                    match rx2.try_recv().unwrap() {
                        Message::Text(t) => assert_eq!(t, "bar"),
                        other => panic!("unexpected {other:?}"),
                    }
                });
            }

            #[test]
            fn sync_send_on_specific_connection() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn1 = WebsocketConnection::new("c1");
                    let conn2 = WebsocketConnection::new("c2");
                    let (tx2, mut rx2) = unbounded_channel::<Message>();
                    {
                        let mut st = conn2.state.lock().await;
                        st.ws_write_tx = Some(tx2);
                    }
                    let common = WebsocketCommon::new(
                        vec![conn1.clone(), conn2.clone()],
                        WebsocketMode::Pool(2),
                        0,
                        None,
                        None,
                    );
                    let res = common
                        .send(
                            "payload".into(),
                            Some("id".into()),
                            false,
                            Duration::from_secs(1),
                            Some(conn2.clone()),
                        )
                        .await
                        .unwrap();
                    assert!(res.is_none());
                    match rx2.try_recv() {
                        Ok(Message::Text(t)) => assert_eq!(t, "payload"),
                        other => panic!("expected Text, got {other:?}"),
                    }
                });
            }

            #[test]
            fn sync_send_with_id_does_not_insert_pending() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("c1");
                    let (tx, mut rx) = unbounded_channel::<Message>();
                    {
                        let mut st = conn.state.lock().await;
                        st.ws_write_tx = Some(tx);
                    }
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    let res = common
                        .send(
                            "msg".into(),
                            Some("id".into()),
                            false,
                            Duration::from_secs(1),
                            Some(conn.clone()),
                        )
                        .await
                        .unwrap();
                    assert!(res.is_none());
                    assert!(conn.state.lock().await.pending_requests.is_empty());
                    match rx.try_recv().unwrap() {
                        Message::Text(t) => assert_eq!(t, "msg"),
                        other => panic!("unexpected {other:?}"),
                    }
                });
            }

            #[test]
            fn sync_send_error_if_not_ready() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("c1");
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    let err = common
                        .send(
                            "msg".into(),
                            Some("id".into()),
                            false,
                            Duration::from_secs(1),
                            Some(conn.clone()),
                        )
                        .await
                        .unwrap_err();
                    assert!(matches!(err, WebsocketError::NotConnected));
                });
            }

            #[test]
            fn sync_send_error_when_no_ready() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("c1");
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    let err = common
                        .send("msg".into(), None, false, Duration::from_secs(1), None)
                        .await
                        .unwrap_err();
                    assert!(matches!(err, WebsocketError::NotConnected));
                });
            }

            #[test]
            fn async_send_and_receive() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("c1");
                    let (tx, mut rx) = unbounded_channel::<Message>();
                    {
                        let mut st = conn.state.lock().await;
                        st.ws_write_tx = Some(tx);
                    }
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    let fut = common
                        .send(
                            "hello".into(),
                            Some("id".into()),
                            true,
                            Duration::from_secs(5),
                            Some(conn.clone()),
                        )
                        .await
                        .unwrap()
                        .unwrap();
                    match rx.try_recv() {
                        Ok(Message::Text(t)) => assert_eq!(t, "hello"),
                        other => panic!("expected Text, got {other:?}"),
                    }
                    {
                        let mut st = conn.state.lock().await;
                        let pr = st.pending_requests.remove("id").unwrap();
                        pr.completion.send(Ok(serde_json::json!("ok"))).unwrap();
                    }
                    let resp = fut.await.unwrap().unwrap();
                    assert_eq!(resp, serde_json::json!("ok"));
                });
            }

            #[test]
            fn async_send_default_connection() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("c1");
                    let (tx, mut rx) = unbounded_channel::<Message>();
                    {
                        let mut st = conn.state.lock().await;
                        st.ws_write_tx = Some(tx);
                    }
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    let fut = common
                        .send(
                            "msg".into(),
                            Some("id".into()),
                            true,
                            Duration::from_secs(5),
                            None,
                        )
                        .await
                        .unwrap()
                        .unwrap();
                    match rx.try_recv() {
                        Ok(Message::Text(t)) => assert_eq!(t, "msg"),
                        _ => panic!("no text"),
                    }
                    {
                        let mut st = conn.state.lock().await;
                        let pr = st.pending_requests.remove("id").unwrap();
                        pr.completion.send(Ok(serde_json::json!(123))).unwrap();
                    }
                    let resp = fut.await.unwrap().unwrap();
                    assert_eq!(resp, serde_json::json!(123));
                });
            }

            #[test]
            fn async_send_error_if_no_id() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("c§");
                    let (tx, _rx) = unbounded_channel::<Message>();
                    {
                        let mut st = conn.state.lock().await;
                        st.ws_write_tx = Some(tx);
                    }
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    let err = common
                        .send(
                            "msg".into(),
                            None,
                            true,
                            Duration::from_secs(1),
                            Some(conn.clone()),
                        )
                        .await
                        .unwrap_err();
                    assert!(matches!(err, WebsocketError::NotConnected));
                });
            }

            #[test]
            fn timeout_rejects_async() {
                TOKIO_SHARED_RT.block_on(async {
                    pause();
                    let conn = WebsocketConnection::new("c1");
                    let (tx, _rx) = unbounded_channel::<Message>();
                    {
                        let mut st = conn.state.lock().await;
                        st.ws_write_tx = Some(tx);
                    }
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    let fut = common
                        .send(
                            "msg".into(),
                            Some("id".into()),
                            true,
                            Duration::from_secs(1),
                            Some(conn.clone()),
                        )
                        .await
                        .unwrap()
                        .unwrap();
                    advance(Duration::from_secs(1)).await;
                    let res = fut.await.unwrap();
                    assert!(res.is_err(), "expected timeout error");
                    assert!(!conn.state.lock().await.pending_requests.contains_key("id"));
                });
            }

            #[test]
            fn async_send_errors_if_no_connection_ready() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("c1");
                    let common = WebsocketCommon::new(
                        vec![conn.clone()],
                        WebsocketMode::Single,
                        0,
                        None,
                        None,
                    );
                    let err = common
                        .send(
                            "msg".into(),
                            Some("id".into()),
                            true,
                            Duration::from_secs(1),
                            None,
                        )
                        .await
                        .unwrap_err();
                    assert!(matches!(err, WebsocketError::NotConnected));
                });
            }
        }
    }

    mod websocket_api {
        use super::*;

        mod initialisation {
            use super::*;

            #[test]
            fn new_initializes_common() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("id");
                    let pool = vec![conn.clone()];

                    let sig_gen = SignatureGenerator::new(
                        Some("api_secret".to_string()),
                        None::<PrivateKey>,
                        None::<String>,
                    );

                    let config = ConfigurationWebsocketApi {
                        api_key: Some("api_key".to_string()),
                        api_secret: Some("api_secret".to_string()),
                        private_key: None,
                        private_key_passphrase: None,
                        ws_url: Some("wss://example".to_string()),
                        mode: WebsocketMode::Single,
                        reconnect_delay: 1000,
                        signature_gen: sig_gen,
                        timeout: 500,
                        time_unit: None,
                        auto_session_relogon: false,
                        agent: None,
                        user_agent: build_user_agent("product"),
                    };

                    let api = WebsocketApi::new(config, pool.clone());

                    assert_eq!(api.common.connection_pool.len(), 1);
                    assert_eq!(api.common.mode, WebsocketMode::Single);

                    let flag = *api.is_connecting.lock().await;
                    assert!(!flag);
                });
            }
        }

        mod connect {
            use super::*;

            #[test]
            fn connect_when_not_connected_establishes() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("id");
                    {
                        let mut st = conn.state.lock().await;
                        st.ws_write_tx = None;
                    }
                    let sig = SignatureGenerator::new(
                        Some("api_secret".into()),
                        None::<PrivateKey>,
                        None::<String>,
                    );
                    let cfg = ConfigurationWebsocketApi {
                        api_key: Some("api_key".into()),
                        api_secret: Some("api_secret".to_string()),
                        private_key: None,
                        private_key_passphrase: None,
                        ws_url: Some("ws://doesnotexist:1".to_string()),
                        mode: WebsocketMode::Single,
                        reconnect_delay: 0,
                        signature_gen: sig,
                        timeout: 10,
                        time_unit: None,
                        auto_session_relogon: false,
                        agent: None,
                        user_agent: build_user_agent("product"),
                    };
                    let api = WebsocketApi::new(cfg, vec![conn.clone()]);
                    let res = api.clone().connect().await;
                    assert!(!matches!(res, Err(WebsocketError::Timeout)));
                });
            }

            #[test]
            fn already_connected_returns_ok() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("id2");
                    let (tx, _) = unbounded_channel();
                    {
                        let mut st = conn.state.lock().await;
                        st.ws_write_tx = Some(tx);
                    }
                    let sig = SignatureGenerator::new(
                        Some("api_secret".to_string()),
                        None::<PrivateKey>,
                        None::<String>,
                    );
                    let cfg = ConfigurationWebsocketApi {
                        api_key: Some("api_key".to_string()),
                        api_secret: Some("api_secret".to_string()),
                        private_key: None,
                        private_key_passphrase: None,
                        ws_url: Some("ws://example.com".to_string()),
                        mode: WebsocketMode::Single,
                        reconnect_delay: 0,
                        signature_gen: sig,
                        timeout: 10,
                        time_unit: None,
                        auto_session_relogon: false,
                        agent: None,
                        user_agent: build_user_agent("product"),
                    };
                    let api = WebsocketApi::new(cfg, vec![conn.clone()]);
                    let res = api.connect().await;
                    assert!(res.is_ok());
                });
            }

            #[test]
            fn not_connected_returns_error() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("id1");
                    let sig = SignatureGenerator::new(
                        Some("api_secret".to_string()),
                        None::<PrivateKey>,
                        None::<String>,
                    );
                    let cfg = ConfigurationWebsocketApi {
                        api_key: Some("api_key".to_string()),
                        api_secret: Some("api_secret".to_string()),
                        private_key: None,
                        private_key_passphrase: None,
                        ws_url: Some("ws://127.0.0.1:9".to_string()),
                        mode: WebsocketMode::Single,
                        reconnect_delay: 0,
                        signature_gen: sig,
                        timeout: 10,
                        time_unit: None,
                        auto_session_relogon: false,
                        agent: None,
                        user_agent: build_user_agent("product"),
                    };
                    let api = WebsocketApi::new(cfg, vec![conn.clone()]);
                    let res = api.connect().await;
                    assert!(res.is_err());
                });
            }

            #[test]
            fn concurrent_calls_both_error_or_ok() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("id3");
                    let sig = SignatureGenerator::new(
                        Some("api_secret".to_string()),
                        None::<PrivateKey>,
                        None::<String>,
                    );
                    let cfg = ConfigurationWebsocketApi {
                        api_key: Some("api_key".to_string()),
                        api_secret: Some("api_secret".to_string()),
                        private_key: None,
                        private_key_passphrase: None,
                        ws_url: Some("wss://invalid-domain".to_string()),
                        mode: WebsocketMode::Single,
                        reconnect_delay: 0,
                        signature_gen: sig,
                        timeout: 10,
                        time_unit: None,
                        auto_session_relogon: false,
                        agent: None,
                        user_agent: build_user_agent("product"),
                    };
                    let api = WebsocketApi::new(cfg, vec![conn.clone()]);
                    let fut1 = tokio::spawn(api.clone().connect());
                    let fut2 = tokio::spawn(api.clone().connect());
                    let r1 = fut1.await.unwrap();
                    let r2 = fut2.await.unwrap();

                    assert!(r1.is_err());
                    assert!(r2.is_err() || r2.is_ok());
                });
            }

            #[test]
            fn pool_failure_is_propagated() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("w");
                    let sig = SignatureGenerator::new(
                        Some("api_secret".to_string()),
                        None::<PrivateKey>,
                        None::<String>,
                    );
                    let cfg = ConfigurationWebsocketApi {
                        api_key: Some("api_key".into()),
                        api_secret: Some("api_secret".to_string()),
                        private_key: None,
                        private_key_passphrase: None,
                        ws_url: Some("ws://doesnotexist:1".to_string()),
                        mode: WebsocketMode::Single,
                        reconnect_delay: 0,
                        signature_gen: sig,
                        timeout: 10,
                        time_unit: None,
                        auto_session_relogon: false,
                        agent: None,
                        user_agent: build_user_agent("product"),
                    };
                    let api = WebsocketApi::new(cfg, vec![conn.clone()]);
                    let res = api.clone().connect().await;
                    match res {
                        Err(WebsocketError::Handshake(_) | WebsocketError::Timeout) => {}
                        _ => panic!("expected handshake or timeout error"),
                    }
                });
            }
        }

        mod send_message {
            use super::*;

            #[test]
            fn unsigned_message() {
                TOKIO_SHARED_RT.block_on(async {
                    let api = create_websocket_api(None, None, None);
                    let conn = &api.common.connection_pool[0];
                    let (tx, mut rx) = unbounded_channel::<Message>();
                    {
                        let mut st = conn.state.lock().await;
                        st.ws_write_tx = Some(tx);
                    }

                    let fut = tokio::spawn({
                        let api = api.clone();
                        async move {
                            let mut params = BTreeMap::new();
                            params.insert("foo".into(), Value::String("bar".into()));
                            let send_res = api
                                .send_message::<Value>(
                                    "method",
                                    params,
                                    WebsocketMessageSendOptions {
                                        with_api_key: false,
                                        is_signed: false,
                                        ..Default::default()
                                    },
                                )
                                .await
                                .unwrap();

                            match send_res {
                                SendWebsocketMessageResult::Single(resp) => resp,
                                SendWebsocketMessageResult::Multiple(_) => {
                                    panic!("expected single response")
                                }
                            }
                        }
                    });

                    let Message::Text(txt) = rx.recv().await.unwrap() else {
                        panic!()
                    };
                    let req: Value = serde_json::from_str(&txt).unwrap();
                    assert_eq!(req["method"], "method");
                    assert_eq!(req["params"]["foo"], "bar");
                    assert!(req["params"].get("apiKey").is_none());
                    assert!(req["params"].get("timestamp").is_none());
                    assert!(req["params"].get("signature").is_none());

                    let id = req["id"].as_str().unwrap().to_string();
                    let mut st = conn.state.lock().await;
                    let pending = st.pending_requests.remove(&id).unwrap();
                    let reply = json!({
                        "id": id,
                        "result": { "x": 42 },
                        "rateLimits": [{ "limit": 7 }]
                    });
                    pending.completion.send(Ok(reply)).unwrap();

                    let resp = fut.await.unwrap();
                    let rate_limits = resp.rate_limits.unwrap_or_default();

                    assert!(rate_limits.is_empty());
                    assert_eq!(resp.raw, json!({"x": 42}));
                });
            }

            #[test]
            fn with_api_key_only() {
                TOKIO_SHARED_RT.block_on(async {
                    let api = create_websocket_api(None, None, None);
                    let conn = &api.common.connection_pool[0];
                    let (tx, mut rx) = unbounded_channel::<Message>();
                    {
                        let mut st = conn.state.lock().await;
                        st.ws_write_tx = Some(tx);
                    }

                    let fut = tokio::spawn({
                        let api = api.clone();
                        async move {
                            let params = BTreeMap::new();
                            let send_res = api
                                .send_message::<Value>(
                                    "method",
                                    params,
                                    WebsocketMessageSendOptions {
                                        with_api_key: true,
                                        is_signed: false,
                                        ..Default::default()
                                    },
                                )
                                .await
                                .unwrap();

                            match send_res {
                                SendWebsocketMessageResult::Single(resp) => resp,
                                SendWebsocketMessageResult::Multiple(_) => {
                                    panic!("expected single response")
                                }
                            }
                        }
                    });

                    let Message::Text(txt) = rx.recv().await.unwrap() else {
                        panic!()
                    };
                    let req: Value = serde_json::from_str(&txt).unwrap();
                    assert_eq!(req["params"]["apiKey"], "api_key");

                    let id = req["id"].as_str().unwrap().to_string();
                    let mut st = conn.state.lock().await;
                    let pending = st.pending_requests.remove(&id).unwrap();
                    pending
                        .completion
                        .send(Ok(json!({
                            "id": id,
                            "result": {},
                            "rateLimits": []
                        })))
                        .unwrap();

                    let resp = fut.await.unwrap();

                    assert_eq!(resp.raw, json!({}));
                    assert!(st.pending_requests.is_empty());
                });
            }

            #[test]
            fn signed_message_has_timestamp_and_signature() {
                TOKIO_SHARED_RT.block_on(async {
                    let api = create_websocket_api(None, None, None);
                    let conn = &api.common.connection_pool[0];
                    let (tx, mut rx) = unbounded_channel::<Message>();
                    {
                        let mut st = conn.state.lock().await;
                        st.ws_write_tx = Some(tx);
                    }

                    let fut = tokio::spawn({
                        let api = api.clone();
                        async move {
                            let mut params = BTreeMap::new();
                            params.insert("foo".into(), Value::String("bar".into()));
                            let send_res = api
                                .send_message::<Value>(
                                    "method",
                                    params,
                                    WebsocketMessageSendOptions {
                                        with_api_key: true,
                                        is_signed: true,
                                        ..Default::default()
                                    },
                                )
                                .await
                                .unwrap();

                            match send_res {
                                SendWebsocketMessageResult::Single(resp) => resp,
                                SendWebsocketMessageResult::Multiple(_) => {
                                    panic!("expected single response")
                                }
                            }
                        }
                    });

                    let Message::Text(txt) = rx.recv().await.unwrap() else {
                        panic!()
                    };
                    let req: Value = serde_json::from_str(&txt).unwrap();
                    let p = &req["params"];
                    assert_eq!(p["apiKey"], "api_key");
                    assert!(p["timestamp"].is_number());
                    assert!(p["signature"].is_string());

                    let id = req["id"].as_str().unwrap().to_string();
                    let mut st = conn.state.lock().await;
                    let pending = st.pending_requests.remove(&id).unwrap();
                    pending
                        .completion
                        .send(Ok(json!({
                            "id": id,
                            "result": { "ok": true },
                            "rateLimits": []
                        })))
                        .unwrap();

                    let resp = fut.await.unwrap();
                    assert_eq!(resp.raw, json!({ "ok": true }));
                });
            }

            #[test]
            fn multi_session_logon() {
                TOKIO_SHARED_RT.block_on(async {
                    let api = create_websocket_api(None, Some(WebsocketMode::Pool(2)), None);
                    let conn0 = &api.common.connection_pool[0];
                    let conn1 = &api.common.connection_pool[1];

                    let (tx0, mut rx0) = unbounded_channel::<Message>();
                    let (tx1, mut rx1) = unbounded_channel::<Message>();
                    {
                        let mut st0 = conn0.state.lock().await;
                        st0.ws_write_tx = Some(tx0);
                    }
                    {
                        let mut st1 = conn1.state.lock().await;
                        st1.ws_write_tx = Some(tx1);
                    }

                    let fut = tokio::spawn({
                        let api = api.clone();
                        async move {
                            let params = BTreeMap::new();
                            let send_res = api
                                .send_message::<Value>(
                                    "method",
                                    params,
                                    WebsocketMessageSendOptions {
                                        is_session_logon: Some(true),
                                        ..Default::default()
                                    },
                                )
                                .await
                                .unwrap();

                            match send_res {
                                SendWebsocketMessageResult::Multiple(v) => v,
                                SendWebsocketMessageResult::Single(_) => {
                                    panic!("expected multiple responses")
                                }
                            }
                        }
                    });

                    let Message::Text(txt0) = rx0.recv().await.unwrap() else {
                        panic!()
                    };
                    let Message::Text(txt1) = rx1.recv().await.unwrap() else {
                        panic!()
                    };
                    let req0: Value = serde_json::from_str(&txt0).unwrap();
                    let req1: Value = serde_json::from_str(&txt1).unwrap();
                    assert_eq!(req0["method"], "method");
                    assert_eq!(req1["method"], "method");
                    let id = req0["id"].as_str().unwrap().to_string();
                    assert_eq!(req1["id"].as_str().unwrap(), &id);

                    {
                        let mut st0 = conn0.state.lock().await;
                        let pending0 = st0.pending_requests.remove(&id).unwrap();
                        pending0
                            .completion
                            .send(Ok(json!({
                                "id": id,
                                "result": { "ok": true },
                                "rateLimits": []
                            })))
                            .unwrap();
                    }
                    {
                        let mut st1 = conn1.state.lock().await;
                        let pending1 = st1.pending_requests.remove(&id).unwrap();
                        pending1
                            .completion
                            .send(Ok(json!({
                                "id": id,
                                "result": { "ok": true },
                                "rateLimits": []
                            })))
                            .unwrap();
                    }

                    let results = fut.await.unwrap();
                    assert_eq!(results.len(), 2);

                    for conn in &api.common.connection_pool {
                        let st = conn.state.lock().await;
                        assert!(st.is_session_logged_on, "should be logged out");
                        assert!(st.session_logon_req.is_some(), "req cleared");

                        // let req = st
                        //     .session_logon_req
                        //     .as_ref()
                        //     .expect("session_logon_req should be Some(_)");
                        // assert_eq!(req.method, "method");
                        // let mut expected = BTreeMap::new();
                        // expected.insert("ok".to_string(), Value::Bool(true));
                        // assert_eq!(
                        //     req.payload, expected,
                        //     "stored payload should be {{ \"ok\": true }}"
                        // );
                        // assert!(
                        //     req.options.is_session_logon.unwrap_or(false),
                        //     "expected options.is_session_logon = true"
                        // );
                    }
                });
            }

            #[test]
            fn multi_session_logout() {
                TOKIO_SHARED_RT.block_on(async {
                    let api = create_websocket_api(None, Some(WebsocketMode::Pool(2)), None);

                    for conn in &api.common.connection_pool {
                        let (tx, _rx) = unbounded_channel::<Message>();
                        let mut st = conn.state.lock().await;
                        st.ws_write_tx = Some(tx);
                        st.is_session_logged_on = true;
                        st.session_logon_req = Some(WebsocketSessionLogonReq {
                            method: "method".into(),
                            payload: BTreeMap::new(),
                            options: WebsocketMessageSendOptions::default(),
                        });
                    }

                    let mut rxs = Vec::new();
                    for conn in &api.common.connection_pool {
                        let rx = {
                            let (tx, rx) = unbounded_channel::<Message>();
                            conn.state.lock().await.ws_write_tx = Some(tx);
                            rx
                        };
                        rxs.push(rx);
                    }

                    let fut = tokio::spawn({
                        let api = api.clone();
                        async move {
                            let send_res = api
                                .send_message::<Value>(
                                    "method",
                                    BTreeMap::new(),
                                    WebsocketMessageSendOptions {
                                        is_signed: false,
                                        with_api_key: false,
                                        is_session_logout: Some(true),
                                        ..Default::default()
                                    },
                                )
                                .await
                                .unwrap();

                            match send_res {
                                SendWebsocketMessageResult::Multiple(v) => v,
                                SendWebsocketMessageResult::Single(_) => panic!("expected multi"),
                            }
                        }
                    });

                    let mut ids = Vec::new();
                    for mut rx in rxs {
                        let Message::Text(txt) = rx.recv().await.unwrap() else {
                            panic!()
                        };
                        let req: Value = serde_json::from_str(&txt).unwrap();
                        assert_eq!(req["method"], "method");
                        ids.push(req["id"].as_str().unwrap().to_string());
                    }

                    assert_eq!(ids[0], ids[1]);

                    for conn in &api.common.connection_pool {
                        let id = &ids[0];
                        let mut st = conn.state.lock().await;
                        let pending = st.pending_requests.remove(id).unwrap();
                        pending
                            .completion
                            .send(Ok(json!({
                                "id": id,
                                "result": {},
                                "rateLimits": []
                            })))
                            .unwrap();
                    }

                    let results = fut.await.unwrap();
                    assert_eq!(results.len(), 2);

                    for conn in &api.common.connection_pool {
                        let st = conn.state.lock().await;
                        assert!(!st.is_session_logged_on, "should be logged out");
                        assert!(st.session_logon_req.is_none(), "req cleared");
                    }
                });
            }

            #[test]
            fn skip_signature_when_logged_on_and_auto_relogon() {
                TOKIO_SHARED_RT.block_on(async {
                    let api = create_websocket_api(None, Some(WebsocketMode::Single), None);
                    let conn = &api.common.connection_pool[0];
                    {
                        let mut st = conn.state.lock().await;
                        st.ws_write_tx = Some(unbounded_channel::<Message>().0);
                        st.is_session_logged_on = true;
                    }

                    let mut rx;
                    {
                        let mut st = conn.state.lock().await;
                        let (tx, new_rx) = unbounded_channel::<Message>();
                        st.ws_write_tx = Some(tx);
                        rx = new_rx;
                    }

                    let fut = tokio::spawn({
                        let api = api.clone();
                        async move {
                            let send_res = api
                                .send_message::<Value>(
                                    "method",
                                    BTreeMap::new(),
                                    WebsocketMessageSendOptions {
                                        is_signed: true,
                                        ..Default::default()
                                    },
                                )
                                .await
                                .unwrap();

                            match send_res {
                                SendWebsocketMessageResult::Single(resp) => resp,
                                SendWebsocketMessageResult::Multiple(_) => {
                                    panic!("expected single")
                                }
                            }
                        }
                    });

                    let Message::Text(txt) = rx.recv().await.unwrap() else {
                        panic!()
                    };
                    let req: Value = serde_json::from_str(&txt).unwrap();
                    let p = &req["params"];
                    assert!(p.get("timestamp").is_some());
                    assert!(p.get("signature").is_none());

                    let id = req["id"].as_str().unwrap();
                    let mut st = conn.state.lock().await;
                    let pending = st.pending_requests.remove(id).unwrap();
                    pending
                        .completion
                        .send(Ok(json!({
                            "id": id,
                            "result": {},
                            "rateLimits": []
                        })))
                        .unwrap();

                    let resp = fut.await.unwrap();
                    assert_eq!(resp.raw, json!({}));
                });
            }

            #[test]
            fn include_signature_when_logged_on_and_no_auto_relogon() {
                TOKIO_SHARED_RT.block_on(async {
                    let api = create_websocket_api(None, Some(WebsocketMode::Single), Some(false));
                    let conn = &api.common.connection_pool[0];
                    {
                        let mut st = conn.state.lock().await;
                        st.ws_write_tx = Some(unbounded_channel::<Message>().0);
                        st.is_session_logged_on = true;
                    }

                    let mut rx;
                    {
                        let mut st = conn.state.lock().await;
                        let (tx, new_rx) = unbounded_channel::<Message>();
                        st.ws_write_tx = Some(tx);
                        rx = new_rx;
                    }

                    let fut = tokio::spawn({
                        let api = api.clone();
                        async move {
                            let send_res = api
                                .send_message::<Value>(
                                    "method",
                                    BTreeMap::new(),
                                    WebsocketMessageSendOptions {
                                        is_signed: true,
                                        ..Default::default()
                                    },
                                )
                                .await
                                .unwrap();

                            match send_res {
                                SendWebsocketMessageResult::Single(resp) => resp,
                                SendWebsocketMessageResult::Multiple(_) => {
                                    panic!("expected single")
                                }
                            }
                        }
                    });

                    let Message::Text(txt) = rx.recv().await.unwrap() else {
                        panic!()
                    };
                    let req: Value = serde_json::from_str(&txt).unwrap();
                    let p = &req["params"];
                    assert!(p.get("timestamp").is_some());
                    assert!(p.get("signature").is_some());

                    let id = req["id"].as_str().unwrap();
                    let mut st = conn.state.lock().await;
                    let pending = st.pending_requests.remove(id).unwrap();
                    pending
                        .completion
                        .send(Ok(json!({
                            "id": id,
                            "result": {},
                            "rateLimits": []
                        })))
                        .unwrap();

                    let resp = fut.await.unwrap();
                    assert_eq!(resp.raw, json!({}));
                });
            }

            #[test]
            fn error_if_not_connected() {
                TOKIO_SHARED_RT.block_on(async {
                    let api = create_websocket_api(None, None, None);
                    let conn = &api.common.connection_pool[0];
                    {
                        let mut st = conn.state.lock().await;
                        st.ws_write_tx = None;
                    }
                    let params = BTreeMap::new();
                    let err = api
                        .send_message::<Value>(
                            "method",
                            params,
                            WebsocketMessageSendOptions {
                                with_api_key: false,
                                is_signed: false,
                                ..Default::default()
                            },
                        )
                        .await
                        .unwrap_err();
                    matches!(err, WebsocketError::NotConnected);
                });
            }
        }

        mod prepare_url {
            use super::*;

            #[test]
            fn no_time_unit() {
                TOKIO_SHARED_RT.block_on(async {
                    let api = create_websocket_api(None, None, None);
                    let url = "wss://example.com/ws".to_string();
                    assert_eq!(api.prepare_url(&url), url);
                });
            }

            #[test]
            fn appends_time_unit() {
                TOKIO_SHARED_RT.block_on(async {
                    let api = create_websocket_api(Some(TimeUnit::Millisecond), None, None);
                    let base = "wss://example.com/ws".to_string();
                    let got = api.prepare_url(&base);
                    assert_eq!(got, format!("{base}?timeUnit=millisecond"));
                });
            }

            #[test]
            fn handles_existing_query() {
                TOKIO_SHARED_RT.block_on(async {
                    let api = create_websocket_api(Some(TimeUnit::Microsecond), None, None);
                    let base = "wss://example.com/ws?foo=bar".to_string();
                    let got = api.prepare_url(&base);
                    assert_eq!(got, format!("{base}&timeUnit=microsecond"));
                });
            }
        }

        mod on_open {
            use super::*;

            fn create_websocket_api_and_conn() -> (Arc<WebsocketApi>, Arc<WebsocketConnection>) {
                let sig_gen = SignatureGenerator::new(
                    Some("api_secret".to_string()),
                    None::<_>,
                    None::<String>,
                );
                let config = ConfigurationWebsocketApi {
                    api_key: Some("api_key".to_string()),
                    api_secret: Some("api_secret".to_string()),
                    private_key: None,
                    private_key_passphrase: None,
                    ws_url: Some("wss://example".to_string()),
                    mode: WebsocketMode::Single,
                    reconnect_delay: 0,
                    signature_gen: sig_gen,
                    timeout: 1000,
                    time_unit: None,
                    auto_session_relogon: true,
                    agent: None,
                    user_agent: build_user_agent("product"),
                };
                let conn = WebsocketConnection::new("test-conn");
                let api = WebsocketApi::new(config, vec![conn.clone()]);
                (api, conn)
            }

            #[test]
            fn session_relogon_on_open() {
                TOKIO_SHARED_RT.block_on(async {
                    let (api, conn) = create_websocket_api_and_conn();

                    let req = WebsocketSessionLogonReq {
                        method: "method".into(),
                        payload: {
                            let mut m = BTreeMap::new();
                            m.insert("foo".into(), Value::String("bar".into()));
                            m
                        },
                        options: WebsocketMessageSendOptions {
                            with_api_key: true,
                            is_signed: true,
                            is_session_logon: Some(true),
                            ..Default::default()
                        },
                    };

                    let (tx, mut rx) = unbounded_channel::<Message>();
                    {
                        let mut st = conn.state.lock().await;
                        st.session_logon_req = Some(req.clone());
                        st.is_session_logged_on = false;
                        st.ws_write_tx = Some(tx);
                    }

                    api.on_open("wss://example".to_string(), conn.clone()).await;

                    let Message::Text(raw) = rx.recv().await.unwrap() else {
                        panic!("expected a Text message");
                    };
                    let msg: Value = serde_json::from_str(&raw).unwrap();
                    assert_eq!(msg["method"], "method");
                    assert_eq!(msg["params"]["foo"], "bar");

                    let id = msg["id"].as_str().unwrap().to_string();
                    {
                        let mut st = conn.state.lock().await;
                        let pending = st.pending_requests.remove(&id).expect("pending request");
                        pending
                            .completion
                            .send(Ok(json!({
                                "id": id,
                                "result": {},
                                "rateLimits": []
                            })))
                            .unwrap();
                    }

                    sleep(Duration::from_millis(10)).await;

                    let st = conn.state.lock().await;
                    assert!(st.is_session_logged_on, "should now be logged on");
                });
            }

            #[test]
            fn no_relogon_if_already_logged_on() {
                TOKIO_SHARED_RT.block_on(async {
                    let (api, conn) = create_websocket_api_and_conn();

                    let req = WebsocketSessionLogonReq {
                        method: "method".into(),
                        payload: BTreeMap::new(),
                        options: WebsocketMessageSendOptions {
                            is_session_logon: Some(true),
                            ..Default::default()
                        },
                    };

                    let (tx, mut rx) = unbounded_channel::<Message>();
                    {
                        let mut st = conn.state.lock().await;
                        st.session_logon_req = Some(req);
                        st.is_session_logged_on = true;
                        st.ws_write_tx = Some(tx);
                    }

                    api.on_open("wss://example".to_string(), conn.clone()).await;

                    assert!(rx.try_recv().is_err(), "no re‐logon when already on");

                    let st = conn.state.lock().await;
                    assert!(st.is_session_logged_on);
                });
            }

            #[test]
            fn session_relogon_fails_gracefully() {
                TOKIO_SHARED_RT.block_on(async {
                    let (api, conn) = create_websocket_api_and_conn();

                    let req = WebsocketSessionLogonReq {
                        method: "method".into(),
                        payload: {
                            let mut m = BTreeMap::new();
                            m.insert("x".into(), Value::Number(1.into()));
                            m
                        },
                        options: WebsocketMessageSendOptions {
                            is_session_logon: Some(true),
                            ..Default::default()
                        },
                    };
                    {
                        let mut st = conn.state.lock().await;
                        st.session_logon_req = Some(req);
                        st.is_session_logged_on = false;
                        st.ws_write_tx = None;
                    }

                    api.on_open("wss://example".into(), conn.clone()).await;

                    let st = conn.state.lock().await;
                    assert!(
                        !st.is_session_logged_on,
                        "should remain logged‐off on failure"
                    );
                });
            }

            #[test]
            fn session_relogon_noop_when_no_req() {
                TOKIO_SHARED_RT.block_on(async {
                    let (api, conn) = create_websocket_api_and_conn();

                    {
                        let mut st = conn.state.lock().await;
                        st.session_logon_req = None;
                        st.is_session_logged_on = false;
                        st.ws_write_tx = Some(unbounded_channel::<Message>().0);
                    }

                    api.on_open("wss://example".into(), conn.clone()).await;

                    let st = conn.state.lock().await;
                    assert!(!st.is_session_logged_on, "still logged‐off");
                });
            }
        }

        mod on_message {
            use super::*;

            fn create_websocket_api_and_conn() -> (Arc<WebsocketApi>, Arc<WebsocketConnection>) {
                let sig_gen = SignatureGenerator::new(
                    Some("api_secret".to_string()),
                    None::<_>,
                    None::<String>,
                );
                let config = ConfigurationWebsocketApi {
                    api_key: Some("api_key".to_string()),
                    api_secret: Some("api_secret".to_string()),
                    private_key: None,
                    private_key_passphrase: None,
                    ws_url: Some("wss://example".to_string()),
                    mode: WebsocketMode::Single,
                    reconnect_delay: 0,
                    signature_gen: sig_gen,
                    timeout: 1000,
                    time_unit: None,
                    auto_session_relogon: false,
                    agent: None,
                    user_agent: build_user_agent("product"),
                };
                let conn = WebsocketConnection::new("test");
                let api = WebsocketApi::new(config, vec![conn.clone()]);
                (api, conn)
            }

            #[test]
            fn resolves_pending_and_removes_request() {
                TOKIO_SHARED_RT.block_on(async {
                    let (api, conn) = create_websocket_api_and_conn();
                    let (tx, rx) = oneshot::channel();
                    {
                        let mut st = conn.state.lock().await;
                        st.pending_requests
                            .insert("id1".to_string(), PendingRequest { completion: tx });
                    }
                    let msg = json!({"id":"id1","status":200,"foo":"bar"});
                    api.on_message(msg.to_string(), conn.clone()).await;
                    let got = rx.await.unwrap().unwrap();
                    assert_eq!(got, msg);
                    let st = conn.state.lock().await;
                    assert!(!st.pending_requests.contains_key("id1"));
                });
            }

            #[test]
            fn uses_result_when_present() {
                TOKIO_SHARED_RT.block_on(async {
                    let (api, conn) = create_websocket_api_and_conn();
                    let (tx, rx) = oneshot::channel();
                    {
                        let mut st = conn.state.lock().await;
                        st.pending_requests
                            .insert("id1".to_string(), PendingRequest { completion: tx });
                    }
                    let msg = json!({
                        "id": "id1",
                        "status": 200,
                        "response": [1,2],
                        "result": {"a":1}
                    });
                    api.on_message(msg.to_string(), conn.clone()).await;
                    let got = rx.await.unwrap().unwrap();
                    assert_eq!(got.get("result").unwrap(), &json!({"a":1}));
                });
            }

            #[test]
            fn uses_response_when_no_result() {
                TOKIO_SHARED_RT.block_on(async {
                    let (api, conn) = create_websocket_api_and_conn();
                    let (tx, rx) = oneshot::channel();
                    {
                        let mut st = conn.state.lock().await;
                        st.pending_requests
                            .insert("id1".to_string(), PendingRequest { completion: tx });
                    }
                    let msg = json!({
                        "id": "id1",
                        "status": 200,
                        "response": ["ok"]
                    });
                    api.on_message(msg.to_string(), conn.clone()).await;
                    let got = rx.await.unwrap().unwrap();
                    assert_eq!(got.get("response").unwrap(), &json!(["ok"]));
                });
            }

            #[test]
            fn errors_for_status_ge_400() {
                TOKIO_SHARED_RT.block_on(async {
                    let (api, conn) = create_websocket_api_and_conn();
                    let (tx, rx) = oneshot::channel();
                    {
                        let mut st = conn.state.lock().await;
                        st.pending_requests
                            .insert("bad".to_string(), PendingRequest { completion: tx });
                    }
                    let err_obj = json!({"code":123,"msg":"oops"});
                    let msg = json!({"id":"bad","status":500,"error":err_obj});
                    api.on_message(msg.to_string(), conn.clone()).await;
                    match rx.await.unwrap() {
                        Err(WebsocketError::ResponseError { code, message }) => {
                            assert_eq!(code, 123);
                            assert_eq!(message, "oops");
                        }
                        other => panic!("expected ResponseError, got {other:?}"),
                    }
                    let st = conn.state.lock().await;
                    assert!(!st.pending_requests.contains_key("bad"));
                });
            }

            #[test]
            fn ignores_unknown_id() {
                TOKIO_SHARED_RT.block_on(async {
                    let (api, conn) = create_websocket_api_and_conn();
                    let msg = json!({"id":"nope","status":200});
                    api.on_message(msg.to_string(), conn.clone()).await;
                    let st = conn.state.lock().await;
                    assert!(st.pending_requests.is_empty());
                });
            }

            #[test]
            fn parse_error_ignored() {
                TOKIO_SHARED_RT.block_on(async {
                    let (api, conn) = create_websocket_api_and_conn();
                    api.on_message("not json".to_string(), conn.clone()).await;
                    let st = conn.state.lock().await;
                    assert!(st.pending_requests.is_empty());
                });
            }

            #[test]
            fn error_status_sends_error() {
                TOKIO_SHARED_RT.block_on(async {
                    let (api, conn) = create_websocket_api_and_conn();
                    let (tx, rx) = oneshot::channel();
                    {
                        let mut st = conn.state.lock().await;
                        st.pending_requests
                            .insert("err".to_string(), PendingRequest { completion: tx });
                    }
                    let msg = json!({
                        "id": "err",
                        "status": 500,
                        "error": { "code": 42, "msg": "Bad!" }
                    });
                    api.on_message(msg.to_string(), conn.clone()).await;
                    match rx.await.unwrap() {
                        Err(WebsocketError::ResponseError { code, message }) => {
                            assert_eq!(code, 42);
                            assert_eq!(message, "Bad!");
                        }
                        other => panic!("expected ResponseError, got {other:?}"),
                    }
                });
            }

            #[test]
            fn unknown_id_logs_warning_and_leaves_pending() {
                TOKIO_SHARED_RT.block_on(async {
                    let (api, conn) = create_websocket_api_and_conn();
                    {
                        let mut st = conn.state.lock().await;
                        st.pending_requests.insert(
                            "keep".to_string(),
                            PendingRequest {
                                completion: oneshot::channel().0,
                            },
                        );
                    }
                    api.on_message(
                        json!({ "id": "foo", "status": 200, "result": 1 }).to_string(),
                        conn.clone(),
                    )
                    .await;
                    let st = conn.state.lock().await;
                    assert!(st.pending_requests.contains_key("keep"));
                });
            }
        }
    }

    mod websocket_streams {
        use super::*;

        mod initialisation {
            use super::*;

            #[test]
            fn new_initializes_fields() {
                TOKIO_SHARED_RT.block_on(async {
                    let config = ConfigurationWebsocketStreams {
                        ws_url: Some("wss://example".to_string()),
                        mode: WebsocketMode::Pool(2),
                        reconnect_delay: 500,
                        time_unit: None,
                        agent: None,
                        user_agent: build_user_agent("product"),
                    };
                    let conn1 = WebsocketConnection::new("c1");
                    let conn2 = WebsocketConnection::new("c2");
                    let api =
                        WebsocketStreams::new(config.clone(), vec![conn1.clone(), conn2.clone()]);

                    assert_eq!(api.common.connection_pool.len(), 2);
                    assert!(Arc::ptr_eq(&api.common.connection_pool[0], &conn1));
                    assert!(Arc::ptr_eq(&api.common.connection_pool[1], &conn2));
                    assert_eq!(api.configuration.ws_url, Some("wss://example".to_string()));
                    let flag = api.is_connecting.lock().await;
                    assert!(!*flag);
                });
            }
        }

        mod connect {
            use super::*;

            #[test]
            fn establishes_successfully() {
                TOKIO_SHARED_RT.block_on(async {
                    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
                    let port = listener.local_addr().unwrap().port();

                    tokio::spawn(async move {
                        for _ in 0..2 {
                            if let Ok((stream, _)) = listener.accept().await {
                                let mut ws = accept_async(stream).await.unwrap();
                                ws.close(None).await.ok();
                            }
                        }
                    });

                    let create_websocket_streams = |ws_url: &str| {
                        let c1 = WebsocketConnection::new("c1");
                        let c2 = WebsocketConnection::new("c2");
                        let config = ConfigurationWebsocketStreams {
                            ws_url: Some(ws_url.to_string()),
                            mode: WebsocketMode::Pool(2),
                            reconnect_delay: 500,
                            time_unit: None,
                            agent: None,
                            user_agent: build_user_agent("product"),
                        };
                        WebsocketStreams::new(config, vec![c1, c2])
                    };

                    let url = format!("ws://127.0.0.1:{port}");
                    let ws = create_websocket_streams(&url);

                    let res = ws.connect(vec!["stream1".into()]).await;
                    assert!(res.is_ok());
                });
            }

            #[test]
            fn refused_returns_error() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws = create_websocket_streams(Some("ws://127.0.0.1:9"), None);
                    let res = ws.connect(vec!["stream1".into()]).await;
                    assert!(res.is_err());
                });
            }

            #[test]
            fn invalid_url_returns_error() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws = create_websocket_streams(Some("not-a-url"), None);
                    let res = ws.connect(vec!["s".into()]).await;
                    assert!(res.is_err());
                });
            }
        }

        mod disconnect {
            use super::*;

            #[test]
            fn disconnect_clears_state_and_streams() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws = create_websocket_streams(None, None);
                    let conn = &ws.common.connection_pool[0];
                    {
                        let mut state = conn.state.lock().await;
                        state.stream_callbacks.insert("s1".to_string(), Vec::new());
                        state.pending_subscriptions.push_back("s2".to_string());
                    }
                    {
                        let mut map = ws.connection_streams.lock().await;
                        map.insert("s3".to_string(), Arc::clone(conn));
                    }

                    let res = ws.disconnect().await;
                    assert!(res.is_ok());

                    let state = conn.state.lock().await;
                    assert!(state.stream_callbacks.is_empty());
                    assert!(state.pending_subscriptions.is_empty());

                    let map = ws.connection_streams.lock().await;
                    assert!(map.is_empty());
                });
            }
        }

        mod subscribe {
            use super::*;

            #[test]
            fn empty_list_does_nothing() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws = create_websocket_streams(None, None);
                    ws.clone().subscribe(Vec::new(), None).await;
                    let map = ws.connection_streams.lock().await;
                    assert!(map.is_empty());
                });
            }

            #[test]
            fn queue_when_not_ready() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws = create_websocket_streams(None, None);
                    let conn = ws.common.connection_pool[0].clone();
                    ws.clone().subscribe(vec!["s1".into()], None).await;
                    let state = conn.state.lock().await;
                    let pending: Vec<String> =
                        state.pending_subscriptions.iter().cloned().collect();
                    assert_eq!(pending, vec!["s1".to_string()]);
                });
            }

            #[test]
            fn only_one_subscription_per_stream() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws = create_websocket_streams(None, None);
                    let conn = ws.common.connection_pool[0].clone();
                    ws.clone().subscribe(vec!["s1".into()], None).await;
                    ws.clone().subscribe(vec!["s1".into()], None).await;
                    let state = conn.state.lock().await;
                    let pending: Vec<String> =
                        state.pending_subscriptions.iter().cloned().collect();
                    assert_eq!(pending, vec!["s1".to_string()]);
                });
            }

            #[test]
            fn multiple_streams_assigned() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws = create_websocket_streams(None, None);
                    ws.clone()
                        .subscribe(vec!["s1".into(), "s2".into()], None)
                        .await;
                    let map = ws.connection_streams.lock().await;
                    assert!(map.contains_key("s1"));
                    assert!(map.contains_key("s2"));
                });
            }

            #[test]
            fn existing_stream_not_reassigned() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws = create_websocket_streams(None, None);
                    ws.clone().subscribe(vec!["s1".into()], None).await;
                    let first_id = {
                        let map = ws.connection_streams.lock().await;
                        map.get("s1").unwrap().id.clone()
                    };
                    ws.clone()
                        .subscribe(vec!["s1".into(), "s2".into()], None)
                        .await;
                    let map = ws.connection_streams.lock().await;
                    let second_id = map.get("s1").unwrap().id.clone();
                    assert_eq!(first_id, second_id);
                    assert!(map.contains_key("s2"));
                });
            }
        }

        mod unsubscribe {
            use super::*;

            #[test]
            fn removes_stream_with_no_callbacks() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws = create_websocket_streams(None, None);
                    let conn = ws.common.connection_pool[0].clone();

                    {
                        let (tx, _rx) = unbounded_channel::<Message>();
                        let mut st = conn.state.lock().await;
                        st.ws_write_tx = Some(tx);
                    }

                    {
                        let mut map = ws.connection_streams.lock().await;
                        map.insert("s1".to_string(), conn.clone());
                    }
                    {
                        let mut st = conn.state.lock().await;
                        st.stream_callbacks.insert("s1".to_string(), Vec::new());
                    }

                    ws.unsubscribe(vec!["s1".to_string()], None).await;

                    assert!(!ws.connection_streams.lock().await.contains_key("s1"));
                    assert!(!conn.state.lock().await.stream_callbacks.contains_key("s1"));
                });
            }

            #[test]
            fn preserves_stream_with_callbacks() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws = create_websocket_streams(None, None);
                    let conn = ws.common.connection_pool[1].clone();

                    {
                        let mut map = ws.connection_streams.lock().await;
                        map.insert("s2".to_string(), conn.clone());
                    }
                    {
                        let mut state = conn.state.lock().await;
                        state
                            .stream_callbacks
                            .insert("s2".to_string(), vec![Arc::new(|_: &Value| {})]);
                    }

                    ws.unsubscribe(vec!["s2".to_string()], None).await;

                    assert!(ws.connection_streams.lock().await.contains_key("s2"));
                    assert!(conn.state.lock().await.stream_callbacks.contains_key("s2"));
                });
            }

            #[test]
            fn does_not_send_if_callbacks_exist() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws = create_websocket_streams(None, None);
                    let conn = ws.common.connection_pool[0].clone();
                    {
                        let mut map = ws.connection_streams.lock().await;
                        map.insert("s1".to_string(), conn.clone());
                    }
                    {
                        let mut state = conn.state.lock().await;
                        state.stream_callbacks.insert(
                            "s1".to_string(),
                            vec![Arc::new(|_: &Value| {}), Arc::new(|_: &Value| {})],
                        );
                    }
                    ws.unsubscribe(vec!["s1".into()], None).await;
                    assert!(ws.connection_streams.lock().await.contains_key("s1"));
                    assert!(conn.state.lock().await.stream_callbacks.contains_key("s1"));
                });
            }

            #[test]
            fn warns_if_not_associated() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws = create_websocket_streams(None, None);
                    ws.unsubscribe(vec!["nope".into()], None).await;
                });
            }

            #[test]
            fn empty_list_does_nothing() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws = create_websocket_streams(None, None);
                    let before = ws.connection_streams.lock().await.len();
                    ws.unsubscribe(Vec::<String>::new(), None).await;
                    let after = ws.connection_streams.lock().await.len();
                    assert_eq!(before, after);
                });
            }

            #[test]
            fn invalid_custom_id_falls_back() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws = create_websocket_streams(None, None);
                    let conn = ws.common.connection_pool[0].clone();
                    {
                        let mut map = ws.connection_streams.lock().await;
                        map.insert("foo".to_string(), conn.clone());
                    }
                    {
                        let mut state = conn.state.lock().await;
                        let (tx, _rx) = unbounded_channel();
                        state.ws_write_tx = Some(tx);
                        state.stream_callbacks.insert("foo".to_string(), Vec::new());
                    }
                    ws.unsubscribe(vec!["foo".into()], Some("bad-id".into()))
                        .await;
                    assert!(!ws.connection_streams.lock().await.contains_key("foo"));
                });
            }

            #[test]
            fn removes_even_without_write_channel() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws = create_websocket_streams(None, None);
                    let conn = ws.common.connection_pool[0].clone();
                    {
                        let mut map = ws.connection_streams.lock().await;
                        map.insert("x".to_string(), conn.clone());
                    }
                    {
                        let mut state = conn.state.lock().await;
                        let (tx, _rx) = unbounded_channel();
                        state.ws_write_tx = Some(tx);
                        state.stream_callbacks.insert("x".to_string(), Vec::new());
                    }
                    ws.unsubscribe(vec!["x".into()], None).await;
                    assert!(!ws.connection_streams.lock().await.contains_key("x"));
                });
            }
        }

        mod is_subscribed {
            use super::*;

            #[test]
            fn returns_false_when_not_subscribed() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws = create_websocket_streams(None, None);
                    assert!(!ws.is_subscribed("unknown").await);
                });
            }

            #[test]
            fn returns_true_when_subscribed() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws = create_websocket_streams(None, None);
                    let conn = ws.common.connection_pool[0].clone();
                    {
                        let mut map = ws.connection_streams.lock().await;
                        map.insert("stream1".to_string(), conn);
                    }
                    assert!(ws.is_subscribed("stream1").await);
                });
            }
        }

        mod prepare_url {
            use super::*;

            #[test]
            fn without_time_unit_returns_base_url() {
                TOKIO_SHARED_RT.block_on(async {
                    let conns = vec![
                        WebsocketConnection::new("c1"),
                        WebsocketConnection::new("c2"),
                    ];
                    let config = ConfigurationWebsocketStreams {
                        ws_url: Some("wss://example".to_string()),
                        mode: WebsocketMode::Single,
                        reconnect_delay: 100,
                        time_unit: None,
                        agent: None,
                        user_agent: build_user_agent("product"),
                    };
                    let ws = WebsocketStreams::new(config, conns);
                    let url = ws.prepare_url(&["s1".into(), "s2".into()]);
                    assert_eq!(url, "wss://example/stream?streams=s1/s2");
                });
            }

            #[test]
            fn with_time_unit_appends_parameter() {
                TOKIO_SHARED_RT.block_on(async {
                    let conns = vec![WebsocketConnection::new("c1")];
                    let config = ConfigurationWebsocketStreams {
                        ws_url: Some("wss://example".to_string()),
                        mode: WebsocketMode::Single,
                        reconnect_delay: 100,
                        time_unit: Some(TimeUnit::Millisecond),
                        agent: None,
                        user_agent: build_user_agent("product"),
                    };
                    let ws = WebsocketStreams::new(config, conns);
                    let url = ws.prepare_url(&["a".into()]);
                    assert_eq!(url, "wss://example/stream?streams=a&timeUnit=millisecond");
                });
            }

            #[test]
            fn multiple_streams_and_time_unit() {
                TOKIO_SHARED_RT.block_on(async {
                    let conns = vec![WebsocketConnection::new("c1")];
                    let config = ConfigurationWebsocketStreams {
                        ws_url: Some("wss://example".to_string()),
                        mode: WebsocketMode::Single,
                        reconnect_delay: 100,
                        time_unit: Some(TimeUnit::Microsecond),
                        agent: None,
                        user_agent: build_user_agent("product"),
                    };
                    let ws = WebsocketStreams::new(config, conns);
                    let url = ws.prepare_url(&["x".into(), "y".into(), "z".into()]);
                    assert_eq!(
                        url,
                        "wss://example/stream?streams=x/y/z&timeUnit=microsecond"
                    );
                });
            }
        }

        mod handle_stream_assignment {
            use super::*;

            #[test]
            fn assigns_new_streams_to_connections() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws = create_websocket_streams(None, None);
                    let groups = ws
                        .clone()
                        .handle_stream_assignment(vec!["s1".into(), "s2".into()])
                        .await;
                    let mut seen_streams = HashSet::new();
                    for (_conn, streams) in &groups {
                        for s in streams {
                            seen_streams.insert(s);
                        }
                    }
                    assert_eq!(
                        seen_streams,
                        ["s1".to_string(), "s2".to_string()].iter().collect()
                    );
                    assert_eq!(groups.len(), 1);
                });
            }

            #[test]
            fn reuses_existing_connection_for_duplicate_stream() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws = create_websocket_streams(None, None);
                    let _ = ws.clone().handle_stream_assignment(vec!["s1".into()]).await;
                    let groups = ws
                        .clone()
                        .handle_stream_assignment(vec!["s1".into(), "s3".into()])
                        .await;
                    let mut all_streams = Vec::new();
                    for (_conn, streams) in groups {
                        all_streams.extend(streams);
                    }
                    all_streams.sort();
                    assert_eq!(all_streams, vec!["s1".to_string(), "s3".to_string()]);
                });
            }

            #[test]
            fn empty_stream_list_returns_empty() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws = create_websocket_streams(None, None);
                    let groups = ws.clone().handle_stream_assignment(vec![]).await;
                    assert!(groups.is_empty());
                });
            }

            #[test]
            fn closed_or_reconnecting_forces_reassignment_of_stream() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws = create_websocket_streams(None, None);
                    let mut groups = ws.clone().handle_stream_assignment(vec!["s1".into()]).await;
                    let (conn, _) = groups.pop().unwrap();
                    {
                        let mut st = conn.state.lock().await;
                        st.close_initiated = true;
                    }
                    let groups2 = ws.clone().handle_stream_assignment(vec!["s2".into()]).await;
                    assert_eq!(groups2.len(), 1);
                    let (_new_conn, streams) = &groups2[0];
                    assert_eq!(streams, &vec!["s2".to_string()]);
                });
            }

            #[test]
            fn no_available_connections_falls_back_to_one() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws = create_websocket_streams(None, Some(vec![]));
                    let assigned = ws.handle_stream_assignment(vec!["foo".into()]).await;
                    assert_eq!(assigned.len(), 1);
                    let (_conn, streams) = &assigned[0];
                    assert_eq!(streams.as_slice(), &["foo".to_string()]);
                });
            }

            #[test]
            fn single_connection_groups_multiple_streams() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("c1");
                    let ws = create_websocket_streams(None, Some(vec![conn.clone()]));
                    let assigned = ws
                        .handle_stream_assignment(vec!["s1".into(), "s2".into()])
                        .await;
                    assert_eq!(assigned.len(), 1);
                    let (assigned_conn, streams) = &assigned[0];
                    assert!(Arc::ptr_eq(assigned_conn, &conn));
                    assert_eq!(streams.len(), 2);
                    assert!(streams.contains(&"s1".to_string()));
                    assert!(streams.contains(&"s2".to_string()));
                });
            }

            #[test]
            fn reuse_existing_healthy_connection() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("c");
                    let ws = create_websocket_streams(None, Some(vec![conn.clone()]));
                    let _ = ws.handle_stream_assignment(vec!["s1".into()]).await;
                    let second = ws.handle_stream_assignment(vec!["s1".into()]).await;
                    assert_eq!(second.len(), 1);
                    let (assigned_conn, streams) = &second[0];
                    assert!(Arc::ptr_eq(assigned_conn, &conn));
                    assert_eq!(streams.as_slice(), &["s1".to_string()]);
                });
            }

            #[test]
            fn mix_new_and_assigned_streams() {
                TOKIO_SHARED_RT.block_on(async {
                    let conn = WebsocketConnection::new("c");
                    let ws = create_websocket_streams(None, Some(vec![conn.clone()]));
                    let _ = ws
                        .handle_stream_assignment(vec!["s1".into(), "s2".into()])
                        .await;
                    let mixed = ws
                        .handle_stream_assignment(vec!["s2".into(), "s3".into()])
                        .await;
                    assert_eq!(mixed.len(), 1);
                    let (assigned_conn, streams) = &mixed[0];
                    assert!(Arc::ptr_eq(assigned_conn, &conn));
                    let mut got = streams.clone();
                    got.sort();
                    assert_eq!(got, vec!["s2".to_string(), "s3".to_string()]);
                });
            }
        }

        mod send_subscription_payload {
            use super::*;

            #[test]
            fn subscribe_payload_with_custom_id_fallbacks_if_invalid() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws: Arc<WebsocketStreams> =
                        create_websocket_streams(Some("ws://example.com"), None);
                    let conn = &ws.common.connection_pool[0];
                    let (tx, mut rx) = unbounded_channel();
                    {
                        let mut st = conn.state.lock().await;
                        st.ws_write_tx = Some(tx);
                    }
                    ws.send_subscription_payload(
                        conn,
                        &vec!["s1".to_string()],
                        Some("badid".to_string()),
                    );
                    let msg = rx.recv().await.expect("no message sent");
                    if let Message::Text(txt) = msg {
                        let v: serde_json::Value = serde_json::from_str(&txt).unwrap();
                        assert_eq!(v["method"], "SUBSCRIBE");
                        let id = v["id"].as_str().unwrap();
                        assert_ne!(id, "badid");
                        assert!(Regex::new(r"^[0-9a-fA-F]{32}$").unwrap().is_match(id));
                    } else {
                        panic!("unexpected message: {msg:?}");
                    }
                });
            }

            #[test]
            fn subscribe_payload_with_and_without_custom_id() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws: Arc<WebsocketStreams> =
                        create_websocket_streams(Some("ws://unused"), None);
                    let conn = &ws.common.connection_pool[0];
                    let (tx, mut rx) = unbounded_channel();
                    {
                        let mut st = conn.state.lock().await;
                        st.ws_write_tx = Some(tx);
                    }
                    ws.send_subscription_payload(
                        conn,
                        &vec!["a".to_string(), "b".to_string()],
                        Some("deadbeefdeadbeefdeadbeefdeadbeef".to_string()),
                    );
                    let msg1 = rx.recv().await.unwrap();
                    ws.send_subscription_payload(conn, &vec!["x".to_string()], None);
                    let msg2 = rx.recv().await.unwrap();

                    if let Message::Text(txt1) = msg1 {
                        let v1: serde_json::Value = serde_json::from_str(&txt1).unwrap();
                        assert_eq!(v1["id"], "deadbeefdeadbeefdeadbeefdeadbeef");
                        assert_eq!(
                            v1["params"].as_array().unwrap(),
                            &vec![serde_json::json!("a"), serde_json::json!("b")]
                        );
                    } else {
                        panic!()
                    }

                    if let Message::Text(txt2) = msg2 {
                        let v2: serde_json::Value = serde_json::from_str(&txt2).unwrap();
                        assert_eq!(v2["method"], "SUBSCRIBE");
                        let params = v2["params"].as_array().unwrap();
                        assert_eq!(params.len(), 1);
                        assert_eq!(params[0], "x");
                        let id2 = v2["id"].as_str().unwrap();
                        assert!(Regex::new(r"^[0-9a-fA-F]{32}$").unwrap().is_match(id2));
                    } else {
                        panic!()
                    }
                });
            }
        }

        mod on_open {
            use super::*;

            #[test]
            fn sends_pending_subscriptions() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws: Arc<WebsocketStreams> =
                        create_websocket_streams(Some("ws://example.com"), None);
                    let conn = &ws.common.connection_pool[0];
                    let (tx, mut rx) = unbounded_channel();
                    {
                        let mut st = conn.state.lock().await;
                        st.ws_write_tx = Some(tx);
                        st.pending_subscriptions.push_back("foo".to_string());
                        st.pending_subscriptions.push_back("bar".to_string());
                    }
                    ws.on_open("ws://example.com".to_string(), conn.clone())
                        .await;
                    let msg = rx.recv().await.expect("no subscription sent");
                    if let Message::Text(txt) = msg {
                        let v: Value = serde_json::from_str(&txt).unwrap();
                        assert_eq!(v["method"], "SUBSCRIBE");
                        let params = v["params"].as_array().unwrap();
                        assert_eq!(
                            params,
                            &vec![Value::String("foo".into()), Value::String("bar".into())]
                        );
                    } else {
                        panic!("unexpected message: {msg:?}");
                    }
                    let st_after = conn.state.lock().await;
                    assert!(st_after.pending_subscriptions.is_empty());
                });
            }

            #[test]
            fn with_no_pending_subscriptions_sends_nothing() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws: Arc<WebsocketStreams> =
                        create_websocket_streams(Some("ws://example.com"), None);
                    let conn = &ws.common.connection_pool[0];
                    let (tx, mut rx) = unbounded_channel();
                    {
                        let mut st = conn.state.lock().await;
                        st.ws_write_tx = Some(tx);
                    }
                    ws.on_open("ws://example.com".to_string(), conn.clone())
                        .await;
                    assert!(rx.try_recv().is_err(), "unexpected message sent");
                });
            }

            #[test]
            fn clears_pending_without_write_channel() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws: Arc<WebsocketStreams> =
                        create_websocket_streams(Some("ws://example.com"), None);
                    let conn = &ws.common.connection_pool[0];
                    {
                        let mut st = conn.state.lock().await;
                        st.pending_subscriptions.push_back("solo".to_string());
                    }
                    ws.on_open("ws://example.com".to_string(), conn.clone())
                        .await;
                    let st_after = conn.state.lock().await;
                    assert!(st_after.pending_subscriptions.is_empty());
                });
            }
        }

        mod on_message {
            use super::*;

            #[test]
            fn invokes_registered_callback() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws: Arc<WebsocketStreams> =
                        create_websocket_streams(Some("ws://example.com"), None);
                    let conn = &ws.common.connection_pool[0];
                    let called = Arc::new(AtomicBool::new(false));
                    let called_clone = called.clone();

                    {
                        let mut st = conn.state.lock().await;
                        st.stream_callbacks
                            .entry("stream1".to_string())
                            .or_default()
                            .push(
                                (Box::new(move |_: &Value| {
                                    called_clone.store(true, Ordering::SeqCst);
                                })
                                    as Box<dyn Fn(&Value) + Send + Sync>)
                                    .into(),
                            );
                    }

                    let msg = json!({
                        "stream": "stream1",
                        "data": { "key": "value" }
                    })
                    .to_string();

                    ws.on_message(msg, conn.clone()).await;

                    assert!(called.load(Ordering::SeqCst));
                });
            }

            #[test]
            fn invokes_all_registered_callbacks() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws: Arc<WebsocketStreams> =
                        create_websocket_streams(Some("ws://example.com"), None);
                    let conn = &ws.common.connection_pool[0];
                    let counter = Arc::new(AtomicUsize::new(0));

                    {
                        let mut st = conn.state.lock().await;
                        let entry = st.stream_callbacks.entry("s".into()).or_default();
                        let c1 = counter.clone();
                        entry.push(
                            (Box::new(move |_: &Value| {
                                c1.fetch_add(1, Ordering::SeqCst);
                            }) as Box<dyn Fn(&Value) + Send + Sync>)
                                .into(),
                        );
                        let c2 = counter.clone();
                        entry.push(
                            (Box::new(move |_: &Value| {
                                c2.fetch_add(1, Ordering::SeqCst);
                            }) as Box<dyn Fn(&Value) + Send + Sync>)
                                .into(),
                        );
                    }

                    let msg = json!({"stream":"s","data":42}).to_string();
                    ws.on_message(msg, conn.clone()).await;

                    assert_eq!(counter.load(Ordering::SeqCst), 2);
                });
            }

            #[test]
            fn handles_null_data_field() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws: Arc<WebsocketStreams> =
                        create_websocket_streams(Some("ws://example.com"), None);
                    let conn = &ws.common.connection_pool[0];
                    let called = Arc::new(AtomicUsize::new(0));
                    {
                        let mut st = conn.state.lock().await;
                        st.stream_callbacks.entry("n".into()).or_default().push(
                            (Box::new({
                                let c = called.clone();
                                move |data: &Value| {
                                    if data.is_null() {
                                        c.fetch_add(1, Ordering::SeqCst);
                                    }
                                }
                            }) as Box<dyn Fn(&Value) + Send + Sync>)
                                .into(),
                        );
                    }
                    let msg = json!({"stream":"n","data":null}).to_string();
                    ws.on_message(msg, conn.clone()).await;
                    assert_eq!(called.load(Ordering::SeqCst), 1);
                });
            }

            #[test]
            fn with_invalid_json_does_not_panic() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws: Arc<WebsocketStreams> =
                        create_websocket_streams(Some("ws://example.com"), None);
                    let conn = &ws.common.connection_pool[0];
                    let bad = "not a json";
                    ws.on_message(bad.to_string(), conn.clone()).await;
                });
            }

            #[test]
            fn without_stream_field_does_nothing() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws: Arc<WebsocketStreams> =
                        create_websocket_streams(Some("ws://example.com"), None);
                    let conn = &ws.common.connection_pool[0];
                    let msg = json!({ "data": { "foo": 1 } }).to_string();
                    ws.on_message(msg, conn.clone()).await;
                });
            }

            #[test]
            fn with_unregistered_stream_does_not_panic() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws: Arc<WebsocketStreams> =
                        create_websocket_streams(Some("ws://example.com"), None);
                    let conn = &ws.common.connection_pool[0];
                    let msg = json!({
                        "stream": "nope",
                        "data": { "foo": 1 }
                    })
                    .to_string();
                    ws.on_message(msg, conn.clone()).await;
                });
            }
        }

        mod get_reconnect_url {
            use super::*;

            #[test]
            fn single_stream_reconnect_url() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws: Arc<WebsocketStreams> =
                        create_websocket_streams(Some("ws://example.com"), None);
                    let c0 = ws.common.connection_pool[0].clone();
                    {
                        let mut map = ws.connection_streams.lock().await;
                        map.insert("s1".to_string(), c0.clone());
                    }
                    let url = ws.get_reconnect_url("default_url".into(), c0).await;
                    assert_eq!(url, "ws://example.com/stream?streams=s1");
                });
            }

            #[test]
            fn multiple_streams_same_connection() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws: Arc<WebsocketStreams> =
                        create_websocket_streams(Some("ws://example.com"), None);
                    let c0 = ws.common.connection_pool[0].clone();
                    {
                        let mut map = ws.connection_streams.lock().await;
                        map.insert("a".to_string(), c0.clone());
                        map.insert("b".to_string(), c0.clone());
                    }
                    let url = ws.get_reconnect_url("default_url".into(), c0).await;
                    let suffix = url
                        .strip_prefix("ws://example.com/stream?streams=")
                        .unwrap();
                    let parts: Vec<_> = suffix.split('&').next().unwrap().split('/').collect();
                    let set = parts.into_iter().collect::<std::collections::HashSet<_>>();
                    assert_eq!(set, ["a", "b"].iter().copied().collect());
                });
            }

            #[test]
            fn reconnect_url_with_time_unit() {
                TOKIO_SHARED_RT.block_on(async {
                    let mut ws: Arc<WebsocketStreams> =
                        create_websocket_streams(Some("ws://example.com"), None);
                    Arc::get_mut(&mut ws).unwrap().configuration.time_unit =
                        Some(TimeUnit::Microsecond);
                    let c0 = ws.common.connection_pool[0].clone();
                    {
                        let mut map = ws.connection_streams.lock().await;
                        map.insert("x".to_string(), c0.clone());
                    }
                    let url = ws.get_reconnect_url("default_url".into(), c0).await;
                    assert_eq!(
                        url,
                        "ws://example.com/stream?streams=x&timeUnit=microsecond"
                    );
                });
            }
        }
    }

    mod websocket_stream {
        use super::*;

        mod on {
            use super::*;

            #[test]
            fn registers_callback_and_stream_callback_for_websocket_streams() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws_base = create_websocket_streams(Some("example.com"), None);
                    let stream_name = "s1".to_string();
                    let conn = ws_base.common.connection_pool[0].clone();
                    {
                        let mut map = ws_base.connection_streams.lock().await;
                        map.insert(stream_name.clone(), conn.clone());
                    }
                    {
                        let mut state = conn.state.lock().await;
                        state
                            .stream_callbacks
                            .insert(stream_name.clone(), Vec::new());
                    }
                    let stream = Arc::new(WebsocketStream::<Value> {
                        websocket_base: WebsocketBase::WebsocketStreams(ws_base.clone()),
                        stream_or_id: stream_name.clone(),
                        callback: Mutex::new(None),
                        id: None,
                        _phantom: PhantomData,
                    });
                    let called = Arc::new(Mutex::new(false));
                    let called_clone = called.clone();
                    stream
                        .on("message", move |v: Value| {
                            let mut lock = called_clone.blocking_lock();
                            *lock = v == Value::String("x".into());
                        })
                        .await;
                    let cb_guard = stream.callback.lock().await;
                    assert!(cb_guard.is_some());
                    let cbs = {
                        let state = conn.state.lock().await;
                        state.stream_callbacks.get(&stream_name).unwrap().clone()
                    };
                    assert_eq!(cbs.len(), 1);
                });
            }

            #[test]
            fn message_twice_registers_two_wrappers_for_websocket_streams() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws_base = create_websocket_streams(Some("example.com"), None);
                    let stream_name = "s2".to_string();
                    let conn = ws_base.common.connection_pool[0].clone();
                    {
                        let mut map = ws_base.connection_streams.lock().await;
                        map.insert(stream_name.clone(), conn.clone());
                    }
                    {
                        let mut state = conn.state.lock().await;
                        state
                            .stream_callbacks
                            .insert(stream_name.clone(), Vec::new());
                    }
                    let stream = Arc::new(WebsocketStream::<Value> {
                        websocket_base: WebsocketBase::WebsocketStreams(ws_base.clone()),
                        stream_or_id: stream_name.clone(),
                        callback: Mutex::new(None),
                        id: None,
                        _phantom: PhantomData,
                    });
                    stream.on("message", |_| {}).await;
                    stream.on("message", |_| {}).await;
                    let state = conn.state.lock().await;
                    let callbacks = state.stream_callbacks.get(&stream_name).unwrap();
                    assert_eq!(callbacks.len(), 2);
                });
            }

            #[test]
            fn ignores_non_message_event_for_websocket_streams() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws_base = create_websocket_streams(Some("example.com"), None);
                    let stream = Arc::new(WebsocketStream::<Value> {
                        websocket_base: WebsocketBase::WebsocketStreams(ws_base.clone()),
                        stream_or_id: "s".into(),
                        callback: Mutex::new(None),
                        id: None,
                        _phantom: PhantomData,
                    });
                    stream.on("open", |_| {}).await;
                    let guard = stream.callback.lock().await;
                    assert!(guard.is_none());
                });
            }

            #[test]
            fn registers_callback_and_stream_callback_for_websocket_api() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws_base = create_websocket_api(None, None, None);

                    {
                        let mut stream_callbacks = ws_base.stream_callbacks.lock().await;
                        stream_callbacks.insert("id1".to_string(), Vec::new());
                    }

                    let stream = Arc::new(WebsocketStream::<Value> {
                        websocket_base: WebsocketBase::WebsocketApi(ws_base.clone()),
                        stream_or_id: "id1".to_string(),
                        callback: Mutex::new(None),
                        id: None,
                        _phantom: PhantomData,
                    });

                    let called = Arc::new(Mutex::new(false));
                    let called_clone = called.clone();
                    stream
                        .on("message", move |v: Value| {
                            let mut lock = called_clone.blocking_lock();
                            *lock = v == Value::String("x".into());
                        })
                        .await;

                    let cb_guard = stream.callback.lock().await;
                    assert!(cb_guard.is_some());

                    let stream_callbacks = ws_base.stream_callbacks.lock().await;
                    let callbacks = stream_callbacks.get("id1").unwrap();
                    assert_eq!(callbacks.len(), 1);
                });
            }

            #[test]
            fn message_twice_registers_two_wrappers_for_websocket_api() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws_base = create_websocket_api(None, None, None);

                    let stream = Arc::new(WebsocketStream::<Value> {
                        websocket_base: WebsocketBase::WebsocketApi(ws_base.clone()),
                        stream_or_id: "id2".to_string(),
                        callback: Mutex::new(None),
                        id: None,
                        _phantom: PhantomData,
                    });

                    stream.on("message", |_| {}).await;
                    stream.on("message", |_| {}).await;

                    let stream_callbacks = ws_base.stream_callbacks.lock().await;
                    let callbacks = stream_callbacks.get("id2").unwrap();
                    assert_eq!(callbacks.len(), 2);
                });
            }

            #[test]
            fn ignores_non_message_event_for_websocket_api() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws_base = create_websocket_api(None, None, None);

                    let stream = Arc::new(WebsocketStream::<Value> {
                        websocket_base: WebsocketBase::WebsocketApi(ws_base.clone()),
                        stream_or_id: "id3".into(),
                        callback: Mutex::new(None),
                        id: None,
                        _phantom: PhantomData,
                    });

                    stream.on("open", |_| {}).await;

                    let guard = stream.callback.lock().await;
                    assert!(guard.is_none());

                    let stream_callbacks = ws_base.stream_callbacks.lock().await;
                    assert!(stream_callbacks.get("id3").is_none());
                    assert!(stream_callbacks.is_empty());
                });
            }
        }

        mod on_message {
            use super::*;

            #[test]
            fn on_message_registers_callback_for_websocket_streams() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws_base = create_websocket_streams(Some("example.com"), None);
                    let stream_name = "s".to_string();
                    let conn = ws_base.common.connection_pool[0].clone();
                    {
                        let mut map = ws_base.connection_streams.lock().await;
                        map.insert(stream_name.clone(), conn.clone());
                    }
                    {
                        let mut state = conn.state.lock().await;
                        state
                            .stream_callbacks
                            .insert(stream_name.clone(), Vec::new());
                    }
                    let stream = Arc::new(WebsocketStream::<Value> {
                        websocket_base: WebsocketBase::WebsocketStreams(ws_base.clone()),
                        stream_or_id: stream_name.clone(),
                        callback: Mutex::new(None),
                        id: None,
                        _phantom: PhantomData,
                    });
                    stream.on_message(|_v| {});
                    let callbacks = &conn.state.lock().await.stream_callbacks[&stream_name];
                    assert_eq!(callbacks.len(), 1);
                });
            }

            #[test]
            fn on_message_twice_registers_two_callbacks_for_websocket_streams() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws_base = create_websocket_streams(Some("example.com"), None);
                    let stream_name = "s".to_string();
                    let conn = ws_base.common.connection_pool[0].clone();
                    {
                        let mut map = ws_base.connection_streams.lock().await;
                        map.insert(stream_name.clone(), conn.clone());
                    }
                    {
                        let mut state = conn.state.lock().await;
                        state
                            .stream_callbacks
                            .insert(stream_name.clone(), Vec::new());
                    }
                    let stream = Arc::new(WebsocketStream::<Value> {
                        websocket_base: WebsocketBase::WebsocketStreams(ws_base.clone()),
                        stream_or_id: stream_name.clone(),
                        callback: Mutex::new(None),
                        id: None,
                        _phantom: PhantomData,
                    });
                    stream.on_message(|_v| {});
                    stream.on_message(|_v| {});
                    let callbacks = &conn.state.lock().await.stream_callbacks[&stream_name];
                    assert_eq!(callbacks.len(), 2);
                });
            }

            #[test]
            fn on_message_registers_callback_for_websocket_api() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws_base = create_websocket_api(None, None, None);
                    let identifier = "id1".to_string();

                    let stream = Arc::new(WebsocketStream::<Value> {
                        websocket_base: WebsocketBase::WebsocketApi(ws_base.clone()),
                        stream_or_id: identifier.clone(),
                        callback: Mutex::new(None),
                        id: None,
                        _phantom: PhantomData,
                    });

                    stream.on_message(|_v: Value| {});

                    let stream_callbacks = ws_base.stream_callbacks.lock().await;
                    let callbacks = stream_callbacks.get(&identifier).unwrap();
                    assert_eq!(callbacks.len(), 1);
                });
            }

            #[test]
            fn on_message_twice_registers_two_callbacks_for_websocket_api() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws_base = create_websocket_api(None, None, None);
                    let identifier = "id2".to_string();

                    let stream = Arc::new(WebsocketStream::<Value> {
                        websocket_base: WebsocketBase::WebsocketApi(ws_base.clone()),
                        stream_or_id: identifier.clone(),
                        callback: Mutex::new(None),
                        id: None,
                        _phantom: PhantomData,
                    });

                    stream.on_message(|_v: Value| {});
                    stream.on_message(|_v: Value| {});

                    let stream_callbacks = ws_base.stream_callbacks.lock().await;
                    let callbacks = stream_callbacks.get(&identifier).unwrap();
                    assert_eq!(callbacks.len(), 2);
                });
            }
        }

        mod unsubscribe {
            use super::*;

            #[test]
            fn without_callback_does_nothing() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws_base = create_websocket_streams(Some("example.com"), None);
                    let stream_name = "s1".to_string();
                    let conn = ws_base.common.connection_pool[0].clone();
                    {
                        let mut map = ws_base.connection_streams.lock().await;
                        map.insert(stream_name.clone(), conn.clone());
                    }
                    let mut state = conn.state.lock().await;
                    state.stream_callbacks.insert(stream_name.clone(), vec![]);
                    drop(state);
                    let stream = Arc::new(WebsocketStream::<Value> {
                        websocket_base: WebsocketBase::WebsocketStreams(ws_base.clone()),
                        stream_or_id: stream_name.clone(),
                        callback: Mutex::new(None),
                        id: None,
                        _phantom: PhantomData,
                    });
                    stream.unsubscribe().await;
                    let state = conn.state.lock().await;
                    assert!(state.stream_callbacks.contains_key(&stream_name));
                });
            }

            #[test]
            fn removes_registered_callback_and_clears_state() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws_base = create_websocket_streams(Some("example.com"), None);
                    let stream_name = "s2".to_string();
                    let conn = ws_base.common.connection_pool[0].clone();
                    {
                        let mut map = ws_base.connection_streams.lock().await;
                        map.insert(stream_name.clone(), conn.clone());
                    }
                    {
                        let mut state = conn.state.lock().await;
                        state
                            .stream_callbacks
                            .insert(stream_name.clone(), Vec::new());
                    }
                    let stream = Arc::new(WebsocketStream::<Value> {
                        websocket_base: WebsocketBase::WebsocketStreams(ws_base.clone()),
                        stream_or_id: stream_name.clone(),
                        callback: Mutex::new(None),
                        id: None,
                        _phantom: PhantomData,
                    });
                    stream.on("message", |_| {}).await;
                    {
                        let guard = stream.callback.lock().await;
                        assert!(guard.is_some());
                    }
                    stream.unsubscribe().await;
                    sleep(Duration::from_millis(10)).await;
                    let guard = stream.callback.lock().await;
                    assert!(guard.is_none());
                    let state = conn.state.lock().await;
                    assert!(
                        state
                            .stream_callbacks
                            .get(&stream_name)
                            .is_none_or(std::vec::Vec::is_empty)
                    );
                });
            }

            #[test]
            fn without_callback_does_nothing_for_websocket_api() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws_base = create_websocket_api(None, None, None);
                    let identifier = "id1".to_string();

                    {
                        let mut stream_callbacks = ws_base.stream_callbacks.lock().await;
                        stream_callbacks.insert(identifier.clone(), Vec::new());
                    }

                    let stream = Arc::new(WebsocketStream::<Value> {
                        websocket_base: WebsocketBase::WebsocketApi(ws_base.clone()),
                        stream_or_id: identifier.clone(),
                        callback: Mutex::new(None),
                        id: None,
                        _phantom: PhantomData,
                    });

                    stream.unsubscribe().await;

                    let stream_callbacks = ws_base.stream_callbacks.lock().await;
                    assert!(stream_callbacks.contains_key(&identifier));
                    let callbacks = stream_callbacks.get(&identifier).unwrap();
                    assert!(callbacks.is_empty());
                });
            }

            #[test]
            fn removes_registered_callback_and_clears_state_for_websocket_api() {
                TOKIO_SHARED_RT.block_on(async {
                    let ws_base = create_websocket_api(None, None, None);
                    let identifier = "id2".to_string();

                    {
                        let mut stream_callbacks = ws_base.stream_callbacks.lock().await;
                        stream_callbacks.insert(identifier.clone(), Vec::new());
                    }

                    let stream = Arc::new(WebsocketStream::<Value> {
                        websocket_base: WebsocketBase::WebsocketApi(ws_base.clone()),
                        stream_or_id: identifier.clone(),
                        callback: Mutex::new(None),
                        id: None,
                        _phantom: PhantomData,
                    });

                    stream.on("message", |_| {}).await;

                    {
                        let stream_callbacks = ws_base.stream_callbacks.lock().await;
                        let callbacks = stream_callbacks
                            .get(&identifier)
                            .expect("Entry for 'id2' should exist");
                        assert_eq!(callbacks.len(), 1);
                    }

                    stream.unsubscribe().await;

                    {
                        let guard = stream.callback.lock().await;
                        assert!(guard.is_none());
                    }

                    {
                        let stream_callbacks = ws_base.stream_callbacks.lock().await;
                        let callbacks = stream_callbacks
                            .get(&identifier)
                            .expect("Entry for 'id2' should still exist");
                        assert!(callbacks.is_empty());
                    }
                });
            }
        }
    }

    mod create_stream_handler {
        use super::*;

        #[test]
        fn create_stream_handler_without_id_registers_stream() {
            TOKIO_SHARED_RT.block_on(async {
                let ws = create_websocket_streams(Some("ws://example.com"), None);
                let stream_name = "foo".to_string();
                let handler = create_stream_handler::<serde_json::Value>(
                    WebsocketBase::WebsocketStreams(ws.clone()),
                    stream_name.clone(),
                    None,
                )
                .await;
                assert_eq!(handler.stream_or_id, stream_name);
                assert!(handler.id.is_none());
                let map = ws.connection_streams.lock().await;
                assert!(map.contains_key(&stream_name));
            });
        }

        #[test]
        fn create_stream_handler_with_custom_id_registers_stream_and_id() {
            TOKIO_SHARED_RT.block_on(async {
                let ws = create_websocket_streams(Some("ws://example.com"), None);
                let stream_name = "bar".to_string();
                let custom_id = Some("my-custom-id".to_string());
                let handler = create_stream_handler::<serde_json::Value>(
                    WebsocketBase::WebsocketStreams(ws.clone()),
                    stream_name.clone(),
                    custom_id.clone(),
                )
                .await;
                assert_eq!(handler.stream_or_id, stream_name);
                assert_eq!(handler.id, custom_id);
                let map = ws.connection_streams.lock().await;
                assert!(map.contains_key(&stream_name));
            });
        }

        #[test]
        fn create_stream_handler_without_id_registers_api_stream() {
            TOKIO_SHARED_RT.block_on(async {
                let ws_base = create_websocket_api(None, None, None);
                let identifier = "foo-api".to_string();

                let handler = create_stream_handler::<Value>(
                    WebsocketBase::WebsocketApi(ws_base.clone()),
                    identifier.clone(),
                    None,
                )
                .await;

                assert_eq!(handler.stream_or_id, identifier);
                assert!(handler.id.is_none());
            });
        }

        #[test]
        fn create_stream_handler_with_custom_id_registers_api_stream_and_id() {
            TOKIO_SHARED_RT.block_on(async {
                let ws_base = create_websocket_api(None, None, None);
                let identifier = "bar-api".to_string();
                let custom_id = Some("custom-123".to_string());

                let handler = create_stream_handler::<Value>(
                    WebsocketBase::WebsocketApi(ws_base.clone()),
                    identifier.clone(),
                    custom_id.clone(),
                )
                .await;

                assert_eq!(handler.stream_or_id, identifier);
                assert_eq!(handler.id, custom_id);
            });
        }
    }

    mod websocket_connection_failure_reason {
        use super::*;
        use std::io::{Error as IoError, ErrorKind};
        use tokio_tungstenite::tungstenite::Error as TungsteniteError;

        #[test]
        fn from_tungstenite_error_classifies_connection_closed() {
            let error = TungsteniteError::ConnectionClosed;
            let reason = WebsocketConnectionFailureReason::from_tungstenite_error(&error);
            assert!(matches!(
                reason,
                WebsocketConnectionFailureReason::ConnectionReset
            ));
            assert!(reason.should_reconnect());
        }

        #[test]
        fn from_tungstenite_error_classifies_already_closed() {
            let error = TungsteniteError::AlreadyClosed;
            let reason = WebsocketConnectionFailureReason::from_tungstenite_error(&error);
            assert!(matches!(
                reason,
                WebsocketConnectionFailureReason::ConnectionReset
            ));
            assert!(reason.should_reconnect());
        }

        #[test]
        fn from_tungstenite_error_classifies_io_errors() {
            // Test ConnectionReset
            let io_error = IoError::new(ErrorKind::ConnectionReset, "connection reset");
            let error = TungsteniteError::Io(io_error);
            let reason = WebsocketConnectionFailureReason::from_tungstenite_error(&error);
            assert!(matches!(
                reason,
                WebsocketConnectionFailureReason::ConnectionReset
            ));
            assert!(reason.should_reconnect());

            // Test ConnectionAborted
            let io_error = IoError::new(ErrorKind::ConnectionAborted, "connection aborted");
            let error = TungsteniteError::Io(io_error);
            let reason = WebsocketConnectionFailureReason::from_tungstenite_error(&error);
            assert!(matches!(
                reason,
                WebsocketConnectionFailureReason::ConnectionReset
            ));
            assert!(reason.should_reconnect());

            // Test TimedOut
            let io_error = IoError::new(ErrorKind::TimedOut, "timed out");
            let error = TungsteniteError::Io(io_error);
            let reason = WebsocketConnectionFailureReason::from_tungstenite_error(&error);
            assert!(matches!(
                reason,
                WebsocketConnectionFailureReason::NetworkInterruption
            ));
            assert!(reason.should_reconnect());

            // Test UnexpectedEof
            let io_error = IoError::new(ErrorKind::UnexpectedEof, "unexpected eof");
            let error = TungsteniteError::Io(io_error);
            let reason = WebsocketConnectionFailureReason::from_tungstenite_error(&error);
            assert!(matches!(
                reason,
                WebsocketConnectionFailureReason::StreamEnded
            ));
            assert!(reason.should_reconnect());

            // Test PermissionDenied
            let io_error = IoError::new(ErrorKind::PermissionDenied, "permission denied");
            let error = TungsteniteError::Io(io_error);
            let reason = WebsocketConnectionFailureReason::from_tungstenite_error(&error);
            assert!(matches!(
                reason,
                WebsocketConnectionFailureReason::AuthenticationFailure
            ));
            assert!(!reason.should_reconnect());

            // Test other IO errors default to NetworkInterruption
            let io_error = IoError::other("other error");
            let error = TungsteniteError::Io(io_error);
            let reason = WebsocketConnectionFailureReason::from_tungstenite_error(&error);
            assert!(matches!(
                reason,
                WebsocketConnectionFailureReason::NetworkInterruption
            ));
            assert!(reason.should_reconnect());
        }

        #[test]
        fn from_tungstenite_error_classifies_protocol_errors() {
            // Protocol error -> ProtocolViolation
            use tokio_tungstenite::tungstenite::error::ProtocolError;
            let protocol_error = ProtocolError::ResetWithoutClosingHandshake;
            let error = TungsteniteError::Protocol(protocol_error);
            let reason = WebsocketConnectionFailureReason::from_tungstenite_error(&error);
            assert!(matches!(
                reason,
                WebsocketConnectionFailureReason::ProtocolViolation
            ));
            assert!(!reason.should_reconnect());

            // UTF8 error -> ProtocolViolation
            let error = TungsteniteError::Utf8;
            let reason = WebsocketConnectionFailureReason::from_tungstenite_error(&error);
            assert!(matches!(
                reason,
                WebsocketConnectionFailureReason::ProtocolViolation
            ));
            assert!(!reason.should_reconnect());
        }

        #[test]
        fn from_close_code_classifies_standard_codes() {
            // Normal closure
            let reason = WebsocketConnectionFailureReason::from_close_code(1000, false);
            assert!(matches!(
                reason,
                WebsocketConnectionFailureReason::NormalClose
            ));
            assert!(!reason.should_reconnect());

            // Going away (server restart) -> ServerTemporaryError
            let reason = WebsocketConnectionFailureReason::from_close_code(1001, false);
            assert!(matches!(
                reason,
                WebsocketConnectionFailureReason::ServerTemporaryError
            ));
            assert!(reason.should_reconnect());

            // Protocol error -> ProtocolViolation
            let reason = WebsocketConnectionFailureReason::from_close_code(1002, false);
            assert!(matches!(
                reason,
                WebsocketConnectionFailureReason::ProtocolViolation
            ));
            assert!(!reason.should_reconnect());

            // Abnormal closure -> UnexpectedClose
            let reason = WebsocketConnectionFailureReason::from_close_code(1006, false);
            assert!(matches!(
                reason,
                WebsocketConnectionFailureReason::UnexpectedClose
            ));
            assert!(reason.should_reconnect());

            // Policy violation -> PermanentServerError
            let reason = WebsocketConnectionFailureReason::from_close_code(1008, false);
            assert!(matches!(
                reason,
                WebsocketConnectionFailureReason::PermanentServerError
            ));
            assert!(!reason.should_reconnect());

            // Server error -> ServerTemporaryError
            let reason = WebsocketConnectionFailureReason::from_close_code(1011, false);
            assert!(matches!(
                reason,
                WebsocketConnectionFailureReason::ServerTemporaryError
            ));
            assert!(reason.should_reconnect());

            // TLS handshake failure -> ConfigurationError
            let reason = WebsocketConnectionFailureReason::from_close_code(1015, false);
            assert!(matches!(
                reason,
                WebsocketConnectionFailureReason::ConfigurationError
            ));
            assert!(!reason.should_reconnect());

            // Business/application errors -> PermanentServerError
            let reason = WebsocketConnectionFailureReason::from_close_code(4000, false);
            assert!(matches!(
                reason,
                WebsocketConnectionFailureReason::PermanentServerError
            ));
            assert!(!reason.should_reconnect());

            let reason = WebsocketConnectionFailureReason::from_close_code(4999, false);
            assert!(matches!(
                reason,
                WebsocketConnectionFailureReason::PermanentServerError
            ));
            assert!(!reason.should_reconnect());

            // Unknown codes default to UnexpectedClose
            let reason = WebsocketConnectionFailureReason::from_close_code(9999, false);
            assert!(matches!(
                reason,
                WebsocketConnectionFailureReason::UnexpectedClose
            ));
            assert!(reason.should_reconnect());
        }

        #[test]
        fn from_close_code_handles_user_initiated() {
            // Any code with user_initiated=true should return UserInitiatedClose
            let reason = WebsocketConnectionFailureReason::from_close_code(1000, true);
            assert!(matches!(
                reason,
                WebsocketConnectionFailureReason::UserInitiatedClose
            ));
            assert!(!reason.should_reconnect());

            let reason = WebsocketConnectionFailureReason::from_close_code(1006, true);
            assert!(matches!(
                reason,
                WebsocketConnectionFailureReason::UserInitiatedClose
            ));
            assert!(!reason.should_reconnect());

            let reason = WebsocketConnectionFailureReason::from_close_code(4000, true);
            assert!(matches!(
                reason,
                WebsocketConnectionFailureReason::UserInitiatedClose
            ));
            assert!(!reason.should_reconnect());
        }

        #[test]
        fn should_reconnect_logic() {
            // Reconnectable failures
            assert!(WebsocketConnectionFailureReason::NetworkInterruption.should_reconnect());
            assert!(WebsocketConnectionFailureReason::ConnectionReset.should_reconnect());
            assert!(WebsocketConnectionFailureReason::ServerTemporaryError.should_reconnect());
            assert!(WebsocketConnectionFailureReason::UnexpectedClose.should_reconnect());
            assert!(WebsocketConnectionFailureReason::StreamEnded.should_reconnect());

            // Non-reconnectable failures
            assert!(!WebsocketConnectionFailureReason::AuthenticationFailure.should_reconnect());
            assert!(!WebsocketConnectionFailureReason::ProtocolViolation.should_reconnect());
            assert!(!WebsocketConnectionFailureReason::ConfigurationError.should_reconnect());
            assert!(!WebsocketConnectionFailureReason::UserInitiatedClose.should_reconnect());
            assert!(!WebsocketConnectionFailureReason::PermanentServerError.should_reconnect());
            assert!(!WebsocketConnectionFailureReason::NormalClose.should_reconnect());
        }

        #[test]
        fn debug_and_clone_work() {
            let reason = WebsocketConnectionFailureReason::NetworkInterruption;
            let cloned = reason;
            let debug_str = format!("{:?}", reason);

            assert!(matches!(
                cloned,
                WebsocketConnectionFailureReason::NetworkInterruption
            ));
            assert!(debug_str.contains("NetworkInterruption"));
        }
    }
}
