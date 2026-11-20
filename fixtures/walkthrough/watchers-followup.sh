#!/usr/bin/env bash
set -euo pipefail
: "${DATABASE_URL:?Use a fresh disposable database}"
export BILIBI_ENV=development
repo=$(realpath -- "$(dirname -- "$0")/../..")
platform=$(realpath -- "$repo/../../..")
cargo build --locked --manifest-path "$platform/Cargo.toml" -p bilibi-supply-chain-walkthrough --bin rules-source -p bilibi-web --example watchers-domains-walkthrough
target_dir=$(cargo metadata --locked --format-version 1 --manifest-path "$platform/Cargo.toml" | python3 -c 'import json, sys; print(json.load(sys.stdin)["target_directory"])')
"$target_dir/debug/rules-source"
BILIBI_RULES_SOURCE_BIN="$target_dir/debug/rules-source" BILIBI_WATCHERS_SOURCE_HELPER="$repo/fixtures/walkthrough/watchers-source-hook.sh" "$target_dir/debug/examples/watchers-domains-walkthrough" supply_chain
