use std::collections::HashMap;

use anyhow::{Context, Ok};
use axum::{Json, Router, http::StatusCode, routing::post};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct WidgetNode {
    id: u32,
    parent_id: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncPayload {
    widget_nodes: HashMap<u32, WidgetNode>,
}

async fn sync(Json(payload): Json<SyncPayload>) -> StatusCode {
    println!("成功接收到同步数据:");

    for (id, node) in &payload.widget_nodes {
        println!("Widget ID: {}, Parent ID: {:?}", id, node.parent_id);
    }

    StatusCode::OK
}

pub async fn serve(
    addr: &str,
    shutdown_receiver: tokio::sync::oneshot::Receiver<()>,
) -> anyhow::Result<()> {
    let router = Router::new().route("/sync", post(sync));

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
