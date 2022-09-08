# Local Development

`init_db.sh` can be run to start a local database container for edihkal-server.

## Configuration

Before running `init_db.sh` you may configure the environment variables used in `envs`.

- DB_PASSWORD

Running `init_db.sh` will automagically create the database and run migrations to set up the schema.

To run the migrations when the database container is already started, you can set the `SKIP_STARTUP` environment variable:

```
SKIP_STARTUP=1 ./init_db.sh
```
