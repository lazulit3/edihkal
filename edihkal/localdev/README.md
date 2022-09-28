# Local Development

`init_db.sh` can be run to start a local database container for edihkal.

## Configuration

Before running `init_db.sh` you may configure the environment variables used in `envs`.

The `sqlx-cli` tool is used to create the databas and run migrations. You can install it by running:

```sh
cargo install sqlx-cli --no-default-features -F postgres,rustls
```

## Usage

Running `init_db.sh` will automagically create the database and run migrations to set up the schema.

To run the migrations when the database container is already started, you can set the `SKIP_STARTUP` environment variable:

```
SKIP_STARTUP=1 ./init_db.sh
```
