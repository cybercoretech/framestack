#!/usr/bin/env bash
# Smoke test for the Leptos + Axum reference application.
#
# Run after `framestack build --yes --manifest examples/rust-leptos-axum`.
# Starts the built server on a local port, checks that it serves the
# server-rendered page, the health route and the WebAssembly client, and
# always stops the server on exit.
#
# Usage: scripts/smoke-leptos-axum.sh [PORT]   (default port 3080)
set -euo pipefail

port="${1:-3080}"
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
example="$repo_root/examples/rust-leptos-axum"
name="framestack-reference-leptos-axum"
address="127.0.0.1:$port"
base="http://$address"

# Cargo may put build output outside the example (CARGO_TARGET_DIR).
target_dir="$(cargo metadata --format-version 1 --no-deps --manifest-path "$example/Cargo.toml" |
  python3 -c 'import json, sys; print(json.load(sys.stdin)["target_directory"])')"
server="$target_dir/release/$name"
[[ -x "$server" ]] || { echo "smoke: server binary not found at $server; build first" >&2; exit 2; }

log="$(mktemp)"
server_pid=""
cleanup() {
  if [[ -n "$server_pid" ]] && kill -0 "$server_pid" 2>/dev/null; then
    kill "$server_pid" 2>/dev/null || true
    wait "$server_pid" 2>/dev/null || true
  fi
  rm -f "$log"
}
trap cleanup EXIT

fail() {
  echo "smoke: FAIL: $*" >&2
  echo "--- server log ---" >&2
  cat "$log" >&2
  exit 1
}

# The release server reads its site location from these variables.
(
  cd "$example"
  LEPTOS_OUTPUT_NAME="$name" \
    LEPTOS_SITE_ROOT="target/site" \
    LEPTOS_SITE_PKG_DIR="pkg" \
    LEPTOS_SITE_ADDR="$address" \
    LEPTOS_ENV="PROD" \
    exec "$server"
) >"$log" 2>&1 &
server_pid=$!

# Wait up to 60 seconds for the health route.
for _ in $(seq 1 60); do
  if curl -fsS "$base/healthz" >/dev/null 2>&1; then
    break
  fi
  kill -0 "$server_pid" 2>/dev/null || fail "server exited before it was ready"
  sleep 1
done

health="$(curl -fsS "$base/healthz")" || fail "GET /healthz did not respond"
[[ "$health" == "ok" ]] || fail "GET /healthz returned '$health', expected 'ok'"
echo "smoke: GET /healthz -> ok"

page="$(curl -fsS "$base/")" || fail "GET / did not respond"
[[ "$page" == *"FrameStack reference: Leptos + Axum"* ]] || fail "GET / is missing the heading"
[[ "$page" == *"Clicked 0 times"* ]] || fail "GET / is missing the server-rendered counter"
echo "smoke: GET / -> server-rendered page with the counter"

wasm="/pkg/$name.wasm"
[[ "$page" == *"$name"* ]] || fail "GET / does not load the client bundle"
status="$(curl -s -o /dev/null -w '%{http_code}' "$base$wasm")"
[[ "$status" == "200" ]] || fail "GET $wasm returned $status, expected 200"
echo "smoke: GET $wasm -> 200"

echo "smoke: PASS"
