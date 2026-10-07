#!/usr/bin/env bash
# Runs the stale-direct-addrs example while turning Wi-Fi off and back on.
# Linux with NetworkManager or macOS; takes about 50 s and leaves Wi-Fi on.
set -euo pipefail
cd "$(dirname "$0")/../.."

if command -v nmcli >/dev/null; then
  wifi() { nmcli radio wifi "$1"; }
else
  dev=$(networksetup -listallhardwareports | awk '/Wi-Fi/{getline; print $2}')
  wifi() { networksetup -setairportpower "$dev" "$1"; }
fi

cargo build -q --example stale-direct-addrs
SECONDS=0
target/debug/examples/stale-direct-addrs &
pid=$!
trap 'kill $pid 2>/dev/null; wifi on' EXIT

say() { printf '%5d s  >>> %s\n' "$SECONDS" "$1"; }
sleep 3; say "wifi off"; wifi off
sleep 5; say "wifi on";  wifi on
sleep 40
