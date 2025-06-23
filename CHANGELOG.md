# Changelog

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
