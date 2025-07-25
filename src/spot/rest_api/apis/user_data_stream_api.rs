/*
 * Binance Spot REST API
 *
 * OpenAPI Specifications for the Binance Spot REST API
 *
 * API documents:
 * - [Github rest-api documentation file](https://github.com/binance/binance-spot-api-docs/blob/master/rest-api.md)
 * - [General API information for rest-api on website](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/general-api-information)
 *
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use async_trait::async_trait;
use derive_builder::Builder;
use reqwest;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeMap;

use crate::common::{
    config::ConfigurationRestApi,
    models::{ParamBuildError, RestApiResponse},
    utils::send_request,
};
use crate::spot::rest_api::models;

const HAS_TIME_UNIT: bool = true;

#[async_trait]
pub trait UserDataStreamApi: Send + Sync {
    async fn delete_user_data_stream(
        &self,
        params: DeleteUserDataStreamParams,
    ) -> anyhow::Result<RestApiResponse<Value>>;
    async fn new_user_data_stream(
        &self,
    ) -> anyhow::Result<RestApiResponse<models::NewUserDataStreamResponse>>;
    async fn put_user_data_stream(
        &self,
        params: PutUserDataStreamParams,
    ) -> anyhow::Result<RestApiResponse<Value>>;
}

#[derive(Debug, Clone)]
pub struct UserDataStreamApiClient {
    configuration: ConfigurationRestApi,
}

impl UserDataStreamApiClient {
    pub fn new(configuration: ConfigurationRestApi) -> Self {
        Self { configuration }
    }
}

/// Request parameters for the [`delete_user_data_stream`] operation.
///
/// This struct holds all of the inputs you can pass when calling
/// [`delete_user_data_stream`](#method.delete_user_data_stream).
#[derive(Clone, Debug, Builder)]
#[builder(pattern = "owned", build_fn(error = "ParamBuildError"))]
pub struct DeleteUserDataStreamParams {
    ///
    /// The `listen_key` parameter.
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub listen_key: String,
}

impl DeleteUserDataStreamParams {
    /// Create a builder for [`delete_user_data_stream`].
    ///
    /// Required parameters:
    ///
    /// * `listen_key` — String
    ///
    #[must_use]
    pub fn builder(listen_key: String) -> DeleteUserDataStreamParamsBuilder {
        DeleteUserDataStreamParamsBuilder::default().listen_key(listen_key)
    }
}
/// Request parameters for the [`put_user_data_stream`] operation.
///
/// This struct holds all of the inputs you can pass when calling
/// [`put_user_data_stream`](#method.put_user_data_stream).
#[derive(Clone, Debug, Builder)]
#[builder(pattern = "owned", build_fn(error = "ParamBuildError"))]
pub struct PutUserDataStreamParams {
    ///
    /// The `listen_key` parameter.
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub listen_key: String,
}

impl PutUserDataStreamParams {
    /// Create a builder for [`put_user_data_stream`].
    ///
    /// Required parameters:
    ///
    /// * `listen_key` — String
    ///
    #[must_use]
    pub fn builder(listen_key: String) -> PutUserDataStreamParamsBuilder {
        PutUserDataStreamParamsBuilder::default().listen_key(listen_key)
    }
}

#[async_trait]
impl UserDataStreamApi for UserDataStreamApiClient {
    async fn delete_user_data_stream(
        &self,
        params: DeleteUserDataStreamParams,
    ) -> anyhow::Result<RestApiResponse<Value>> {
        let DeleteUserDataStreamParams { listen_key } = params;

        let mut query_params = BTreeMap::new();

        query_params.insert("listenKey".to_string(), json!(listen_key));

        send_request::<Value>(
            &self.configuration,
            "/api/v3/userDataStream",
            reqwest::Method::DELETE,
            query_params,
            if HAS_TIME_UNIT {
                self.configuration.time_unit
            } else {
                None
            },
            false,
        )
        .await
    }

    async fn new_user_data_stream(
        &self,
    ) -> anyhow::Result<RestApiResponse<models::NewUserDataStreamResponse>> {
        let query_params = BTreeMap::new();

        send_request::<models::NewUserDataStreamResponse>(
            &self.configuration,
            "/api/v3/userDataStream",
            reqwest::Method::POST,
            query_params,
            if HAS_TIME_UNIT {
                self.configuration.time_unit
            } else {
                None
            },
            false,
        )
        .await
    }

    async fn put_user_data_stream(
        &self,
        params: PutUserDataStreamParams,
    ) -> anyhow::Result<RestApiResponse<Value>> {
        let PutUserDataStreamParams { listen_key } = params;

        let mut query_params = BTreeMap::new();

        query_params.insert("listenKey".to_string(), json!(listen_key));

        send_request::<Value>(
            &self.configuration,
            "/api/v3/userDataStream",
            reqwest::Method::PUT,
            query_params,
            if HAS_TIME_UNIT {
                self.configuration.time_unit
            } else {
                None
            },
            false,
        )
        .await
    }
}

#[cfg(all(test, feature = "spot"))]
mod tests {
    use super::*;
    use crate::TOKIO_SHARED_RT;
    use crate::{errors::ConnectorError, models::DataFuture, models::RestApiRateLimit};
    use async_trait::async_trait;
    use std::collections::HashMap;

    struct DummyRestApiResponse<T> {
        inner: Box<dyn FnOnce() -> DataFuture<Result<T, ConnectorError>> + Send + Sync>,
        status: u16,
        headers: HashMap<String, String>,
        rate_limits: Option<Vec<RestApiRateLimit>>,
    }

    impl<T> From<DummyRestApiResponse<T>> for RestApiResponse<T> {
        fn from(dummy: DummyRestApiResponse<T>) -> Self {
            Self {
                data_fn: dummy.inner,
                status: dummy.status,
                headers: dummy.headers,
                rate_limits: dummy.rate_limits,
            }
        }
    }

    struct MockUserDataStreamApiClient {
        force_error: bool,
    }

    #[async_trait]
    impl UserDataStreamApi for MockUserDataStreamApiClient {
        async fn delete_user_data_stream(
            &self,
            _params: DeleteUserDataStreamParams,
        ) -> anyhow::Result<RestApiResponse<Value>> {
            if self.force_error {
                return Err(
                    ConnectorError::ConnectorClientError("ResponseError".to_string()).into(),
                );
            }

            let dummy_response = Value::Null;

            let dummy = DummyRestApiResponse {
                inner: Box::new(move || Box::pin(async move { Ok(dummy_response) })),
                status: 200,
                headers: HashMap::new(),
                rate_limits: None,
            };

            Ok(dummy.into())
        }

        async fn new_user_data_stream(
            &self,
        ) -> anyhow::Result<RestApiResponse<models::NewUserDataStreamResponse>> {
            if self.force_error {
                return Err(
                    ConnectorError::ConnectorClientError("ResponseError".to_string()).into(),
                );
            }

            let resp_json: Value = serde_json::from_str(r#"{"listenKey":"pqia91ma19a5s61cv6a81va65sdf19v8a65a1a5s61cv6a81va65sdf19v8a65a1"}"#).unwrap();
            let dummy_response: models::NewUserDataStreamResponse =
                serde_json::from_value(resp_json.clone())
                    .expect("should parse into models::NewUserDataStreamResponse");

            let dummy = DummyRestApiResponse {
                inner: Box::new(move || Box::pin(async move { Ok(dummy_response) })),
                status: 200,
                headers: HashMap::new(),
                rate_limits: None,
            };

            Ok(dummy.into())
        }

        async fn put_user_data_stream(
            &self,
            _params: PutUserDataStreamParams,
        ) -> anyhow::Result<RestApiResponse<Value>> {
            if self.force_error {
                return Err(
                    ConnectorError::ConnectorClientError("ResponseError".to_string()).into(),
                );
            }

            let dummy_response = Value::Null;

            let dummy = DummyRestApiResponse {
                inner: Box::new(move || Box::pin(async move { Ok(dummy_response) })),
                status: 200,
                headers: HashMap::new(),
                rate_limits: None,
            };

            Ok(dummy.into())
        }
    }

    #[test]
    fn delete_user_data_stream_required_params_success() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockUserDataStreamApiClient { force_error: false };

            let params = DeleteUserDataStreamParams::builder("listenKey".to_string())
                .build()
                .unwrap();

            let expected_response = Value::Null;

            let resp = client
                .delete_user_data_stream(params)
                .await
                .expect("Expected a response");
            let data_future = resp.data();
            let actual_response = data_future.await.unwrap();
            assert_eq!(actual_response, expected_response);
        });
    }

    #[test]
    fn delete_user_data_stream_optional_params_success() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockUserDataStreamApiClient { force_error: false };

            let params = DeleteUserDataStreamParams::builder("listenKey".to_string())
                .build()
                .unwrap();

            let expected_response = Value::Null;

            let resp = client
                .delete_user_data_stream(params)
                .await
                .expect("Expected a response");
            let data_future = resp.data();
            let actual_response = data_future.await.unwrap();
            assert_eq!(actual_response, expected_response);
        });
    }

    #[test]
    fn delete_user_data_stream_response_error() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockUserDataStreamApiClient { force_error: true };

            let params = DeleteUserDataStreamParams::builder("listenKey".to_string())
                .build()
                .unwrap();

            match client.delete_user_data_stream(params).await {
                Ok(_) => panic!("Expected an error"),
                Err(err) => {
                    assert_eq!(err.to_string(), "Connector client error: ResponseError");
                }
            }
        });
    }

    #[test]
    fn new_user_data_stream_required_params_success() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockUserDataStreamApiClient { force_error: false };


            let resp_json: Value = serde_json::from_str(r#"{"listenKey":"pqia91ma19a5s61cv6a81va65sdf19v8a65a1a5s61cv6a81va65sdf19v8a65a1"}"#).unwrap();
            let expected_response : models::NewUserDataStreamResponse = serde_json::from_value(resp_json.clone()).expect("should parse into models::NewUserDataStreamResponse");

            let resp = client.new_user_data_stream().await.expect("Expected a response");
            let data_future = resp.data();
            let actual_response = data_future.await.unwrap();
            assert_eq!(actual_response, expected_response);
        });
    }

    #[test]
    fn new_user_data_stream_optional_params_success() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockUserDataStreamApiClient { force_error: false };


            let resp_json: Value = serde_json::from_str(r#"{"listenKey":"pqia91ma19a5s61cv6a81va65sdf19v8a65a1a5s61cv6a81va65sdf19v8a65a1"}"#).unwrap();
            let expected_response : models::NewUserDataStreamResponse = serde_json::from_value(resp_json.clone()).expect("should parse into models::NewUserDataStreamResponse");

            let resp = client.new_user_data_stream().await.expect("Expected a response");
            let data_future = resp.data();
            let actual_response = data_future.await.unwrap();
            assert_eq!(actual_response, expected_response);
        });
    }

    #[test]
    fn new_user_data_stream_response_error() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockUserDataStreamApiClient { force_error: true };

            match client.new_user_data_stream().await {
                Ok(_) => panic!("Expected an error"),
                Err(err) => {
                    assert_eq!(err.to_string(), "Connector client error: ResponseError");
                }
            }
        });
    }

    #[test]
    fn put_user_data_stream_required_params_success() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockUserDataStreamApiClient { force_error: false };

            let params = PutUserDataStreamParams::builder("listenKey".to_string())
                .build()
                .unwrap();

            let expected_response = Value::Null;

            let resp = client
                .put_user_data_stream(params)
                .await
                .expect("Expected a response");
            let data_future = resp.data();
            let actual_response = data_future.await.unwrap();
            assert_eq!(actual_response, expected_response);
        });
    }

    #[test]
    fn put_user_data_stream_optional_params_success() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockUserDataStreamApiClient { force_error: false };

            let params = PutUserDataStreamParams::builder("listenKey".to_string())
                .build()
                .unwrap();

            let expected_response = Value::Null;

            let resp = client
                .put_user_data_stream(params)
                .await
                .expect("Expected a response");
            let data_future = resp.data();
            let actual_response = data_future.await.unwrap();
            assert_eq!(actual_response, expected_response);
        });
    }

    #[test]
    fn put_user_data_stream_response_error() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockUserDataStreamApiClient { force_error: true };

            let params = PutUserDataStreamParams::builder("listenKey".to_string())
                .build()
                .unwrap();

            match client.put_user_data_stream(params).await {
                Ok(_) => panic!("Expected an error"),
                Err(err) => {
                    assert_eq!(err.to_string(), "Connector client error: ResponseError");
                }
            }
        });
    }
}
