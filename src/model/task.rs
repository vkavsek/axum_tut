use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::ctx::Ctx;

use crate::model::Result;

use super::ModelManager;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TaskForCreate {
    pub title: String,
}

#[derive(Deserialize)]
pub struct TaskForUpdate {
    pub title: Option<String>,
}

pub struct TaskBmc;

impl TaskBmc {
    pub async fn create(_ctx: &Ctx, mm: &ModelManager, task_c: TaskForCreate) -> Result<i64> {
        let db = mm.db();

        let res = sqlx::query_as::<_, (i64,)>(
            r#"
            INSERT INTO task (title) values ($1) returning id
            "#,
        )
        .bind(task_c.title)
        .fetch_one(db)
        .await?;

        todo!()
    }
}
