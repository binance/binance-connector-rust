# Changelog

## 62.0.0 - 2026-07-15

**Derivatives Trading Coin Futures**

### Changed (9)

#### REST API

- Modified response for `cancel_multiple_orders()` (`DELETE /dapi/v1/batchOrders`):
  - items: property `avgPrice` deleted
  - items: property `cumBase` deleted
  - items: item property `avgPrice` deleted
  - items: item property `cumBase` deleted

- Modified response for `place_multiple_orders()` (`POST /dapi/v1/batchOrders`):
  - items: property `avgPrice` deleted
  - items: property `cumBase` deleted
  - items: item property `avgPrice` deleted
  - items: item property `cumBase` deleted

- Modified response for `modify_multiple_orders()` (`PUT /dapi/v1/batchOrders`):
  - items: property `avgPrice` deleted
  - items: property `cumBase` deleted
  - items: item property `avgPrice` deleted
  - items: item property `cumBase` deleted

- Modified response for `cancel_order()` (`DELETE /dapi/v1/order`):
  - property `cumBase` deleted
  - property `avgPrice` deleted

- Modified response for `new_order()` (`POST /dapi/v1/order`):
  - property `cumBase` deleted
  - property `avgPrice` deleted

- Modified response for `modify_order()` (`PUT /dapi/v1/order`):
  - property `cumBase` deleted
  - property `avgPrice` deleted

#### WebSocket API

- Modified response for `cancel_order()` (`order.cancel` method):
  - `result`: property `cumBase` deleted
  - `result`: property `avgPrice` deleted

- Modified response for `modify_order()` (`order.modify` method):
  - `result`: property `avgPrice` deleted
  - `result`: property `cumBase` deleted

- Modified response for `new_order()` (`order.place` method):
  - `result`: property `avgPrice` deleted
  - `result`: property `cumBase` deleted

**Derivatives Trading Portfolio Margin**

### Changed (4)

#### REST API

- Modified response for `cancel_cm_order()` (`DELETE /papi/v1/cm/order`):
  - property `cumBase` deleted
  - property `avgPrice` deleted

- Modified response for `new_cm_order()` (`POST /papi/v1/cm/order`):
  - property `cumBase` deleted
  - property `avgPrice` deleted

- Modified response for `cancel_um_order()` (`DELETE /papi/v1/um/order`):
  - property `cumQuote` deleted
  - property `avgPrice` deleted

- Modified response for `new_um_order()` (`POST /papi/v1/um/order`):
  - property `cumQuote` deleted
  - property `avgPrice` deleted

**Derivatives Trading Usds Futures**

### Changed (9)

#### REST API

- Modified response for `cancel_multiple_orders()` (`DELETE /fapi/v1/batchOrders`):
  - items: property `cumQuote` deleted
  - items: item property `cumQuote` deleted

- Modified response for `place_multiple_orders()` (`POST /fapi/v1/batchOrders`):
  - items: property `cumQuote` deleted
  - items: property `avgPrice` deleted
  - items: item property `cumQuote` deleted
  - items: item property `avgPrice` deleted

- Modified response for `modify_multiple_orders()` (`PUT /fapi/v1/batchOrders`):
  - items: property `cumBase` deleted
  - items: property `avgPrice` deleted
  - items: item property `cumBase` deleted
  - items: item property `avgPrice` deleted

- Modified response for `cancel_order()` (`DELETE /fapi/v1/order`):
  - property `cumQuote` deleted
  - property `avgPrice` deleted

- Modified response for `new_order()` (`POST /fapi/v1/order`):
  - property `cumQuote` deleted
  - property `avgPrice` deleted

- Modified response for `modify_order()` (`PUT /fapi/v1/order`):
  - property `cumBase` deleted
  - property `avgPrice` deleted

#### WebSocket API

- Modified response for `cancel_order()` (`order.cancel` method):
  - `result`: property `cumQuote` deleted

- Modified response for `modify_order()` (`order.modify` method):
  - `result`: property `cumQuote` deleted
  - `result`: property `avgPrice` deleted

- Modified response for `new_order()` (`order.place` method):
  - `result`: property `avgPrice` deleted
  - `result`: property `cumQuote` deleted

**NFT**

### Deleted (1)

- Removed NFT REST API module.

## 61.0.0 - 2026-07-13

### Added (14)

- `create_sub_account_api_key()` (`POST /sapi/v1/sub-account/subAccountApi`)
- `delete_sub_account_api_key()` (`DELETE /sapi/v1/sub-account/subAccountApi`)
- `exit_special_key_mode()` (`POST /sapi/v1/margin/exit-special-key-mode`)
- `full_depth()` (`GET /bapi/defi/v1/public/alpha-trade/fullDepth`)
- `get_vip_loan_repayment_history()` (`GET /sapi/v1/loan/vip/repay/history`)
- `liquidation_loan_repay()` (`POST /sapi/v1/margin/liquidation-loan/repay`)
- `modify_sub_account_api_key_permission()` (`POST /sapi/v1/sub-account/subAccountApiPermission`)
- `query_liquidation_loan()` (`GET /sapi/v1/margin/liquidation-loan`)
- `query_liquidation_loan_repay_history()` (`GET /sapi/v1/margin/liquidation-loan/repay-history`)
- `query_sub_account_api_key()` (`GET /sapi/v1/sub-account/subAccountApi`)
- `tradfi_options_contract()` (`POST /eapi/v1/stock/contract`)
- `hour24_ticker()` (`<symbol>@optionTicker<expirationDate>` stream)
- `index_price_stream()` (`<pair>@indexPrice@<updateSpeed>` stream)
- `open_interest()` (`<underlying>@openInterest@<expirationDate>` stream)

### Changed (279)

- Added parameter `autoCompoundPlan`
  - affected methods:
    - `change_auto_compound_status()` (`POST /sapi/v1/dci/product/auto_compound/edit-status`)
- Added parameter `lang`
  - affected methods:
    - `get_yield_arena_activities()` (`GET /sapi/v1/earn/arena/activities`)
- Added parameter `recvWindow`
  - affected methods:
    - `get_country_list()` (`GET /sapi/v1/localentity/country/list`)
    - `get_region_list()` (`GET /sapi/v1/localentity/region/list`)
- Added parameter `trailingDelta`
  - affected methods:
    - `margin_account_new_order()` (`POST /sapi/v1/margin/order`)
- Deleted parameter `AutoCompoundPlan`
  - affected methods:
    - `change_auto_compound_status()` (`POST /sapi/v1/dci/product/auto_compound/edit-status`)
- Deleted parameter `recvWindow`
  - affected methods:
    - `get_c2_c_trade_history()` (`GET /sapi/v1/c2c/orderMatch/listUserOrderHistory`)
- Deleted parameter `signature`
  - affected methods:
    - `submit_deposit_questionnaire()` (`PUT /sapi/v1/localentity/broker/deposit/provide-info`)
    - `broker_withdraw()` (`POST /sapi/v1/localentity/broker/withdraw/apply`)
- Modified parameter `accountType`:
  - enum added: `SPOT`, `MARGIN`
  - affected methods:
    - `dustlog()` (`GET /sapi/v1/asset/dribblet`)
    - `dust_transfer()` (`POST /sapi/v1/asset/dust`)
    - `get_assets_that_can_be_converted_into_bnb()` (`POST /sapi/v1/asset/dust-btc`)
- Modified parameter `algoType`:
  - enum added: `CONDITIONAL`
  - affected methods:
    - `new_um_algo_order()` (`POST /papi/v1/um/algo/order`)
    - `new_algo_order()` (`POST /fapi/v1/algoOrder`)
- Modified parameter `aprPeriod`:
  - enum added: `DAY`, `YEAR`
  - affected methods:
    - `get_rate_history()` (`GET /sapi/v1/simple-earn/flexible/history/rateHistory`)
- Modified parameter `archived`:
  - enum added: `true`, `false`
  - affected methods:
    - `get_margin_borrow_loan_interest_history()` (`GET /papi/v1/margin/marginInterestHistory`)
    - `query_margin_loan_record()` (`GET /papi/v1/margin/marginLoan`)
    - `query_margin_repay_record()` (`GET /papi/v1/margin/repayLoan`)
- Modified parameter `asset`:
  - enum added: `LDUSDT`, `RWUSD`
  - affected methods:
    - `transfer_ldusdt_rwusd_for_portfolio_margin()` (`POST /sapi/v1/portfolio/earn-asset-transfer`)
- Modified parameter `asset`:
  - enum added: `USDC`, `USDT`
  - affected methods:
    - `get_bfusd_subscription_history()` (`GET /sapi/v1/bfusd/history/subscriptionHistory`)
    - `get_rwusd_subscription_history()` (`GET /sapi/v1/rwusd/history/subscriptionHistory`)
- Modified parameter `asset`:
  - enum added: `USDT`, `USDC`
  - affected methods:
    - `subscribe_rwusd()` (`POST /sapi/v1/rwusd/subscribe`)
- Modified parameter `asset`:
  - enum added: `WBETH`, `BETH`
  - affected methods:
    - `redeem_eth()` (`POST /sapi/v1/eth-staking/eth/redeem`)
- Modified parameter `assetNames`:
  - type `array` → `string`
  - affected methods:
    - `small_liability_exchange()` (`POST /sapi/v1/margin/exchange-small-liability`)
- Modified parameter `autoCompoundPlan`:
  - enum added: `NONE`, `STANDARD`, `ADVANCED`
  - affected methods:
    - `subscribe_dual_investment_products()` (`POST /sapi/v1/dci/product/subscribe`)
- Modified parameter `autoRepay`:
  - enum added: `true`, `false`
  - affected methods:
    - `change_auto_repay_futures_status()` (`POST /papi/v1/repay-futures-switch`)
    - `change_auto_repay_futures_status()` (`POST /sapi/v1/portfolio/repay-futures-switch`)
- Modified parameter `autoRepayAtCancel`:
  - enum added: `true`, `false`
  - affected methods:
    - `new_margin_order()` (`POST /papi/v1/margin/order`)
- Modified parameter `batchOrders`:
  - items: required added: `side`, `type`, `quantity`, `symbol`
  - items.`activationPrice`: type `string` → `number`
  - items.`callbackRate`: type `string` → `number`
  - items.`price`: type `string` → `number`
  - items.`priceMatch`: enum removed: `NONE`
  - items.`priceProtect`: enum added: `true`, `false`
  - items.`quantity`: type `string` → `number`
  - items.`reduceOnly`: enum added: `true`, `false`
  - items.`selfTradePreventionMode`: enum removed: `NONE`
  - items.`stopPrice`: type `string` → `number`
  - items.`activationPrice`: type `string` → `number`
  - items.`callbackRate`: type `string` → `number`
  - items.`price`: type `string` → `number`
  - items.`priceMatch`: enum removed: `NONE`
  - items.`priceProtect`: enum added: `true`, `false`
  - items.`quantity`: type `string` → `number`
  - items.`reduceOnly`: enum added: `true`, `false`
  - items.`selfTradePreventionMode`: enum removed: `NONE`
  - items.`stopPrice`: type `string` → `number`
  - affected methods:
    - `place_multiple_orders()` (`POST /dapi/v1/batchOrders`)
- Modified parameter `batchOrders`:
  - items: required added: `side`, `timestamp`, `symbol`
  - items: property `timestamp` added
  - items.`orderId`: type `string` → `integer`
  - items.`price`: type `string` → `number`
  - items.`quantity`: type `string` → `number`
  - items.`recvWindow`: type `string` → `integer`
  - items: item property `timestamp` added
  - items.`orderId`: type `string` → `integer`
  - items.`price`: type `string` → `number`
  - items.`quantity`: type `string` → `number`
  - items.`recvWindow`: type `string` → `integer`
  - affected methods:
    - `modify_multiple_orders()` (`PUT /dapi/v1/batchOrders`)
- Modified parameter `batchOrders`:
  - items.`goodTillDate`: type `string` → `integer`
  - items.`price`: type `string` → `number`
  - items.`priceMatch`: enum removed: `NONE`
  - items.`quantity`: type `string` → `number`
  - items.`reduceOnly`: enum added: `true`, `false`
  - items.`selfTradePreventionMode`: enum added: `NONE`
  - items.`type`: enum added: `LIMIT`, `MARKET`, `STOP`, `STOP_MARKET`, `TAKE_PROFIT`, `TAKE_PROFIT_MARKET`, `TRAILING_STOP_MARKET`
  - items.`goodTillDate`: type `string` → `integer`
  - items.`price`: type `string` → `number`
  - items.`priceMatch`: enum removed: `NONE`
  - items.`quantity`: type `string` → `number`
  - items.`reduceOnly`: enum added: `true`, `false`
  - items.`selfTradePreventionMode`: enum added: `NONE`
  - items.`type`: enum added: `LIMIT`, `MARKET`, `STOP`, `STOP_MARKET`, `TAKE_PROFIT`, `TAKE_PROFIT_MARKET`, `TRAILING_STOP_MARKET`
  - affected methods:
    - `place_multiple_orders()` (`POST /fapi/v1/batchOrders`)
- Modified parameter `batchOrders`:
  - items: property `timestamp` added
  - items.`orderId`: type `string` → `integer`
  - items.`price`: type `string` → `number`
  - items.`priceMatch`: enum removed: `NONE`
  - items.`quantity`: type `string` → `number`
  - items.`recvWindow`: type `string` → `integer`
  - items.`stopPrice`: type `string` → `number`
  - items: item property `timestamp` added
  - items.`orderId`: type `string` → `integer`
  - items.`price`: type `string` → `number`
  - items.`priceMatch`: enum removed: `NONE`
  - items.`quantity`: type `string` → `number`
  - items.`recvWindow`: type `string` → `integer`
  - items.`stopPrice`: type `string` → `number`
  - affected methods:
    - `modify_multiple_orders()` (`PUT /fapi/v1/batchOrders`)
- Modified parameter `cancelRestrictions`:
  - enum removed: `NEW`, `PARTIALLY_FILLED`
  - affected methods:
    - `delete_order()` (`DELETE /api/v3/order`)
    - `order_cancel_replace()` (`POST /api/v3/order/cancelReplace`)
- Modified parameter `clientAlgoId`:
  - minLength `0` → `32`
  - maxLength `null` → `32`
  - affected methods:
    - `time_weighted_average_price_future_algo()` (`POST /sapi/v1/algo/futures/newOrderTwap`)
    - `volume_participation_future_algo()` (`POST /sapi/v1/algo/futures/newOrderVp`)
    - `time_weighted_average_price_spot_algo()` (`POST /sapi/v1/algo/spot/newOrderTwap`)
- Modified parameter `closePosition`:
  - enum added: `true`, `false`
  - affected methods:
    - `new_algo_order()` (`POST /fapi/v1/algoOrder`)
    - `test_order()` (`POST /fapi/v1/order/test`)
- Modified parameter `contractType`:
  - enum removed: `CURRENT_QUARTER_DELIVERING`, `NEXT_QUARTER_DELIVERING`, `PERPETUAL_DELIVERING`
  - affected methods:
    - `continuous_contract_kline_candlestick_data()` (`GET /dapi/v1/continuousKlines`)
    - `basis()` (`GET /futures/data/basis`)
- Modified parameter `contractType`:
  - enum removed: `CURRENT_QUARTER_DELIVERING`, `NEXT_QUARTER_DELIVERING`, `PERPETUAL_DELIVERING`
  - enum added: `ALL`
  - affected methods:
    - `open_interest_statistics()` (`GET /futures/data/openInterestHist`)
    - `taker_buy_sell_volume()` (`GET /futures/data/takerBuySellVol`)
- Modified parameter `contractType`:
  - enum removed: `CURRENT_MONTH`, `NEXT_MONTH`, `PERPETUAL_DELIVERING`
  - enum added: `TRADIFI_PERPETUAL`
  - affected methods:
    - `continuous_contract_kline_candlestick_data()` (`GET /fapi/v1/continuousKlines`)
- Modified parameter `contractType`:
  - enum removed: `CURRENT_MONTH`, `NEXT_MONTH`, `PERPETUAL_DELIVERING`
  - affected methods:
    - `basis()` (`GET /futures/data/basis`)
- Modified parameter `currency`:
  - enum added: `USDT`
  - affected methods:
    - `account_funding_flow()` (`GET /eapi/v1/bill`)
- Modified parameter `deltaEnabled`:
  - enum added: `true`, `false`
  - affected methods:
    - `switch_delta_mode()` (`POST /sapi/v1/portfolio/delta-mode`)
- Modified parameter `deltaLimit`:
  - required: `false` → `true`
  - affected methods:
    - `set_market_maker_protection_config()` (`POST /eapi/v1/mmpSet`)
- Modified parameter `depositId`:
  - type `string` → `integer`
  - affected methods:
    - `deposit_history_v2()` (`GET /sapi/v2/localentity/deposit/history`)
- Modified parameter `destAccount`:
  - enum added: `SPOT`, `FUND`
  - affected methods:
    - `redeem_flexible_product()` (`POST /sapi/v1/simple-earn/flexible/redeem`)
- Modified parameter `direction`:
  - enum added: `ADDITIONAL`, `REDUCED`
  - affected methods:
    - `flexible_loan_adjust_ltv()` (`POST /sapi/v2/loan/flexible/adjust/ltv`)
- Modified parameter `dualSidePosition`:
  - enum added: `true`, `false`
  - affected methods:
    - `change_cm_position_mode()` (`POST /papi/v1/cm/positionSide/dual`)
    - `change_um_position_mode()` (`POST /papi/v1/um/positionSide/dual`)
- Modified parameter `expiredType`:
  - enum added: `1_D`, `3_D`, `7_D`, `30_D`
  - affected methods:
    - `place_limit_order()` (`POST /sapi/v1/convert/limit/placeOrder`)
- Modified parameter `externalUid`:
  - maxLength `null` → `400`
  - affected methods:
    - `redeem_a_binance_gift_card()` (`POST /sapi/v1/giftcard/redeemCode`)
- Modified parameter `feeBurn`:
  - enum added: `true`, `false`
  - affected methods:
    - `toggle_bnb_burn_on_um_futures_trade()` (`POST /papi/v1/um/feeBurn`)
- Modified parameter `from`:
  - enum added: `SPOT`, `MARGIN`
  - affected methods:
    - `portfolio_margin_pro_bankruptcy_loan_repay()` (`POST /sapi/v1/portfolio/repay`)
    - `repay_futures_negative_balance()` (`POST /sapi/v1/portfolio/repay-futures-negative-balance`)
- Modified parameter `fromAccountType`:
  - enum added: `SPOT`, `USDT_FUTURE`, `COIN_FUTURE`, `MARGIN`, `ISOLATED_MARGIN`
  - affected methods:
    - `universal_transfer()` (`POST /sapi/v1/sub-account/universalTransfer`)
- Modified parameter `fromSymbol`:
  - enum added: `ISOLATEDMARGIN_MARGIN`, `ISOLATEDMARGIN_ISOLATEDMARGIN`
  - affected methods:
    - `query_user_universal_transfer_history()` (`GET /sapi/v1/asset/transfer`)
    - `user_universal_transfer()` (`POST /sapi/v1/asset/transfer`)
- Modified parameter `frozenTimeInMilliseconds`:
  - required: `false` → `true`
  - affected methods:
    - `set_market_maker_protection_config()` (`POST /eapi/v1/mmpSet`)
- Modified parameter `incomeType`:
  - enum added: `TRANSFER`, `WELCOME_BONUS`, `FUNDING_FEE`, `REALIZED_PNL`, `COMMISSION`, `INSURANCE_CLEAR`, `DELIVERED_SETTELMENT`
  - affected methods:
    - `get_income_history()` (`GET /dapi/v1/income`)
    - `get_cm_income_history()` (`GET /papi/v1/cm/income`)
- Modified parameter `incomeType`:
  - enum added: `TRANSFER`, `WELCOME_BONUS`, `REALIZED_PNL`, `FUNDING_FEE`, `COMMISSION`, `INSURANCE_CLEAR`, `REFERRAL_KICKBACK`, `COMMISSION_REBATE`, `API_REBATE`, `CONTEST_REWARD`, `CROSS_COLLATERAL_TRANSFER`, `OPTIONS_PREMIUM_FEE`, `OPTIONS_SETTLE_PROFIT`, `INTERNAL_TRANSFER`, `AUTO_EXCHANGE`, `DELIVERED_SETTELMENT`, `COIN_SWAP_DEPOSIT`, `COIN_SWAP_WITHDRAW`, `POSITION_LIMIT_INCREASE_FEE`
  - affected methods:
    - `get_um_income_history()` (`GET /papi/v1/um/income`)
- Modified parameter `incomeType`:
  - enum added: `TRANSFER`, `WELCOME_BONUS`, `REALIZED_PNL`, `FUNDING_FEE`, `COMMISSION`, `INSURANCE_CLEAR`, `REFERRAL_KICKBACK`, `COMMISSION_REBATE`, `API_REBATE`, `CONTEST_REWARD`, `CROSS_COLLATERAL_TRANSFER`, `OPTIONS_PREMIUM_FEE`, `OPTIONS_SETTLE_PROFIT`, `INTERNAL_TRANSFER`, `AUTO_EXCHANGE`, `DELIVERED_SETTELMENT`, `COIN_SWAP_DEPOSIT`, `COIN_SWAP_WITHDRAW`, `POSITION_LIMIT_INCREASE_FEE`, `STRATEGY_UMFUTURES_TRANSFER`, `FEE_RETURN`, `BFUSD_REWARD`
  - affected methods:
    - `get_income_history()` (`GET /fapi/v1/income`)
- Modified parameter `interval`:
  - enum added: `1s`, `15s`, `1m`, `3m`, `5m`, `15m`, `30m`, `1h`, `2h`, `4h`, `6h`, `8h`, `12h`, `1d`, `3d`, `1w`, `1M`
  - affected methods:
    - `klines()` (`GET /bapi/defi/v1/public/alpha-trade/klines`)
- Modified parameter `interval`:
  - enum added: `1m`, `3m`, `5m`, `15m`, `30m`, `1h`, `2h`, `4h`, `6h`, `8h`, `12h`, `1d`, `3d`, `1w`, `1M`
  - affected methods:
    - `kline_candlestick_data()` (`GET /eapi/v1/klines`)
- Modified parameter `interval`:
  - enum removed: `1s`
  - affected methods:
    - `continuous_contract_kline_candlestick_data()` (`GET /fapi/v1/continuousKlines`)
    - `index_price_kline_candlestick_data()` (`GET /fapi/v1/indexPriceKlines`)
    - `kline_candlestick_data()` (`GET /fapi/v1/klines`)
    - `mark_price_kline_candlestick_data()` (`GET /fapi/v1/markPriceKlines`)
    - `premium_index_kline_data()` (`GET /fapi/v1/premiumIndexKlines`)
- Modified parameter `isIsolated`:
  - enum added: `TRUE`, `FALSE`
  - affected methods:
    - `query_margin_accounts_all_oco()` (`GET /sapi/v1/margin/allOrderList`)
    - `query_margin_accounts_all_orders()` (`GET /sapi/v1/margin/allOrders`)
    - `query_prevented_matches()` (`GET /sapi/v1/margin/myPreventedMatches`)
    - `query_margin_accounts_trade_list()` (`GET /sapi/v1/margin/myTrades`)
    - `query_margin_accounts_open_oco()` (`GET /sapi/v1/margin/openOrderList`)
    - `margin_account_cancel_all_open_orders_on_a_symbol()` (`DELETE /sapi/v1/margin/openOrders`)
    - `query_margin_accounts_open_orders()` (`GET /sapi/v1/margin/openOrders`)
    - `margin_account_cancel_order()` (`DELETE /sapi/v1/margin/order`)
    - `query_margin_accounts_order()` (`GET /sapi/v1/margin/order`)
    - `margin_account_new_order()` (`POST /sapi/v1/margin/order`)
    - `margin_account_new_oco()` (`POST /sapi/v1/margin/order/oco`)
    - `margin_account_new_oto()` (`POST /sapi/v1/margin/order/oto`)
    - `margin_account_new_otoco()` (`POST /sapi/v1/margin/order/otoco`)
    - `margin_account_cancel_oco()` (`DELETE /sapi/v1/margin/orderList`)
    - `query_margin_accounts_oco()` (`GET /sapi/v1/margin/orderList`)
    - `query_current_margin_order_count_usage()` (`GET /sapi/v1/margin/rateLimit/order`)
- Modified parameter `isIsolated`:
  - enum added: `TRUE`, `FALSE`
  - affected methods:
    - `margin_account_borrow_repay()` (`POST /sapi/v1/margin/borrow-repay`)
- Modified parameter `isIsolated`:
  - type `boolean` → `string`
  - enum added: `TRUE`, `FALSE`
  - affected methods:
    - `get_future_hourly_interest_rate()` (`GET /sapi/v1/margin/next-hourly-interest-rate`)
- Modified parameter `legs`:
  - items: required added: `type`, `quantity`, `symbol`, `side`
  - items: property `price` added
  - items: property `quantity` added
  - items: property `side` added
  - items: property `symbol` added
  - items: property `type` added
  - items: item property `price` added
  - items: item property `quantity` added
  - items: item property `side` added
  - items: item property `symbol` added
  - items: item property `type` added
  - affected methods:
    - `new_block_trade_order()` (`POST /eapi/v1/block/order/create`)
- Modified parameter `liquidity`:
  - enum added: `MAKER`, `TAKER`
  - affected methods:
    - `new_block_trade_order()` (`POST /eapi/v1/block/order/create`)
- Modified parameter `needBtcValuation`:
  - type `string` → `boolean`
  - affected methods:
    - `funding_wallet()` (`POST /sapi/v1/asset/get-funding-asset`)
- Modified parameter `newOrderRespType`:
  - enum added: `FULL`
  - affected methods:
    - `new_margin_order()` (`POST /papi/v1/margin/order`)
