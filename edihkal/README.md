# `edihkal` Service

`edihkal` contains edihkal's API service.

To run `edihkal` with the default configuration for local development:

```
cargo run
```

`edihkal` outputs tracing logs formatted for `bunyan`.
It's recommended to pipe output to `bunyan` for prettier output:

```
cargo run | bunyan
```
