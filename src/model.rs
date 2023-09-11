//! Simplistic Model Layer
//! (with mock-store layer)

#![allow(unused)]
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// ————>    TICKET TYPES
/// Gets sent to the client so it needs to be serializable.
#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}
impl Ticket {
    fn from(id: u64, title: String) -> Self {
        Self { id, title }
    }
}

/// Payload that is sent for Create API.
#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}
// <————    TICKET TYPES

/// Shouldn't be used in production as the Vec grows infinitely.
#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }

    // ————> CRUD Implementation
    pub async fn create_ticket(&self, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let id = store.len() as u64;

        if ticket_fc.title.is_empty() {
            return Err(Error::EmptyTitle);
        }

        let ticket = Ticket::from(id, ticket_fc.title);

        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        // Could return Error if the list is empty
        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TicketIdNotFound(id))
    }
    // TODO —> update ticket list?
    // <———— CRUD Implementation
}