- Modified parameter `newOrderRespType`:
  - enum removed: `MARKET`, `LIMIT`
  - affected methods:
    - `new_order()` (`POST /api/v3/order`)
    - `order_cancel_replace()` (`POST /api/v3/order/cancelReplace`)
    - `order_oco()` (`POST /api/v3/order/oco`)
    - `order_test()` (`POST /api/v3/order/test`)
    - `order_list_oco()` (`POST /api/v3/orderList/oco`)
    - `order_list_opo()` (`POST /api/v3/orderList/opo`)
    - `order_list_opoco()` (`POST /api/v3/orderList/opoco`)
    - `order_list_oto()` (`POST /api/v3/orderList/oto`)
    - `order_list_otoco()` (`POST /api/v3/orderList/otoco`)
    - `sor_order()` (`POST /api/v3/sor/order`)
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Modified parameter `optionType`:
  - enum added: `CALL`, `PUT`
  - affected methods:
    - `get_dual_investment_product_list()` (`GET /sapi/v1/dci/product/list`)
- Modified parameter `orderIdList`:
  - maxItems `null` → `10`
  - affected methods:
    - `cancel_multiple_orders()` (`DELETE /dapi/v1/batchOrders`)
- Modified parameter `orderIdList`:
  - maxLength `null` → `10`
  - affected methods:
    - `cancel_multiple_orders()` (`DELETE /fapi/v1/batchOrders`)
- Modified parameter `orders`:
  - items: required added: `symbol`, `side`, `type`, `quantity`
  - items.`isMmp`: type `string` → `boolean`
  - items.`postOnly`: type `string` → `boolean`
  - items.`price`: type `string` → `number`
  - items.`quantity`: type `string` → `number`
  - items.`reduceOnly`: type `string` → `boolean`
  - items.`isMmp`: type `string` → `boolean`
  - items.`postOnly`: type `string` → `boolean`
  - items.`price`: type `string` → `number`
  - items.`quantity`: type `string` → `number`
  - items.`reduceOnly`: type `string` → `boolean`
  - affected methods:
    - `place_multiple_orders()` (`POST /eapi/v1/batchOrders`)
- Modified parameter `origClientOrderIdList`:
  - maxItems `null` → `10`
  - affected methods:
    - `cancel_multiple_orders()` (`DELETE /dapi/v1/batchOrders`)
- Modified parameter `origClientOrderIdList`:
  - maxLength `null` → `10`
  - affected methods:
    - `cancel_multiple_orders()` (`DELETE /fapi/v1/batchOrders`)
- Modified parameter `pegOffsetType`:
  - enum removed: `NON_REPRESENTABLE`
  - affected methods:
    - `new_order()` (`POST /api/v3/order`)
    - `order_cancel_replace()` (`POST /api/v3/order/cancelReplace`)
    - `order_test()` (`POST /api/v3/order/test`)
- Modified parameter `pegPriceType`:
  - enum removed: `NON_REPRESENTABLE`
  - affected methods:
    - `new_order()` (`POST /api/v3/order`)
    - `order_cancel_replace()` (`POST /api/v3/order/cancelReplace`)
    - `order_test()` (`POST /api/v3/order/test`)
- Modified parameter `pendingAboveTimeInForce`:
  - enum added: `GTC`, `IOC`, `FOK`
  - affected methods:
    - `margin_account_new_otoco()` (`POST /sapi/v1/margin/order/otoco`)
- Modified parameter `pendingAboveType`:
  - enum added: `LIMIT_MAKER`, `STOP_LOSS`, `STOP_LOSS_LIMIT`
  - affected methods:
    - `margin_account_new_otoco()` (`POST /sapi/v1/margin/order/otoco`)
- Modified parameter `pendingBelowTimeInForce`:
  - enum added: `GTC`, `IOC`, `FOK`
  - affected methods:
    - `margin_account_new_otoco()` (`POST /sapi/v1/margin/order/otoco`)
- Modified parameter `pendingBelowType`:
  - enum added: `LIMIT_MAKER`, `STOP_LOSS`, `STOP_LOSS_LIMIT`
  - affected methods:
    - `margin_account_new_otoco()` (`POST /sapi/v1/margin/order/otoco`)
- Modified parameter `pendingSide`:
  - enum added: `BUY`, `SELL`
  - affected methods:
    - `margin_account_new_oto()` (`POST /sapi/v1/margin/order/oto`)
    - `margin_account_new_otoco()` (`POST /sapi/v1/margin/order/otoco`)
- Modified parameter `pendingTimeInForce`:
  - enum added: `GTC`, `IOC`, `FOK`
  - affected methods:
    - `margin_account_new_oto()` (`POST /sapi/v1/margin/order/oto`)
- Modified parameter `pendingType`:
  - enum added: `LIMIT`, `MARKET`, `STOP_LOSS`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT`, `TAKE_PROFIT_LIMIT`, `LIMIT_MAKER`
  - affected methods:
    - `margin_account_new_oto()` (`POST /sapi/v1/margin/order/oto`)
- Modified parameter `permissionMode`:
  - enum added: `TRADE`, `READ`
  - affected methods:
    - `create_special_key()` (`POST /sapi/v1/margin/apiKey`)
- Modified parameter `permissions`:
  - items: enum added: `SPOT`, `MARGIN`, `LEVERAGED`, `TRD_GRP_002`, `TRD_GRP_003`, `TRD_GRP_004`, `TRD_GRP_005`, `TRD_GRP_006`, `TRD_GRP_007`, `TRD_GRP_008`, `TRD_GRP_009`, `TRD_GRP_010`, `TRD_GRP_011`, `TRD_GRP_012`, `TRD_GRP_013`, `TRD_GRP_014`, `TRD_GRP_015`, `TRD_GRP_016`, `TRD_GRP_017`, `TRD_GRP_018`, `TRD_GRP_019`, `TRD_GRP_020`, `TRD_GRP_021`, `TRD_GRP_022`, `TRD_GRP_023`, `TRD_GRP_024`, `TRD_GRP_025`
  - affected methods:
    - `exchange_info()` (`GET /api/v3/exchangeInfo`)
- Modified parameter `positionId`:
  - type `integer` → `string`
  - affected methods:
    - `get_locked_redemption_record()` (`GET /sapi/v1/simple-earn/locked/history/redemptionRecord`)
    - `get_locked_rewards_history()` (`GET /sapi/v1/simple-earn/locked/history/rewardsRecord`)
    - `get_locked_product_position()` (`GET /sapi/v1/simple-earn/locked/position`)
    - `get_on_chain_yields_locked_redemption_record()` (`GET /sapi/v1/onchain-yields/locked/history/redemptionRecord`)
    - `get_on_chain_yields_locked_product_position()` (`GET /sapi/v1/onchain-yields/locked/position`)
- Modified parameter `positionSide`:
  - enum added: `BOTH`, `LONG`, `SHORT`
  - affected methods:
    - `time_weighted_average_price_future_algo()` (`POST /sapi/v1/algo/futures/newOrderTwap`)
    - `volume_participation_future_algo()` (`POST /sapi/v1/algo/futures/newOrderVp`)
- Modified parameter `positionSide`:
  - enum removed: `BOTH`, `LONG`, `SHORT`
  - affected methods:
    - `new_algo_order()` (`POST /fapi/v1/algoOrder`)
    - `new_order()` (`POST /fapi/v1/order`)
    - `modify_isolated_position_margin()` (`POST /fapi/v1/positionMargin`)
- Modified parameter `priceMatch`:
  - enum removed: `NONE`
  - affected methods:
    - `new_order()` (`POST /dapi/v1/order`)
    - `modify_order()` (`PUT /dapi/v1/order`)
    - `new_cm_order()` (`POST /papi/v1/cm/order`)
    - `modify_cm_order()` (`PUT /papi/v1/cm/order`)
    - `new_um_conditional_order()` (`POST /papi/v1/um/conditional/order`)
    - `new_um_order()` (`POST /papi/v1/um/order`)
    - `modify_um_order()` (`PUT /papi/v1/um/order`)
    - `new_algo_order()` (`POST /fapi/v1/algoOrder`)
    - `new_order()` (`POST /fapi/v1/order`)
    - `modify_order()` (`PUT /fapi/v1/order`)
    - `test_order()` (`POST /fapi/v1/order/test`)
- Modified parameter `priceMatch`:
  - enum removed: `NONE`
  - affected methods:
    - `new_um_algo_order()` (`POST /papi/v1/um/algo/order`)
- Modified parameter `priceProtect`:
  - enum added: `true`, `false`
  - affected methods:
    - `new_order()` (`POST /dapi/v1/order`)
    - `new_cm_conditional_order()` (`POST /papi/v1/cm/conditional/order`)
    - `new_um_algo_order()` (`POST /papi/v1/um/algo/order`)
    - `new_um_conditional_order()` (`POST /papi/v1/um/conditional/order`)
    - `new_algo_order()` (`POST /fapi/v1/algoOrder`)
    - `test_order()` (`POST /fapi/v1/order/test`)
- Modified parameter `productType`:
  - enum added: `UM`
  - affected methods:
    - `move_position_for_sub_account()` (`POST /sapi/v1/sub-account/futures/move-position`)
- Modified parameter `qtyLimit`:
  - required: `false` → `true`
  - affected methods:
    - `set_market_maker_protection_config()` (`POST /eapi/v1/mmpSet`)
- Modified parameter `redeemTo`:
  - enum added: `SPOT`, `FLEXIBLE`
  - affected methods:
    - `set_locked_product_redeem_option()` (`POST /sapi/v1/simple-earn/locked/setRedeemOption`)
    - `set_on_chain_yields_locked_product_redeem_option()` (`POST /sapi/v1/onchain-yields/locked/setRedeemOption`)
- Modified parameter `redeemTo`:
  - enum added: `SPOT`, `FLEXIBLE`
  - affected methods:
    - `subscribe_locked_product()` (`POST /sapi/v1/simple-earn/locked/subscribe`)
- Modified parameter `redeemTo`:
  - enum added: `SPOT`, `FLEXIBLE`
  - affected methods:
    - `subscribe_on_chain_yields_locked_product()` (`POST /sapi/v1/onchain-yields/locked/subscribe`)
- Modified parameter `reduceOnly`:
  - enum added: `true`, `false`
  - affected methods:
    - `new_order()` (`POST /dapi/v1/order`)
    - `new_cm_order()` (`POST /papi/v1/cm/order`)
    - `new_um_algo_order()` (`POST /papi/v1/um/algo/order`)
    - `new_um_conditional_order()` (`POST /papi/v1/um/conditional/order`)
    - `new_um_order()` (`POST /papi/v1/um/order`)
    - `new_algo_order()` (`POST /fapi/v1/algoOrder`)
    - `new_order()` (`POST /fapi/v1/order`)
    - `test_order()` (`POST /fapi/v1/order/test`)
- Modified parameter `repaymentType`:
  - enum added: `1`, `2`
  - affected methods:
    - `flexible_loan_repay()` (`POST /sapi/v2/loan/flexible/repay`)
- Modified parameter `selfTradePreventionMode`:
  - enum added: `NONE`
  - affected methods:
    - `new_order()` (`POST /eapi/v1/order`)
- Modified parameter `selfTradePreventionMode`:
  - enum added: `NONE`
  - affected methods:
    - `new_algo_order()` (`POST /fapi/v1/algoOrder`)
    - `test_order()` (`POST /fapi/v1/order/test`)
- Modified parameter `selfTradePreventionMode`:
  - enum added: `EXPIRE_TAKER`, `EXPIRE_MAKER`, `EXPIRE_BOTH`, `NONE`
  - affected methods:
    - `margin_account_new_order()` (`POST /sapi/v1/margin/order`)
    - `margin_account_new_oco()` (`POST /sapi/v1/margin/order/oco`)
    - `margin_account_new_oto()` (`POST /sapi/v1/margin/order/oto`)
    - `margin_account_new_otoco()` (`POST /sapi/v1/margin/order/otoco`)
- Modified parameter `selfTradePreventionMode`:
  - enum removed: `NON_REPRESENTABLE`
  - affected methods:
    - `new_order()` (`POST /api/v3/order`)
    - `order_cancel_replace()` (`POST /api/v3/order/cancelReplace`)
    - `order_oco()` (`POST /api/v3/order/oco`)
    - `order_test()` (`POST /api/v3/order/test`)
    - `order_list_oco()` (`POST /api/v3/orderList/oco`)
    - `order_list_opo()` (`POST /api/v3/orderList/opo`)
    - `order_list_opoco()` (`POST /api/v3/orderList/opoco`)
    - `order_list_oto()` (`POST /api/v3/orderList/oto`)
    - `order_list_otoco()` (`POST /api/v3/orderList/otoco`)
    - `sor_order()` (`POST /api/v3/sor/order`)
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Modified parameter `side`:
  - enum added: `BUY`, `SELL`
  - affected methods:
    - `query_historical_algo_orders_future_algo()` (`GET /sapi/v1/algo/futures/historicalOrders`)
    - `time_weighted_average_price_future_algo()` (`POST /sapi/v1/algo/futures/newOrderTwap`)
    - `volume_participation_future_algo()` (`POST /sapi/v1/algo/futures/newOrderVp`)
    - `query_historical_algo_orders_spot_algo()` (`GET /sapi/v1/algo/spot/historicalOrders`)
    - `time_weighted_average_price_spot_algo()` (`POST /sapi/v1/algo/spot/newOrderTwap`)
    - `place_limit_order()` (`POST /sapi/v1/convert/limit/placeOrder`)
- Modified parameter `sideEffectType`:
  - enum added: `AUTO_BORROW_REPAY`
  - affected methods:
    - `new_margin_order()` (`POST /papi/v1/margin/order`)
- Modified parameter `sideEffectType`:
  - enum added: `NO_SIDE_EFFECT`, `MARGIN_BUY`, `AUTO_REPAY`, `AUTO_BORROW_REPAY`
  - affected methods:
    - `margin_account_new_order()` (`POST /sapi/v1/margin/order`)
    - `margin_account_new_oco()` (`POST /sapi/v1/margin/order/oco`)
- Modified parameter `sideEffectType`:
  - enum added: `NO_SIDE_EFFECT`, `MARGIN_BUY`
  - affected methods:
    - `margin_account_new_oto()` (`POST /sapi/v1/margin/order/oto`)
    - `margin_account_new_otoco()` (`POST /sapi/v1/margin/order/otoco`)
- Modified parameter `sourceAccount`:
  - enum added: `SPOT`, `FUND`, `ALL`
  - affected methods:
    - `subscribe_flexible_product()` (`POST /sapi/v1/simple-earn/flexible/subscribe`)
    - `subscribe_locked_product()` (`POST /sapi/v1/simple-earn/locked/subscribe`)
    - `subscribe_on_chain_yields_locked_product()` (`POST /sapi/v1/onchain-yields/locked/subscribe`)
- Modified parameter `status`:
  - enum added: `PENDING`, `PURCHASE_SUCCESS`, `SETTLED`, `PURCHASE_FAIL`, `REFUNDING`, `REFUND_SUCCESS`, `SETTLING`
  - affected methods:
    - `get_dual_investment_positions()` (`GET /sapi/v1/dci/product/positions`)
- Modified parameter `status`:
  - type `string` → `integer`
  - affected methods:
    - `add_ip_restriction_for_sub_account_api_key()` (`POST /sapi/v2/sub-account/subAccountApi/ipRestriction`)
- Modified parameter `status`:
  - enum added: `0`, `1`, `2`, `6`, `7`, `8`
  - affected methods:
    - `deposit_history()` (`GET /sapi/v1/capital/deposit/hisrec`)
- Modified parameter `stopLimitTimeInForce`:
  - enum added: `GTC`, `FOK`, `IOC`
  - affected methods:
    - `margin_account_new_oco()` (`POST /sapi/v1/margin/order/oco`)
- Modified parameter `strategyType`:
  - enum removed: `LIMIT_MAKER`
  - affected methods:
    - `new_cm_conditional_order()` (`POST /papi/v1/cm/conditional/order`)
    - `new_um_conditional_order()` (`POST /papi/v1/um/conditional/order`)
- Modified parameter `subAccountId`:
  - type `integer` → `string`
  - affected methods:
    - `one_click_arrival_deposit_apply()` (`POST /sapi/v1/capital/deposit/credit-apply`)
- Modified parameter `symbol`:
  - required: `false` → `true`
  - affected methods:
    - `account_trade_list()` (`GET /eapi/v1/userTrades`)
- Modified parameter `symbol`:
  - required: `true` → `false`
  - affected methods:
    - `query_all_cm_orders()` (`GET /papi/v1/cm/allOrders`)
- Modified parameter `symbol`:
  - required: `true` → `false`
  - affected methods:
    - `margin_account_borrow_repay()` (`POST /sapi/v1/margin/borrow-repay`)
- Modified parameter `symbolStatus`:
  - enum removed: `END_OF_DAY`, `NON_REPRESENTABLE`
  - affected methods:
    - `depth()` (`GET /api/v3/depth`)
    - `exchange_info()` (`GET /api/v3/exchangeInfo`)
    - `execution_rules()` (`GET /api/v3/executionRules`)
    - `reference_price_calculation()` (`GET /api/v3/referencePrice/calculation`)
    - `ticker_book_ticker()` (`GET /api/v3/ticker/bookTicker`)
    - `ticker_price()` (`GET /api/v3/ticker/price`)
    - `ticker_trading_day()` (`GET /api/v3/ticker/tradingDay`)
- Modified parameter `symbolStatus`:
  - enum removed: `END_OF_DAY`, `NON_REPRESENTABLE`
  - affected methods:
    - `ticker()` (`GET /api/v3/ticker`)
    - `ticker24hr()` (`GET /api/v3/ticker/24hr`)
- Modified parameter `timeInForce`:
  - enum removed: `GTX`
  - affected methods:
    - `new_margin_order()` (`POST /papi/v1/margin/order`)
- Modified parameter `timeInForce`:
  - enum added: `GTD`
  - affected methods:
    - `new_um_algo_order()` (`POST /papi/v1/um/algo/order`)
- Modified parameter `timeInForce`:
  - enum added: `GTD`
  - affected methods:
    - `new_um_conditional_order()` (`POST /papi/v1/um/conditional/order`)
    - `new_um_order()` (`POST /papi/v1/um/order`)
- Modified parameter `timeInForce`:
  - enum removed: `NON_REPRESENTABLE`
  - affected methods:
    - `new_order()` (`POST /api/v3/order`)
    - `order_cancel_replace()` (`POST /api/v3/order/cancelReplace`)
    - `order_test()` (`POST /api/v3/order/test`)
    - `sor_order()` (`POST /api/v3/sor/order`)
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Modified parameter `toAccountType`:
  - enum added: `SPOT`, `USDT_FUTURE`, `COIN_FUTURE`, `MARGIN`, `ISOLATED_MARGIN`
  - affected methods:
    - `universal_transfer()` (`POST /sapi/v1/sub-account/universalTransfer`)
- Modified parameter `toSymbol`:
  - enum added: `MARGIN_ISOLATEDMARGIN`, `ISOLATEDMARGIN_ISOLATEDMARGIN`
  - affected methods:
    - `query_user_universal_transfer_history()` (`GET /sapi/v1/asset/transfer`)
    - `user_universal_transfer()` (`POST /sapi/v1/asset/transfer`)
- Modified parameter `tradeType`:
  - enum added: `BUY`, `SELL`
  - affected methods:
    - `get_c2_c_trade_history()` (`GET /sapi/v1/c2c/orderMatch/listUserOrderHistory`)
- Modified parameter `transferFunctionAccountType`:
  - enum added: `SPOT`, `MARGIN`, `ISOLATED_MARGIN`, `USDT_FUTURE`, `COIN_FUTURE`
  - affected methods:
    - `query_managed_sub_account_transfer_log_sub_account_trading()` (`GET /sapi/v1/managed-subaccount/query-trans-log`)
    - `query_managed_sub_account_transfer_log_master_account_investor()` (`GET /sapi/v1/managed-subaccount/queryTransLogForInvestor`)
    - `query_managed_sub_account_transfer_log_master_account_trading()` (`GET /sapi/v1/managed-subaccount/queryTransLogForTradeParent`)
- Modified parameter `transferSide`:
  - enum added: `TO_UM`, `FROM_UM`
  - affected methods:
    - `bnb_transfer()` (`POST /papi/v1/bnb-transfer`)
    - `bnb_transfer()` (`POST /sapi/v1/portfolio/bnb-transfer`)
- Modified parameter `transferType`:
  - enum added: `EARN_TO_FUTURE`, `FUTURE_TO_EARN`
  - affected methods:
    - `get_transferable_earn_asset_balance_for_portfolio_margin()` (`GET /sapi/v1/portfolio/earn-asset-balance`)
    - `transfer_ldusdt_rwusd_for_portfolio_margin()` (`POST /sapi/v1/portfolio/earn-asset-transfer`)
- Modified parameter `type`:
  - enum added: `borrowIn`, `collateralSpent`, `repayAmount`, `collateralReturn`, `addCollateral`, `removeCollateral`, `collateralReturnAfterLiquidation`
  - affected methods:
    - `get_crypto_loans_income_history()` (`GET /sapi/v1/loan/income`)
- Modified parameter `type`:
  - type `string` → `integer`
  - enum removed: `LIMIT`, `MARKET`, `STOP`, `STOP_MARKET`, `TAKE_PROFIT`, `TAKE_PROFIT_MARKET`, `TRAILING_STOP_MARKET`
  - affected methods:
    - `modify_isolated_position_margin()` (`POST /dapi/v1/positionMargin`)
- Modified parameter `type`:
  - enum added: `STOP_LOSS`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT`, `TAKE_PROFIT_LIMIT`, `LIMIT_MAKER`
  - affected methods:
    - `new_margin_order()` (`POST /papi/v1/margin/order`)
- Modified parameter `type`:
  - enum removed: `LIMIT`, `MARKET`
  - enum added: `STOP`, `TAKE_PROFIT`, `STOP_MARKET`, `TAKE_PROFIT_MARKET`, `TRAILING_STOP_MARKET`
  - affected methods:
    - `new_um_algo_order()` (`POST /papi/v1/um/algo/order`)
- Modified parameter `type`:
  - enum added: `LIMIT`, `MARKET`, `STOP`, `STOP_MARKET`, `TAKE_PROFIT`, `TAKE_PROFIT_MARKET`, `TRAILING_STOP_MARKET`
  - affected methods:
    - `new_algo_order()` (`POST /fapi/v1/algoOrder`)
- Modified parameter `type`:
  - enum added: `LIMIT`, `MARKET`, `STOP`, `STOP_MARKET`, `TAKE_PROFIT`, `TAKE_PROFIT_MARKET`, `TRAILING_STOP_MARKET`
  - affected methods:
    - `new_order()` (`POST /fapi/v1/order`)
    - `test_order()` (`POST /fapi/v1/order/test`)
- Modified parameter `type`:
  - type `string` → `integer`
  - affected methods:
    - `modify_isolated_position_margin()` (`POST /fapi/v1/positionMargin`)
- Modified parameter `type`:
  - type `integer` → `string`
  - affected methods:
    - `get_position_margin_change_history()` (`GET /fapi/v1/positionMargin/history`)
- Modified parameter `type`:
  - enum added: `MARGIN`, `ISOLATED`
  - affected methods:
    - `query_margin_available_inventory()` (`GET /sapi/v1/margin/available-inventory`)
    - `margin_manual_liquidation()` (`POST /sapi/v1/margin/manual-liquidation`)
- Modified parameter `type`:
  - enum added: `BORROW`, `REPAY`
  - affected methods:
    - `query_borrow_repay_records_in_margin_account()` (`GET /sapi/v1/margin/borrow-repay`)
    - `margin_account_borrow_repay()` (`POST /sapi/v1/margin/borrow-repay`)
- Modified parameter `type`:
  - enum added: `TRANSFER`, `BORROW`, `REPAY`, `BUY_INCOME`, `BUY_EXPENSE`, `SELL_INCOME`, `SELL_EXPENSE`, `TRADING_COMMISSION`, `BUY_LIQUIDATION`, `SELL_LIQUIDATION`, `REPAY_LIQUIDATION`, `OTHER_LIQUIDATION`, `LIQUIDATION_FEE`, `SMALL_BALANCE_CONVERT`, `COMMISSION_RETURN`, `SMALL_CONVERT`
  - affected methods:
    - `query_cross_isolated_margin_capital_flow()` (`GET /sapi/v1/margin/capital-flow`)
- Modified parameter `type`:
  - enum added: `LIMIT`, `MARKET`, `STOP_LOSS`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT`, `TAKE_PROFIT_LIMIT`, `LIMIT_MAKER`
  - affected methods:
    - `margin_account_new_order()` (`POST /sapi/v1/margin/order`)
- Modified parameter `type`:
  - enum added: `ROLL_IN`, `ROLL_OUT`
  - affected methods:
    - `get_cross_margin_transfer_history()` (`GET /sapi/v1/margin/transfer`)
- Modified parameter `type`:
  - enum added: `FAST`, `STANDARD`
  - affected methods:
    - `redeem_bfusd()` (`POST /sapi/v1/bfusd/redeem`)
    - `redeem_rwusd()` (`POST /sapi/v1/rwusd/redeem`)
- Modified parameter `type`:
  - required: `true` → `false`
  - enum added: `BONUS`, `REALTIME`, `REWARDS`, `ALL`
  - affected methods:
    - `get_flexible_rewards_history()` (`GET /sapi/v1/simple-earn/flexible/history/rewardsRecord`)
- Modified parameter `type`:
  - enum removed: `NON_REPRESENTABLE`
  - affected methods:
    - `new_order()` (`POST /api/v3/order`)
    - `order_cancel_replace()` (`POST /api/v3/order/cancelReplace`)
    - `order_test()` (`POST /api/v3/order/test`)
- Modified parameter `type`:
  - enum removed: `STOP_LOSS`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT`, `TAKE_PROFIT_LIMIT`, `LIMIT_MAKER`, `NON_REPRESENTABLE`
  - affected methods:
    - `sor_order()` (`POST /api/v3/sor/order`)
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Modified parameter `type`:
  - enum added: `CLAIM`, `DISTRIBUTE`
  - affected methods:
    - `get_boost_rewards_history()` (`GET /sapi/v1/sol-staking/sol/history/boostRewardsHistory`)
- Modified parameter `type`:
  - enum added: `SPOT`, `MARGIN`, `FUTURES`
  - affected methods:
    - `query_managed_sub_account_snapshot()` (`GET /sapi/v1/managed-subaccount/accountSnapshot`)
    - `daily_account_snapshot()` (`GET /sapi/v1/accountSnapshot`)
- Modified parameter `type`:
  - enum added: `DELEGATE`, `UNDELEGATE`
  - affected methods:
    - `query_user_delegation_history()` (`GET /sapi/v1/asset/custody/transfer-history`)
