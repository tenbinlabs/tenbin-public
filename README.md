# tenbin-public

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)

Minimal public-surface crate consumed by [Tenbin](https://tenbin.xyz)'s public-facing services (currently the verification service).

This crate exists as a shared dependency between Tenbin's internal monorepo and its public services. Anything that doesn't need to cross that boundary stays internal.

## Usage

Add as a git dependency:

```toml
[dependencies]
tenbin-public = { git = "https://github.com/tenbinlabs/tenbin-public", branch = "main" }
```

## Modules

### hidden_road
HTTP client for the Hidden Road API and the request/response types it uses.

### types
Wire-format types for broker-facing traffic. Currently only the Hidden Road slice.

### tracing
OpenTelemetry / `tracing` setup helpers. Optional Sentry integration is behind the `sentry` feature flag.

## Features

- `sentry` — enable Sentry integration in the tracing module.

## License

Apache-2.0. See [LICENSE](LICENSE).
