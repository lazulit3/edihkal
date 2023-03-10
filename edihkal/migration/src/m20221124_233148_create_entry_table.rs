use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_drug_table::Drug;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Entry::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Entry::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Entry::Time).date_time().not_null())
                    .col(ColumnDef::new(Entry::DrugId).uuid().not_null())
                    .col(ColumnDef::new(Entry::Dose).unsigned().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-entry-drug_id")
                            .from(Entry::Table, Entry::DrugId)
                            .to(Drug::Table, Drug::Id),
                    )
                    // TODO: Units of measurement
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Entry::Table).to_owned()).await
    }
}

#[derive(Iden)]
enum Entry {
    Table,
    Id,
    Time,
    DrugId,
    Dose,
}
