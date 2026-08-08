use anyhow::{Context, Ok};
use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{app_state::AppState, ui::inspector::{WidgetNodeMap}};

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncPayload {
    widget_nodes: WidgetNodeMap,
}

async fn sync(State(state): State<AppState>, Json(payload): Json<SyncPayload>) -> StatusCode {
    state.widget_nodes.store(Arc::new(payload.widget_nodes));
    if let Err(e) = state.sender.send(()) {
        log::error!("Failed to sync widgets : {e}")
    }

    StatusCode::OK
}

pub async fn serve(
    addr: &str,
    shutdown_receiver: tokio::sync::oneshot::Receiver<()>,
    app_state: AppState,
) -> anyhow::Result<()> {
    let router = Router::new()
        .route("/sync", post(sync))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context(format!("Failed to listen {addr}"))?;

    axum::serve(listener, router)
        .with_graceful_shutdown(async move {
            let _ = shutdown_receiver.await;
        })
        .await
        .context("Failed to start server")?;

    Ok(())
}
