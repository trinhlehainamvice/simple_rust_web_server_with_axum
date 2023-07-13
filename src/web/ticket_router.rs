use crate::error::Result;
use crate::model::{ClientTicket, ModelController, ServerTicket};
use axum::extract::{Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};
use crate::context::Context;

pub fn route(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(read_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}

// REST API
async fn create_ticket(
    ctx: Result<Context>,
    State(mc): State<ModelController>,
    Json(ticket): Json<ClientTicket>,
) -> Result<Json<ServerTicket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");

    let ticket = mc.create_ticket(ctx?, ticket).await?;

    Ok(Json(ticket))
}

async fn read_tickets(State(mc): State<ModelController>) -> Result<Json<Vec<ServerTicket>>> {
    println!("->> {:<12} - read_tickets", "HANDLER");

    let tickets = mc.read_tickets().await?;

    Ok(Json(tickets))
}

async fn delete_ticket(
    ctx: Result<Context>,
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<ServerTicket>> {
    println!("->> {:<12} - delete_ticket", "HANDLER");

    let removed_ticket = mc.delete_ticket(ctx?, id).await?;

    Ok(Json(removed_ticket))
}
