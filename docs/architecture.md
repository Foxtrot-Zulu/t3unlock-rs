# Architecture Overview

- `src/cli.rs`: CLI definition (clap), man/completions generators.
- `src/usb/`: USB facade with `lowlevel.rs` (rusb helpers) and `proto.rs` (message frames & control transfers).
- `src/errors.rs`: domain errors for user-friendly messages.
- `src/logging.rs`: tracing subscriber with `RUST_LOG` env filter.
- `src/main.rs`: command dispatch and high-level flows.
