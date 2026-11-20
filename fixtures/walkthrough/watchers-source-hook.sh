#!/usr/bin/env bash
set -euo pipefail
: "${DATABASE_URL:?Use the walkthrough database}"
: "${BILIBI_RULES_SOURCE_BIN:?Run watchers-followup.sh to build and locate the source helper}"
# The parent builds both executables together; callbacks must not rebuild them.
BILIBI_ENV=development exec "$BILIBI_RULES_SOURCE_BIN" "$@"