- Modified parameter `type`:
  - enum added: `MAIN_UMFUTURE`, `MAIN_CMFUTURE`, `MAIN_MARGIN`, `UMFUTURE_MAIN`, `UMFUTURE_MARGIN`, `CMFUTURE_MAIN`, `CMFUTURE_MARGIN`, `MARGIN_MAIN`, `MARGIN_UMFUTURE`, `MARGIN_CMFUTURE`, `ISOLATEDMARGIN_MARGIN`, `MARGIN_ISOLATEDMARGIN`, `ISOLATEDMARGIN_ISOLATEDMARGIN`, `MAIN_FUNDING`, `FUNDING_MAIN`, `FUNDING_UMFUTURE`, `UMFUTURE_FUNDING`, `MARGIN_FUNDING`, `FUNDING_MARGIN`, `FUNDING_CMFUTURE`, `CMFUTURE_FUNDING`, `MAIN_OPTION`, `OPTION_MAIN`, `UMFUTURE_OPTION`, `OPTION_UMFUTURE`, `MARGIN_OPTION`, `OPTION_MARGIN`, `FUNDING_OPTION`, `OPTION_FUNDING`, `MAIN_PORTFOLIO_MARGIN`, `PORTFOLIO_MARGIN_MAIN`
  - affected methods:
    - `user_universal_transfer()` (`POST /sapi/v1/asset/transfer`)
- Modified parameter `underlying`:
  - required: `false` → `true`
  - affected methods:
    - `get_market_maker_protection_config()` (`GET /eapi/v1/mmp`)
    - `reset_market_maker_protection_config()` (`POST /eapi/v1/mmpReset`)
    - `set_market_maker_protection_config()` (`POST /eapi/v1/mmpSet`)
- Modified parameter `urgency`:
  - enum added: `LOW`, `MEDIUM`, `HIGH`
  - affected methods:
    - `volume_participation_future_algo()` (`POST /sapi/v1/algo/futures/newOrderVp`)
- Modified parameter `validTime`:
  - enum added: `10s`, `30s`, `1m`
  - affected methods:
    - `send_quote_request()` (`POST /sapi/v1/convert/getQuote`)
- Modified parameter `walletType`:
  - enum added: `SPOT`, `FUNDING`, `EARN`, `SPOT_FUNDING`, `FUNDING_EARN`, `SPOT_FUNDING_EARN`, `SPOT_EARN`
  - affected methods:
    - `send_quote_request()` (`POST /sapi/v1/convert/getQuote`)
    - `place_limit_order()` (`POST /sapi/v1/convert/limit/placeOrder`)
- Modified parameter `windowSize`:
  - enum added: `7d`
  - affected methods:
    - `ticker()` (`GET /api/v3/ticker`)
- Modified parameter `windowTimeInMilliseconds`:
  - required: `false` → `true`
  - affected methods:
    - `set_market_maker_protection_config()` (`POST /eapi/v1/mmpSet`)
- Modified parameter `workingSide`:
  - enum added: `BUY`, `SELL`
  - affected methods:
    - `margin_account_new_oto()` (`POST /sapi/v1/margin/order/oto`)
    - `margin_account_new_otoco()` (`POST /sapi/v1/margin/order/otoco`)
- Modified parameter `workingTimeInForce`:
  - enum added: `GTC`, `IOC`, `FOK`
  - affected methods:
    - `margin_account_new_oto()` (`POST /sapi/v1/margin/order/oto`)
- Modified parameter `workingTimeInForce`:
  - enum added: `GTC`, `IOC`, `FOK`
  - affected methods:
    - `margin_account_new_otoco()` (`POST /sapi/v1/margin/order/otoco`)
- Modified parameter `workingType`:
  - enum added: `CONTRACT_PRICE`
  - affected methods:
    - `new_cm_conditional_order()` (`POST /papi/v1/cm/conditional/order`)
    - `new_um_conditional_order()` (`POST /papi/v1/um/conditional/order`)
- Modified parameter `workingType`:
  - enum added: `CONTRACT_PRICE`
  - affected methods:
    - `new_um_algo_order()` (`POST /papi/v1/um/algo/order`)
- Modified parameter `workingType`:
  - enum added: `LIMIT`, `LIMIT_MAKER`
  - affected methods:
    - `margin_account_new_oto()` (`POST /sapi/v1/margin/order/oto`)
    - `margin_account_new_otoco()` (`POST /sapi/v1/margin/order/otoco`)
- Modified response for `get_c2_c_trade_history()` (`GET /sapi/v1/c2c/orderMatch/listUserOrderHistory`):
  - property `message` added
  - property `success` added
  - property `total` added
  - property `code` added
  - property `data` added
  - property `totalPrice` deleted
  - property `tradeType` deleted
  - property `takerAmount` deleted
  - property `amount` deleted
  - property `asset` deleted
  - property `fiat` deleted
  - property `unitPrice` deleted
  - property `orderNumber` deleted
  - property `orderStatus` deleted
  - property `takerCommission` deleted
  - property `takerCommissionRate` deleted
  - property `payMethodName` deleted
  - property `advNo` deleted
  - property `commission` deleted
  - property `additionalKycVerify` deleted
  - property `fiatSymbol` deleted
  - property `counterPartNickName` deleted
  - property `createTime` deleted

- Modified response for `place_multiple_orders()` (`POST /dapi/v1/batchOrders`):
  - items: property `closePosition` added
  - items: item property `closePosition` added

- Modified response for `order_book()` (`GET /dapi/v1/depth`):
  - `asks`.items: minItems `0` → `2`
  - `asks`.items: maxItems `null` → `2`
  - `bids`.items: minItems `0` → `2`
  - `bids`.items: maxItems `null` → `2`

- Modified response for `mark_price_kline_candlestick_data()` (`GET /dapi/v1/markPriceKlines`):
  - items.items: oneOf added 2 schema(s)
  - items.items: oneOf removed 2 schema(s)

- Modified response for `cancel_multiple_option_orders()` (`DELETE /eapi/v1/batchOrders`):
  - items: property `fee` added
  - items: item property `fee` added

- Modified response for `place_multiple_orders()` (`POST /eapi/v1/batchOrders`):
  - items: property `postOnly` added
  - items: property `fee` added
  - items: item property `postOnly` added
  - items: item property `fee` added

- Modified response for `order_book()` (`GET /eapi/v1/depth`):
  - `asks`.items: minItems `0` → `2`
  - `asks`.items: maxItems `null` → `2`
  - `bids`.items: minItems `0` → `2`
  - `bids`.items: maxItems `null` → `2`

- Modified response for `exchange_information()` (`GET /eapi/v1/exchangeInfo`):
  - `optionSymbols`.items: property `underlyingType` added
  - `optionSymbols`.items: property `contractType` added
  - `optionSymbols`.items: property `nakedSell` added
  - `optionSymbols`.items: item property `underlyingType` added
  - `optionSymbols`.items: item property `contractType` added
  - `optionSymbols`.items: item property `nakedSell` added

- Modified response for `query_single_order()` (`GET /eapi/v1/order`):
  - property `postOnly` added

- Modified response for `new_order()` (`POST /eapi/v1/order`):
  - property `fee` added
  - property `postOnly` added

- Modified response for `account_balance()` (`GET /papi/v1/balance`):
  - oneOf modified

- Modified response for `query_current_um_open_algo_order()` (`GET /papi/v1/um/algo/algoOrder`):
  - property `tpOrderType` deleted
  - property `icebergQuantity` deleted
  - property `tpTriggerPrice` deleted
  - property `tpPrice` deleted
  - property `slTriggerPrice` deleted
  - property `slPrice` deleted

- Modified response for `query_um_algo_order_history()` (`GET /papi/v1/um/algo/allAlgoOrders`):
  - items: property `slPrice` deleted
  - items: property `tpTriggerPrice` deleted
  - items: property `icebergQuantity` deleted
  - items: property `tpPrice` deleted
  - items: property `slTriggerPrice` deleted
  - items: property `tpOrderType` deleted
  - items: item property `slPrice` deleted
  - items: item property `tpTriggerPrice` deleted
  - items: item property `icebergQuantity` deleted
  - items: item property `tpPrice` deleted
  - items: item property `slTriggerPrice` deleted
  - items: item property `tpOrderType` deleted

- Modified response for `query_all_current_um_open_algo_orders()` (`GET /papi/v1/um/algo/openAlgoOrders`):
  - items: property `slPrice` deleted
  - items: property `tpPrice` deleted
  - items: property `tpTriggerPrice` deleted
  - items: property `actualPrice` deleted
  - items: property `actualOrderId` deleted
  - items: property `icebergQuantity` deleted
  - items: property `slTriggerPrice` deleted
  - items: property `tpOrderType` deleted
  - items: item property `slPrice` deleted
  - items: item property `tpPrice` deleted
  - items: item property `tpTriggerPrice` deleted
  - items: item property `actualPrice` deleted
  - items: item property `actualOrderId` deleted
  - items: item property `icebergQuantity` deleted
  - items: item property `slTriggerPrice` deleted
  - items: item property `tpOrderType` deleted

- Modified response for `new_um_algo_order()` (`POST /papi/v1/um/algo/order`):
  - property `icebergQuantity` deleted

- Modified response for `get_um_income_history()` (`GET /papi/v1/um/income`):
  - items.`tranId`: type `integer` → `string`
  - items.`tranId`: type `integer` → `string`

- Modified response for `order_book()` (`GET /fapi/v1/depth`):
  - `asks`.items: minItems `0` → `2`
  - `asks`.items: maxItems `null` → `2`
  - `bids`.items: minItems `0` → `2`
  - `bids`.items: maxItems `null` → `2`

- Modified response for `query_insurance_fund_balance_snapshot()` (`GET /fapi/v1/insuranceBalance`):
  - oneOf modified

- Modified response for `notional_and_leverage_brackets()` (`GET /fapi/v1/leverageBracket`):
  - oneOf modified

- Modified response for `query_order()` (`GET /fapi/v1/order`):
  - property `goodTillDate` added
  - property `selfTradePreventionMode` added
  - property `priceMatch` added

- Modified response for `mark_price()` (`GET /fapi/v1/premiumIndex`):
  - oneOf modified

- Modified response for `rpi_order_book()` (`GET /fapi/v1/rpiDepth`):
  - `asks`.items: minItems `0` → `2`
  - `asks`.items: maxItems `null` → `2`
  - `bids`.items: minItems `0` → `2`
  - `bids`.items: maxItems `null` → `2`

- Modified response for `adl_risk()` (`GET /fapi/v1/symbolAdlRisk`):
  - oneOf modified

- Modified response for `ticker24hr_price_change_statistics()` (`GET /fapi/v1/ticker/24hr`):
  - oneOf modified

- Modified response for `symbol_order_book_ticker()` (`GET /fapi/v1/ticker/bookTicker`):
  - oneOf modified

- Modified response for `symbol_price_ticker()` (`GET /fapi/v1/ticker/price`):
  - oneOf modified

- Modified response for `symbol_price_ticker_v2()` (`GET /fapi/v2/ticker/price`):
  - oneOf modified

- Modified response for `long_short_ratio()` (`GET /futures/data/globalLongShortAccountRatio`):
  - items.`timestamp`: type `string` → `integer`
  - items.`timestamp`: type `string` → `integer`

- Modified response for `open_interest_statistics()` (`GET /futures/data/openInterestHist`):
  - items.`timestamp`: type `string` → `integer`
  - items.`timestamp`: type `string` → `integer`

- Modified response for `taker_buy_sell_volume()` (`GET /futures/data/takerlongshortRatio`):
  - items.`timestamp`: type `string` → `integer`
  - items.`timestamp`: type `string` → `integer`

- Modified response for `top_trader_long_short_ratio_accounts()` (`GET /futures/data/topLongShortAccountRatio`):
  - items.`timestamp`: type `string` → `integer`
  - items.`timestamp`: type `string` → `integer`

- Modified response for `top_trader_long_short_ratio_positions()` (`GET /futures/data/topLongShortPositionRatio`):
  - items.`timestamp`: type `string` → `integer`
  - items.`timestamp`: type `string` → `integer`

- Modified response for `get_dual_investment_positions()` (`GET /sapi/v1/dci/product/positions`):
  - `list`.items: property `subscriptionTime` added
  - `list`.items: item property `subscriptionTime` added

- Modified response for `query_margin_available_inventory()` (`GET /sapi/v1/margin/available-inventory`):
  - `assets`: property `TVK` deleted
  - `assets`: property `MATIC` deleted
  - `assets`: property `SHIB` deleted
  - `assets`: property `STPT` deleted

- Modified response for `query_cross_isolated_margin_capital_flow()` (`GET /sapi/v1/margin/capital-flow`):
  - items: property `note` added
  - items: item property `note` added

- Modified response for `statistic_list()` (`GET /sapi/v1/mining/statistics/user/status`):
  - `data`.`profitToday`: property `BSV` deleted
  - `data`.`profitToday`: property `BTC` deleted
  - `data`.`profitToday`: property `BCH` deleted
  - `data`.`profitYesterday`: property `BTC` deleted
  - `data`.`profitYesterday`: property `BCH` deleted
  - `data`.`profitYesterday`: property `BSV` deleted

- Modified response for `get_pay_trade_history()` (`GET /sapi/v1/pay/transactions`):
  - `data`.items.`fundsDetail`.items.`walletAssetCost`: property `1` deleted
  - `data`.items.`fundsDetail`.items.`walletAssetCost`: property `2` deleted
  - `data`.items.`fundsDetail`.items.`walletAssetCost`: property `1` deleted
  - `data`.items.`fundsDetail`.items.`walletAssetCost`: property `2` deleted
  - `data`.items.`fundsDetail`.items.`walletAssetCost`: property `1` deleted
  - `data`.items.`fundsDetail`.items.`walletAssetCost`: property `2` deleted
  - `data`.items.`fundsDetail`.items.`walletAssetCost`: property `1` deleted
  - `data`.items.`fundsDetail`.items.`walletAssetCost`: property `2` deleted

- Modified response for `all_orders()` (`GET /api/v3/allOrders`):
  - items: property `workingFloor` added
  - items: property `pegOffsetValue` added
  - items: property `preventedQuantity` added
  - items: property `trailingTime` added
  - items: property `expiryReason` added
  - items: property `pegPriceType` added
  - items: property `trailingDelta` added
  - items: property `usedSor` added
  - items: property `preventedMatchId` added
  - items: property `strategyType` added
  - items: property `pegOffsetType` added
  - items: property `strategyId` added
  - items: property `peggedPrice` added
  - items: item property `workingFloor` added
  - items: item property `pegOffsetValue` added
  - items: item property `preventedQuantity` added
  - items: item property `trailingTime` added
  - items: item property `expiryReason` added
  - items: item property `pegPriceType` added
  - items: item property `trailingDelta` added
  - items: item property `usedSor` added
  - items: item property `preventedMatchId` added
  - items: item property `strategyType` added
  - items: item property `pegOffsetType` added
  - items: item property `strategyId` added
  - items: item property `peggedPrice` added

- Modified response for `depth()` (`GET /api/v3/depth`):
  - `asks`.items: minItems `0` → `2`
  - `asks`.items: maxItems `null` → `2`
  - `bids`.items: minItems `0` → `2`
  - `bids`.items: maxItems `null` → `2`

- Modified response for `exchange_info()` (`GET /api/v3/exchangeInfo`):
  - property `sors` added
  - `exchangeFilters`.items: oneOf modified
  - `symbols`.items.`filters`.items: oneOf modified
  - `symbols`.items.`filters`.items: oneOf modified

- Modified response for `klines()` (`GET /api/v3/klines`):
  - items.items: oneOf added 2 schema(s)
  - items.items: oneOf removed 2 schema(s)

- Modified response for `my_filters()` (`GET /api/v3/myFilters`):
  - `assetFilters`.items: oneOf modified
  - `exchangeFilters`.items: oneOf modified
  - `symbolFilters`.items: oneOf modified

- Modified response for `get_open_orders()` (`GET /api/v3/openOrders`):
  - items: property `pegOffsetType` added
  - items: property `usedSor` added
  - items: property `preventedQuantity` added
  - items: property `trailingDelta` added
  - items: property `trailingTime` added
  - items: property `pegPriceType` added
  - items: property `peggedPrice` added
  - items: property `workingFloor` added
  - items: property `expiryReason` added
  - items: property `pegOffsetValue` added
  - items: property `strategyType` added
  - items: property `strategyId` added
  - items: property `preventedMatchId` added
  - items: item property `pegOffsetType` added
  - items: item property `usedSor` added
  - items: item property `preventedQuantity` added
  - items: item property `trailingDelta` added
  - items: item property `trailingTime` added
  - items: item property `pegPriceType` added
  - items: item property `peggedPrice` added
  - items: item property `workingFloor` added
  - items: item property `expiryReason` added
  - items: item property `pegOffsetValue` added
  - items: item property `strategyType` added
  - items: item property `strategyId` added
  - items: item property `preventedMatchId` added

- Modified response for `delete_order()` (`DELETE /api/v3/order`):
  - property `strategyType` added
  - property `trailingTime` added
  - property `preventedMatchId` added
  - property `trailingDelta` added
  - property `pegOffsetValue` added
  - property `preventedQuantity` added
  - property `pegPriceType` added
  - property `pegOffsetType` added
  - property `peggedPrice` added
  - property `icebergQty` added
  - property `workingFloor` added
  - property `expiryReason` added
  - property `strategyId` added
  - property `usedSor` added
  - property `stopPrice` added

- Modified response for `get_order()` (`GET /api/v3/order`):
  - property `pegOffsetValue` added
  - property `pegOffsetType` added
  - property `preventedMatchId` added
  - property `trailingTime` added
  - property `trailingDelta` added
  - property `preventedQuantity` added
  - property `strategyType` added
  - property `peggedPrice` added
  - property `usedSor` added
  - property `workingFloor` added
  - property `pegPriceType` added
  - property `strategyId` added
  - property `expiryReason` added

- Modified response for `order_amend_keep_priority()` (`PUT /api/v3/order/amend/keepPriority`):
  - `amendedOrder`: property `icebergQty` added
  - `amendedOrder`: property `trailingTime` added
  - `amendedOrder`: property `expiryReason` added
  - `amendedOrder`: property `pegOffsetValue` added
  - `amendedOrder`: property `trailingDelta` added
  - `amendedOrder`: property `pegOffsetType` added
  - `amendedOrder`: property `pegPriceType` added
  - `amendedOrder`: property `usedSor` added
  - `amendedOrder`: property `strategyId` added
  - `amendedOrder`: property `peggedPrice` added
  - `amendedOrder`: property `stopPrice` added
  - `amendedOrder`: property `strategyType` added
  - `amendedOrder`: property `preventedQuantity` added
  - `amendedOrder`: property `preventedMatchId` added
  - `amendedOrder`: property `workingFloor` added

- Modified response for `order_cancel_replace()` (`POST /api/v3/order/cancelReplace`):
  - property `code` deleted
  - property `data` deleted
  - property `msg` deleted
  - `cancelResponse`: property `pegPriceType` added
  - `cancelResponse`: property `preventedMatchId` added
  - `cancelResponse`: property `expiryReason` added
  - `cancelResponse`: property `icebergQty` added
  - `cancelResponse`: property `pegOffsetType` added
  - `cancelResponse`: property `strategyType` added
  - `cancelResponse`: property `preventedQuantity` added
  - `cancelResponse`: property `trailingDelta` added
  - `cancelResponse`: property `stopPrice` added
  - `cancelResponse`: property `workingFloor` added
  - `cancelResponse`: property `trailingTime` added
  - `cancelResponse`: property `strategyId` added
  - `cancelResponse`: property `peggedPrice` added
  - `cancelResponse`: property `pegOffsetValue` added
  - `cancelResponse`: property `usedSor` added
  - `newOrderResponse`: property `strategyId` added
  - `newOrderResponse`: property `pegOffsetValue` added
  - `newOrderResponse`: property `icebergQty` added
  - `newOrderResponse`: property `pegOffsetType` added
  - `newOrderResponse`: property `trailingTime` added
  - `newOrderResponse`: property `workingFloor` added
  - `newOrderResponse`: property `stopPrice` added
  - `newOrderResponse`: property `expiryReason` added
  - `newOrderResponse`: property `preventedQuantity` added
  - `newOrderResponse`: property `peggedPrice` added
  - `newOrderResponse`: property `usedSor` added
  - `newOrderResponse`: property `preventedMatchId` added
  - `newOrderResponse`: property `strategyType` added
  - `newOrderResponse`: property `trailingDelta` added
  - `newOrderResponse`: property `pegPriceType` added
  - `newOrderResponse`.`fills`.items: type `string` → `object`
  - `newOrderResponse`.`fills`.items: property `price` added
  - `newOrderResponse`.`fills`.items: property `qty` added
  - `newOrderResponse`.`fills`.items: property `tradeId` added
  - `newOrderResponse`.`fills`.items: property `commission` added
  - `newOrderResponse`.`fills`.items: property `commissionAsset` added
  - `newOrderResponse`.`fills`.items: item property `price` added
  - `newOrderResponse`.`fills`.items: item property `qty` added
  - `newOrderResponse`.`fills`.items: item property `tradeId` added
  - `newOrderResponse`.`fills`.items: item property `commission` added
  - `newOrderResponse`.`fills`.items: item property `commissionAsset` added

- Modified response for `delete_order_list()` (`DELETE /api/v3/orderList`):
  - `orderReports`.items: property `expiryReason` added
  - `orderReports`.items: property `pegOffsetValue` added
  - `orderReports`.items: property `usedSor` added
  - `orderReports`.items: property `peggedPrice` added
  - `orderReports`.items: property `strategyType` added
  - `orderReports`.items: property `pegPriceType` added
  - `orderReports`.items: property `trailingDelta` added
  - `orderReports`.items: property `icebergQty` added
  - `orderReports`.items: property `pegOffsetType` added
  - `orderReports`.items: property `preventedQuantity` added
  - `orderReports`.items: property `preventedMatchId` added
  - `orderReports`.items: property `strategyId` added
  - `orderReports`.items: property `trailingTime` added
  - `orderReports`.items: property `workingFloor` added
  - `orderReports`.items: property `selfTradePreventionMode` deleted
  - `orderReports`.items: item property `expiryReason` added
  - `orderReports`.items: item property `pegOffsetValue` added
  - `orderReports`.items: item property `usedSor` added
  - `orderReports`.items: item property `peggedPrice` added
  - `orderReports`.items: item property `strategyType` added
  - `orderReports`.items: item property `pegPriceType` added
  - `orderReports`.items: item property `trailingDelta` added
  - `orderReports`.items: item property `icebergQty` added
  - `orderReports`.items: item property `pegOffsetType` added
  - `orderReports`.items: item property `preventedQuantity` added
  - `orderReports`.items: item property `preventedMatchId` added
  - `orderReports`.items: item property `strategyId` added
  - `orderReports`.items: item property `trailingTime` added
  - `orderReports`.items: item property `workingFloor` added
  - `orderReports`.items: item property `selfTradePreventionMode` deleted

- Modified response for `order_list_opo()` (`POST /api/v3/orderList/opo`):
  - `orderReports`.items: property `trailingTime` added
  - `orderReports`.items: property `trailingDelta` added
  - `orderReports`.items: property `stopPrice` added
  - `orderReports`.items: property `icebergQty` added
  - `orderReports`.items: property `pegOffsetType` added
  - `orderReports`.items: property `preventedMatchId` added
  - `orderReports`.items: property `expiryReason` added
  - `orderReports`.items: property `pegPriceType` added
  - `orderReports`.items: property `pegOffsetValue` added
  - `orderReports`.items: property `peggedPrice` added
  - `orderReports`.items: property `preventedQuantity` added
  - `orderReports`.items: property `usedSor` added
  - `orderReports`.items: property `strategyType` added
  - `orderReports`.items: property `workingFloor` added
  - `orderReports`.items: property `strategyId` added
  - `orderReports`.items: item property `trailingTime` added
  - `orderReports`.items: item property `trailingDelta` added
  - `orderReports`.items: item property `stopPrice` added
  - `orderReports`.items: item property `icebergQty` added
  - `orderReports`.items: item property `pegOffsetType` added
  - `orderReports`.items: item property `preventedMatchId` added
  - `orderReports`.items: item property `expiryReason` added
  - `orderReports`.items: item property `pegPriceType` added
  - `orderReports`.items: item property `pegOffsetValue` added
  - `orderReports`.items: item property `peggedPrice` added
  - `orderReports`.items: item property `preventedQuantity` added
  - `orderReports`.items: item property `usedSor` added
  - `orderReports`.items: item property `strategyType` added
  - `orderReports`.items: item property `workingFloor` added
  - `orderReports`.items: item property `strategyId` added

- Modified response for `order_list_opoco()` (`POST /api/v3/orderList/opoco`):
  - `orderReports`.items: property `preventedQuantity` added
  - `orderReports`.items: property `workingFloor` added
  - `orderReports`.items: property `strategyType` added
  - `orderReports`.items: property `expiryReason` added
  - `orderReports`.items: property `peggedPrice` added
  - `orderReports`.items: property `preventedMatchId` added
  - `orderReports`.items: property `usedSor` added
  - `orderReports`.items: property `strategyId` added
  - `orderReports`.items: property `pegPriceType` added
  - `orderReports`.items: property `pegOffsetType` added
  - `orderReports`.items: property `trailingDelta` added
  - `orderReports`.items: property `trailingTime` added
  - `orderReports`.items: property `icebergQty` added
  - `orderReports`.items: property `pegOffsetValue` added
  - `orderReports`.items: item property `preventedQuantity` added
  - `orderReports`.items: item property `workingFloor` added
  - `orderReports`.items: item property `strategyType` added
  - `orderReports`.items: item property `expiryReason` added
  - `orderReports`.items: item property `peggedPrice` added
  - `orderReports`.items: item property `preventedMatchId` added
  - `orderReports`.items: item property `usedSor` added
  - `orderReports`.items: item property `strategyId` added
  - `orderReports`.items: item property `pegPriceType` added
  - `orderReports`.items: item property `pegOffsetType` added
  - `orderReports`.items: item property `trailingDelta` added
  - `orderReports`.items: item property `trailingTime` added
  - `orderReports`.items: item property `icebergQty` added
  - `orderReports`.items: item property `pegOffsetValue` added

