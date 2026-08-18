#!/usr/bin/env bash
set -euo pipefail

: "${FLOCK_SIGNAL_API_TOKEN:?set FLOCK_SIGNAL_API_TOKEN}"
BASE_URL="${FLOCK_SIGNAL_BASE_URL:-http://127.0.0.1:8080}"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

while IFS= read -r event; do
  [[ -z "$event" ]] && continue
  curl --fail-with-body --silent --show-error \
    -H "Authorization: Bearer ${FLOCK_SIGNAL_API_TOKEN}" \
    -H "Content-Type: application/json" \
    --data "$event" \
    "${BASE_URL}/v1/safety-events"
  printf '\n'
done < "${ROOT}/simulator/sample-events.jsonl"
