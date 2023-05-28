# Local Development

This document explains how to run `edihkal` for local development on your workstation.

## Getting Started

### Quick Start

Here's a summary of the process (described in more detail below). Run these commands at the top of the edihkal workspace:

```sh
mkcert \
  --key-file edihkal/localdev/self-signed-certs/key.pem \
  --cert-file edihkal/localdev/self-signed-certs/cert.pem \
  localhost 127.0.0.1 ::1

docker-compose up
```

### Generate Self-Signed Certificates for TLS

`edihkal` requires a certificate & key to secure traffic with TLS / HTTPS.

These can be generated using [`mkcert`](https://github.com/FiloSottile/mkcert):

```sh
mkcert --key-file self-signed-certs/key.pem \
--cert-file self-signed-certs/cert.pem \
localhost 127.0.0.1 ::1
```

See [`self-signed-certs/`](self-signed-certs/) for details.

### Using `docker-compose.yml`

The recommended approach to running localdev is to use the [`docker-compose.yml`](/docker-compose.yml) spec to run edihkal (and other required services e.g. database) in containers.

**Requirements:**
- A container engine like Docker or Podman must be installed to run containers.
- `docker-compose` (or an equivalent tool) must be installed to use the `docker-compose.yml` specification.

Start the containers using `docker-compose`:

```sh
docker-compose up
```

After the containers have started, the edihkal API will be listening on `https://localhost:8443` and the database will be listening on `localhost:5432`.

To ensure that new images are built (with updated code from your workspace), use the `--build` flag:

```sh
docker-compose up --build
```
