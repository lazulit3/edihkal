pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_drug_table;
mod m20221124_233148_create_entry_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_drug_table::Migration),
            Box::new(m20221124_233148_create_entry_table::Migration),
        ]
    }
}