- Modified response for `order_list_oto()` (`POST /api/v3/orderList/oto`):
  - `orderReports`.items: property `preventedQuantity` added
  - `orderReports`.items: property `pegOffsetType` added
  - `orderReports`.items: property `pegOffsetValue` added
  - `orderReports`.items: property `icebergQty` added
  - `orderReports`.items: property `preventedMatchId` added
  - `orderReports`.items: property `usedSor` added
  - `orderReports`.items: property `pegPriceType` added
  - `orderReports`.items: property `peggedPrice` added
  - `orderReports`.items: property `trailingDelta` added
  - `orderReports`.items: property `workingFloor` added
  - `orderReports`.items: property `stopPrice` added
  - `orderReports`.items: property `strategyType` added
  - `orderReports`.items: property `expiryReason` added
  - `orderReports`.items: property `strategyId` added
  - `orderReports`.items: property `trailingTime` added
  - `orderReports`.items: item property `preventedQuantity` added
  - `orderReports`.items: item property `pegOffsetType` added
  - `orderReports`.items: item property `pegOffsetValue` added
  - `orderReports`.items: item property `icebergQty` added
  - `orderReports`.items: item property `preventedMatchId` added
  - `orderReports`.items: item property `usedSor` added
  - `orderReports`.items: item property `pegPriceType` added
  - `orderReports`.items: item property `peggedPrice` added
  - `orderReports`.items: item property `trailingDelta` added
  - `orderReports`.items: item property `workingFloor` added
  - `orderReports`.items: item property `stopPrice` added
  - `orderReports`.items: item property `strategyType` added
  - `orderReports`.items: item property `expiryReason` added
  - `orderReports`.items: item property `strategyId` added
  - `orderReports`.items: item property `trailingTime` added

- Modified response for `order_list_otoco()` (`POST /api/v3/orderList/otoco`):
  - `orderReports`.items: property `trailingTime` added
  - `orderReports`.items: property `pegOffsetType` added
  - `orderReports`.items: property `peggedPrice` added
  - `orderReports`.items: property `usedSor` added
  - `orderReports`.items: property `strategyId` added
  - `orderReports`.items: property `preventedMatchId` added
  - `orderReports`.items: property `preventedQuantity` added
  - `orderReports`.items: property `pegPriceType` added
  - `orderReports`.items: property `trailingDelta` added
  - `orderReports`.items: property `strategyType` added
  - `orderReports`.items: property `expiryReason` added
  - `orderReports`.items: property `icebergQty` added
  - `orderReports`.items: property `pegOffsetValue` added
  - `orderReports`.items: property `workingFloor` added
  - `orderReports`.items: item property `trailingTime` added
  - `orderReports`.items: item property `pegOffsetType` added
  - `orderReports`.items: item property `peggedPrice` added
  - `orderReports`.items: item property `usedSor` added
  - `orderReports`.items: item property `strategyId` added
  - `orderReports`.items: item property `preventedMatchId` added
  - `orderReports`.items: item property `preventedQuantity` added
  - `orderReports`.items: item property `pegPriceType` added
  - `orderReports`.items: item property `trailingDelta` added
  - `orderReports`.items: item property `strategyType` added
  - `orderReports`.items: item property `expiryReason` added
  - `orderReports`.items: item property `icebergQty` added
  - `orderReports`.items: item property `pegOffsetValue` added
  - `orderReports`.items: item property `workingFloor` added

- Modified response for `ticker()` (`GET /api/v3/ticker`):
  - oneOf modified

- Modified response for `ticker24hr()` (`GET /api/v3/ticker/24hr`):
  - oneOf modified

- Modified response for `ticker_book_ticker()` (`GET /api/v3/ticker/bookTicker`):
  - oneOf modified

- Modified response for `ticker_price()` (`GET /api/v3/ticker/price`):
  - oneOf modified

- Modified response for `ticker_trading_day()` (`GET /api/v3/ticker/tradingDay`):
  - oneOf modified

- Modified response for `ui_klines()` (`GET /api/v3/uiKlines`):
  - items.items: oneOf added 2 schema(s)
  - items.items: oneOf removed 2 schema(s)

- Modified response for `deposit_history_travel_rule()` (`GET /sapi/v1/localentity/deposit/history`):
  - items: property `travelRuleStatusV2` added
  - items: property `completeTime` added
  - items: property `unlockConfirm` deleted
  - items: property `walletType` deleted
  - items: item property `travelRuleStatusV2` added
  - items: item property `completeTime` added
  - items: item property `unlockConfirm` deleted
  - items: item property `walletType` deleted

- Marked `order_oco()` (`POST /api/v3/order/oco`) as deprecated.
- Marked `symbol_price_ticker()` (`GET /fapi/v1/ticker/price`) as deprecated.

- Modified parameter `algoType`:
  - enum added: `CONDITIONAL`
  - affected methods:
    - `new_algo_order()` (`algoOrder.place` method)
- Modified parameter `cancelRestrictions`:
  - enum removed: `NEW`, `PARTIALLY_FILLED`
  - affected methods:
    - `order_cancel()` (`order.cancel` method)
    - `order_cancel_replace()` (`order.cancelReplace` method)
- Modified parameter `closePosition`:
  - enum added: `true`, `false`
  - affected methods:
    - `new_order()` (`order.place` method)
    - `new_algo_order()` (`algoOrder.place` method)
- Modified parameter `newOrderRespType`:
  - enum removed: `MARKET`, `LIMIT`
  - affected methods:
    - `order_cancel_replace()` (`order.cancelReplace` method)
    - `order_place()` (`order.place` method)
    - `order_test()` (`order.test` method)
    - `order_list_place()` (`orderList.place` method)
    - `order_list_place_oco()` (`orderList.place.oco` method)
    - `order_list_place_opo()` (`orderList.place.opo` method)
    - `order_list_place_opoco()` (`orderList.place.opoco` method)
    - `order_list_place_oto()` (`orderList.place.oto` method)
    - `order_list_place_otoco()` (`orderList.place.otoco` method)
    - `sor_order_place()` (`sor.order.place` method)
    - `sor_order_test()` (`sor.order.test` method)
- Modified parameter `pegOffsetType`:
  - enum removed: `NON_REPRESENTABLE`
  - affected methods:
    - `order_cancel_replace()` (`order.cancelReplace` method)
    - `order_place()` (`order.place` method)
    - `order_test()` (`order.test` method)
- Modified parameter `pegPriceType`:
  - enum removed: `NON_REPRESENTABLE`
  - affected methods:
    - `order_cancel_replace()` (`order.cancelReplace` method)
    - `order_place()` (`order.place` method)
    - `order_test()` (`order.test` method)
- Modified parameter `priceMatch`:
  - enum removed: `NONE`
  - affected methods:
    - `modify_order()` (`order.modify` method)
    - `new_order()` (`order.place` method)
    - `new_algo_order()` (`algoOrder.place` method)
    - `modify_order()` (`order.modify` method)
    - `new_order()` (`order.place` method)
- Modified parameter `priceProtect`:
  - enum added: `true`, `false`
  - affected methods:
    - `new_order()` (`order.place` method)
    - `new_algo_order()` (`algoOrder.place` method)
- Modified parameter `reduceOnly`:
  - enum added: `true`, `false`
  - affected methods:
    - `new_order()` (`order.place` method)
    - `new_algo_order()` (`algoOrder.place` method)
    - `new_order()` (`order.place` method)
- Modified parameter `selfTradePreventionMode`:
  - enum added: `NONE`
  - affected methods:
    - `new_algo_order()` (`algoOrder.place` method)
- Modified parameter `selfTradePreventionMode`:
  - enum added: `NONE`
  - affected methods:
    - `new_order()` (`order.place` method)
- Modified parameter `selfTradePreventionMode`:
  - enum removed: `NON_REPRESENTABLE`
  - affected methods:
    - `order_cancel_replace()` (`order.cancelReplace` method)
    - `order_place()` (`order.place` method)
    - `order_test()` (`order.test` method)
    - `order_list_place()` (`orderList.place` method)
    - `order_list_place_oco()` (`orderList.place.oco` method)
    - `order_list_place_opo()` (`orderList.place.opo` method)
    - `order_list_place_opoco()` (`orderList.place.opoco` method)
    - `order_list_place_oto()` (`orderList.place.oto` method)
    - `order_list_place_otoco()` (`orderList.place.otoco` method)
    - `sor_order_place()` (`sor.order.place` method)
    - `sor_order_test()` (`sor.order.test` method)
- Modified parameter `symbolStatus`:
  - enum removed: `END_OF_DAY`, `NON_REPRESENTABLE`
  - affected methods:
    - `depth()` (`depth` method)
    - `exchange_info()` (`exchangeInfo` method)
    - `execution_rules()` (`executionRules` method)
    - `ticker()` (`ticker` method)
    - `ticker24hr()` (`ticker.24hr` method)
    - `ticker_book()` (`ticker.book` method)
    - `ticker_price()` (`ticker.price` method)
    - `ticker_trading_day()` (`ticker.tradingDay` method)
- Modified parameter `symbolStatus`:
  - enum removed: `END_OF_DAY`, `NON_REPRESENTABLE`
  - affected methods:
    - `reference_price_calculation()` (`referencePrice.calculation` method)
- Modified parameter `timeInForce`:
  - enum removed: `GTX`, `GTD`, `RPI`
  - affected methods:
    - `new_algo_order()` (`algoOrder.place` method)
- Modified parameter `timeInForce`:
  - enum removed: `NON_REPRESENTABLE`
  - affected methods:
    - `order_cancel_replace()` (`order.cancelReplace` method)
    - `order_place()` (`order.place` method)
    - `order_test()` (`order.test` method)
    - `sor_order_place()` (`sor.order.place` method)
    - `sor_order_test()` (`sor.order.test` method)
- Modified parameter `type`:
  - enum added: `STOP_MARKET`, `TAKE_PROFIT_MARKET`, `STOP`, `TAKE_PROFIT`, `TRAILING_STOP_MARKET`
  - affected methods:
    - `new_algo_order()` (`algoOrder.place` method)
- Modified parameter `type`:
  - enum added: `LIMIT`, `MARKET`
  - affected methods:
    - `new_order()` (`order.place` method)
- Modified parameter `type`:
  - enum removed: `NON_REPRESENTABLE`
  - affected methods:
    - `order_cancel_replace()` (`order.cancelReplace` method)
    - `order_place()` (`order.place` method)
    - `order_test()` (`order.test` method)
