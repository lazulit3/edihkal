# Local Development

## Manage LocalDev DB Container Using `init_db.sh`

`init_db.sh` can be run to setup a database container for edihkal for local development use.

### Requirements for Running `init_db.sh`

* Install `pg_isready` to detect when the database service finishes initializing before continuing setup.

* Install a container tool such as `podman` or `docker`.

### Usage

Optionally configure the database service using the following environment variables:

* `POSTGRES_DB`
* `POSTGRES_HOST`
* `POSTGRES_PASSWORD`
* `POSTGRES_PORT`
* `POSTGRES_USER`

The plain usage will start the database container and wait for the service to be ready.

```sh
./init_db.sh 
```
