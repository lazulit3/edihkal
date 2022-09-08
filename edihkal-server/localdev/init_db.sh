#!/usr/bin/env bash
set -eo pipefail
set -x

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install sqlx-cli --no-default-features --features postgres"
  echo >&2 "to install it."
  exit 1
fi

localdev_dir="$(dirname -- $( readlink -f -- "$0"; ))"
source "${localdev_dir}/envs"

# Set to skip container start if edihcal-timescaledb is already running
if [[ -z "${SKIP_STARTUP}" ]]
then
  podman run --name edihkal-timescaledb \
             -p "127.0.0.1:${DB_PORT}:5432" \
             -e POSTGRES_DB=${DB_NAME} \
             -e POSTGRES_PASSWORD=${DB_PASSWORD} \
             -e POSTGRES_USER=${DB_USER} \
             -d \
             timescale/timescaledb:latest-pg14
fi

sqlx database create
sqlx migrate run
