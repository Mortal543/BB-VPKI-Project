#!/usr/bin/env bash
set -euo pipefail

# Continuously run the BB-VPKI benchmark, write per-run logs, and sleep between runs.
# Usage:
#   SLEEP_SECONDS=60 ./scripts/continuous_run.sh
# Default sleep between runs is 60 seconds.

SLEEP_SECONDS=${SLEEP_SECONDS:-60}
OUTDIR="logs"
mkdir -p "${OUTDIR}"

echo "Starting continuous-run loop. Logs will be written to ${OUTDIR}/"

trap 'echo "Received stop signal, exiting"; exit 0' SIGINT SIGTERM

while true; do
  TS=$(date -u +"%Y%m%dT%H%M%SZ")
  LOGFILE="${OUTDIR}/benchmark-${TS}.log"
  echo "=== Starting run at ${TS} (logs -> ${LOGFILE}) ==="
  # Run the release binary via cargo; tee to log file so you can inspect live output
  cargo run --release 2>&1 | tee "${LOGFILE}"
  echo "=== Run finished at $(date -u +"%Y%m%dT%H%M%SZ") ==="
  echo "Sleeping for ${SLEEP_SECONDS}s before next run..."
  sleep "${SLEEP_SECONDS}"
done
