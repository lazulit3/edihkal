# Development 

## Backend Development

### Adding a New Entity to the Database

This section describes how to add a new entity to edihkal's database.

Edihkal uses [SeaORM](https://www.sea-ql.org/SeaORM/) to:
- manage the database schema,
- apply database [`migration`s](/edihkal/migration/),
- generate [`entity`](/entity/) types defining data `Model`s,
- and query the database using `Entity` types derived in [`entity`](/entity/).

An outline of the process for adding a new entity:
- Add a new migration to [`migration`](/edihkal/migration/) defining the new entity's schema.
- Apply database migrations to bring localdev DB up to date.
- Use `sea-orm-cli` to generate code defining entities in [`entity`](/entity/).
- Resolve any diffs in the generated code to avoid clobbering manual changes. (https://github.com/lazulit3/edihkal/issues/29)

Working with migrations and entities requires the `sea-orm-cli` utility.
[Install `sea-orm-cli`](https://www.sea-ql.org/SeaORM/docs/migration/setting-up-migration/#creating-migration-directory) if needed.

Ensure that your local database container is running, see:
[`/edihkal/localdev/README.md`](/edihkal/localdev/)

`sea-orm-cli` uses the environment variables defined in `.env` to connect to the local database container.
If your database container is not using the default values found in `.env` and `edihkal/localdev/init_db.sh`, you may need to adjust, or specify the `--database-url` parameter.

[Generate a new database migration](https://www.sea-ql.org/SeaORM/docs/migration/writing-migration/) e.g.
```sh
# Run from the edihkal crate.
cd edihkal/

sea-orm-cli migrate generate create_foobar_table
```

[Define the generated migration.](https://www.sea-ql.org/SeaORM/docs/migration/writing-migration/#defining-migration)

Apply the migrations to a fresh database; the local database schema needs to be up to date because `sea-orm-cli` uses the DB to determine how to generate the entity code in later steps.

```sh
sea-orm-cli migrate fresh
```

Now generate/update SeaORM entities for the [`entity`](/entity/) crate (see SeaORM's [Generating Entity Files](https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli/#generating-entity-files)):

```sh
# build/generate-entity-from-db-schema.sh

sea-orm-cli generate entity \
    --lib \
    --model-extra-derives 'edihkal_macros::DeriveNewModel' \
    --output-dir entity/src/ \
    --with-serde both
```

Due to some manual modifications made on top of the generated code (https://github.com/lazulit3/edihkal/issues/29), there will be some conflicts to resolve.
Review the previous version and resolve the diffs as needed.
