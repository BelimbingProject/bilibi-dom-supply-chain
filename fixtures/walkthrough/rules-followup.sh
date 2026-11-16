#!/usr/bin/env bash
set -euo pipefail
: "${DATABASE_URL:?Use a fresh disposable database}"
repo=$(realpath -- "$(dirname -- "$0")/../..")
platform=$(realpath -- "$repo/../../..")
cargo run --locked --manifest-path "$platform/Cargo.toml" -p bilibi-supply-chain-walkthrough --bin rules-source
BILIBI_RULES_SOURCE_HELPER="$repo/fixtures/walkthrough/rules-source-hook.sh" cargo run --locked --manifest-path "$platform/Cargo.toml" -p bilibi-web --example rules-domains-walkthrough -- supply_chain
