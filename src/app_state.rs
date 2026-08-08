use arc_swap::ArcSwap;
use rustc_hash::FxHashMap;
use std::sync::Arc;

use crate::ui::inspector::WidgetNodeMap;

#[derive(Clone)]
pub struct AppState {
    pub widget_nodes: Arc<ArcSwap<WidgetNodeMap>>,
    pub sender: tokio::sync::mpsc::UnboundedSender<()>,
}

impl AppState {
    pub fn new(sender: tokio::sync::mpsc::UnboundedSender<()>) -> Self {
        Self {
            sender,
            widget_nodes: Arc::new(ArcSwap::new(FxHashMap::default().into())),
        }
    }
}

impl gpui::Global for AppState {}
