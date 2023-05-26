# Self-Signed Certificates for TLS in Local Development

`edihkal/localdev/self-signed-certs` stores a self-signed certificate and key used by edihkal (for TLS) for local development.

Edihkal's default localdev configuration expects `cert.pem` and `key.pem`:

https://github.com/lazulit3/edihkal/blob/a7f5a8233e3a425e4acd894b0ac35feca250a55f/edihkal/configuration/localdev.yaml#L3-L5

## Setup

### Setup Using `mkcert`

A self-signed certificate and key can be generated using [`mkcert`](https://github.com/FiloSottile/mkcert).

`mkcert` has an added benefit of ensuring that the local CA used to sign the certificate is installed on in your workstation's trust store (which allows `edid` and `edihkal_client` to verify the certificate even in localdev).

To generate a certificate for `localhost`:

```sh
mkcert  --key-file key.pem --cert-file cert.pem localhost 127.0.0.1 ::1
```
