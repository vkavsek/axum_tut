use serde::{Deserialize, Serialize};
use sqlb::{Fields, HasFields};
use sqlx::{postgres::PgRow, prelude::FromRow};
use uuid::Uuid;

use crate::{
    crypt::{pwd, EncryptContent},
    ctx::Ctx,
};

use super::{
    base::{self, DbBmc},
    ModelManager, Result,
};

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
}

#[derive(Deserialize)]
pub struct UserForCreate {
    pub username: String,
    pub pwd_clear: String,
}

#[derive(Fields)]
struct UserForInsert {
    username: String,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForLogin {
    pub id: i64,
    pub username: String,

    pub pwd: Option<String>, // Encrypted, #_scheme_id#....
    pub pwd_salt: Uuid,
    pub token_salt: Uuid,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForAuth {
    pub id: i64,
    pub username: String,

    pub token_salt: Uuid,
}

/// Marker trait to mark the types that can be accepted or returned by UserBmc
pub trait UserBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl UserBy for User {}
impl UserBy for UserForLogin {}
impl UserBy for UserForAuth {}

/// User Backend Model Controller
pub struct UserBmc;

impl DbBmc for UserBmc {
    const TABLE: &'static str = "user";
}

impl UserBmc {
    pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
    where
        E: UserBy,
    {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn first_by_username<E>(
        _ctx: &Ctx,
        mm: &ModelManager,
        username: &str,
    ) -> Result<Option<E>>
    where
        E: UserBy,
    {
        let db = mm.db();

        let user = sqlb::select()
            .table(Self::TABLE)
            .and_where("username", "=", username)
            .fetch_optional::<_, E>(db)
            .await?;

        Ok(user)
    }

    pub async fn update_pwd(ctx: &Ctx, mm: &ModelManager, id: i64, pwd_clear: &str) -> Result<()> {
        let db = mm.db();

        let user: UserForLogin = Self::get(ctx, mm, id).await?;

        let pwd = pwd::encrypt_pwd(&EncryptContent {
            content: pwd_clear.into(),
            salt: user.pwd_salt.to_string(),
        })?;

        sqlb::update()
            .table(Self::TABLE)
            .and_where("id", "=", id)
            .data(vec![("pwd", pwd).into()])
            .exec(db)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::_dev_utils;

    use super::*;
    use anyhow::{Context, Result};
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_first_ok_demo1() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_username = "demo1";

        let user: User = UserBmc::first_by_username(&ctx, &mm, fx_username)
            .await?
            .context("Should have user 'demo1'")?;

        assert_eq!(user.username, fx_username);

        Ok(())
    }
}
