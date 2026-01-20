pub mod common;
pub use common::config;
pub use common::constants;
pub use common::errors;
pub use common::logger;
pub use common::models;

#[cfg(feature = "algo")]
pub mod algo;
#[cfg(feature = "alpha")]
pub mod alpha;
#[cfg(feature = "c2c")]
pub mod c2c;
#[cfg(feature = "convert")]
pub mod convert;
#[cfg(feature = "copy_trading")]
pub mod copy_trading;
#[cfg(feature = "crypto_loan")]
pub mod crypto_loan;
#[cfg(feature = "derivatives_trading_coin_futures")]
pub mod derivatives_trading_coin_futures;
#[cfg(feature = "derivatives_trading_options")]
pub mod derivatives_trading_options;
#[cfg(feature = "derivatives_trading_portfolio_margin")]
pub mod derivatives_trading_portfolio_margin;
#[cfg(feature = "derivatives_trading_portfolio_margin_pro")]
pub mod derivatives_trading_portfolio_margin_pro;
#[cfg(feature = "derivatives_trading_usds_futures")]
pub mod derivatives_trading_usds_futures;
#[cfg(feature = "dual_investment")]
pub mod dual_investment;
#[cfg(feature = "fiat")]
pub mod fiat;
#[cfg(feature = "gift_card")]
pub mod gift_card;
#[cfg(feature = "margin_trading")]
pub mod margin_trading;
#[cfg(feature = "mining")]
pub mod mining;
#[cfg(feature = "nft")]
pub mod nft;
#[cfg(feature = "pay")]
pub mod pay;
#[cfg(feature = "rebate")]
pub mod rebate;
#[cfg(feature = "simple_earn")]
pub mod simple_earn;
#[cfg(feature = "spot")]
pub mod spot;
#[cfg(feature = "staking")]
pub mod staking;
#[cfg(feature = "sub_account")]
pub mod sub_account;
#[cfg(feature = "vip_loan")]
pub mod vip_loan;
#[cfg(feature = "wallet")]
pub mod wallet;

#[cfg(test)]
static TOKIO_SHARED_RT: std::sync::LazyLock<tokio::runtime::Runtime> =
    std::sync::LazyLock::new(|| {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to build shared Tokio Runtime")
    });
