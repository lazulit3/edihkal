#!/usr/bin/env sh

# Example command for generating & updating `entity` from DB schema.

set -e

# TODO: Not all entities need DeriveNewModel (e.g. user).
sea-orm-cli generate entity \
    --lib \
    --model-extra-derives 'sea_skipper::DeriveNewModel' \
    --output-dir entity/src/ \
    --with-serde both \
    "$@"
