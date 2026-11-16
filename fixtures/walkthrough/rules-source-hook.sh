#!/usr/bin/env bash
set -euo pipefail
: "${DATABASE_URL:?Use the walkthrough database}"
repo=$(realpath -- "$(dirname -- "$0")/../..")
platform=$(realpath -- "$repo/../../..")
BILIBI_ENV=development cargo run --locked --manifest-path "$platform/Cargo.toml" -p bilibi-supply-chain-walkthrough --bin rules-source -- "$@"