- Modified parameter `type`:
  - enum removed: `STOP_LOSS`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT`, `TAKE_PROFIT_LIMIT`, `LIMIT_MAKER`, `NON_REPRESENTABLE`
  - affected methods:
    - `sor_order_place()` (`sor.order.place` method)
    - `sor_order_test()` (`sor.order.test` method)
- Modified parameter `windowSize`:
  - enum added: `7d`
  - affected methods:
    - `ticker()` (`ticker` method)
- Modified response for `query_order()` (`order.status` method):
  - `result`: property `cumQty` added

- Modified response for `order_book()` (`depth` method):
  - property `asks` added
  - property `bids` added
  - `result`: property `bids` deleted
  - `result`: property `asks` deleted

- Modified response for `query_order()` (`order.status` method):
  - `result`: property `selfTradePreventionMode` added
  - `result`: property `goodTillDate` added
  - `result`: property `priceMatch` added

- Modified response for `symbol_order_book_ticker()` (`ticker.book` method):
  - oneOf modified

- Modified response for `symbol_price_ticker()` (`ticker.price` method):
  - oneOf modified

- Modified response for `all_orders()` (`allOrders` method):
  - `result`.items: property `trailingDelta` added
  - `result`.items: property `expiryReason` added
  - `result`.items: property `peggedPrice` added
  - `result`.items: property `pegOffsetType` added
  - `result`.items: property `trailingTime` added
  - `result`.items: property `usedSor` added
  - `result`.items: property `strategyId` added
  - `result`.items: property `strategyType` added
  - `result`.items: property `workingFloor` added
  - `result`.items: property `pegOffsetValue` added
  - `result`.items: property `pegPriceType` added
  - `result`.items: item property `trailingDelta` added
  - `result`.items: item property `expiryReason` added
  - `result`.items: item property `peggedPrice` added
  - `result`.items: item property `pegOffsetType` added
  - `result`.items: item property `trailingTime` added
  - `result`.items: item property `usedSor` added
  - `result`.items: item property `strategyId` added
  - `result`.items: item property `strategyType` added
  - `result`.items: item property `workingFloor` added
  - `result`.items: item property `pegOffsetValue` added
  - `result`.items: item property `pegPriceType` added

- Modified response for `depth()` (`depth` method):
  - `result`.`asks`.items: minItems `0` → `2`
  - `result`.`asks`.items: maxItems `null` → `2`
  - `result`.`bids`.items: minItems `0` → `2`
  - `result`.`bids`.items: maxItems `null` → `2`

- Modified response for `exchange_info()` (`exchangeInfo` method):
  - property `exchangeFilters` added
  - property `serverTime` added
  - property `sors` added
  - property `symbols` added
  - property `timezone` added
  - property `status` deleted
  - property `id` deleted
  - property `result` deleted

- Modified response for `klines()` (`klines` method):
  - `result`.items: minItems `0` → `12`
  - `result`.items: maxItems `null` → `12`
  - `result`.items.items: oneOf added 2 schema(s)
  - `result`.items.items: oneOf removed 2 schema(s)

- Modified response for `my_filters()` (`myFilters` method):
  - property `assetFilters` added
  - property `exchangeFilters` added
  - property `symbolFilters` added
  - property `result` deleted
  - property `status` deleted
  - property `id` deleted

- Modified response for `open_orders_cancel_all()` (`openOrders.cancelAll` method):
  - `result`.items: property `preventedMatchId` added
  - `result`.items: property `usedSor` added
  - `result`.items: property `workingFloor` added
  - `result`.items: property `expiryReason` added
  - `result`.items: property `pegOffsetValue` added
  - `result`.items: property `preventedQuantity` added
  - `result`.items: property `pegPriceType` added
  - `result`.items: property `peggedPrice` added
  - `result`.items: property `pegOffsetType` added
  - `result`.items.`orderReports`.items: property `workingFloor` added
  - `result`.items.`orderReports`.items: property `expiryReason` added
  - `result`.items.`orderReports`.items: property `trailingDelta` added
  - `result`.items.`orderReports`.items: property `pegPriceType` added
  - `result`.items.`orderReports`.items: property `peggedPrice` added
  - `result`.items.`orderReports`.items: property `strategyType` added
  - `result`.items.`orderReports`.items: property `preventedQuantity` added
  - `result`.items.`orderReports`.items: property `trailingTime` added
  - `result`.items.`orderReports`.items: property `usedSor` added
  - `result`.items.`orderReports`.items: property `preventedMatchId` added
  - `result`.items.`orderReports`.items: property `pegOffsetValue` added
  - `result`.items.`orderReports`.items: property `icebergQty` added
  - `result`.items.`orderReports`.items: property `pegOffsetType` added
  - `result`.items.`orderReports`.items: property `strategyId` added
  - `result`.items.`orderReports`.items: item property `workingFloor` added
  - `result`.items.`orderReports`.items: item property `expiryReason` added
  - `result`.items.`orderReports`.items: item property `trailingDelta` added
  - `result`.items.`orderReports`.items: item property `pegPriceType` added
  - `result`.items.`orderReports`.items: item property `peggedPrice` added
  - `result`.items.`orderReports`.items: item property `strategyType` added
  - `result`.items.`orderReports`.items: item property `preventedQuantity` added
  - `result`.items.`orderReports`.items: item property `trailingTime` added
  - `result`.items.`orderReports`.items: item property `usedSor` added
  - `result`.items.`orderReports`.items: item property `preventedMatchId` added
  - `result`.items.`orderReports`.items: item property `pegOffsetValue` added
  - `result`.items.`orderReports`.items: item property `icebergQty` added
  - `result`.items.`orderReports`.items: item property `pegOffsetType` added
  - `result`.items.`orderReports`.items: item property `strategyId` added
  - `result`.items: item property `preventedMatchId` added
  - `result`.items: item property `usedSor` added
  - `result`.items: item property `workingFloor` added
  - `result`.items: item property `expiryReason` added
  - `result`.items: item property `pegOffsetValue` added
  - `result`.items: item property `preventedQuantity` added
  - `result`.items: item property `pegPriceType` added
  - `result`.items: item property `peggedPrice` added
  - `result`.items: item property `pegOffsetType` added
  - `result`.items.`orderReports`.items: property `workingFloor` added
  - `result`.items.`orderReports`.items: property `expiryReason` added
  - `result`.items.`orderReports`.items: property `trailingDelta` added
  - `result`.items.`orderReports`.items: property `pegPriceType` added
  - `result`.items.`orderReports`.items: property `peggedPrice` added
  - `result`.items.`orderReports`.items: property `strategyType` added
  - `result`.items.`orderReports`.items: property `preventedQuantity` added
  - `result`.items.`orderReports`.items: property `trailingTime` added
  - `result`.items.`orderReports`.items: property `usedSor` added
  - `result`.items.`orderReports`.items: property `preventedMatchId` added
  - `result`.items.`orderReports`.items: property `pegOffsetValue` added
  - `result`.items.`orderReports`.items: property `icebergQty` added
  - `result`.items.`orderReports`.items: property `pegOffsetType` added
  - `result`.items.`orderReports`.items: property `strategyId` added
  - `result`.items.`orderReports`.items: item property `workingFloor` added
  - `result`.items.`orderReports`.items: item property `expiryReason` added
  - `result`.items.`orderReports`.items: item property `trailingDelta` added
  - `result`.items.`orderReports`.items: item property `pegPriceType` added
  - `result`.items.`orderReports`.items: item property `peggedPrice` added
  - `result`.items.`orderReports`.items: item property `strategyType` added
  - `result`.items.`orderReports`.items: item property `preventedQuantity` added
  - `result`.items.`orderReports`.items: item property `trailingTime` added
  - `result`.items.`orderReports`.items: item property `usedSor` added
  - `result`.items.`orderReports`.items: item property `preventedMatchId` added
  - `result`.items.`orderReports`.items: item property `pegOffsetValue` added
  - `result`.items.`orderReports`.items: item property `icebergQty` added
  - `result`.items.`orderReports`.items: item property `pegOffsetType` added
  - `result`.items.`orderReports`.items: item property `strategyId` added

- Modified response for `open_orders_status()` (`openOrders.status` method):
  - `result`.items: property `pegOffsetValue` added
  - `result`.items: property `peggedPrice` added
  - `result`.items: property `strategyId` added
  - `result`.items: property `strategyType` added
  - `result`.items: property `usedSor` added
  - `result`.items: property `trailingDelta` added
  - `result`.items: property `workingFloor` added
  - `result`.items: property `expiryReason` added
  - `result`.items: property `preventedMatchId` added
  - `result`.items: property `pegPriceType` added
  - `result`.items: property `trailingTime` added
  - `result`.items: property `pegOffsetType` added
  - `result`.items: property `preventedQuantity` added
  - `result`.items: item property `pegOffsetValue` added
  - `result`.items: item property `peggedPrice` added
  - `result`.items: item property `strategyId` added
  - `result`.items: item property `strategyType` added
  - `result`.items: item property `usedSor` added
  - `result`.items: item property `trailingDelta` added
  - `result`.items: item property `workingFloor` added
  - `result`.items: item property `expiryReason` added
  - `result`.items: item property `preventedMatchId` added
  - `result`.items: item property `pegPriceType` added
  - `result`.items: item property `trailingTime` added
  - `result`.items: item property `pegOffsetType` added
  - `result`.items: item property `preventedQuantity` added

- Modified response for `order_amend_keep_priority()` (`order.amend.keepPriority` method):
  - `result`.`amendedOrder`: property `trailingTime` added
  - `result`.`amendedOrder`: property `expiryReason` added
  - `result`.`amendedOrder`: property `strategyType` added
  - `result`.`amendedOrder`: property `peggedPrice` added
  - `result`.`amendedOrder`: property `pegOffsetType` added
  - `result`.`amendedOrder`: property `pegOffsetValue` added
  - `result`.`amendedOrder`: property `stopPrice` added
  - `result`.`amendedOrder`: property `trailingDelta` added
  - `result`.`amendedOrder`: property `pegPriceType` added
  - `result`.`amendedOrder`: property `icebergQty` added
  - `result`.`amendedOrder`: property `workingFloor` added
  - `result`.`amendedOrder`: property `preventedMatchId` added
  - `result`.`amendedOrder`: property `strategyId` added
  - `result`.`amendedOrder`: property `usedSor` added
  - `result`.`amendedOrder`: property `preventedQuantity` added

- Modified response for `order_cancel()` (`order.cancel` method):
  - `result`: property `workingFloor` added
  - `result`: property `peggedPrice` added
  - `result`: property `pegOffsetType` added
  - `result`: property `preventedMatchId` added
  - `result`: property `preventedQuantity` added
  - `result`: property `pegPriceType` added
  - `result`: property `usedSor` added
  - `result`: property `pegOffsetValue` added
  - `result`: property `trailingTime` added
  - `result`: property `expiryReason` added
  - `result`.`orderReports`.items: property `peggedPrice` added
  - `result`.`orderReports`.items: property `workingFloor` added
  - `result`.`orderReports`.items: property `pegOffsetType` added
  - `result`.`orderReports`.items: property `pegPriceType` added
  - `result`.`orderReports`.items: property `preventedQuantity` added
  - `result`.`orderReports`.items: property `trailingDelta` added
  - `result`.`orderReports`.items: property `strategyId` added
  - `result`.`orderReports`.items: property `trailingTime` added
  - `result`.`orderReports`.items: property `pegOffsetValue` added
  - `result`.`orderReports`.items: property `expiryReason` added
  - `result`.`orderReports`.items: property `icebergQty` added
  - `result`.`orderReports`.items: property `strategyType` added
  - `result`.`orderReports`.items: property `usedSor` added
  - `result`.`orderReports`.items: property `preventedMatchId` added
  - `result`.`orderReports`.items: item property `peggedPrice` added
  - `result`.`orderReports`.items: item property `workingFloor` added
  - `result`.`orderReports`.items: item property `pegOffsetType` added
  - `result`.`orderReports`.items: item property `pegPriceType` added
  - `result`.`orderReports`.items: item property `preventedQuantity` added
  - `result`.`orderReports`.items: item property `trailingDelta` added
  - `result`.`orderReports`.items: item property `strategyId` added
  - `result`.`orderReports`.items: item property `trailingTime` added
  - `result`.`orderReports`.items: item property `pegOffsetValue` added
  - `result`.`orderReports`.items: item property `expiryReason` added
  - `result`.`orderReports`.items: item property `icebergQty` added
  - `result`.`orderReports`.items: item property `strategyType` added
  - `result`.`orderReports`.items: item property `usedSor` added
  - `result`.`orderReports`.items: item property `preventedMatchId` added

- Modified response for `order_cancel_replace()` (`order.cancelReplace` method):
  - `result`.`cancelResponse`: property `icebergQty` added
  - `result`.`cancelResponse`: property `preventedMatchId` added
  - `result`.`cancelResponse`: property `workingFloor` added
  - `result`.`cancelResponse`: property `strategyId` added
  - `result`.`cancelResponse`: property `preventedQuantity` added
  - `result`.`cancelResponse`: property `pegPriceType` added
  - `result`.`cancelResponse`: property `pegOffsetValue` added
  - `result`.`cancelResponse`: property `stopPrice` added
  - `result`.`cancelResponse`: property `trailingDelta` added
  - `result`.`cancelResponse`: property `expiryReason` added
  - `result`.`cancelResponse`: property `trailingTime` added
  - `result`.`cancelResponse`: property `pegOffsetType` added
  - `result`.`cancelResponse`: property `strategyType` added
  - `result`.`cancelResponse`: property `usedSor` added
  - `result`.`cancelResponse`: property `peggedPrice` added
  - `result`.`newOrderResponse`: property `strategyType` added
  - `result`.`newOrderResponse`: property `pegOffsetType` added
  - `result`.`newOrderResponse`: property `strategyId` added
  - `result`.`newOrderResponse`: property `stopPrice` added
  - `result`.`newOrderResponse`: property `workingFloor` added
  - `result`.`newOrderResponse`: property `pegOffsetValue` added
  - `result`.`newOrderResponse`: property `preventedMatchId` added
  - `result`.`newOrderResponse`: property `trailingTime` added
  - `result`.`newOrderResponse`: property `expiryReason` added
  - `result`.`newOrderResponse`: property `pegPriceType` added
  - `result`.`newOrderResponse`: property `icebergQty` added
  - `result`.`newOrderResponse`: property `preventedQuantity` added
  - `result`.`newOrderResponse`: property `usedSor` added
  - `result`.`newOrderResponse`: property `peggedPrice` added
  - `result`.`newOrderResponse`: property `trailingDelta` added

- Modified response for `order_place()` (`order.place` method):
  - `result`: property `strategyId` added
  - `result`: property `trailingDelta` added
  - `result`: property `pegOffsetType` added
  - `result`: property `stopPrice` added
  - `result`: property `workingFloor` added
  - `result`: property `usedSor` added
  - `result`: property `trailingTime` added
  - `result`: property `peggedPrice` added
  - `result`: property `pegOffsetValue` added
  - `result`: property `pegPriceType` added
  - `result`: property `expiryReason` added
  - `result`: property `strategyType` added
  - `result`: property `icebergQty` added
  - `result`: property `preventedMatchId` added
  - `result`: property `preventedQuantity` added

- Modified response for `order_status()` (`order.status` method):
  - `result`: property `expiryReason` added
  - `result`: property `pegPriceType` added
  - `result`: property `pegOffsetType` added
  - `result`: property `peggedPrice` added
  - `result`: property `usedSor` added
  - `result`: property `workingFloor` added
  - `result`: property `pegOffsetValue` added

- Modified response for `order_list_cancel()` (`orderList.cancel` method):
  - `result`.`orderReports`.items: property `trailingTime` added
  - `result`.`orderReports`.items: property `pegOffsetValue` added
  - `result`.`orderReports`.items: property `icebergQty` added
  - `result`.`orderReports`.items: property `workingFloor` added
  - `result`.`orderReports`.items: property `pegPriceType` added
  - `result`.`orderReports`.items: property `preventedQuantity` added
  - `result`.`orderReports`.items: property `strategyType` added
  - `result`.`orderReports`.items: property `strategyId` added
  - `result`.`orderReports`.items: property `peggedPrice` added
  - `result`.`orderReports`.items: property `trailingDelta` added
  - `result`.`orderReports`.items: property `expiryReason` added
  - `result`.`orderReports`.items: property `preventedMatchId` added
  - `result`.`orderReports`.items: property `pegOffsetType` added
  - `result`.`orderReports`.items: property `usedSor` added
  - `result`.`orderReports`.items: item property `trailingTime` added
  - `result`.`orderReports`.items: item property `pegOffsetValue` added
  - `result`.`orderReports`.items: item property `icebergQty` added
  - `result`.`orderReports`.items: item property `workingFloor` added
  - `result`.`orderReports`.items: item property `pegPriceType` added
  - `result`.`orderReports`.items: item property `preventedQuantity` added
  - `result`.`orderReports`.items: item property `strategyType` added
  - `result`.`orderReports`.items: item property `strategyId` added
  - `result`.`orderReports`.items: item property `peggedPrice` added
  - `result`.`orderReports`.items: item property `trailingDelta` added
  - `result`.`orderReports`.items: item property `expiryReason` added
  - `result`.`orderReports`.items: item property `preventedMatchId` added
  - `result`.`orderReports`.items: item property `pegOffsetType` added
  - `result`.`orderReports`.items: item property `usedSor` added

- Modified response for `order_list_place()` (`orderList.place` method):
  - `result`.`orderReports`.items: property `workingFloor` added
  - `result`.`orderReports`.items: property `pegPriceType` added
  - `result`.`orderReports`.items: property `trailingDelta` added
  - `result`.`orderReports`.items: property `usedSor` added
  - `result`.`orderReports`.items: property `preventedMatchId` added
  - `result`.`orderReports`.items: property `preventedQuantity` added
  - `result`.`orderReports`.items: property `pegOffsetValue` added
  - `result`.`orderReports`.items: property `strategyId` added
  - `result`.`orderReports`.items: property `expiryReason` added
  - `result`.`orderReports`.items: property `strategyType` added
  - `result`.`orderReports`.items: property `trailingTime` added
  - `result`.`orderReports`.items: property `peggedPrice` added
  - `result`.`orderReports`.items: property `icebergQty` added
  - `result`.`orderReports`.items: property `pegOffsetType` added
  - `result`.`orderReports`.items: item property `workingFloor` added
  - `result`.`orderReports`.items: item property `pegPriceType` added
  - `result`.`orderReports`.items: item property `trailingDelta` added
  - `result`.`orderReports`.items: item property `usedSor` added
  - `result`.`orderReports`.items: item property `preventedMatchId` added
  - `result`.`orderReports`.items: item property `preventedQuantity` added
  - `result`.`orderReports`.items: item property `pegOffsetValue` added
  - `result`.`orderReports`.items: item property `strategyId` added
  - `result`.`orderReports`.items: item property `expiryReason` added
  - `result`.`orderReports`.items: item property `strategyType` added
  - `result`.`orderReports`.items: item property `trailingTime` added
  - `result`.`orderReports`.items: item property `peggedPrice` added
  - `result`.`orderReports`.items: item property `icebergQty` added
  - `result`.`orderReports`.items: item property `pegOffsetType` added

- Modified response for `order_list_place_oco()` (`orderList.place.oco` method):
  - `result`.`orderReports`.items: property `workingFloor` added
  - `result`.`orderReports`.items: property `peggedPrice` added
  - `result`.`orderReports`.items: property `strategyId` added
  - `result`.`orderReports`.items: property `usedSor` added
  - `result`.`orderReports`.items: property `pegPriceType` added
  - `result`.`orderReports`.items: property `pegOffsetType` added
  - `result`.`orderReports`.items: property `preventedMatchId` added
  - `result`.`orderReports`.items: property `expiryReason` added
  - `result`.`orderReports`.items: property `pegOffsetValue` added
  - `result`.`orderReports`.items: property `strategyType` added
  - `result`.`orderReports`.items: property `icebergQty` added
  - `result`.`orderReports`.items: property `trailingDelta` added
  - `result`.`orderReports`.items: property `preventedQuantity` added
  - `result`.`orderReports`.items: property `trailingTime` added
  - `result`.`orderReports`.items: item property `workingFloor` added
  - `result`.`orderReports`.items: item property `peggedPrice` added
  - `result`.`orderReports`.items: item property `strategyId` added
  - `result`.`orderReports`.items: item property `usedSor` added
  - `result`.`orderReports`.items: item property `pegPriceType` added
  - `result`.`orderReports`.items: item property `pegOffsetType` added
  - `result`.`orderReports`.items: item property `preventedMatchId` added
  - `result`.`orderReports`.items: item property `expiryReason` added
  - `result`.`orderReports`.items: item property `pegOffsetValue` added
  - `result`.`orderReports`.items: item property `strategyType` added
  - `result`.`orderReports`.items: item property `icebergQty` added
  - `result`.`orderReports`.items: item property `trailingDelta` added
  - `result`.`orderReports`.items: item property `preventedQuantity` added
  - `result`.`orderReports`.items: item property `trailingTime` added

- Modified response for `order_list_place_opo()` (`orderList.place.opo` method):
  - `result`.`orderReports`.items: property `strategyId` added
  - `result`.`orderReports`.items: property `preventedMatchId` added
  - `result`.`orderReports`.items: property `pegPriceType` added
  - `result`.`orderReports`.items: property `expiryReason` added
  - `result`.`orderReports`.items: property `trailingTime` added
  - `result`.`orderReports`.items: property `pegOffsetValue` added
  - `result`.`orderReports`.items: property `strategyType` added
  - `result`.`orderReports`.items: property `peggedPrice` added
  - `result`.`orderReports`.items: property `usedSor` added
  - `result`.`orderReports`.items: property `stopPrice` added
  - `result`.`orderReports`.items: property `icebergQty` added
  - `result`.`orderReports`.items: property `trailingDelta` added
  - `result`.`orderReports`.items: property `pegOffsetType` added
  - `result`.`orderReports`.items: property `workingFloor` added
  - `result`.`orderReports`.items: property `preventedQuantity` added
  - `result`.`orderReports`.items: item property `strategyId` added
  - `result`.`orderReports`.items: item property `preventedMatchId` added
  - `result`.`orderReports`.items: item property `pegPriceType` added
  - `result`.`orderReports`.items: item property `expiryReason` added
  - `result`.`orderReports`.items: item property `trailingTime` added
  - `result`.`orderReports`.items: item property `pegOffsetValue` added
  - `result`.`orderReports`.items: item property `strategyType` added
  - `result`.`orderReports`.items: item property `peggedPrice` added
  - `result`.`orderReports`.items: item property `usedSor` added
  - `result`.`orderReports`.items: item property `stopPrice` added
  - `result`.`orderReports`.items: item property `icebergQty` added
  - `result`.`orderReports`.items: item property `trailingDelta` added
  - `result`.`orderReports`.items: item property `pegOffsetType` added
  - `result`.`orderReports`.items: item property `workingFloor` added
  - `result`.`orderReports`.items: item property `preventedQuantity` added

- Modified response for `order_list_place_opoco()` (`orderList.place.opoco` method):
  - `result`.`orderReports`.items: property `expiryReason` added
  - `result`.`orderReports`.items: property `pegPriceType` added
  - `result`.`orderReports`.items: property `strategyType` added
  - `result`.`orderReports`.items: property `trailingTime` added
  - `result`.`orderReports`.items: property `strategyId` added
  - `result`.`orderReports`.items: property `usedSor` added
  - `result`.`orderReports`.items: property `pegOffsetValue` added
  - `result`.`orderReports`.items: property `peggedPrice` added
  - `result`.`orderReports`.items: property `trailingDelta` added
  - `result`.`orderReports`.items: property `workingFloor` added
  - `result`.`orderReports`.items: property `preventedQuantity` added
  - `result`.`orderReports`.items: property `icebergQty` added
  - `result`.`orderReports`.items: property `pegOffsetType` added
  - `result`.`orderReports`.items: property `preventedMatchId` added
  - `result`.`orderReports`.items: item property `expiryReason` added
  - `result`.`orderReports`.items: item property `pegPriceType` added
  - `result`.`orderReports`.items: item property `strategyType` added
  - `result`.`orderReports`.items: item property `trailingTime` added
  - `result`.`orderReports`.items: item property `strategyId` added
  - `result`.`orderReports`.items: item property `usedSor` added
  - `result`.`orderReports`.items: item property `pegOffsetValue` added
  - `result`.`orderReports`.items: item property `peggedPrice` added
  - `result`.`orderReports`.items: item property `trailingDelta` added
  - `result`.`orderReports`.items: item property `workingFloor` added
  - `result`.`orderReports`.items: item property `preventedQuantity` added
  - `result`.`orderReports`.items: item property `icebergQty` added
  - `result`.`orderReports`.items: item property `pegOffsetType` added
  - `result`.`orderReports`.items: item property `preventedMatchId` added

- Modified response for `order_list_place_oto()` (`orderList.place.oto` method):
  - `result`.`orderReports`.items: property `preventedMatchId` added
  - `result`.`orderReports`.items: property `trailingDelta` added
  - `result`.`orderReports`.items: property `pegPriceType` added
  - `result`.`orderReports`.items: property `strategyType` added
  - `result`.`orderReports`.items: property `expiryReason` added
  - `result`.`orderReports`.items: property `pegOffsetValue` added
  - `result`.`orderReports`.items: property `strategyId` added
  - `result`.`orderReports`.items: property `stopPrice` added
  - `result`.`orderReports`.items: property `pegOffsetType` added
  - `result`.`orderReports`.items: property `usedSor` added
  - `result`.`orderReports`.items: property `icebergQty` added
  - `result`.`orderReports`.items: property `trailingTime` added
  - `result`.`orderReports`.items: property `peggedPrice` added
  - `result`.`orderReports`.items: property `preventedQuantity` added
  - `result`.`orderReports`.items: property `workingFloor` added
  - `result`.`orderReports`.items: item property `preventedMatchId` added
  - `result`.`orderReports`.items: item property `trailingDelta` added
  - `result`.`orderReports`.items: item property `pegPriceType` added
  - `result`.`orderReports`.items: item property `strategyType` added
  - `result`.`orderReports`.items: item property `expiryReason` added
  - `result`.`orderReports`.items: item property `pegOffsetValue` added
  - `result`.`orderReports`.items: item property `strategyId` added
  - `result`.`orderReports`.items: item property `stopPrice` added
  - `result`.`orderReports`.items: item property `pegOffsetType` added
  - `result`.`orderReports`.items: item property `usedSor` added
  - `result`.`orderReports`.items: item property `icebergQty` added
  - `result`.`orderReports`.items: item property `trailingTime` added
  - `result`.`orderReports`.items: item property `peggedPrice` added
  - `result`.`orderReports`.items: item property `preventedQuantity` added
  - `result`.`orderReports`.items: item property `workingFloor` added

- Modified response for `order_list_place_otoco()` (`orderList.place.otoco` method):
  - `result`.`orderReports`.items: property `pegOffsetType` added
  - `result`.`orderReports`.items: property `strategyId` added
  - `result`.`orderReports`.items: property `trailingDelta` added
  - `result`.`orderReports`.items: property `expiryReason` added
  - `result`.`orderReports`.items: property `pegPriceType` added
  - `result`.`orderReports`.items: property `trailingTime` added
  - `result`.`orderReports`.items: property `preventedMatchId` added
  - `result`.`orderReports`.items: property `pegOffsetValue` added
  - `result`.`orderReports`.items: property `usedSor` added
  - `result`.`orderReports`.items: property `peggedPrice` added
  - `result`.`orderReports`.items: property `preventedQuantity` added
  - `result`.`orderReports`.items: property `icebergQty` added
  - `result`.`orderReports`.items: property `strategyType` added
  - `result`.`orderReports`.items: property `workingFloor` added
  - `result`.`orderReports`.items: item property `pegOffsetType` added
  - `result`.`orderReports`.items: item property `strategyId` added
  - `result`.`orderReports`.items: item property `trailingDelta` added
  - `result`.`orderReports`.items: item property `expiryReason` added
  - `result`.`orderReports`.items: item property `pegPriceType` added
  - `result`.`orderReports`.items: item property `trailingTime` added
  - `result`.`orderReports`.items: item property `preventedMatchId` added
  - `result`.`orderReports`.items: item property `pegOffsetValue` added
  - `result`.`orderReports`.items: item property `usedSor` added
  - `result`.`orderReports`.items: item property `peggedPrice` added
  - `result`.`orderReports`.items: item property `preventedQuantity` added
  - `result`.`orderReports`.items: item property `icebergQty` added
  - `result`.`orderReports`.items: item property `strategyType` added
  - `result`.`orderReports`.items: item property `workingFloor` added

- Modified response for `reference_price()` (`referencePrice` method):
  - property `rateLimits` added

- Modified response for `reference_price_calculation()` (`referencePrice.calculation` method):
  - property `rateLimits` added

- Modified response for `sor_order_place()` (`sor.order.place` method):
  - `result`.items: property `expiryReason` added
  - `result`.items: property `icebergQty` added
  - `result`.items: property `pegPriceType` added
  - `result`.items: property `preventedQuantity` added
  - `result`.items: property `preventedMatchId` added
  - `result`.items: property `strategyId` added
  - `result`.items: property `stopPrice` added
  - `result`.items: property `pegOffsetValue` added
  - `result`.items: property `trailingTime` added
  - `result`.items: property `strategyType` added
  - `result`.items: property `peggedPrice` added
  - `result`.items: property `pegOffsetType` added
  - `result`.items: property `trailingDelta` added
  - `result`.items: item property `expiryReason` added
  - `result`.items: item property `icebergQty` added
  - `result`.items: item property `pegPriceType` added
  - `result`.items: item property `preventedQuantity` added
  - `result`.items: item property `preventedMatchId` added
  - `result`.items: item property `strategyId` added
  - `result`.items: item property `stopPrice` added
  - `result`.items: item property `pegOffsetValue` added
  - `result`.items: item property `trailingTime` added
  - `result`.items: item property `strategyType` added
  - `result`.items: item property `peggedPrice` added
  - `result`.items: item property `pegOffsetType` added
  - `result`.items: item property `trailingDelta` added

- Modified response for `ticker()` (`ticker` method):
  - oneOf modified

- Modified response for `ticker24hr()` (`ticker.24hr` method):
  - oneOf modified

- Modified response for `ticker_book()` (`ticker.book` method):
  - oneOf modified

- Modified response for `ticker_price()` (`ticker.price` method):
  - oneOf modified

- Modified response for `ui_klines()` (`uiKlines` method):
  - `result`.items: minItems `0` → `12`
  - `result`.items: maxItems `null` → `12`
  - `result`.items.items: oneOf added 2 schema(s)
  - `result`.items.items: oneOf removed 2 schema(s)

- Marked `order_list_place()` (`orderList.place` method) as deprecated.

- Modified parameter `contractType`:
  - enum added: `perpetual`, `current_quarter`, `next_quarter`
  - affected methods:
    - `continuous_contract_kline_candlestick_streams()` (`<pair>_<contractType>@continuousKline_<interval>` stream)
- Modified parameter `contractType`:
  - enum added: `perpetual`, `current_quarter`, `next_quarter`, `tradifi_perpetual`
  - affected methods:
    - `continuous_contract_kline_candlestick_streams()` (`<pair>_<contractType>@continuousKline_<interval>` stream)
- Modified parameter `interval`:
  - enum added: `1m`, `3m`, `5m`, `15m`, `30m`, `1h`, `2h`, `4h`, `6h`, `8h`, `12h`, `1d`, `3d`, `1w`, `1M`
  - affected methods:
    - `index_kline_candlestick_streams()` (`<pair>@indexPriceKline_<interval>` stream)
    - `continuous_contract_kline_candlestick_streams()` (`<pair>_<contractType>@continuousKline_<interval>` stream)
    - `kline_candlestick_streams()` (`<symbol>@kline_<interval>` stream)
    - `mark_price_kline_candlestick_streams()` (`<symbol>@markPriceKline_<interval>` stream)
    - `kline_candlestick_streams()` (`<symbol>@kline_<interval>` stream)
- Modified parameter `interval`:
  - enum added: `1m`, `3m`, `5m`, `15m`, `30m`, `1h`, `2h`, `4h`, `6h`, `12h`, `1d`, `3d`, `1w`
  - affected methods:
    - `kline_candlestick_streams()` (`<symbol>@kline_<interval>` stream)
- Modified parameter `interval`:
  - enum added: `1s`, `1m`, `3m`, `5m`, `15m`, `30m`, `1h`, `2h`, `4h`, `6h`, `8h`, `12h`, `1d`, `3d`, `1w`, `1M`
  - affected methods:
    - `continuous_contract_kline_candlestick_streams()` (`<pair>_<contractType>@continuousKline_<interval>` stream)
- Modified parameter `level`:
  - enum added: `5`, `10`, `20`
  - affected methods:
    - `partial_book_depth_streams()` (`<symbol>@depth<level>@<updateSpeed>` stream)
- Modified parameter `levels`:
  - type `integer` → `string`
  - enum added: `5`, `10`, `20`
  - affected methods:
    - `partial_book_depth_streams()` (`<symbol>@depth<levels>@<updateSpeed>` stream)
    - `partial_book_depth_streams()` (`<symbol>@depth<levels>@<updateSpeed>` stream)
- Modified parameter `updateSpeed`:
  - enum added: `1s`
  - affected methods:
    - `mark_price_of_all_symbols_of_a_pair()` (`<pair>@markPrice@<updateSpeed>` stream)
    - `mark_price_stream()` (`<symbol>@markPrice@<updateSpeed>` stream)
    - `mark_price_stream_for_all_market()` (`!markPrice@arr@<updateSpeed>` stream)
    - `mark_price_stream()` (`<symbol>@markPrice@<updateSpeed>` stream)
- Modified parameter `updateSpeed`:
  - enum added: `100ms`, `500ms`
  - affected methods:
    - `partial_book_depth_streams()` (`<symbol>@depth<levels>@<updateSpeed>` stream)
    - `diff_book_depth_streams()` (`<symbol>@depth@<updateSpeed>` stream)
    - `partial_book_depth_streams()` (`<symbol>@depth<levels>@<updateSpeed>` stream)
    - `diff_book_depth_streams()` (`<symbol>@depth@<updateSpeed>` stream)
- Modified parameter `updateSpeed`:
  - required: `false` → `true`
  - enum added: `100ms`, `500ms`
  - affected methods:
    - `partial_book_depth_streams()` (`<symbol>@depth<level>@<updateSpeed>` stream)
    - `diff_book_depth_streams()` (`<symbol>@depth@<updateSpeed>` stream)
- Modified parameter `updateSpeed`:
  - enum added: `100ms`
  - affected methods:
    - `partial_book_depth()` (`<symbol>@depth<levels>@<updateSpeed>` stream)
    - `diff_book_depth()` (`<symbol>@depth@<updateSpeed>` stream)
- Modified response for `partial_book_depth()` (`<symbol>@depth<levels>@<updateSpeed>` stream):
  - `asks`.items: minItems `0` → `2`
  - `asks`.items: maxItems `null` → `2`
  - `bids`.items: minItems `0` → `2`
  - `bids`.items: maxItems `null` → `2`

- Modified response for `diff_book_depth()` (`<symbol>@depth@<updateSpeed>` stream):
  - `a`.items: minItems `0` → `2`
  - `a`.items: maxItems `null` → `2`
  - `b`.items: minItems `0` → `2`
  - `b`.items: maxItems `null` → `2`

### Removed (4)

- `check_collateral_repay_rate_stable_rate()` (`GET /sapi/v1/loan/repay/collateral/rate`)
- `/<pair>@index_price()` (`<pair>@indexPrice` stream)
- `/<symbol>@option_ticker()` (`<symbol>@optionTicker` stream)
- `/underlying@option_open_interest@<expiration_date>()` (`underlying@optionOpenInterest@<expirationDate>` stream)

## 60.0.0 - 2026-07-06

**Derivatives Trading Coin Futures**

### Added (1)

#### WebSocket Streams

- `index_price_stream()` (`<pair>@indexPrice` stream)

### Removed (2)

#### REST API

- `classic_portfolio_margin_account_information()` (`GET /dapi/v1/pmAccountInfo`)

#### WebSocket Streams

- `/<pair>@index_price@<update_speed>()` (`<pair>@indexPrice@<updateSpeed>` stream)

**Wallet**

### Changed (1)

- Modified response for `dustlog()` (`GET /sapi/v1/asset/dribblet`):
  - `userAssetDribblets`.items.`userAssetDribbletDetails`.items: property `targetAsset` added
  - `userAssetDribblets`.items.`userAssetDribbletDetails`.items: item property `targetAsset` added
  - `userAssetDribblets`.items.`userAssetDribbletDetails`.items: property `targetAsset` added
  - `userAssetDribblets`.items.`userAssetDribbletDetails`.items: item property `targetAsset` added

## 59.0.0 - 2026-06-30

**Derivatives Trading Coin Futures**

### Changed (1)

#### REST API

- Modified response for `query_index_price_constituents()` (`GET /dapi/v1/constituents`):
  - `constituents`.items: property `price` added
  - `constituents`.items: property `weight` added
  - `constituents`.items: item property `price` added
  - `constituents`.items: item property `weight` added

**Wallet**

### Changed (2)

- Modified response for `broker_withdraw()` (`POST /sapi/v1/localentity/broker/withdraw/apply`):
  - property `accepted` added
  - property `accpted` deleted

- Modified response for `withdraw_travel_rule()` (`POST /sapi/v1/localentity/withdraw/apply`):
  - property `accepted` added
  - property `accpted` deleted

## 58.0.0 - 2026-06-29

### Added (1)

- Added W3W Prediction REST API (`w3w_prediction` module).

## 57.0.0 - 2026-06-22

**Derivatives Trading Usds Futures**

### Changed (1)

#### WebSocket Streams

- Modified response for `individual_symbol_book_ticker_streams()` (`<symbol>@bookTicker` stream):
  - property `ps` added

## 56.0.0 - 2026-06-16

**Vip Loan**

### Added (2)

- `query_vip_loan_fixed_rate_market()` (`GET /sapi/v1/loan/vip/fixed/market`)
- `vip_loan_fixed_rate_borrow()` (`POST /sapi/v1/loan/vip/fixed/borrow`)

## 55.0.0 - 2026-06-10

**Derivatives Trading Coin Futures**

### Changed (13)

#### WebSocket Streams

- Modified response for `all_book_tickers_stream()` (`!bookTicker` stream):
  - property `st` added

- Modified response for `contract_info_stream()` (`!contractInfo` stream):
  - property `st` added

- Modified response for `all_market_liquidation_order_streams()` (`!forceOrder@arr` stream):
  - property `st` added

- Modified response for `all_market_mini_tickers_stream()` (`!miniTicker@arr` stream):
  - items: property `st` added
  - items: item property `st` added

- Modified response for `all_market_tickers_streams()` (`!ticker@arr` stream):
  - items: property `st` added
  - items: item property `st` added

- Modified response for `aggregate_trade_streams()` (`<symbol>@aggTrade` stream):
  - property `st` added

- Modified response for `individual_symbol_book_ticker_streams()` (`<symbol>@bookTicker` stream):
  - property `st` added

- Modified response for `partial_book_depth_streams()` (`<symbol>@depth<levels>@<updateSpeed>` stream):
  - property `st` added

- Modified response for `diff_book_depth_streams()` (`<symbol>@depth@<updateSpeed>` stream):
  - property `st` added

- Modified response for `mark_price_stream()` (`<symbol>@markPrice@<updateSpeed>` stream):
  - property `st` added

- Modified response for `individual_symbol_mini_ticker_stream()` (`<symbol>@miniTicker` stream):
  - property `st` added

- Modified response for `individual_symbol_ticker_streams()` (`<symbol>@ticker` stream):
  - property `st` added

- Modified response for `mark_price_of_all_symbols_of_a_pair()` (`<pair>@markPrice@<updateSpeed>` stream):
  - items: property `st` added
  - items: item property `st` added

**Derivatives Trading Usds Futures**

### Changed (15)

#### REST API

- Modified response for `asset_index()` (`GET /fapi/v1/assetIndex`):
  - oneOf added 2 schema(s)
  - oneOf removed 2 schema(s)

#### WebSocket Streams

- Modified response for `all_book_tickers_stream()` (`!bookTicker` stream):
  - property `ps` added
  - property `st` added

- Modified response for `contract_info_stream()` (`!contractInfo` stream):
  - property `st` added
  - property `ps` deleted

- Modified response for `all_market_liquidation_order_streams()` (`!forceOrder@arr` stream):
  - property `ps` added
  - property `st` added

- Modified response for `mark_price_stream_for_all_market()` (`!markPrice@arr@<updateSpeed>` stream):
  - items: property `st` added
  - items: item property `st` added

- Modified response for `all_market_mini_tickers_stream()` (`!miniTicker@arr` stream):
  - items: property `ps` added
  - items: property `st` added
  - items: item property `ps` added
  - items: item property `st` added

- Modified response for `all_market_tickers_streams()` (`!ticker@arr` stream):
  - items: property `ps` added
  - items: property `st` added
  - items: item property `ps` added
  - items: item property `st` added

- Modified response for `aggregate_trade_streams()` (`<symbol>@aggTrade` stream):
  - property `st` added

- Modified response for `individual_symbol_book_ticker_streams()` (`<symbol>@bookTicker` stream):
  - property `st` added

- Modified response for `partial_book_depth_streams()` (`<symbol>@depth<levels>@<updateSpeed>` stream):
  - property `ps` added
  - property `st` added

- Modified response for `diff_book_depth_streams()` (`<symbol>@depth@<updateSpeed>` stream):
  - property `ps` added
  - property `st` added

- Modified response for `mark_price_stream()` (`<symbol>@markPrice@<updateSpeed>` stream):
  - property `st` added

- Modified response for `individual_symbol_mini_ticker_stream()` (`<symbol>@miniTicker` stream):
  - property `st` added
  - property `ps` added

- Modified response for `rpi_diff_book_depth_streams()` (`<symbol>@rpiDepth@500ms` stream):
  - property `st` added
  - property `ps` added

- Modified response for `individual_symbol_ticker_streams()` (`<symbol>@ticker` stream):
  - property `st` added
  - property `ps` added

## 54.0.0 - 2026-06-08

**Fiat**

### Changed (1)

- Added parameters `account_type`, `agency` and `bank_code_for_pix` on `AccountInfo`
  - affected methods:
    - `fiat_withdraw()` (`POST /sapi/v2/fiat/withdraw`)

## 53.0.0 - 2026-06-05

**Derivatives Trading Usds Futures**

### Changed (9)

#### REST API

- Modified response for `compressed_aggregate_trades_list()` (`GET /fapi/v1/aggTrades`):
  - items: property `nq` added
  - items: item property `nq` added

- Modified response for `query_algo_order()` (`GET /fapi/v1/algoOrder`):
  - property `actualQty` added
  - property `actualType` added
  - property `tpPrice` deleted
  - property `tpTriggerPrice` deleted
  - property `slPrice` deleted
  - property `slTriggerPrice` deleted

- Modified response for `trading_schedule()` (`GET /fapi/v1/tradingSchedule`):
  - `marketSchedules`: property `KR_EQUITY` added

#### WebSocket API

- Deleted parameter `activationPrice`
  - affected methods:
    - `new_order()` (`order.place` method)
- Deleted parameter `callbackRate`
  - affected methods:
    - `new_order()` (`order.place` method)
- Deleted parameter `closePosition`
  - affected methods:
    - `new_order()` (`order.place` method)
- Deleted parameter `priceProtect`
  - affected methods:
    - `new_order()` (`order.place` method)
- Deleted parameter `stopPrice`
  - affected methods:
    - `new_order()` (`order.place` method)
- Deleted parameter `workingType`
  - affected methods:
    - `new_order()` (`order.place` method)

**Sub Account**

### Changed (1)

- Added parameter `includeSource`
  - affected methods:
    - `get_sub_account_deposit_history()` (`GET /sapi/v1/capital/deposit/subHisrec`)

## 52.0.0 - 2026-06-02

**Simple Earn**

### Added (1)

- `get_yield_arena_activities()` (`GET /sapi/v1/earn/arena/activities`)

**Wallet**

### Added (2)

- `get_country_list()` (`GET /sapi/v1/localentity/country/list`)
- `get_region_list()` (`GET /sapi/v1/localentity/region/list`)

## 51.0.0 - 2026-05-26

### Changed (3)

- Fix race in WebSocket API `send` where a fast response could be dropped before the pending request was registered.
- Fix duplicate subscription events caused by spurious reconnect after WebSocket connection renewal.
- Fix flaky WebSocket unit tests.

**Wallet**

### Changed (1)

- Added parameter `accountType`
  - affected methods:
    - `dust_convert()` (`POST /sapi/v1/asset/dust-convert/convert`)
    - `dust_convertible_assets()` (`POST /sapi/v1/asset/dust-convert/query-convertible-assets`)

## 50.0.0 - 2026-05-20

**Simple Earn**

### Changed (3)

- Modified response for `get_locked_redemption_record()` (`GET /sapi/v1/simple-earn/locked/history/redemptionRecord`):
  - `rows`.items.`deliverDate`: type `string` → `integer`
  - `rows`.items.`deliverDate`: type `string` → `integer`

- Modified response for `get_locked_product_position()` (`GET /sapi/v1/simple-earn/locked/position`):
  - `rows`.items.`deliverDate`: type `string` → `integer`
  - `rows`.items.`nextPayDate`: type `string` → `integer`
  - `rows`.items.`partialAmtDeliverDate`: type `string` → `integer`
  - `rows`.items.`purchaseTime`: type `string` → `integer`
  - `rows`.items.`rewardsEndDate`: type `string` → `integer`
  - `rows`.items.`deliverDate`: type `string` → `integer`
  - `rows`.items.`nextPayDate`: type `string` → `integer`
  - `rows`.items.`partialAmtDeliverDate`: type `string` → `integer`
  - `rows`.items.`purchaseTime`: type `string` → `integer`
  - `rows`.items.`rewardsEndDate`: type `string` → `integer`

- Modified response for `get_locked_subscription_preview()` (`GET /sapi/v1/simple-earn/locked/subscriptionPreview`):
  - items.`deliverDate`: type `string` → `integer`
  - items.`nextPayDate`: type `string` → `integer`
  - items.`nextSubscriptionDate`: type `string` → `integer`
  - items.`rewardsEndDate`: type `string` → `integer`
  - items.`valueDate`: type `string` → `integer`
  - items.`deliverDate`: type `string` → `integer`
  - items.`nextPayDate`: type `string` → `integer`
  - items.`nextSubscriptionDate`: type `string` → `integer`
  - items.`rewardsEndDate`: type `string` → `integer`
  - items.`valueDate`: type `string` → `integer`

## 49.0.0 - 2026-05-12

**Derivatives Trading Usds Futures**

### Changed (1)

#### WebSocket Streams

- Modified response for `Listenkeyexpired`:
  - `E`: type `string` → `integer`

**Spot**

### Added (1)

#### WebSocket Streams

- `block_trade()` (`<symbol>@blockTrade` stream)

## 48.0.1 - 2026-05-08

### Changed (1)

- Fix bug with unordered WS Streams events.

## 48.0.0 - 2026-05-06

**Spot**

### Added (2)

#### REST API

- `historical_block_trades()` (`GET /api/v3/historicalBlockTrades`)

#### WebSocket API

- `block_trades_historical()` (`blockTrades.historical` method)

### Changed (3)

#### REST API

- Modified parameter `selfTradePreventionMode`:
  - enum added: `TRANSFER`
  - affected methods:
    - `new_order()` (`POST /api/v3/order`)
    - `order_cancel_replace()` (`POST /api/v3/order/cancelReplace`)
    - `order_oco()` (`POST /api/v3/order/oco`)
    - `order_test()` (`POST /api/v3/order/test`)
    - `order_list_oco()` (`POST /api/v3/orderList/oco`)
    - `order_list_opo()` (`POST /api/v3/orderList/opo`)
    - `order_list_opoco()` (`POST /api/v3/orderList/opoco`)
    - `order_list_oto()` (`POST /api/v3/orderList/oto`)
    - `order_list_otoco()` (`POST /api/v3/orderList/otoco`)
    - `sor_order()` (`POST /api/v3/sor/order`)
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Marked `order_oco()` (`POST /api/v3/order/oco`) as deprecated.

#### WebSocket API

- Modified parameter `selfTradePreventionMode`:
  - enum added: `TRANSFER`
  - affected methods:
    - `order_cancel_replace()` (`order.cancelReplace` method)
    - `order_place()` (`order.place` method)
    - `order_test()` (`order.test` method)
    - `order_list_place()` (`orderList.place` method)
    - `order_list_place_oco()` (`orderList.place.oco` method)
    - `order_list_place_opo()` (`orderList.place.opo` method)
    - `order_list_place_opoco()` (`orderList.place.opoco` method)
    - `order_list_place_oto()` (`orderList.place.oto` method)
    - `order_list_place_otoco()` (`orderList.place.otoco` method)
    - `sor_order_place()` (`sor.order.place` method)
    - `sor_order_test()` (`sor.order.test` method)

**Sub Account**

### Changed (2)

- Added parameter `rows`
  - affected methods:
    - `get_move_position_history_for_sub_account()` (`GET /sapi/v1/sub-account/futures/move-position`)
- Deleted parameter `row`
  - affected methods:
    - `get_move_position_history_for_sub_account()` (`GET /sapi/v1/sub-account/futures/move-position`)

## 47.0.0 - 2026-05-05

**Derivatives Trading Portfolio Margin**

### Changed (4)

#### REST API

- Deleted parameter `closePosition`
  - affected methods:
    - `new_um_algo_order()` (`POST /papi/v1/um/algo/order`)
- Modified parameter `quantity`:
  - required: `false` → `true`
  - affected methods:
    - `new_um_algo_order()` (`POST /papi/v1/um/algo/order`)
- Modified response for `new_um_algo_order()` (`POST /papi/v1/um/algo/order`):
  - property `closePosition` deleted
- Modified response for `cancel_um_algo_order()` (`DELETE /papi/v1/um/algo/order`):
  - property `complete` added
  - property `clientAlgoId` deleted
  - property `code` deleted
  - property `msg` deleted
  - property `algoId` deleted

## 46.0.0 - 2026-04-20

**C2c**

### Changed (1)

- Modified response for `get_c2_c_trade_history()` (`GET /sapi/v1/c2c/orderMatch/listUserOrderHistory`):
  - property `commission` added
  - property `tradeType` added
  - property `unitPrice` added
  - property `advNo` added
  - property `fiat` added
  - property `takerCommission` added
  - property `payMethodName` added
  - property `asset` added
  - property `fiatSymbol` added
  - property `createTime` added
  - property `takerAmount` added
  - property `amount` added
  - property `orderNumber` added
  - property `totalPrice` added
  - property `additionalKycVerify` added
  - property `takerCommissionRate` added
  - property `counterPartNickName` added
  - property `orderStatus` added
  - property `code` deleted
  - property `data` deleted
  - property `message` deleted
  - property `success` deleted
  - property `total` deleted

**Derivatives Trading Portfolio Margin**

### Added (7)

#### REST API

- `cancel_all_um_algo_open_orders()` (`DELETE /papi/v1/um/algo/allOpenOrders`)
- `cancel_um_algo_order()` (`DELETE /papi/v1/um/algo/order`)
- `new_um_algo_order()` (`POST /papi/v1/um/algo/order`)
- `query_all_current_um_open_algo_orders()` (`GET /papi/v1/um/algo/openAlgoOrders`)
- `query_current_um_open_algo_order()` (`GET /papi/v1/um/algo/algoOrder`)
- `query_um_algo_order_history()` (`GET /papi/v1/um/algo/allAlgoOrders`)

#### WebSocket Streams

- Added `AlgoUpdate` on `UserDataStreamEventsResponse` list.

### Changed (7)

#### REST API

- Marked `cancel_all_um_open_conditional_orders()` (`DELETE /papi/v1/um/conditional/allOpenOrders`) as deprecated.
- Marked `cancel_um_conditional_order()` (`DELETE /papi/v1/um/conditional/order`) as deprecated.
- Marked `new_um_conditional_order()` (`POST /papi/v1/um/conditional/order`) as deprecated.
- Marked `query_all_current_um_open_conditional_orders()` (`GET /papi/v1/um/conditional/openOrders`) as deprecated.
- Marked `query_all_um_conditional_orders()` (`GET /papi/v1/um/conditional/allOrders`) as deprecated.
- Marked `query_current_um_open_conditional_order()` (`GET /papi/v1/um/conditional/openOrder`) as deprecated.
- Marked `query_um_conditional_order_history()` (`GET /papi/v1/um/conditional/orderHistory`) as deprecated.

**Derivatives Trading Portfolio Margin Pro**

### Added (1)

#### WebSocket Streams

- Added `PmProAccountUpdate` on `UserDataStreamEventsResponse` list.

**Derivatives Trading Usds Futures**

### Changed (3)

#### REST API

- Deleted parameter `page`
  - affected methods:
    - `query_all_algo_orders()` (`GET /fapi/v1/allAlgoOrders`)
- Modified parameter `interval`:
  - enum added: `1s`
  - affected methods:
    - `continuous_contract_kline_candlestick_data()` (`GET /fapi/v1/continuousKlines`)
    - `index_price_kline_candlestick_data()` (`GET /fapi/v1/indexPriceKlines`)
    - `kline_candlestick_data()` (`GET /fapi/v1/klines`)
    - `mark_price_kline_candlestick_data()` (`GET /fapi/v1/markPriceKlines`)
    - `premium_index_kline_data()` (`GET /fapi/v1/premiumIndexKlines`)
- Modified parameter `limit`:
  - required: `true` → `false`
  - affected methods:
    - `basis()` (`GET /futures/data/basis`)

**Spot**

### Changed (1)

#### WebSocket API

- Modified response for `reference_price()` (`referencePrice` method):
  - `result`: property `msg` added
  - `result`: property `code` added

## 45.0.0 - 2026-04-15

**Derivatives Trading Portfolio Margin**

### Added (1)

#### REST API

- `futures_tradfi_perps_contract()` (`POST /papi/v1/um/stock/contract`)

### Changed (1)

#### REST API

- Modified response for `get_um_income_history()` (`GET /papi/v1/um/income`):
  - items.`tranId`: type `string` → `integer`
  - items.`tranId`: type `string` → `integer`

**Derivatives Trading Portfolio Margin Pro**

### Added (3)

#### REST API

- `delete_margin_call_level()` (`DELETE /sapi/v1/portfolio/margin-call-level`)
- `get_margin_call_level()` (`GET /sapi/v1/portfolio/margin-call-level`)
- `set_margin_call_level()` (`POST /sapi/v1/portfolio/margin-call-level`)

**Pay**

### Changed (1)

- Modified response for `get_pay_trade_history()` (`GET /sapi/v1/pay/transactions`):
  - `data`.items.`fundsDetail`.items.`walletAssetCost`: type `array` → `object`
  - `data`.items.`fundsDetail`.items.`walletAssetCost`: property `2` added
  - `data`.items.`fundsDetail`.items.`walletAssetCost`: property `1` added
  - `data`.items.`fundsDetail`.items.`walletAssetCost`: type `array` → `object`
  - `data`.items.`fundsDetail`.items.`walletAssetCost`: property `2` added
  - `data`.items.`fundsDetail`.items.`walletAssetCost`: property `1` added
  - `data`.items.`fundsDetail`.items.`walletAssetCost`: type `array` → `object`
  - `data`.items.`fundsDetail`.items.`walletAssetCost`: property `2` added
  - `data`.items.`fundsDetail`.items.`walletAssetCost`: property `1` added
  - `data`.items.`fundsDetail`.items.`walletAssetCost`: type `array` → `object`
  - `data`.items.`fundsDetail`.items.`walletAssetCost`: property `2` added
  - `data`.items.`fundsDetail`.items.`walletAssetCost`: property `1` added

## 44.0.1 - 2026-03-24

### Changed (2)

- Fix bug with exposed secrets via Debug Trait.
- Mark NFT as Deprecated.

## 44.0.0 - 2026-03-19

**Derivatives Trading Options**

### Changed (9)

#### REST API

- Added parameter `selfTradePreventionMode`
  - affected methods:
    - `new_order()` (`POST /eapi/v1/order`)
- Modified parameter `orders`:
  - items: property `selfTradePreventionMode` added
  - items: item property `selfTradePreventionMode` added
  - affected methods:
    - `place_multiple_orders()` (`POST /eapi/v1/batchOrders`)
- Modified response for `cancel_multiple_option_orders()` (`DELETE /eapi/v1/batchOrders`):
  - items: property `selfTradePreventionMode` added
  - items: item property `selfTradePreventionMode` added

- Modified response for `place_multiple_orders()` (`POST /eapi/v1/batchOrders`):
  - items: property `selfTradePreventionMode` added
  - items: item property `selfTradePreventionMode` added

- Modified response for `option_margin_account_information()` (`GET /eapi/v1/marginAccount`):
  - property `tradeGroupId` added

- Modified response for `query_current_open_option_orders()` (`GET /eapi/v1/openOrders`):
  - items: property `selfTradePreventionMode` added
  - items: item property `selfTradePreventionMode` added

- Modified response for `cancel_option_order()` (`DELETE /eapi/v1/order`):
  - property `selfTradePreventionMode` added

- Modified response for `query_single_order()` (`GET /eapi/v1/order`):
  - property `selfTradePreventionMode` added

- Modified response for `new_order()` (`POST /eapi/v1/order`):
  - property `selfTradePreventionMode` added

**Derivatives Trading Usds Futures**

### Changed (2)

#### WebSocket Streams

- Modified response for `mark_price_stream_for_all_market()` (`!markPrice@arr@<updateSpeed>` stream):
  - items: property `ap` added
  - items: item property `ap` added

- Modified response for `mark_price_stream()` (`<symbol>@markPrice@<updateSpeed>` stream):
  - property `ap` added

**Staking**

### Changed (6)

- Added parameter `purchaseId`
  - affected methods:
    - `get_eth_staking_history()` (`GET /sapi/v1/eth-staking/eth/history/stakingHistory`)
    - `get_sol_staking_history()` (`GET /sapi/v1/sol-staking/sol/history/stakingHistory`)
- Added parameter `redeemId`
  - affected methods:
    - `get_eth_redemption_history()` (`GET /sapi/v1/eth-staking/eth/history/redemptionHistory`)
    - `get_sol_redemption_history()` (`GET /sapi/v1/sol-staking/sol/history/redemptionHistory`)
- Modified response for `redeem_eth()` (`POST /sapi/v1/eth-staking/eth/redeem`):
  - property `redeemId` added

- Modified response for `redeem_sol()` (`POST /sapi/v1/sol-staking/sol/redeem`):
  - property `redeemId` added

- Modified response for `subscribe_sol_staking()` (`POST /sapi/v1/sol-staking/sol/stake`):
  - property `purchaseId` added

- Modified response for `subscribe_eth_staking()` (`POST /sapi/v2/eth-staking/eth/stake`):
  - property `purchaseId` added

## 43.0.0 - 2026-03-12

**Derivatives Trading Options**

### Changed (1)

#### REST API

- Modified response for `cancel_all_option_orders_on_specific_symbol()` (`DELETE /eapi/v1/allOpenOrders`):
  - `code`: type `integer` → `string`

**Derivatives Trading Usds Futures**

### Changed (1)

#### REST API

- Modified response for `query_order()` (`GET /fapi/v1/order`):
  - property `origQty` added
  - property `symbol` added
  - property `cumQuote` added
  - property `priceRate` added
  - property `type` added
  - property `time` added
  - property `orderId` added
  - property `timeInForce` added
  - property `priceProtect` added
  - property `closePosition` added
  - property `activatePrice` added
  - property `stopPrice` added
  - property `executedQty` added
  - property `reduceOnly` added
  - property `avgPrice` added
  - property `price` added
  - property `side` added
  - property `positionSide` added
  - property `updateTime` added
  - property `workingType` added
  - property `origType` added
  - property `clientOrderId` added
  - property `id` deleted
  - property `result` deleted
  - `status`: type `integer` → `string`

**Simple Earn**

### Changed (1)

- Modified response for `get_bfusd_quota_details()` (`GET /sapi/v1/bfusd/quota`):
  - property `subscriptionQuota` added

**Spot**

### Added (7)

#### REST API

- `execution_rules()` (`GET /api/v3/executionRules`)
- `reference_price()` (`GET /api/v3/referencePrice`)
- `reference_price_calculation()` (`GET /api/v3/referencePrice/calculation`)

#### WebSocket API

- `execution_rules()` (`executionRules` method)
- `reference_price()` (`referencePrice` method)
- `reference_price_calculation()` (`referencePrice.calculation` method)

#### WebSocket Streams

- `reference_price()` (`<symbol>@referencePrice` stream)

## 42.0.0 - 2026-03-05

**Derivatives Trading Coin Futures**

### Changed (1)

#### REST API

- Modified response for `exchange_information()` (`GET /dapi/v1/exchangeInfo`):
  - `symbols`.items: property `orderTypes` added
  - `symbols`.items: property `OrderType` deleted
  - `symbols`.items: item property `orderTypes` added
  - `symbols`.items: item property `OrderType` deleted

**Derivatives Trading Usds Futures**

### Changed (2)

#### REST API

- Modified response for `exchange_information()` (`GET /fapi/v1/exchangeInfo`):
  - `symbols`.items: property `orderTypes` added
  - `symbols`.items: property `OrderType` deleted
  - `symbols`.items: item property `orderTypes` added
  - `symbols`.items: item property `OrderType` deleted

- Modified response for `cancel_order()` (`DELETE /fapi/v1/order`):
  - property `avgPrice` added

## 41.0.0 - 2026-03-02

**Wallet**

### Changed (1)

- Modified response for `vasp_list()` (`GET /sapi/v1/localentity/vasp`):
  - items: property `identifier` added
  - items: item property `identifier` added

## 40.1.0 - 2026-02-16

### Added (1)

- Support `rust-tls` backend for non-signed requests (`openssl-tls` remains the default selected option).

## 40.0.0 - 2026-02-13

**Margin Trading**

### Added (2)

#### REST API

- `get_margin_restricted_assets()` (`GET /sapi/v1/margin/restricted-asset`)
- `query_prevented_matches()` (`GET /sapi/v1/margin/myPreventedMatches`)

## 39.0.0 - 2026-02-02

**Derivatives Trading Portfolio Margin**

### Changed (1)

#### REST API

- Modified parameter `strategyType`:
  - enum added: `LIMIT_MAKER`
  - affected methods:
    - `new_cm_conditional_order()` (`POST /papi/v1/cm/conditional/order`)
    - `new_um_conditional_order()` (`POST /papi/v1/um/conditional/order`)

**Sub Account**

### Changed (2)

- Added parameter `limit`
  - affected methods:
    - `get_summary_of_sub_accounts_futures_account()` (`GET /sapi/v1/sub-account/futures/accountSummary`)
- Added parameter `page`
  - affected methods:
    - `get_summary_of_sub_accounts_futures_account()` (`GET /sapi/v1/sub-account/futures/accountSummary`)

## 38.0.0 - 2026-01-27

**Convert**

### Changed (1)

- Modified response for `place_limit_order()` (`POST /sapi/v1/convert/limit/placeOrder`):
  - property `orderId` added
  - property `status` added
  - property `toAmount` deleted
  - property `validTimestamp` deleted
  - property `fromAmount` deleted
  - property `inverseRatio` deleted
  - property `quoteId` deleted
  - property `ratio` deleted

**Derivatives Trading Coin Futures**

### Changed (2)

#### REST API

- Modified response for `cancel_multiple_orders()` (`DELETE /dapi/v1/batchOrders`):
  - items: property `pair` added
  - items: item property `pair` added

- Modified response for `current_all_open_orders()` (`GET /dapi/v1/openOrders`):
  - items: property `pair` added
  - items: item property `pair` added

**Derivatives Trading Usds Futures**

### Changed (6)

#### REST API

- Added parameter `newOrderRespType`
  - affected methods:
    - `new_algo_order()` (`POST /fapi/v1/algoOrder`)
- Modified parameter `batchOrders`:
  - items: property `stopPrice` added
  - items: item property `stopPrice` added
  - affected methods:
    - `modify_multiple_orders()` (`PUT /fapi/v1/batchOrders`)
- Modified response for `place_multiple_orders()` (`POST /fapi/v1/batchOrders`):
  - items: property `closePosition` added
  - items: item property `closePosition` added

- Modified response for `query_order()` (`GET /fapi/v1/order`):
  - property `result` added
  - property `id` added
  - property `executedQty` deleted
  - property `reduceOnly` deleted
  - property `priceProtect` deleted
  - property `closePosition` deleted
  - property `type` deleted
  - property `avgPrice` deleted
  - property `orderId` deleted
  - property `updateTime` deleted
  - property `activatePrice` deleted
  - property `timeInForce` deleted
  - property `cumQuote` deleted
  - property `workingType` deleted
  - property `goodTillDate` deleted
  - property `positionSide` deleted
  - property `side` deleted
  - property `origQty` deleted
  - property `time` deleted
  - property `selfTradePreventionMode` deleted
  - property `clientOrderId` deleted
  - property `origType` deleted
  - property `price` deleted
  - property `priceMatch` deleted
  - property `stopPrice` deleted
  - property `priceRate` deleted
  - property `symbol` deleted
  - `status`: type `string` → `integer`

#### WebSocket API

- Added parameter `newOrderRespType`
  - affected methods:
    - `new_algo_order()` (`algoOrder.place` method)
- Modified response for `position_information_v2()` (`v2/account.position` method):
  - `result`.items: property `unRealizedProfit` added
  - `result`.items: property `unrealizedProfit` deleted
  - `result`.items: item property `unRealizedProfit` added
  - `result`.items: item property `unrealizedProfit` deleted

**Margin Trading**

### Added (1)

#### REST API

- `get_margin_asset_risk_based_liquidation_ratio()` (`GET /sapi/v1/margin/risk-based-liquidation-ratio`)

**Spot**

### Changed (1)

#### WebSocket API

- Added parameter `recvWindow`
  - affected methods:
    - `user_data_stream_subscribe_signature()` (`userDataStream.subscribe.signature` method)

**Wallet**

### Changed (1)

- Modified response for `asset_dividend_record()` (`GET /sapi/v1/asset/assetDividend`):
  - `rows`.items: property `direction` added
  - `rows`.items: item property `direction` added

## 37.0.0 - 2026-01-20

### Added (1)

- Expose `code` on REST API response errors.

**Alpha**

- Initial release.

**Derivatives Trading Portfolio Margin**

### Changed (1)

#### REST API

- Modified response for `query_current_cm_open_order()` (`GET /papi/v1/cm/openOrder`):
  - type `object` → `array`
  - property `side` deleted
  - property `reduceOnly` deleted
  - property `orderId` deleted
  - property `updateTime` deleted
  - property `status` deleted
  - property `avgPrice` deleted
  - property `clientOrderId` deleted
  - property `type` deleted
  - property `time` deleted
  - property `cumBase` deleted
  - property `executedQty` deleted
  - property `price` deleted
  - property `symbol` deleted
  - property `positionSide` deleted
  - property `origQty` deleted
  - property `origType` deleted
  - property `pair` deleted
  - property `timeInForce` deleted

**Mining**

### Changed (2)

- Deleted parameter `userName`
  - affected methods:
    - `hashrate_resale_detail()` (`GET /sapi/v1/mining/hash-transfer/profit/details`)
- Modified response for `hashrate_resale_list()` (`GET /sapi/v1/mining/hash-transfer/config/details/list`):
  - `data`.`configDetails`.items: property `type` added
  - `data`.`configDetails`.items: item property `type` added

**Simple Earn**

### Changed (1)

- Modified response for `get_bfusd_quota_details()` (`GET /sapi/v1/bfusd/quota`):
  - property `subscribeEnable` deleted
  - property `redeemEnable` deleted

**Vip Loan**

### Changed (1)

- Modified response for `get_vip_loan_ongoing_orders()` (`GET /sapi/v1/loan/vip/ongoing/orders`):
  - `rows`.items: property `loanRate` added
  - `rows`.items: item property `loanRate` added

**Wallet**

### Changed (4)

- Added parameter `accountType`
  - affected methods:
    - `dustlog()` (`GET /sapi/v1/asset/dribblet`)
- Added parameter `asset`
  - affected methods:
    - `asset_detail()` (`GET /sapi/v1/asset/assetDetail`)
- Modified response for `withdraw_history_v1()` (`GET /sapi/v1/localentity/withdraw/history`):
  - items: property `addressTag` deleted
  - items: item property `addressTag` deleted

- Modified response for `withdraw_history_v2()` (`GET /sapi/v2/localentity/withdraw/history`):
  - items: property `addressTag` deleted
  - items: item property `addressTag` deleted

## 36.0.0 - 2026-01-13

**Crypto Loan**

### Added (1)

- `get_flexible_loan_interest_rate_history()` (`GET /sapi/v2/loan/interestRateHistory`)

**Derivatives Trading Coin Futures**

### Added (1)

#### REST API

- `place_multiple_orders()` (`POST /dapi/v1/batchOrders`)

**Derivatives Trading Options**

- Update REST API and Websocket Streams to match latest API changes.

**Derivatives Trading Portfolio Margin Pro**

### Added (2)

#### REST API

- `get_delta_mode_status()` (`GET /sapi/v1/portfolio/delta-mode`)
- `switch_delta_mode()` (`POST /sapi/v1/portfolio/delta-mode`)

**Derivatives Trading Usds Futures**

### Changed (12)

#### REST API

- Added parameter `algo_id`
  - affected methods:
    - `cancel_algo_order()` (`DELETE /fapi/v1/algoOrder`)
- Added parameter `client_algo_id`
  - affected methods:
    - `cancel_algo_order()` (`DELETE /fapi/v1/algoOrder`)
- Deleted parameter `algo_id`
  - affected methods:
    - `cancel_algo_order()` (`DELETE /fapi/v1/algoOrder`)
- Deleted parameter `client_algo_id`
  - affected methods:
    - `cancel_algo_order()` (`DELETE /fapi/v1/algoOrder`)
- Modified response for `symbol_configuration()` (`GET /fapi/v1/symbolConfig`):
  - items.`is_auto_add_margin`: type `string` → `boolean`
  - items.`is_auto_add_margin`: type `string` → `boolean`

#### WebSocket API

- Added parameter `algo_id`
  - affected methods:
    - `cancel_algo_order()` (`algoOrder.cancel` method)
- Added parameter `client_algo_id`
  - affected methods:
    - `cancel_algo_order()` (`algoOrder.cancel` method)
- Deleted parameter `algo_id`
  - affected methods:
    - `cancel_algo_order()` (`algoOrder.cancel` method)
- Deleted parameter `client_algo_id`
  - affected methods:
    - `cancel_algo_order()` (`algoOrder.cancel` method)
- Added parameter `activate_price`
  - affected methods:
    - `new_algo_order()` (`algoOrder.place` method)
- Deleted parameter `activation_price`
  - affected methods:
    - `new_algo_order()` (`algoOrder.place` method)

#### WebSocket Streams

- Modified response for `aggregate_trade_streams()` (`<symbol>@aggTrade` stream):
  - property `nq` added

**Dual Investment**

### Changed (1)

- Update documentation URLs.

**VIP Loan**

### Added (2)

- `get_vip_loan_accrued_interest()` (`GET /sapi/v1/loan/vip/accruedInterest`)
- `get_vip_loan_interest_rate_history()` (`GET /sapi/v1/loan/vip/interestRateHistory`)

**Wallet**

### Added (1)

- `submit_deposit_questionnaire_v2()` (`PUT /sapi/v2/localentity/deposit/provide-info`)

### Changed (1)

- Modified parameter `deposit_id`:
  - type `string` → `integer`
  - affected methods:
    - `submit_deposit_questionnaire()` (`PUT /sapi/v1/localentity/broker/deposit/provide-info`)

## 35.0.0 - 2025-12-19

### Changed (1)

- Support integer randomisation on WS streams subscription/unsubscription.

**Derivatives Trading Options**

### Changed (1)

#### WebSocket Streams

- Modified parameter `id`:
  - type `string` → `integer`
  - affected methods:
    - `partial_book_depth_streams()` (`<symbol>@depth<levels>@<updateSpeed>` stream)
    - `index_price_streams()` (`<symbol>@index` stream)
    - `kline_candlestick_streams()` (`<symbol>@kline_<interval>` stream)
    - `ticker24_hour()` (`<symbol>@ticker` stream)
    - `trade_streams()` (`<symbol>@trade` stream)
    - `mark_price()` (`<underlyingAsset>@markPrice` stream)
    - `open_interest()` (`<underlyingAsset>@openInterest@<expirationDate>` stream)
    - `ticker24_hour_by_underlying_asset_and_expiration_data()` (`<underlyingAsset>@ticker@<expirationDate>` stream)
    - `new_symbol_info()` (`option_pair` stream)

**Derivatives Trading Usds Futures**

### Changed (2)

#### REST API

- Added parameter `activatePrice`
  - affected methods:
    - `new_algo_order()` (`POST /fapi/v1/algoOrder`)
- Deleted parameter `activationPrice`
  - affected methods:
    - `new_algo_order()` (`POST /fapi/v1/algoOrder`)

**Spot**

### Added (1)

#### WebSocket API

- `order_amend_keep_priority()` (`order.amend.keepPriority` method)

**Wallet**

### Added (2)

- `dust_convert()` (`POST /sapi/v1/asset/dust-convert/convert`)
- `dust_convertible_assets()` (`POST /sapi/v1/asset/dust-convert/query-convertible-assets`)

## 34.0.0 - 2025-12-16

### Changed (1)

- Support request body params on `send_request` and `send_signed_request` functions across all products.

**Derivatives Trading Coin Futures**

### Changed (1)

#### REST API

- Modified parameter `batchOrders`:
  - items.`orderId`: type `integer` → `string`
  - items.`price`: type `number` → `string`
  - items.`quantity`: type `number` → `string`
  - items.`recvWindow`: type `integer` → `string`
  - items.`orderId`: type `integer` → `string`
  - items.`price`: type `number` → `string`
  - items.`quantity`: type `number` → `string`
  - items.`recvWindow`: type `integer` → `string`
  - affected methods:
    - `modify_multiple_orders()` (`PUT /dapi/v1/batchOrders`)

**Derivatives Trading Options**

### Changed (1)

#### REST API

- Modified parameter `orders`:
  - items.`isMmp`: type `boolean` → `string`
  - items.`postOnly`: type `boolean` → `string`
  - items.`price`: type `number` → `string`
  - items.`quantity`: type `number` → `string`
  - items.`reduceOnly`: type `boolean` → `string`
  - items.`isMmp`: type `boolean` → `string`
  - items.`postOnly`: type `boolean` → `string`
  - items.`price`: type `number` → `string`
  - items.`quantity`: type `number` → `string`
  - items.`reduceOnly`: type `boolean` → `string`
  - affected methods:
    - `place_multiple_orders()` (`POST /eapi/v1/batchOrders`)

**Derivatives Trading Portfolio Margin**

### Changed (1)

#### REST API

- Modified response for `um_position_adl_quantile_estimation()` (`GET /papi/v1/um/adlQuantile`):
  - items.`adlQuantile`: property `HEDGE` deleted

**Derivatives Trading Usds Futures**

### Added (3)

#### REST API

- `futures_tradfi_perps_contract()` (`POST /fapi/v1/stock/contract`)
- `trading_schedule()` (`GET /fapi/v1/tradingSchedule`)

#### WebSocket Streams

- `trading_session_stream()` (`tradingSession` stream)

### Changed (11)

#### REST API

- Deleted parameter `activationPrice`
  - affected methods:
    - `new_order()` (`POST /fapi/v1/order`)
- Deleted parameter `callbackRate`
  - affected methods:
    - `new_order()` (`POST /fapi/v1/order`)
- Deleted parameter `closePosition`
  - affected methods:
    - `new_order()` (`POST /fapi/v1/order`)
- Deleted parameter `priceProtect`
  - affected methods:
    - `new_order()` (`POST /fapi/v1/order`)
- Deleted parameter `stopPrice`
  - affected methods:
    - `new_order()` (`POST /fapi/v1/order`)
- Deleted parameter `workingType`
  - affected methods:
    - `new_order()` (`POST /fapi/v1/order`)
- Modified parameter `batchOrders`:
  - items: property `activationPrice` deleted
  - items: property `callbackRate` deleted
  - items: property `stopPrice` deleted
  - items: property `workingType` deleted
  - items: property `priceProtect` deleted
  - items: item property `activationPrice` deleted
  - items: item property `callbackRate` deleted
  - items: item property `stopPrice` deleted
  - items: item property `workingType` deleted
  - items: item property `priceProtect` deleted
  - affected methods:
    - `place_multiple_orders()` (`POST /fapi/v1/batchOrders`)
- Modified parameter `batchOrders`:
  - items.`orderId`: type `integer` → `string`
  - items.`price`: type `number` → `string`
  - items.`quantity`: type `number` → `string`
  - items.`recvWindow`: type `integer` → `string`
  - items.`orderId`: type `integer` → `string`
  - items.`price`: type `number` → `string`
  - items.`quantity`: type `number` → `string`
  - items.`recvWindow`: type `integer` → `string`
  - affected methods:
    - `modify_multiple_orders()` (`PUT /fapi/v1/batchOrders`)

- Modified response for `place_multiple_orders()` (`POST /fapi/v1/batchOrders`):
  - items: property `priceRate` deleted
  - items: property `activatePrice` deleted
  - items: item property `priceRate` deleted
  - items: item property `activatePrice` deleted

- Modified response for `new_order()` (`POST /fapi/v1/order`):
  - property `activatePrice` deleted
  - property `priceRate` deleted

#### WebSocket API

- Modified response for `cancel_algo_order()` (`algoOrder.cancel` method):
  - `result`: property `msg` added
  - `result`: property `code` added
  - `result`: property `icebergQuantity` deleted
  - `result`: property `orderType` deleted
  - `result`: property `createTime` deleted
  - `result`: property `algoStatus` deleted
  - `result`: property `reduceOnly` deleted
  - `result`: property `updateTime` deleted
  - `result`: property `triggerPrice` deleted
  - `result`: property `positionSide` deleted
  - `result`: property `priceMatch` deleted
  - `result`: property `closePosition` deleted
  - `result`: property `timeInForce` deleted
  - `result`: property `quantity` deleted
  - `result`: property `goodTillDate` deleted
  - `result`: property `triggerTime` deleted
  - `result`: property `priceProtect` deleted
  - `result`: property `workingType` deleted
  - `result`: property `algoType` deleted
  - `result`: property `price` deleted
  - `result`: property `side` deleted
  - `result`: property `selfTradePreventionMode` deleted
  - `result`: property `symbol` deleted

**Fiat**

### Added (3)

- `deposit()` (`POST /sapi/v1/fiat/deposit`)
- `fiat_withdraw()` (`POST /sapi/v2/fiat/withdraw`)
- `get_order_detail()` (`GET /sapi/v1/fiat/get-order-detail`)

**Spot**

### Added (4)

#### REST API

- `order_list_opo()` (`POST /api/v3/orderList/opo`)
- `order_list_opoco()` (`POST /api/v3/orderList/opoco`)

#### WebSocket API

- `order_list_place_opo()` (`orderList.place.opo` method)
- `order_list_place_opoco()` (`orderList.place.opoco` method)

### Changed (2)

#### REST API

- Modified response for `exchange_info()` (`GET /api/v3/exchangeInfo`):
  - `symbols`.items: property `opoAllowed` added
  - `symbols`.items: item property `opoAllowed` added

#### WebSocket API

- Modified response for `exchange_info()` (`exchangeInfo` method):
  - `result`.`symbols`.items: property `opoAllowed` added
  - `result`.`symbols`.items: item property `opoAllowed` added

### Removed (2)

#### WebSocket API

- `/order.amend.keep_priority()` (`order.amend.keepPriority` method)

#### WebSocket Streams

- `/!ticker@arr()` (`!ticker@arr` stream)

## 33.0.1 - 2025-12-01

### Changed (2)

- Removed unstable `if let` chains on rust versions older than 1.90.0.
- Added logger initialisation on examples.

## 33.0.0 - 2025-11-27

### Changed (1)

- The crate no longer auto-initializes a global `tracing` subscriber. Logging configuration is now opt-in and must be performed by the application. The existing helper `logger::init()` is still available, but it is no longer called automatically by the library.

  - If you previously relied on the connector to set up logging for you, call this once in your application before creating any clients:

    ```rust
    use binance_sdk::logger;

    fn main() {
      logger::init();
    }
    ```

  - If your application already configures a global `tracing` subscriber, nothing changes: `logger::init()` is not called by the library and if you call it yourself it will early-return when a subscriber is already set.

**Derivatives Trading Usds Futures**

### Added (2)

#### REST API

- `rpi_order_book()` (`GET /fapi/v1/rpiDepth`)

#### WebSocket Streams

- `rpi_diff_book_depth_streams()` (`<symbol>@rpiDepth@500ms` stream)

### Changed (2)

#### REST API

- Modified response for `user_commission_rate()` (`GET /dapi/v1/commissionRate`):
  - property `rpi_commission_rate` added

#### WebSocket Streams

- Modified `UserDataStreamEventsResponse` for `AlgoUpdate`:
  - `o`: property `rm` added

**Simple Earn**

### Changed (2)

- Modified response for `get_bfusd_account()` (`GET /sapi/v1/bfusd/account`):
  - property `usdt_profit` added
  - property `bfusd_profit` added
  - property `total_profit` removed

- Modified response for `get_bfusd_rewards_history()` (`GET /sapi/v1/bfusd/history/rewardsHistory`):
  - `rows`.items: property `reward_asset` added

## 32.0.0 - 2025-11-20

**Derivatives Trading Portfolio Margin Pro**

### Changed (1)

#### REST API

- Renamed `transfer_ldusdt_for_portfolio_margin()` to `transfer_ldusdt_rwusd_for_portfolio_margin()`.

**Derivatives Trading Usds Futures**

### Added (1)

#### REST API

- `adl_risk()` (`GET /fapi/v1/symbolAdlRisk`)

## 31.0.0 - 2025-11-18

**Derivatives Trading Options**

### Changed (1)

#### REST API

- Renamed `symbol_price_ticker()` to `index_price_ticker()`.

### Changed (1)

#### WebSocket Streams

- Modified response for `trade_streams()` (`<symbol>@trade` method):
  - `t`: number -> string

**Derivatives Trading Portfolio Margin**

### Changed (1)

#### WebSocket Streams

- Modified response for `user_data()` method:
  - removed `m_uppercase` from `Executionreport`

**Derivatives Trading Usds Futures**

### Changed (5)

#### REST API

- Modified parameter `batchOrders`:
  - items.`timeInForce`: enum added: `RPI`
  - items.`timeInForce`: enum added: `RPI`
  - affected methods:
    - `place_multiple_orders()` (`POST /fapi/v1/batchOrders`)
- Modified parameter `timeInForce`:
  - enum added: `RPI`
  - affected methods:
    - `new_algo_order()` (`POST /fapi/v1/algoOrder`)
    - `new_order()` (`POST /fapi/v1/order`)
    - `test_order()` (`POST /fapi/v1/order/test`)
- Modified response for `old_trades_lookup()` (`GET /fapi/v1/historicalTrades`):
  - items: property `isRPITrade` added
  - items: item property `isRPITrade` added

- Modified response for `recent_trades_list()` (`GET /fapi/v1/trades`):
  - items: property `isRPITrade` added
  - items: item property `isRPITrade` added

#### WebSocket API

- Modified parameter `timeInForce`:
  - enum added: `RPI`
  - affected methods:
    - `new_algo_order()` (`algoOrder.place` method)
    - `new_order()` (`order.place` method)

**Fiat**

### Removed (2)

- `fiat_withdraw()` (`GET /sapi/v2/fiat/withdraw`)
- `get_order_detail()` (`GET /sapi/v1/fiat/get-order-detail`)

**Spot**

### Changed (1)

#### WebSocket Streams

- Marked `all_ticker()` (`!ticker@arr` stream) as deprecated.

## 30.0.0 - 2025-11-10

**C2C**

### Changed (4)

- Added parameter `endTimestamp`
  - affected methods:
    - `get_c2_c_trade_history()` (`GET /sapi/v1/c2c/orderMatch/listUserOrderHistory`)
- Added parameter `startTimestamp`
  - affected methods:
    - `get_c2_c_trade_history()` (`GET /sapi/v1/c2c/orderMatch/listUserOrderHistory`)
- Deleted parameter `endTime`
  - affected methods:
    - `get_c2_c_trade_history()` (`GET /sapi/v1/c2c/orderMatch/listUserOrderHistory`)
- Deleted parameter `startTime`
  - affected methods:
    - `get_c2_c_trade_history()` (`GET /sapi/v1/c2c/orderMatch/listUserOrderHistory`)

**Derivatives Trading Portfolio Margin Pro**

### Removed (2)

#### REST API

- `mint_bfusd_for_portfolio_margin()` (`POST /sapi/v1/portfolio/mint`)
- `redeem_bfusd_for_portfolio_margin()` (`POST /sapi/v1/portfolio/redeem`)

**Derivatives Trading Usds Futures**

### Added (2)

#### WebSocket API

- `cancel_algo_order()` (`algoOrder.cancel` method)
- `new_algo_order()` (`algoOrder.place` method)

**Fiat**

### Added (2)

- `fiat_withdraw()` (`GET /sapi/v2/fiat/withdraw`)
- `get_order_detail()` (`GET /sapi/v1/fiat/get-order-detail`)

**Margin Trading**

### Removed (6)

#### REST API

- `close_isolated_margin_user_data_stream()` (`DELETE /sapi/v1/userDataStream/isolated`)
- `close_margin_user_data_stream()` (`DELETE /sapi/v1/userDataStream`)
- `keepalive_isolated_margin_user_data_stream()` (`PUT /sapi/v1/userDataStream/isolated`)
- `keepalive_margin_user_data_stream()` (`PUT /sapi/v1/userDataStream`)
- `start_isolated_margin_user_data_stream()` (`POST /sapi/v1/userDataStream/isolated`)
- `start_margin_user_data_stream()` (`POST /sapi/v1/userDataStream`)

## 29.0.0 - 2025-11-07

**Derivatives Trading Usds Futures**

### Added (6)

#### REST API

- `cancel_algo_order()` (`DELETE /fapi/v1/algoOrder`)
- `cancel_all_algo_open_orders()` (`DELETE /fapi/v1/algoOpenOrders`)
- `current_all_algo_open_orders()` (`GET /fapi/v1/openAlgoOrders`)
- `new_algo_order()` (`POST /fapi/v1/algoOrder`)
- `query_algo_order()` (`GET /fapi/v1/algoOrder`)
- `query_all_algo_orders()` (`GET /fapi/v1/allAlgoOrders`)

## 28.0.0 - 2025-11-06

**C2c**

### Changed (2)

- Added parameter `rows`
  - affected methods:
    - `get_c2_c_trade_history()` (`GET /sapi/v1/c2c/orderMatch/listUserOrderHistory`)
- Added parameter `tradeType`
  - affected methods:
    - `get_c2_c_trade_history()` (`GET /sapi/v1/c2c/orderMatch/listUserOrderHistory`)

## 27.0.0 - 2025-10-30

**Simple Earn**

### Added (8)

- `get_bfusd_account()` (`GET /sapi/v1/bfusd/account`)
- `get_bfusd_quota_details()` (`GET /sapi/v1/bfusd/quota`)
- `get_bfusd_rate_history()` (`GET /sapi/v1/bfusd/history/rateHistory`)
- `get_bfusd_redemption_history()` (`GET /sapi/v1/bfusd/history/redemptionHistory`)
- `get_bfusd_rewards_history()` (`GET /sapi/v1/bfusd/history/rewardsHistory`)
- `get_bfusd_subscription_history()` (`GET /sapi/v1/bfusd/history/subscriptionHistory`)
- `redeem_bfusd()` (`POST /sapi/v1/bfusd/redeem`)
- `subscribe_bfusd()` (`POST /sapi/v1/bfusd/subscribe`)

**Spot**

### Changed (2)

#### REST API

- Added parameter `symbolStatus`
  - affected methods:
    - `depth()` (`GET /api/v3/depth`)
    - `ticker()` (`GET /api/v3/ticker`)
    - `ticker24hr()` (`GET /api/v3/ticker/24hr`)
    - `ticker_book_ticker()` (`GET /api/v3/ticker/bookTicker`)
    - `ticker_price()` (`GET /api/v3/ticker/price`)
    - `ticker_trading_day()` (`GET /api/v3/ticker/tradingDay`)

#### WebSocket API

- Added parameter `symbolStatus`
  - affected methods:
    - `depth()` (`depth` method)
    - `ticker()` (`ticker` method)
    - `ticker24hr()` (`ticker.24hr` method)
    - `ticker_book()` (`ticker.book` method)
    - `ticker_price()` (`ticker.price` method)
    - `ticker_trading_day()` (`ticker.tradingDay` method)

**Staking**

### Changed (1)

- Modified response for `get_current_eth_staking_quota()` (`GET /sapi/v1/eth-staking/eth/quota`):
  - property `minRedeemAmount` added
  - property `redeemPeriod` added
  - property `stakeable` added
  - property `commissionFee` added
  - property `redeemable` added
  - property `calculating` added
  - property `minStakeAmount` added

## 26.0.0 - 2025-10-27

**Derivatives Trading Usds Futures**

### Changed (1)

#### REST API

- Marked `symbol_price_ticker` (`GET /fapi/v1/ticker/price`) as deprecated.

**Margin Trading**

### Changed (6)

#### REST API

- Marked `close_isolated_margin_user_data_stream` (`DELETE /sapi/v1/userDataStream/isolated`) as deprecated
- Marked `close_margin_user_data_stream` (`DELETE /sapi/v1/userDataStream`) as deprecated
- Marked `keepalive_isolated_margin_user_data_stream` (`PUT /sapi/v1/userDataStream/isolated`) as deprecated
- Marked `keepalive_margin_user_data_stream` (`PUT /sapi/v1/userDataStream`) as deprecated
- Marked `start_isolated_margin_user_data_stream` (`POST /sapi/v1/userDataStream/isolated`) as deprecated
- Marked `start_margin_user_data_stream` (`POST /sapi/v1/userDataStream`) as deprecated

**Spot**

### Changed (2)

#### REST API

- Marked `order_oco` (`POST /api/v3/order/oco`) as deprecated.

#### WebSocket API

- Marked `order_list_place` (`orderList.place` method) as deprecated.

### Removed (6)

#### REST API

- `delete_user_data_stream()` (`DELETE /api/v3/userDataStream`)
- `new_user_data_stream()` (`POST /api/v3/userDataStream`)
- `put_user_data_stream()` (`PUT /api/v3/userDataStream`)

#### WebSocket API

- `/user_data_stream.ping()` (`userDataStream.ping` method)
- `/user_data_stream.start()` (`userDataStream.start` method)
- `/user_data_stream.stop()` (`userDataStream.stop` method)

## 25.0.0 - 2025-10-20

**Derivatives Trading Usds Futures**

### Changed (1)

#### WebSocket Streams

- Modified User Data Streams response for `OrderTradeUpdateO`:
  - `er` added 

## 24.0.0 - 2025-10-09

**Derivatives Trading Coin Futures**

### Changed (1)

#### REST API

- Modified response for `query_order()` (`GET /dapi/v1/order`):
  - property `positionSide` added

**Derivatives Trading Options**

### Changed (4)

#### REST API

- Deleted parameter `price`
  - affected methods:
    - `new_block_trade_order()` (`POST /eapi/v1/block/order/create`)
- Deleted parameter `quantity`
  - affected methods:
    - `new_block_trade_order()` (`POST /eapi/v1/block/order/create`)
- Deleted parameter `side`
  - affected methods:
    - `new_block_trade_order()` (`POST /eapi/v1/block/order/create`)
- Deleted parameter `symbol`
  - affected methods:
    - `new_block_trade_order()` (`POST /eapi/v1/block/order/create`)

**Spot**

### Changed (4)

#### REST API

- Modified response for `exchange_info()` (`GET /api/v3/exchangeInfo`):
  - modified `exchange_filters` and `symbols`.`filters`

- Modified response for `my_filters()` (`GET /api/v3/myFilters`):
  - modified `asset_filters`, `exchange_filters` and `symbol_filters`

#### WebSocket API

- Modified response for `exchange_info()` (`exchangeInfo` method):
  - modified `result`.`exchange_filters` and `result`.`symbols`.`filters`

- Modified response for `my_filters()` (`myFilters` method):
  - modified `result`.`asset_filters`, `result`.`exchange_filters` and `result`.`symbol_filters`

## 23.0.0 - 2025-10-06

**Derivatives Trading Options**

### Changed (1)

#### REST API

- Deleted parameter `limit`
  - affected methods:
    - `query_current_open_option_orders()` (`GET /eapi/v1/openOrders`)

**Sub Account**

### Changed (1)

- Modified parameter `orderArgs`:
  - item property `positionSide` added
  - item property `quantity` added
  - item property `symbol` added
  - affected methods:
    - `move_position_for_sub_account()` (`POST /sapi/v1/sub-account/futures/move-position`)

## 22.0.0 - 2025-09-29

**Derivatives Trading Portfolio Margin Pro**

### Changed (2)

#### REST API

- Modified response for `mint_bfusd_for_portfolio_margin()` (`POST /sapi/v1/portfolio/mint`):
  - property `mintRate` added
  - property `rate` deleted

- Modified response for `redeem_bfusd_for_portfolio_margin()` (`POST /sapi/v1/portfolio/redeem`):
  - property `redeemRate` added
  - property `rate` deleted

## 21.0.0 - 2025-09-24

**Spot**

### Changed (2)

#### WebSocket API

- Modified parameter `belowTimeInForce`:
  - enum removed: `belowType`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT_LIMIT`
  - enum added: `GTC`, `IOC`, `FOK`
  - affected methods:
    - `order_list_place_oco()` (`orderList.place.oco` method)

