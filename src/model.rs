//! Simplistic Model Layer
//! (with mock-store layer)

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// ————>    TICKET TYPES
#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64, 
    pub title: String, 
}

pub struct TicketForCreate{

}

// <————    TICKET TYPES 

