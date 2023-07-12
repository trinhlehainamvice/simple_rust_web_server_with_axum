use crate::error::Result;
use crate::model::{ModelController, Ticket, TicketMessage};
use axum::extract::{Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};

pub fn route(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(read_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}

// REST API
async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket): Json<Ticket>,
) -> Result<Json<TicketMessage>> {
    println!("->> {:<12} - create_ticket", "HANDLER");

    let ticket = mc.create_ticket(ticket).await?;

    Ok(Json(ticket))
}

async fn read_tickets(State(mc): State<ModelController>) -> Result<Json<Vec<TicketMessage>>> {
    println!("->> {:<12} - read_tickets", "HANDLER");

    let tickets = mc.read_tickets().await?;

    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<TicketMessage>> {
    println!("->> {:<12} - delete_ticket", "HANDLER");

    let removed_ticket = mc.delete_ticket(id).await?;

    Ok(Json(removed_ticket))
}
