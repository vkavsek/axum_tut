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

/// Initializes enviroment for local development.
/// (for early development, will be called from main()).
pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        debug!("{:<12} - init_dev_all()", "FOR-DEV-ONLY");
        dev_db::init_dev_db().await.unwrap();
    })
    .await;
}

/// Initializes enviroment for tests and the `ModelManager` and returns it to the caller.
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

/// A helper function thath uses TaskBmc to add an array of `Task`s created from `titles` to the
/// DB and returns an array of `Task`s that were created, or an error if encountered.
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
