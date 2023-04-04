#!/usr/bin/env sh

set -e

sea-orm-cli generate entity \
    --lib \
    --model-extra-derives 'edihkal_macros::DeriveNewModel' \
    --output-dir entity/src/ \
    --with-serde both \
    "$@"
