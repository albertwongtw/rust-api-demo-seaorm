use extension::postgres::Type;
use sea_orm_migration::{prelude::*, schema::*};
use sea_orm::{EnumIter, Iterable};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(UserStatus)
                    .values(UserStatusVariants::iter())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_auto(Users::Key))
                    .col(string(Users::Username).unique_key().not_null())
                    .col(binary(Users::Password).not_null())
                    .col(enumeration(Users::Status, Alias::new("user_status"), UserStatusVariants::iter()))
                    .col(boolean(Users::IsDeleted).not_null())
                    .col(integer(Users::CreatedBy).not_null())
                    .col(date_time(Users::CreatedDatetime).not_null())
                    .col(integer(Users::UpdatedBy))
                    .col(date_time(Users::UpdatedDateTime))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).if_exists().to_owned())
            .await?;
        manager.drop_type(Type::drop().name(UserStatus).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
struct UserStatus;

#[derive(DeriveIden, EnumIter)]
enum UserStatusVariants  {
    #[sea_orm(iden = "active")]
    Active,
    #[sea_orm(iden = "in_active")]
    InActive,
    #[sea_orm(iden = "disabled")]
    Disabled,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    #[sea_orm(iden = "user_key")]
    Key,
    Username,
    Password,
    Status,
    #[sea_orm(iden = "is_deleted")]
    IsDeleted,
    #[sea_orm(iden = "created_by")]
    CreatedBy,
    #[sea_orm(iden = "created_datetime")]
    CreatedDatetime,
    #[sea_orm(iden = "updated_by")]
    UpdatedBy,
    #[sea_orm(iden = "udpated_datetime")]
    UpdatedDateTime,
}
