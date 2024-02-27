use serde::{Deserialize, Serialize};
use sqlb::{Fields, HasFields, SqlBuilder};
use sqlx::{postgres::PgRow, prelude::FromRow};
use uuid::Uuid;

use crate::ctx::Ctx;

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
    pub async fn create<E>(ctx: &Ctx, mm: &ModelManager, data: E) -> Result<i64>
    where
        E: UserBy,
    {
        base::create::<Self, _>(ctx, mm, data).await
    }
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

    pub async fn list<E>(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<E>>
    where
        E: UserBy,
    {
        base::list::<Self, _>(ctx, mm).await
    }

    pub async fn update<E>(ctx: &Ctx, mm: &ModelManager, id: i64, data: E) -> Result<()>
    where
        E: UserBy,
    {
        base::update::<Self, _>(ctx, mm, id, data).await
    }

    pub async fn delete<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()>
    where
        E: UserBy,
    {
        base::delete::<Self>(ctx, mm, id).await
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
