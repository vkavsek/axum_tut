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

pub mod error;

pub use self::error::{Error, Result};

pub struct ModelMan {
    // db: Db,
}

impl ModelMan {
    pub async fn new() -> Result<Self> {
        // FIXME: TBC
        Ok(ModelMan {})
    }
}
// ————>    TICKET TYPES
/// Gets sent to the client so it needs to be serializable.
#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub cid: u64,
    pub id: u64,
    pub title: String,
}
impl Ticket {
    fn from(ctx: Ctx, id: u64, title: String) -> Self {
        Self {
            cid: ctx.user_id(),
            id,
            title,
        }
    }
}

/// Payload that is sent for Create API.
#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

/// Shouldn't be used in production as the Vec grows infinitely.
#[derive(Clone)]
pub struct ModelManager {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}
impl ModelManager {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }

    // ————> CRUD Implementation
    pub async fn create_ticket(&self, ctx: Ctx, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let id = store.len() as u64;

        if ticket_fc.title.is_empty() {
            return Err(Error::ModelEmptyTitle);
        }

        let ticket = Ticket::from(ctx, id, ticket_fc.title);

        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self, ctx: Ctx) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        // Could only return tickets that the user created.
        // Could return Error if the list is empty
        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    pub async fn delete_ticket(&self, _ctx: Ctx, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        // Could only work if the client created the ticket
        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::ModelTicketIdNotFound(id))
    }
    // TODO: update ticket list?
    // <———— CRUD Implementation
}
