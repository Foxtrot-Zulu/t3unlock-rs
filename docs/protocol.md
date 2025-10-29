# USB Protocol Notes (PLACEHOLDER)

This scaffold uses placeholder request IDs and a trivial frame format. Replace with the actual values from the reference implementation and verified device traces.

Suggested steps:
1. Inspect the model repository (Java) to extract VID/PID, interface index, bRequest values, wValue/wIndex usage, timeouts, and frame formats.
2. Validate by observing USB traffic on Linux with `usbmon` or `Wireshark` during an unlock attempt.
3. Update:
   - `usb/lowlevel.rs` for request IDs and interface
   - `usb/proto.rs` for frame layout and status parsing
4. Add unit tests that fix the protocol behavior.
