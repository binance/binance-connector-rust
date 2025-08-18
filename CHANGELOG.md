# Changelog

## 12.0.0 - 2025-08-18

**Margin Trading**

### Added (1)

#### REST API

- `get_limit_price_pairs()` (`GET /sapi/v1/margin/limit-price-pairs`)

**Spot**

### Added (2)

#### WebSocket API

- `session_subscriptions()` (`session.subscriptions` method)
- `user_data_stream_subscribe_signature()` (`userDataStream.subscribe.signature` method)

### Changed (82)

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

- Modified response for `account_commission()` method (`POST /account.commission`):
  - `result`: property `specialCommission` added

- Modified response for `exchange_info()` method (`POST /exchangeInfo`):
  - `result`.`exchangeFilters`: item property `maxNumOrderLists` added
  - `result`.`exchangeFilters`: item property `maxNumOrderAmends` added
  - `result`.`symbols`: item property `pegInstructionsAllowed` added
  - `result`.`symbols`.`filters`: item property `maxNumOrderLists` added
  - `result`.`symbols`.`filters`: item property `maxNumOrderAmends` added

- Modified response for `order_test()` method (`POST /order.test`):
  - `result`: property `specialCommissionForOrder` added

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
