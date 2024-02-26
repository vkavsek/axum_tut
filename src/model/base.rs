use sqlx::{postgres::PgRow, FromRow};

use crate::{
    ctx::Ctx,
    model::{Error, Result},
};

use super::{task::Task, ModelManager};

/// Database Backend Model Controller
pub trait DbBmc {
    const TABLE: &'static str;
}

pub async fn get<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
where
    MC: DbBmc,
    E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
    let db = mm.db();

    let sql = format!("SELECT * FROM {} WHERE id = $1", MC::TABLE);

    let entity: E = sqlx::query_as(&sql)
        .bind(id)
        .fetch_optional(db)
        .await?
        .ok_or(Error::EntityNotFound {
            entity: MC::TABLE,
            id,
        })?;

    Ok(entity)
}
