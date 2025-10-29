#!/usr/bin/env bash
set -euo pipefail

BIN="${1:-./target/release/t3unlock}"

echo "== status =="
"$BIN" status || true

echo "== doctor =="
"$BIN" doctor

echo "== dry-run unlock =="
"$BIN" unlock --dry-run
