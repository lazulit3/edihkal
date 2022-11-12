use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Drug::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Drug::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Drug::Name).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Drug::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Drug {
    Table,
    Id,
    Name,
}
