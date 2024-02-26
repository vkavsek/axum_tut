mod dev_db;

use tokio::sync::OnceCell;
use tracing::debug;

use crate::{
    ctx::Ctx,
    model::{
        self,
        task::{Task, TaskBmc, TaskForCreate},
        ModelManager,
    },
};

/// Initiealize enviroment for local development.
/// (for early development, will be called  from main()).
pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        debug!("{:<12} - init_dev_all()", "FOR-DEV-ONLY");
        dev_db::init_dev_db().await.unwrap();
    })
    .await;
}

pub async fn init_test() -> ModelManager {
    static INSTANCE: OnceCell<ModelManager> = OnceCell::const_new();
    let mm = INSTANCE
        .get_or_init(|| async {
            init_dev().await;
            ModelManager::init().await.unwrap()
        })
        .await;

    mm.clone()
}

pub async fn seed_tasks(ctx: &Ctx, mm: &ModelManager, titles: &[&str]) -> model::Result<Vec<Task>> {
    let mut tasks = Vec::with_capacity(titles.len());
    for title in titles {
        let id = TaskBmc::create(
            ctx,
            mm,
            TaskForCreate {
                title: title.to_string(),
            },
        )
        .await?;
        let task = TaskBmc::get(ctx, mm, id).await?;

        tasks.push(task);
    }

    Ok(tasks)
}
