use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Block::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Block::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Block::Index).big_integer().not_null())
                    .col(ColumnDef::new(Block::Timestamp).big_integer().not_null())
                    .col(ColumnDef::new(Block::ProofOfWork).big_integer().not_null())
                    .col(ColumnDef::new(Block::PreviousHash).string().not_null())
                    .col(ColumnDef::new(Block::Hash).string().not_null())
                    .col(ColumnDef::new(Block::Data).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Block::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Block {
    Table,
    Id,
    Index,
    Timestamp,
    ProofOfWork,
    PreviousHash,
    Hash,
    Data,
}
