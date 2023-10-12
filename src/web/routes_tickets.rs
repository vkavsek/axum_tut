#![allow(unused)]

use axum::{
    extract::{Path, State},
    routing::{delete, post},
    Json, Router,
};

use crate::{
    ctx::Ctx,
    model::{ModelController, Ticket, TicketForCreate},
    Result,
};

/// You need to provide state to the REST handlers
pub fn routes(mc: ModelController) -> Router {
    Router::new()
        // You can chain multiple method routes on the same URI like get() and post() together.
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}

// ————> REST HANDLERS
// NOTE: We don't have to use Result<Ctx> inside of REST handlers since we know that the result inside
// this extractor is: Ok(Ctx), and NOT: Err(Error), we check for that before in `mw_require_auth`.
// If the result is not Ok(Ctx) we never arrive to this point in our program.
// Client request @ "/api/*" -> middleware::mw_require_auth -> Handlers -> ...
async fn create_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");
    let ticket = mc.create_ticket(ctx, ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>, ctx: Ctx) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list_tickets", "HANDLER");
    let tickets = mc.list_tickets(ctx).await?;

    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
    ctx: Ctx,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - delete_ticket", "HANDLER");
    let ticket = mc.delete_ticket(ctx, id).await?;

    Ok(Json(ticket))
}
// <———— REST HANDLERS
