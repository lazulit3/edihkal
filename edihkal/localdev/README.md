# Local Development

## Quick Start

```sh
./init_db.sh

mkcert --key-file self-signed-certs/key.pem \
       --cert-file self-signed-certs/cert.pem \
       localhost 127.0.0.1 ::1

cd ..
cargo run | bunyan
```

## Setup

### Self-Signed Certificates for TLS

`edihkal` requires a certificate & key for TLS / HTTPS. This can be generated using `mkcert`:

```sh
mkcert --key-file self-signed-certs/key.pem \
       --cert-file self-signed-certs/cert.pem \
       localhost 127.0.0.1 ::1
```

See [`self-signed-certs/`](self-signed-certs/) for details.

### Database

`edihkal` requires a database service (currently Postgres) to run.

### Manage LocalDev DB Container Using `init_db.sh`

`init_db.sh` can be run to setup a database container for edihkal for local development use.

**Requirements:**

* Install `pg_isready` to detect when the database service finishes initializing before continuing setup.
* Install a container tool such as `podman` or `docker`.

### `init_db.sh` Usage

Optionally configure the database service using the following environment variables:

* `POSTGRES_DB`
* `POSTGRES_HOST`
* `POSTGRES_PASSWORD`
* `POSTGRES_PORT`
* `POSTGRES_USER`

If these are not set, default values in the script will be used.

The plain usage will start the database container and wait for the service to be ready.

```sh
./init_db.sh
```
