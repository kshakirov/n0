#!/usr/bin/env bash
set -euo pipefail
export PATH="$HOME/.local/bin:$HOME/.cargo/bin:$PATH"
cd "$(dirname "$0")/../.."
variant="${1:?usage: profile.sh n0|node-http}"
case "$variant" in n0|node-http) ;; *) exit 2 ;; esac
if ss -ltnH '( sport = :18083 )' | grep -q .; then exit 1; fi
result_dir="$PWD/bench/var/post-chunked/profiles/$(date -u +%Y%m%dT%H%M%SZ)-$variant"
mkdir -p "$result_dir"
VARIANT="$variant" PORT=18083 node --prof --no-logfile-per-isolate --logfile="$result_dir/v8.log" bench/post-chunked/server.js > "$result_dir/server.log" 2>&1 &
server_pid=$!
monitor_pid=''
cleanup() {
    kill -TERM "$server_pid" 2>/dev/null || true
    wait "$server_pid" 2>/dev/null || true
    if [[ -n "$monitor_pid" ]]; then
        kill -TERM "$monitor_pid" 2>/dev/null || true
        wait "$monitor_pid" 2>/dev/null || true
    fi
}
trap cleanup EXIT
for attempt in $(seq 1 50); do
    kill -0 "$server_pid"
    if ss -ltnH '( sport = :18083 )' | grep -q .; then break; fi
    sleep 0.1
done
cp "/proc/$server_pid/maps" "$result_dir/maps.txt"
nm -n target/release/n0.node > "$result_dir/addon-symbols.txt"
pidstat -u -r -p "$server_pid" 1 > "$result_dir/process.txt" &
monitor_pid=$!
wrk -t2 -c10 -d10s --latency -s bench/post-chunked/request.lua http://127.0.0.1:18083/upload > "$result_dir/load.txt"
cleanup
trap - EXIT
node --prof-process "$result_dir/v8.log" > "$result_dir/cpu.txt" 2> "$result_dir/process-profile.log"
echo "$result_dir"
