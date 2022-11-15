# Local Development

## Manage LocalDev DB Container Using `init_db.sh`

`init_db.sh` can be run to setup a database container for edihkal for local development use.

### Requirements for Running `init_db.sh`

* Install `pg_isready` to detect when the database service finishes initializing before continuing setup.

* Install a container tool such as `podman` or `docker`. You may set `CONTAINER_TOOL` environment variable to select a tool to use, otherwise `init_db.sh` uses `podman` by default.

### Usage

Optionally configure the database service using the following environment variables:

* `POSTGRES_DB`
* `POSTGRES_HOST`
* `POSTGRES_PASSWORD`
* `POSTGRES_PORT`
* `POSTGRES_USER`

Configure `CONTAINER_TOOL` if you want to manage the container using a different tool than `podman` by default.

The plain usage will start the database container and wait for the service to be ready.

```sh
./init_db.sh 
```

To setup the database using docker instead of podman (default), set `CONTAINER_TOOL`:

```sh
CONTAINER_TOOL="docker" ./init_db.sh
```
