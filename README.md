# passiogo_tui

> This project is still in development.

Rust-based terminal user interface for the Passio Go bus tracking API used at major universities and cities across the U.S.

## Overview

passiogo_tui provides a lightweight terminal UI for querying the Passio Go bus tracking API, making it easy to check vehicle locations, routes, and arrival predictions from the command line.

## What exists now

- Core TUI scaffolding in Rust (using term-based UI crates).
- Basic integration with the Passio Go API to fetch vehicle and route data.
- A simple interactive interface to view active vehicles and route details.

## Roadmap / Future plans

- Add configurable profiles for different universities and agencies.
- Improved filtering, search, and bookmarking of routes/stops.
- Offline caching and rate-limiting to improve responsiveness.
- Packaging and releases for multiple platforms (binaries).
- Integration with system notification APIs for arrival alerts.

## Installation

Prerequisites:
- Rust and Cargo (https://www.rust-lang.org/tools/install)

Build from source:

```bash
git clone https://github.com/ls3205/passiogo_tui.git
cd passiogo_tui
cargo build --release
# Run the binary
./target/release/passiogo_tui
```

## Usage

Run the built binary to launch the terminal UI. Command-line flags and configuration are planned; for now, the app will attempt to connect to the Passio Go API and present interactive views.

## Related projects

This project uses data from the Passio Go API. For a complementary Rust library for interacting with the Passio Go API, see passiogo.rs:
https://github.com/ls3205/passiogo.rs

## Contributing

Contributions are welcome — please open issues or pull requests. If you plan larger changes, open an issue first to discuss the design.

## License

This project is available under the terms of the [MIT license](./LICENSE).
