//! Simplistic Model Layer
//! (with mock-store layer)
//!
//! Model Layer
//!
//! Design:
//!
//! — The Model layer normalizes the application's data type structures and access.
//! — All application code data access must go through the Model Layer.
//! — The 'ModelManager' hold the internal states/resources needed by ModelControllers to access
//! data. (e.g., db_pool, S3 client, redis client).
//! — Model Controllers (e.g., 'TaskBmc', 'ProjectBmc') implement CRUD and other data access
//! methods on a given "entity" (e.g., 'Task', 'Project').
//! ('Bmc' is short for Backend Model Controller)
//! — In frameworks like Axum, ModelManagers are typically used as App State.
//! — ModelManager is designed to be passed as an argument to all Model Controller functions.
//!
#![allow(unused)]

use crate::ctx::Ctx;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

mod error;
mod store;

pub use self::error::{Error, Result};
use self::store::{new_db_pool, Db};

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    /// Constructor
    pub async fn init() -> Result<Self> {
        let db = new_db_pool().await?;

        Ok(ModelManager { db })
    }

    /// Returns the sqlx db pool reference
    /// (Only for the model layer)
    pub(in crate::model) fn db(&self) -> &Db {
        &self.db
    }
}