## 20.0.0 - 2025-09-19

**Convert**

### Changed (1)

- Update `query_limit_open_orders()` HTTP method (`GET` from `POST`)

**Spot**

### Changed (2)

#### REST API

- Modified parameter `recvWindow`:
  - type `integer` → `number`
  - format `int64` → `float`
  - affected methods:
    - `get_account()` (`GET /api/v3/account`)
    - `all_order_list()` (`GET /api/v3/allOrderList`)
    - `all_orders()` (`GET /api/v3/allOrders`)
    - `my_allocations()` (`GET /api/v3/myAllocations`)
    - `my_prevented_matches()` (`GET /api/v3/myPreventedMatches`)
    - `my_trades()` (`GET /api/v3/myTrades`)
    - `open_order_list()` (`GET /api/v3/openOrderList`)
    - `delete_open_orders()` (`DELETE /api/v3/openOrders`)
    - `get_open_orders()` (`GET /api/v3/openOrders`)
    - `delete_order()` (`DELETE /api/v3/order`)
    - `get_order()` (`GET /api/v3/order`)
    - `new_order()` (`POST /api/v3/order`)
    - `order_amend_keep_priority()` (`PUT /api/v3/order/amend/keepPriority`)
    - `order_amendments()` (`GET /api/v3/order/amendments`)
    - `order_cancel_replace()` (`POST /api/v3/order/cancelReplace`)
    - `order_oco()` (`POST /api/v3/order/oco`)
    - `order_test()` (`POST /api/v3/order/test`)
    - `delete_order_list()` (`DELETE /api/v3/orderList`)
    - `get_order_list()` (`GET /api/v3/orderList`)
    - `order_list_oco()` (`POST /api/v3/orderList/oco`)
    - `order_list_oto()` (`POST /api/v3/orderList/oto`)
    - `order_list_otoco()` (`POST /api/v3/orderList/otoco`)
    - `rate_limit_order()` (`GET /api/v3/rateLimit/order`)
    - `sor_order()` (`POST /api/v3/sor/order`)
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)

