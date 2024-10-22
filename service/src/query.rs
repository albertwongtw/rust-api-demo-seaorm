use ::entity::{users, users::Entity as Users};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn verify_user(db: &DbConn, username: &str, password: &[u8]) -> Result<bool, DbErr> {
        match Users::find()
            .filter(users::Column::IsDeleted.eq(false))
            .filter(users::Column::Username.eq(username))
            .filter(users::Column::Password.eq(password))
            .one(db)
            .await {
                Ok(user) => Ok(!user.is_none()),
                Err(e) => Err(e)
            }
    }
}