# t3unlock-rs (scaffold)

Linux-first Rust CLI to unlock Samsung Portable SSD **T3** drives.

> ⚠️ This is a **scaffold**. The USB protocol constants are placeholders until verified against a real device and/or the original Java reference. Dry-run mode lets you exercise the flow safely.

## Quick start

```bash
# install dependencies (Debian/Ubuntu)
sudo apt-get install -y build-essential pkg-config libusb-1.0-0-dev

# build
cargo build --release

# show status (defaults to Samsung VID=04e8, PID=61f1 — override if needed)
./target/release/t3unlock status

# simulate unlock (no USB I/O)
./target/release/t3unlock unlock --dry-run

# try unlock (will prompt for password)
./target/release/t3unlock unlock
```

### Udev (non-root access)

Install the provided rule, then reload udev:

```bash
sudo install -D -m 0644 contrib/udev/99-t3unlock.rules /etc/udev/rules.d/99-t3unlock.rules
sudo udevadm control --reload-rules && sudo udevadm trigger
# replug the device
```

On some distros you may need to add your user to `plugdev` (or equivalent).

## Commands

- `status` — detect device and (placeholder) lock state.
- `unlock` — unlock flow (prompts for password if `--password` not given). Use `--dry-run` to simulate.
- `doctor` — print common Linux diagnostics.
- `gen-completions <bash|zsh|fish>` — emits shell completions to stdout.
- `gen-man <OUTDIR>` — writes `t3unlock.1` manpage to the directory.

## Configuration

- Override USB IDs via env:
  - `T3UNLOCK_VID=04e8 T3UNLOCK_PID=61f1`
- Adjust timeouts with `--timeout-ms` or env `T3UNLOCK_TIMEOUT_MS` (coming soon).

## Security notes

- Passing `--password` on the command line may leak into shell history or process listings. Prefer interactive prompt.
- Password buffers are zeroized after use (`zeroize`). Secrets are never logged.

## Development

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test
```

### Man page and completions

```bash
# man page
./target/release/t3unlock gen-man ./target
# completions (bash)
./target/release/t3unlock gen-completions bash > contrib/completions/t3unlock.bash
```

## CI

See `.github/workflows/ci.yml` for lint/build/test and artifact upload (skeleton).

## How it works (T3)

This tool talks to the Samsung Portable SSD **T3** over **bulk endpoints** (not control transfers):

- Interface: `0`
- Endpoints:
  - OUT (host → device): `0x02`
  - IN  (device → host): `0x81`
- Sequence (based on a known-good Linux unlocker):
  1. **OUT**: 31-byte `unlock` packet
  2. **OUT**: 512-byte `password` packet (ASCII/UTF-8, NUL-terminated/zero-padded)
  3. **IN**:  512-byte `return` → byte `9 == 0x02` means **failure**
  4. **OUT**: 31-byte `relink` packet
  5. **IN**:  512-byte `return` → byte `9 == 0x02` means **failure**

### Defaults

- VID: `0x04e8` (Samsung)
- PID (T3 locked): **`0x61f4`**  
  Override with CLI (`--vid/--pid`) or env (`T3UNLOCK_VID/T3UNLOCK_PID`).

### Permissions (non-root)

Install the udev rule and replug the drive:

sudo install -D -m 0644 contrib/udev/99-t3unlock.rules /etc/udev/rules.d/99-t3unlock.rules
sudo udevadm control --reload-rules && sudo udevadm trigger

Replug the drive; ensure you're in the plugdev group

## Verification checklist (manual, on Linux)

- [ ] `lsusb` shows the T3.
- [ ] `t3unlock status` prints `Present: true`.
- [ ] Install udev rule; reconnection works without sudo.
- [ ] `t3unlock unlock --dry-run` logs the intended sequence.
- [ ] With correct constants in `usb/proto.rs`, `unlock` succeeds with the right password.
```

