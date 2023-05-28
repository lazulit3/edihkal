# Self-Signed Certificates for TLS in Local Development

`edihkal` requires a certificate & key to secure traffic with TLS / HTTPS.

`edihkal/localdev/self-signed-certs` is the default path where edihkal looks for a certificate and key when run with the localdev configuration.

For local development purposes, you can generate a self-signed certificate. However, self-signed certificates should never be used in production!

Edihkal's default localdev configuration expects files named `cert.pem` and `key.pem`:

https://github.com/lazulit3/edihkal/blob/a7f5a8233e3a425e4acd894b0ac35feca250a55f/edihkal/configuration/localdev.yaml#L3-L5

## Setup

### Setup Using `mkcert`

A self-signed certificate and key can be generated using [`mkcert`](https://github.com/FiloSottile/mkcert).

`mkcert` is extra nice for localdev because it automatically creates a CA and adds it to your system's local trust stores. This ensures that clients (e.g. `edid`) run from your system against localdev won't fail when verifying the certificate.

To generate a certificate for `localhost`:

```sh
mkcert --key-file key.pem --cert-file cert.pem localhost 127.0.0.1 ::1
```
