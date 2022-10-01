# Local Development

## Manage LocalDev DB Container Using `init_db.sh`

`init_db.sh` can be run to setup a database container for edihkal for local development use.

### Requirements for Runnning `init_db.sh`

* Install `pg_isready` to detect when the database service finishes initializing before continuing setup.
* Install `sqlx-cli` to configure the database (i.e. run migrations) after database is running. To install `sqlx-cli` you may run:
```sh
cargo install sqlx-cli --no-default-features -F postgres,rustls
```
* Install a container tool such as `podman` or `docker`. You may set `CONTAINER_TOOL` environment variable to select a tool to use, otherwise `init_db.sh` uses `podman` by default.

### Usage

Optionally configure the database service using the following environment variables:

* `POSTGRES_DB`
* `POSTGRES_HOST`
* `POSTGRES_PASSWORD`
* `POSTGRES_PORT`
* `POSTGRES_USER`

Configure `CONTAINER_TOOL` if you want to manage the container using a different tool than `podman` by default.

The plain usage will start the database container, wait for the service to be ready, create the database, and setup the database by running sqlx migrations:

```sh
./init_db.sh 
```

To run sqlx migrations after the database is alread running, set `SKIP_STARTUP`:

``` sh
SKIP_STARTUP=1 ./init_db.sh
```

To setup the database using docker instead of podman (default), set `CONTAINER_TOOL`:

```sh
CONTAINER_TOOL="docker" ./init_db.sh
```
