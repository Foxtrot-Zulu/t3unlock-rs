# Architecture Overview

- `src/cli.rs`: CLI definition (clap), man/completions generators.
- `src/usb/`: USB facade with `lowlevel.rs` (rusb helpers) and `proto.rs` (message frames & control transfers).
- `src/usb/lowlevel.rs`: rusb helpers for **bulk** I/O (endpoints 0x02 OUT, 0x81 IN)
- `src/usb/proto.rs`: builds 31/512 byte frames and runs the unlock/relink sequence
- `src/errors.rs`: domain errors for user-friendly messages.
- `src/logging.rs`: tracing subscriber with `RUST_LOG` env filter.
- `src/main.rs`: command dispatch and high-level flows.
