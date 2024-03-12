use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsBool, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::ctx::Ctx;
use crate::model::Result;

use super::base::{self, DbBmc};
use super::ModelManager;

#[derive(Debug, Clone, FromRow, Fields, Serialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub done: bool,
}

#[derive(Fields, Deserialize)]
pub struct TaskForCreate {
    pub title: String,
}

#[derive(Fields, Default, Deserialize)]
pub struct TaskForUpdate {
    pub title: Option<String>,
    pub done: Option<bool>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct TaskFilter {
    id: Option<OpValsInt64>,
    title: Option<OpValsString>,
    done: Option<OpValsBool>,
}

pub struct TaskBmc;

impl DbBmc for TaskBmc {
    const TABLE: &'static str = "task";
}

impl TaskBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, task_c: TaskForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, task_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Task> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<TaskFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<Task>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn update(ctx: &Ctx, mm: &ModelManager, id: i64, data: TaskForUpdate) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, data).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use crate::_dev_utils::{self, seed_tasks};

    use super::*;
    use crate::model::Error;
    use anyhow::Result;
    use serde_json::json;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        // let ctx = Ctx::root_ctx();
        // let fx_title = "test_create_ok title";
        //
        // let task_c = TaskForCreate {
        //     title: fx_title.to_string(),
        // };
        // let id = TaskBmc::create(&ctx, &mm, task_c).await?;
        //
        // let task = TaskBmc::get(&ctx, &mm, id).await?;
        // assert_eq!(task.title, fx_title);
        //
        // TaskBmc::delete(&ctx, &mm, id).await?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_error_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = TaskBmc::get(&ctx, &mm, fx_id).await;
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "task",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );
        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_update_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test_update_ok - task 01";
        let fx_title_new = "test_update_ok - task 01 - new";
        let fx_task = _dev_utils::seed_tasks(&ctx, &mm, &[fx_title])
            .await?
            .remove(0);

        TaskBmc::update(
            &ctx,
            &mm,
            fx_task.id,
            TaskForUpdate {
                title: Some(fx_title_new.to_string()),
                ..Default::default()
            },
        )
        .await?;

        let task = TaskBmc::get(&ctx, &mm, fx_task.id).await?;
        assert_eq!(task.title, fx_title_new);

        TaskBmc::delete(&ctx, &mm, fx_task.id).await?;
        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_all_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_titles = &["test_list_all_ok-task 01", "test_list_all_ok-task 02"];
        seed_tasks(&ctx, &mm, fx_titles).await?;

        let tasks = TaskBmc::list(&ctx, &mm, None, None).await?;

        let tasks: Vec<_> = tasks
            .into_iter()
            .filter(|t| t.title.starts_with("test_list_all_ok-task"))
            .collect();
        assert!(tasks.len() == 2, "number of seeded tasks.");

        for task in tasks.iter() {
            TaskBmc::delete(&ctx, &mm, task.id).await?;
        }
        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_by_filter_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_titles = &[
            "test_list_by_filter_ok-task 01.a",
            "test_list_by_filter_ok-task 01.b",
            "test_list_by_filter_ok-task 02.a",
            "test_list_by_filter_ok-task 02.b",
            "test_list_by_filter_ok-task 03",
        ];
        seed_tasks(&ctx, &mm, fx_titles).await?;

        let filters: Vec<TaskFilter> = serde_json::from_value(json!([
        {
            "title": {
                "$endsWithAny": [".a", ".b"],
            }
        },
        {
            "title": {
                "$contains": "03"
            }
        }
        ]))?;
        let len = 3;
        let list_options: ListOptions = serde_json::from_value(json!({
            "limit": len,
            "order_bys": "!id",
        }))?;
        let tasks = TaskBmc::list(&ctx, &mm, Some(filters), Some(list_options)).await?;
        assert!(tasks.len() == len, "number of seeded tasks.");

        // Cleanup
        let tasks = TaskBmc::list(
            &ctx,
            &mm,
            Some(serde_json::from_value(
                json!([{"title": {"$startsWith": "test_list_by_filter_ok"}}]),
            )?),
            None,
        )
        .await?;
        assert_eq!(tasks.len(), 5);
        for task in tasks {
            TaskBmc::delete(&ctx, &mm, task.id).await?;
        }

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_error_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        let res = TaskBmc::delete(&ctx, &mm, fx_id).await;
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "task",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
