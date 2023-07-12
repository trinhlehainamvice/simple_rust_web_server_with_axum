use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize)]
pub struct TicketMessage {
    id: u64,
    name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Ticket {
    name: String,
}

#[derive(Clone, Default)]
pub struct ModelController {
    tickets: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub fn new() -> Self {
        Self::default()
    }
}

// implement CRUD for ModelController
impl ModelController {
    pub async fn create_ticket(&self, ticket: Ticket) -> Result<TicketMessage> {
        let mut tickets = self.tickets.lock().unwrap();

        tickets.push(Some(ticket.clone()));

        Ok(TicketMessage {
            id: (tickets.len() - 1) as u64,
            name: ticket.name,
        })
    }

    pub async fn read_tickets(&self) -> Result<Vec<TicketMessage>> {
        let tickets = self.tickets.lock().unwrap();

        let result = tickets
            .iter()
            .enumerate()
            .filter_map(|(id, ticket)| {
                ticket.as_ref().map(|ticket| TicketMessage {
                    id: id as u64,
                    name: ticket.name.clone(),
                })
            })
            .collect();

        Ok(result)
    }

    pub async fn delete_ticket(&self, id: u64) -> Result<TicketMessage> {
        self.tickets
            .lock()
            .unwrap()
            .remove(id as usize)
            .map(|ticket| TicketMessage {
                id,
                name: ticket.name,
            })
            .ok_or(Error::DeleteTicketFailed { id })
    }
}
