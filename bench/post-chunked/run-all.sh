#!/usr/bin/env bash
set -euo pipefail
export PATH="$HOME/.local/bin:$HOME/.cargo/bin:$PATH"
cd "$(dirname "$0")/../.."
for tool in node cargo wrk ss; do
    command -v "$tool" >/dev/null || { echo "Missing tool: $tool" >&2; exit 1; }
done
if ss -ltnH '( sport = :18083 )' | grep -q .; then
    echo 'Port 18083 is occupied. Existing process was left untouched.' >&2
    exit 1
fi
cargo build --release -p addon
cp target/release/libn0.so target/release/n0.node
batch="bench/var/post-chunked/$(date -u +%Y%m%dT%H%M%SZ)-comparison"
mkdir -p "$batch"
echo "POST: 256 KiB, 256 HTTP chunks; results: $batch"
RESULT_DIR="$batch/n0" bash bench/post-chunked/run.sh n0
# Separate the two runs so connections from the first can leave TIME_WAIT.
echo 'N0 finished. Waiting 65 seconds before node:http...'
sleep 65
RESULT_DIR="$batch/node-http" bash bench/post-chunked/run.sh node-http
echo
echo 'Comparison (single preliminary run per server):'
for variant in n0 node-http; do
    echo "$variant:"
    grep -E 'Latency|Requests/sec|Transfer/sec|Socket errors|Non-2xx|completed=' "$batch/$variant/result.txt" || true
done
echo "Full reports and server logs: $batch"
echo 'Allocation profiling is not included in this performance run.'
