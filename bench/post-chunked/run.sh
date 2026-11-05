#!/usr/bin/env bash
set -euo pipefail
export PATH="$HOME/.local/bin:$HOME/.cargo/bin:$PATH"
cd "$(dirname "$0")/../.."
variant="${1:?usage: bash bench/post-chunked/run.sh n0|node-http}"
case "$variant" in n0|node-http) ;; *) exit 2 ;; esac
port=18083
if ss -ltnH "( sport = :$port )" | grep -q .; then
    echo "Port $port is occupied; leaving existing listener untouched" >&2
    exit 1
fi
result_dir="${RESULT_DIR:-bench/var/post-chunked/$(date -u +%Y%m%dT%H%M%SZ)-$variant}"
mkdir -p "$result_dir"
VARIANT="$variant" PORT="$port" node bench/post-chunked/server.js > "$result_dir/server.log" 2>&1 &
server_pid=$!
trap 'kill -TERM "$server_pid" 2>/dev/null || true; wait "$server_pid" 2>/dev/null || true' EXIT
for attempt in $(seq 1 50); do
    kill -0 "$server_pid"
    if ss -ltnH "( sport = :$port )" | grep -q .; then break; fi
    sleep 0.1
done
{
    git rev-parse HEAD
    node --version
    wrk --version 2>&1 || true
    printf 'variant=%s body_bytes=262144 http_chunks=256 chunk_bytes=1024 connections=10 threads=2 duration=10s keepalive=false\n' "$variant"
} > "$result_dir/metadata.txt"
wrk -t2 -c10 -d10s --latency -s bench/post-chunked/request.lua "http://127.0.0.1:$port/upload" | tee "$result_dir/result.txt"
