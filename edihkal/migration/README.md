# `migration` crate

Contains database migrations managed with [SeaORM](https://www.sea-ql.org/SeaORM/).

See: [Development: Adding a New Entity to the Database](docs/development.md#adding-a-new-entity-to-the-database)

## Updating `entity` after changes to `migration`

Ensure that your local database container is running. See:
[edihkal/localdev/README.md]

Apply migrations to a fresh database:

``` sh
cargo run -- fresh
```

Generate entity files (run from workspace root):

``` sh
sea-orm-cli generate entity --lib -o entity/src/ --with-serde both
```

## Running Migrator CLI

- Generate a new migration file
    ```sh
    cargo run -- generate MIGRATION_NAME
    ```
- Apply all pending migrations
    ```sh
    cargo run
    ```
    ```sh
    cargo run -- up
    ```
- Apply first 10 pending migrations
    ```sh
    cargo run -- up -n 10
    ```
- Rollback last applied migrations
    ```sh
    cargo run -- down
    ```
- Rollback last 10 applied migrations
    ```sh
    cargo run -- down -n 10
    ```
- Drop all tables from the database, then reapply all migrations
    ```sh
    cargo run -- fresh
    ```
- Rollback all applied migrations, then reapply all migrations
    ```sh
    cargo run -- refresh
    ```
- Rollback all applied migrations
    ```sh
    cargo run -- reset
    ```
- Check the status of all migrations
    ```sh
    cargo run -- status
    ```
