#!/usr/bin/env bash
set -eo pipefail
set -x

source envs

podman run --name edihkal-timescaledb \
           -p "127.0.0.1:${DB_PORT}:5432" \
           -e POSTGRES_DB=${DB_NAME} \
           -e POSTGRES_PASSWORD=${DB_PASSWORD} \
           -e POSTGRES_USER=${DB_USER} \
           -d \
           timescale/timescaledb:latest-pg14


sqlx database create
