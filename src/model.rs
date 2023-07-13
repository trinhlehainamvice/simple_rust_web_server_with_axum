use crate::context::Context;
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Clone)]
pub struct ServerTicket {
    id: u64,
    user_id: u64,
    name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ClientTicket {
    name: String,
}

#[derive(Clone, Default)]
pub struct ModelController {
    tickets: Arc<Mutex<Vec<Option<ServerTicket>>>>,
}

impl ModelController {
    pub fn new() -> Self {
        Self::default()
    }
}

// implement CRUD for ModelController
impl ModelController {
    pub async fn create_ticket(&self, ctx: Context, ticket: ClientTicket) -> Result<ServerTicket> {
        let mut tickets = self.tickets.lock().unwrap();

        let new_ticket = ServerTicket {
            id: tickets.len() as u64,
            user_id: ctx.user_id,
            name: ticket.name,
        };

        tickets.push(Some(new_ticket.clone()));

        Ok(new_ticket)
    }

    pub async fn read_tickets(&self) -> Result<Vec<ServerTicket>> {
        let tickets = self.tickets.lock().unwrap();

        let result = tickets
            .iter()
            .filter_map(|ticket| ticket.as_ref().cloned())
            .collect();

        Ok(result)
    }

    pub async fn delete_ticket(&self, ctx: Context, id: u64) -> Result<ServerTicket> {
        self.tickets
            .lock()
            .unwrap()
            // TODO: we only empty this id here if any
            .get_mut(id as usize)
            .and_then(|ticket| {
                ticket.take().and_then(|t| {
                    if t.user_id == ctx.user_id {
                        Some(t)
                    } else {
                        *ticket = Some(t);
                        None
                    }
                })
            })
            .ok_or(Error::DeleteTicketFailed { id })
    }
}