#### WebSocket API

- Modified parameter `recvWindow`:
  - type `integer` → `number`
  - format `int64` → `float`
  - affected methods:
    - `account_rate_limits_orders()` (`account.rateLimits.orders` method)
    - `account_status()` (`account.status` method)
    - `all_order_lists()` (`allOrderLists` method)
    - `all_orders()` (`allOrders` method)
    - `my_allocations()` (`myAllocations` method)
    - `my_prevented_matches()` (`myPreventedMatches` method)
    - `my_trades()` (`myTrades` method)
    - `open_order_lists_status()` (`openOrderLists.status` method)
    - `open_orders_cancel_all()` (`openOrders.cancelAll` method)
    - `open_orders_status()` (`openOrders.status` method)
    - `order_amend_keep_priority()` (`order.amend.keepPriority` method)
    - `order_amendments()` (`order.amendments` method)
    - `order_cancel()` (`order.cancel` method)
    - `order_cancel_replace()` (`order.cancelReplace` method)
    - `order_place()` (`order.place` method)
    - `order_status()` (`order.status` method)
    - `order_test()` (`order.test` method)
    - `order_list_cancel()` (`orderList.cancel` method)
    - `order_list_place()` (`orderList.place` method)
    - `order_list_place_oco()` (`orderList.place.oco` method)
    - `order_list_place_oto()` (`orderList.place.oto` method)
    - `order_list_place_otoco()` (`orderList.place.otoco` method)
    - `order_list_status()` (`orderList.status` method)
    - `session_logon()` (`session.logon` method)
    - `sor_order_place()` (`sor.order.place` method)
    - `sor_order_test()` (`sor.order.test` method)

