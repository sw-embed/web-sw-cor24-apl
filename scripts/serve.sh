#!/usr/bin/env bash
set -euo pipefail

PORT=9957

exec trunk serve --port "$PORT" "$@"