**Sub Account**

### Changed (1)

- Modified parameter `email`:
  - required: `true` → `false`
  - affected methods:
    - `query_sub_account_transaction_statistics()` (`GET /sapi/v1/sub-account/transaction-statistics`)

## 19.0.0 - 2025-09-17

**Derivatives Trading Portfolio Margin**

### Changed (2)

#### REST API

- Modified response for `margin_max_borrow()` (`GET /papi/v1/margin/maxBorrowable`):
  - `amount`: type `number` → `string`
  - `borrowLimit`: type `integer` → `string`

- Modified response for `new_margin_order()` (`POST /papi/v1/margin/order`):
  - `marginBuyBorrowAmount`: type `integer` → `string`

## 18.0.0 - 2025-09-15

**Pay**

### Changed (1)

- Modified response for `get_pay_trade_history()` (`GET /sapi/v1/pay/transactions`):
  - `data`.`payerInfo`: property `accountId` deleted

**Wallet**

### Changed (1)

- Modified response for `deposit_history()` (`GET /sapi/v1/capital/deposit/hisrec`):
  - item property `travelRuleStatus` added

## 17.0.0 - 2025-09-09

**Staking**

### Added (3)

- `get_soft_staking_product_list()` (`GET /sapi/v1/soft-staking/list`)
- `get_soft_staking_rewards_history()` (`GET /sapi/v1/soft-staking/history/rewardsRecord`)
- `set_soft_staking()` (`GET /sapi/v1/soft-staking/set`)

**Wallet**

### Changed (1)

- Modified response for `all_coins_information()` (`GET /sapi/v1/capital/config/getall`):
  - `networkList`: item property `withdrawTag` added

## 16.0.0 - 2025-09-05

**Simple Earn**

### Added (8)

- `get_rwusd_account()` (`GET /sapi/v1/rwusd/account`)
- `get_rwusd_quota_details()` (`GET /sapi/v1/rwusd/quota`)
- `get_rwusd_rate_history()` (`GET /sapi/v1/rwusd/history/rateHistory`)
- `get_rwusd_redemption_history()` (`GET /sapi/v1/rwusd/history/redemptionHistory`)
- `get_rwusd_rewards_history()` (`GET /sapi/v1/rwusd/history/rewardsHistory`)
- `get_rwusd_subscription_history()` (`GET /sapi/v1/rwusd/history/subscriptionHistory`)
- `redeem_rwusd()` (`POST /sapi/v1/rwusd/redeem`)
- `subscribe_rwusd()` (`POST /sapi/v1/rwusd/subscribe`)

## 15.0.0 - 2025-09-05

**Derivatives Trading Usds Futures**

### Changed (1)

#### REST API

- Modified response for `notional_and_leverage_brackets()` (`GET /fapi/v1/leverageBracket`):

## 14.0.0 - 2025-08-29

**Simple Earn**

### Changed (1)

- Modified response for `get_simple_earn_locked_product_list()` (`GET /sapi/v1/simple-earn/locked/list`):
  - `rows`.`detail`.`boostEndTime`: type `string` → `integer`

**Wallet**

### Added (1)

- `deposit_history_v2()` (`GET /sapi/v2/localentity/deposit/history`)

## 13.0.0 - 2025-08-26

**Crypto Loan**

### Changed (2)

- Added parameter `collateralAmount`
  - affected methods:
    - `flexible_loan_borrow()` (`POST /sapi/v2/loan/flexible/borrow`)
- Added parameter `loanAmount`
  - affected methods:
    - `flexible_loan_borrow()` (`POST /sapi/v2/loan/flexible/borrow`)

**Derivatives Trading Usds Futures**

### Changed (1)

#### REST API

- Modified response for `account_information_v3()` method (`GET /fapi/v3/account`):
  - `assets`: item property `marginAvailable` deleted

**Vip Loan**

### Changed (1)

- Added parameter `loanTerm`
  - affected methods:
    - `vip_loan_borrow()` (`POST /sapi/v1/loan/vip/borrow`)

## 12.0.0 - 2025-08-21

**Derivatives Trading Coin Futures**

### Changed (1)

#### REST API

- Modified response for `exchange_information()` method (`GET /dapi/v1/exchangeInfo`):
  - `symbols`.`filters`.`multiplierDecimal`: type `integer` → `string`

**Margin Trading**

### Added (1)

#### REST API

- `get_limit_price_pairs()` (`GET /sapi/v1/margin/limit-price-pairs`)

**Simple Earn**

### Changed (2)

- Modified response for `get_simple_earn_flexible_product_list()` method (`GET /sapi/v1/simple-earn/flexible/list`):
  - `rows`.`subscriptionStartTime`: type `string` → `integer`

- Modified response for `get_simple_earn_locked_product_list()` method (`GET /sapi/v1/simple-earn/locked/list`):

**Staking**

### Changed (1)

- Modified response for `get_on_chain_yields_locked_product_list()` method (`GET /sapi/v1/onchain-yields/locked/list`):
  - `rows`.`detail`.`subscriptionStartTime`: type `string` → `integer`

**Spot**

### Added (2)

#### WebSocket API

- `session_subscriptions()` (`session.subscriptions` method)
- `user_data_stream_subscribe_signature()` (`userDataStream.subscribe.signature` method)

### Changed (83)

#### REST API

- Added parameter `abovePegOffsetType`
  - affected methods:
    - `order_list_oco()` (`POST /api/v3/orderList/oco`)
- Added parameter `abovePegOffsetValue`
  - affected methods:
    - `order_list_oco()` (`POST /api/v3/orderList/oco`)
- Added parameter `abovePegPriceType`
  - affected methods:
    - `order_list_oco()` (`POST /api/v3/orderList/oco`)
- Added parameter `belowPegOffsetType`
  - affected methods:
    - `order_list_oco()` (`POST /api/v3/orderList/oco`)
- Added parameter `belowPegOffsetValue`
  - affected methods:
    - `order_list_oco()` (`POST /api/v3/orderList/oco`)
- Added parameter `belowPegPriceType`
  - affected methods:
    - `order_list_oco()` (`POST /api/v3/orderList/oco`)
- Added parameter `icebergQty`
  - affected methods:
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Added parameter `newClientOrderId`
  - affected methods:
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Added parameter `newOrderRespType`
  - affected methods:
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Added parameter `pegOffsetType`
  - affected methods:
    - `new_order()` (`POST /api/v3/order`)
    - `order_cancel_replace()` (`POST /api/v3/order/cancelReplace`)
    - `order_test()` (`POST /api/v3/order/test`)
- Added parameter `pegOffsetValue`
  - affected methods:
    - `new_order()` (`POST /api/v3/order`)
    - `order_cancel_replace()` (`POST /api/v3/order/cancelReplace`)
    - `order_test()` (`POST /api/v3/order/test`)
- Added parameter `pegPriceType`
  - affected methods:
    - `new_order()` (`POST /api/v3/order`)
    - `order_cancel_replace()` (`POST /api/v3/order/cancelReplace`)
    - `order_test()` (`POST /api/v3/order/test`)
- Added parameter `pendingAbovePegOffsetType`
  - affected methods:
    - `order_list_otoco()` (`POST /api/v3/orderList/otoco`)
- Added parameter `pendingAbovePegOffsetValue`
  - affected methods:
    - `order_list_otoco()` (`POST /api/v3/orderList/otoco`)
- Added parameter `pendingAbovePegPriceType`
  - affected methods:
    - `order_list_otoco()` (`POST /api/v3/orderList/otoco`)
- Added parameter `pendingBelowPegOffsetType`
  - affected methods:
    - `order_list_otoco()` (`POST /api/v3/orderList/otoco`)
- Added parameter `pendingBelowPegOffsetValue`
  - affected methods:
    - `order_list_otoco()` (`POST /api/v3/orderList/otoco`)
- Added parameter `pendingBelowPegPriceType`
  - affected methods:
    - `order_list_otoco()` (`POST /api/v3/orderList/otoco`)
- Added parameter `pendingPegOffsetType`
  - affected methods:
    - `order_list_oto()` (`POST /api/v3/orderList/oto`)
- Added parameter `pendingPegOffsetValue`
  - affected methods:
    - `order_list_oto()` (`POST /api/v3/orderList/oto`)
- Added parameter `pendingPegPriceType`
  - affected methods:
    - `order_list_oto()` (`POST /api/v3/orderList/oto`)
- Added parameter `price`
  - affected methods:
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Added parameter `quantity`
  - affected methods:
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Added parameter `recvWindow`
  - affected methods:
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Added parameter `selfTradePreventionMode`
  - affected methods:
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Added parameter `side`
  - affected methods:
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Added parameter `strategyId`
  - affected methods:
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Added parameter `strategyType`
  - affected methods:
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Added parameter `symbol`
  - affected methods:
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Added parameter `timeInForce`
  - affected methods:
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Added parameter `type`
  - affected methods:
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)
- Added parameter `workingPegOffsetType`
  - affected methods:
    - `order_list_oto()` (`POST /api/v3/orderList/oto`)
    - `order_list_otoco()` (`POST /api/v3/orderList/otoco`)
- Added parameter `workingPegOffsetValue`
  - affected methods:
    - `order_list_oto()` (`POST /api/v3/orderList/oto`)
    - `order_list_otoco()` (`POST /api/v3/orderList/otoco`)
- Added parameter `workingPegPriceType`
  - affected methods:
    - `order_list_oto()` (`POST /api/v3/orderList/oto`)
    - `order_list_otoco()` (`POST /api/v3/orderList/otoco`)
- Modified parameter `computeCommissionRates`:
  - affected methods:
    - `order_test()` (`POST /api/v3/order/test`)
    - `sor_order_test()` (`POST /api/v3/sor/order/test`)

- Modified response for `account_commission()` method (`GET /api/v3/account/commission`):
  - property `specialCommission` added

- Modified response for `exchange_info()` method (`GET /api/v3/exchangeInfo`):
  - `exchangeFilters`: item property `maxNumOrderAmends` added
  - `exchangeFilters`: item property `maxNumOrderLists` added
  - `symbols`: item property `pegInstructionsAllowed` added
  - `symbols`.`filters`: item property `maxNumOrderAmends` added
  - `symbols`.`filters`: item property `maxNumOrderLists` added

- Modified response for `order_test()` method (`POST /api/v3/order/test`):
  - property `specialCommissionForOrder` added

#### WebSocket API

- Added parameter `abovePegOffsetType`
  - affected methods:
    - `order_list_place_oco()` (`orderList.place.oco` method)
- Added parameter `abovePegOffsetValue`
  - affected methods:
    - `order_list_place_oco()` (`orderList.place.oco` method)
- Added parameter `abovePegPriceType`
  - affected methods:
    - `order_list_place_oco()` (`orderList.place.oco` method)
- Added parameter `belowPegOffsetType`
  - affected methods:
    - `order_list_place_oco()` (`orderList.place.oco` method)
- Added parameter `belowPegOffsetValue`
  - affected methods:
    - `order_list_place_oco()` (`orderList.place.oco` method)
- Added parameter `belowPegPriceType`
  - affected methods:
    - `order_list_place_oco()` (`orderList.place.oco` method)
- Added parameter `icebergQty`
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Added parameter `newClientOrderId`
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Added parameter `newOrderRespType`
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Added parameter `pegOffsetType`
  - affected methods:
    - `order_cancel_replace()` (`order.cancelReplace` method)
    - `order_place()` (`order.place` method)
    - `order_test()` (`order.test` method)
- Added parameter `pegOffsetValue`
  - affected methods:
    - `order_cancel_replace()` (`order.cancelReplace` method)
    - `order_place()` (`order.place` method)
    - `order_test()` (`order.test` method)
- Added parameter `pegPriceType`
  - affected methods:
    - `order_cancel_replace()` (`order.cancelReplace` method)
    - `order_place()` (`order.place` method)
    - `order_test()` (`order.test` method)
- Added parameter `pendingAbovePegOffsetType`
  - affected methods:
    - `order_list_place_otoco()` (`orderList.place.otoco` method)
- Added parameter `pendingAbovePegOffsetValue`
  - affected methods:
    - `order_list_place_otoco()` (`orderList.place.otoco` method)
- Added parameter `pendingAbovePegPriceType`
  - affected methods:
    - `order_list_place_otoco()` (`orderList.place.otoco` method)
- Added parameter `pendingBelowPegOffsetType`
  - affected methods:
    - `order_list_place_otoco()` (`orderList.place.otoco` method)
- Added parameter `pendingBelowPegOffsetValue`
  - affected methods:
    - `order_list_place_otoco()` (`orderList.place.otoco` method)
- Added parameter `pendingBelowPegPriceType`
  - affected methods:
    - `order_list_place_otoco()` (`orderList.place.otoco` method)
- Added parameter `pendingPegOffsetType`
  - affected methods:
    - `order_list_place_oto()` (`orderList.place.oto` method)
- Added parameter `pendingPegOffsetValue`
  - affected methods:
    - `order_list_place_oto()` (`orderList.place.oto` method)
- Added parameter `pendingPegPriceType`
  - affected methods:
    - `order_list_place_oto()` (`orderList.place.oto` method)
- Added parameter `price`
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Added parameter `quantity`
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Added parameter `quoteOrderQty`
  - affected methods:
    - `order_test()` (`order.test` method)
- Added parameter `recvWindow`
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Added parameter `selfTradePreventionMode`
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Added parameter `side`
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Added parameter `stopPrice`
  - affected methods:
    - `order_test()` (`order.test` method)
- Added parameter `strategyId`
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Added parameter `strategyType`
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Added parameter `subscriptionId`
  - affected methods:
    - `user_data_stream_unsubscribe()` (`userDataStream.unsubscribe` method)
- Added parameter `symbol`
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Added parameter `timeInForce`
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Added parameter `trailingDelta`
  - affected methods:
    - `order_test()` (`order.test` method)
- Added parameter `type`
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Added parameter `workingPegOffsetType`
  - affected methods:
    - `order_list_place_oto()` (`orderList.place.oto` method)
    - `order_list_place_otoco()` (`orderList.place.otoco` method)
- Added parameter `workingPegOffsetValue`
  - affected methods:
    - `order_list_place_oto()` (`orderList.place.oto` method)
    - `order_list_place_otoco()` (`orderList.place.otoco` method)
- Added parameter `workingPegPriceType`
  - affected methods:
    - `order_list_place_oto()` (`orderList.place.oto` method)
    - `order_list_place_otoco()` (`orderList.place.otoco` method)
- Modified parameter `cancelOrderId`:
  - format `int32` → `int64`
  - affected methods:
    - `order_cancel_replace()` (`order.cancelReplace` method)
- Modified parameter `computeCommissionRates`:
  - affected methods:
    - `order_test()` (`order.test` method)
    - `sor_order_test()` (`sor.order.test` method)
- Modified parameter `orderId`:
  - format `int32` → `int64`
  - affected methods:
    - `all_orders()` (`allOrders` method)
    - `my_trades()` (`myTrades` method)
    - `order_cancel()` (`order.cancel` method)
    - `order_status()` (`order.status` method)

- Modified response for `account_commission()` method (`account.commission`):
  - `result`: property `specialCommission` added

- Modified response for `exchange_info()` method (`exchangeInfo`):
  - `result`.`exchangeFilters`: item property `maxNumOrderLists` added
  - `result`.`exchangeFilters`: item property `maxNumOrderAmends` added
  - `result`.`symbols`: item property `pegInstructionsAllowed` added
  - `result`.`symbols`.`filters`: item property `maxNumOrderLists` added
  - `result`.`symbols`.`filters`: item property `maxNumOrderAmends` added

- Modified response for `order_test()` method (`order.test`):
  - `result`: property `specialCommissionForOrder` added

- Modified response for `user_data_stream_subscribe()` method (`userDataStream.subscribe`):
  - `result`: property `subscriptionId` added

## 11.0.0 - 2025-07-29

### Changed (1)

- Enhanced WebSocket reconnection logic with intelligent error classification.

## 10.0.0 - 2025-07-23

**Spot**

### Changed (4)

#### REST API

- Added missing parameters to `order_test()` (`POST /api/v3/order/test`)

#### WebSocket API

- Added missing parameters to `order_test()`

- Modified parameter `cancelOrderId`:
  - format `int32` → `int64`
  - affected methods:
    - `order_cancel_replace()` (`order.cancelReplace` method)
- Modified parameter `orderId`:
  - format `int32` → `int64`
  - affected methods:
    - `all_orders()` (`allOrders` method)
    - `my_trades()` (`myTrades` method)
    - `order_cancel()` (`order.cancel` method)
    - `order_status()` (`order.status` method)

## 9.0.0 - 2025-07-22

### Added (1)

**Wallet**

- `check_questionnaire_requirements()` (`GET /sapi/v1/localentity/questionnaire-requirements`)

### Changed (4)

**Derivatives Trading Options**

#### REST API

- Modified response for `exchange_information()` method (`GET /eapi/v1/exchangeInfo`):
  - `optionSymbols`: item property `liquidationFeeRate` added

- Modified response for `option_margin_account_information()` method (`GET /eapi/v1/marginAccount`):
  - `asset`: item property `adjustedEquity` added
  - `asset`: item property `lpProfit` deleted

**Wallet**

- Added parameter `recvWindow`
  - affected methods:
    - `fetch_address_verification_list()` (`GET /sapi/v1/addressVerify/list`)
    - `vasp_list()` (`GET /sapi/v1/localentity/vasp`)

**Spot**

#### REST API

- Added missing parameters for `order_test()` (`POST /api/v3/order/test`)

## 8.0.0 - 2025-07-14

### Added (1)

- Support session management for WebSocket API (where supported), with auto session re-logon (`auto_session_relogon` option on `ConfigurationWebSocketApi`).

### Changed (2)

- Fixed bug on URL query params generation function.

**Wallet**

- Modified response for `all_coins_information()` method (`GET /sapi/v1/capital/config/getall`):

## 7.0.0 - 2025-07-08

### Added (14)

- Support User, Risk and Trade Data Stream Events for Derivatives, Margin Trading and Spot.
- Support custom headers on REST API requests (`custom_headers` option on `ConfigurationRestApi`).

**Staking**

- `get_on_chain_yields_locked_personal_left_quota()` (`GET /sapi/v1/onchain-yields/locked/personalLeftQuota`)
- `get_on_chain_yields_locked_product_list()` (`GET /sapi/v1/onchain-yields/locked/list`)
- `get_on_chain_yields_locked_product_position()` (`GET /sapi/v1/onchain-yields/locked/position`)
- `get_on_chain_yields_locked_redemption_record()` (`GET /sapi/v1/onchain-yields/locked/history/redemptionRecord`)
- `get_on_chain_yields_locked_rewards_history()` (`GET /sapi/v1/onchain-yields/locked/history/rewardsRecord`)
- `get_on_chain_yields_locked_subscription_preview()` (`GET /sapi/v1/onchain-yields/locked/subscriptionPreview`)
- `get_on_chain_yields_locked_subscription_record()` (`GET /sapi/v1/onchain-yields/locked/history/subscriptionRecord`)
- `on_chain_yields_account()` (`GET /sapi/v1/onchain-yields/account`)
- `redeem_on_chain_yields_locked_product()` (`POST /sapi/v1/onchain-yields/locked/redeem`)
- `set_on_chain_yields_locked_auto_subscribe()` (`POST /sapi/v1/onchain-yields/locked/setAutoSubscribe`)
- `set_on_chain_yields_locked_product_redeem_option()` (`POST /sapi/v1/onchain-yields/locked/setRedeemOption`)
- `subscribe_on_chain_yields_locked_product()` (`POST /sapi/v1/onchain-yields/locked/subscribe`)

### Changed (3)

- Fixed bug with Ed25519 Private Keys passphrase.

**Derivatives Trading Usds Futures**

#### REST API

- Modified response for `open_interest_statistics()` method (`GET /futures/data/openInterestHist`):
  - item property `CMCCirculatingSupply` added
- Fixed bug with duplicated `batchOrders` parameters.

## 6.0.0 - 2025-06-26

### Added (1)

- Added implementation of the `FromStr` trait for enums.

### Changed (10)

- Replaced the bounded broadcast channel in `WebsocketEventEmitter` with an unbounded channel.

**Spot**

#### REST API

- `RateLimits` is unified as a single object
- `ExchangeFilters` is unified as a single object
- Modified response for `exchange_info()` method (`GET /api/v3/exchangeInfo`):
  - `rate_limits`: item property `count` added
- Modified response for `order_cancel_replace()` method (`POST /api/v3/order/cancelReplace`):
  - property `newOrderResponse` added
  - property `newOrderResult` added
  - property `cancelResponse` added
  - property `cancelResult` added
  - `data`.`cancelResponse`: property `code` added
  - `data`.`cancelResponse`: property `msg` added
  - `data`.`newOrderResponse`: property `symbol` added
  - `data`.`newOrderResponse`: property `transactTime` added
  - `data`.`newOrderResponse`: property `clientOrderId` added
  - `data`.`newOrderResponse`: property `orderId` added
  - `data`.`newOrderResponse`: property `orderListId` added

- Modified response for `ticker()` method (`GET /api/v3/ticker`)
- Modified response for `ticker24hr()` method (`GET /api/v3/ticker/24hr`)
- Modified response for `ticker_trading_day()` method (`GET /api/v3/ticker/tradingDay`)

#### WebSocket API

- `RateLimits` is unified as a single object
- `ExchangeFilters` is unified as a single object
- Modified response for `exchange_info()` method (`POST /exchangeInfo`):
  - `rate_limits`: item property `count` added
  - `result`.`rate_limits`: item property `count` added

## 5.0.0 - 2025-06-24

### Changed (3)

- Fixed bug with Rest API signature param order.
- Using `rust_decimal::Decimal` for floating-point numbers.
- Modified response for `exchange_information()` method (`GET /fapi/v1/exchangeInfo`):
  - `assets`.`autoAssetExchange`: type `integer` → `string`
  - `symbols`.`filters`.`multiplierDecimal`: type `integer` → `string`

## 4.0.0 - 2025-06-20

### Changed (4)

- Made the `count` field required in `WebsocketApiRateLimit`.
- Corrected parameter naming to use camelCase instead of snake_case.
- Resolved floating-point precision issues.
- Fixed serialization of reserved keywords (e.g., `r#type`) so the `r#` prefix is no longer included.

## 3.0.0 - 2025-06-19

### Changed (2)

- Added `User-Agent` to `WebSocket` requests and distinguish them per module.
- Renamed enums following rust naming conventions.

## 2.0.1 - 2025-06-18

### Changed (1)

- Fix bug with multiple logger instances.

## 2.0.0 - 2025-06-17

### Added (1)

- `get_list_schedule()` (`GET /sapi/v1/margin/list-schedule`)

## 1.0.0 - 2025-06-12

- Initial release.
