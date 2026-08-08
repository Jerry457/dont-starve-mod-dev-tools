use gpui::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, SharedString, Styled, Window,
    px,
};
use gpui_component::{
    h_flex,
    list::ListItem,
    resizable::{h_resizable, resizable_panel},
    tree::{TreeItem, TreeState, tree},
};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct WidgetNode {
    id: SharedString,
    parent_id: Option<SharedString>,
}

pub type WidgetNodeMap = FxHashMap<SharedString, WidgetNode>;

struct WidgetTree {
    tree_state: Entity<TreeState>,
}

impl WidgetTree {
    pub fn new(context: &mut Context<Self>) -> Self {
        context
            .observe_global::<AppState>(WidgetTree::update_tree)
            .detach();

        Self {
            tree_state: context.new(|context| TreeState::new(context).items(Vec::new())),
        }
    }

    fn update_tree(this: &mut Self, context: &mut Context<Self>) {
        let app_state = context.global::<AppState>();

        let nodes_guard = app_state.widget_nodes.load();

        let new_items = Self::build_tree_items(&nodes_guard);

        this.tree_state.update(context, |tree_state, context| {
            tree_state.set_items(new_items, context)
        });

        context.notify();
    }

    fn build_tree_item(
        widget_node: &WidgetNode,
        parent_to_children: &FxHashMap<Option<SharedString>, Vec<&WidgetNode>>,
    ) -> TreeItem {
        let mut item = TreeItem::new(widget_node.id.clone(), widget_node.id.clone()).expanded(true);

        let current_id_key = Some(widget_node.id.clone());
        if let Some(children) = parent_to_children.get(&current_id_key) {
            let mut child_items = Vec::with_capacity(children.len());
            for child in children {
                child_items.push(Self::build_tree_item(child, parent_to_children));
            }
            item = item.children(child_items);
        }
        item
    }

    fn build_tree_items(widget_nodes: &WidgetNodeMap) -> Vec<TreeItem> {
        let len = widget_nodes.len();
        if len == 0 {
            return Vec::new();
        }

        // <Option<parent_id>, Vec<child>>
        let mut parent_to_children: FxHashMap<Option<SharedString>, Vec<&WidgetNode>> =
            FxHashMap::with_capacity_and_hasher(len, Default::default());

        let mut roots = Vec::with_capacity(len);

        for node in widget_nodes.values() {
            parent_to_children
                .entry(node.parent_id.clone())
                .or_insert_with(|| Vec::with_capacity(4))
                .push(node);

            let is_root = match &node.parent_id {
                Some(parent_id) => !widget_nodes.contains_key(parent_id),
                None => true,
            };
            if is_root {
                roots.push(node);
            }
        }

        let mut root_items = Vec::with_capacity(roots.len());
        for root in roots {
            root_items.push(Self::build_tree_item(root, &parent_to_children));
        }

        root_items
    }
}

impl Render for WidgetTree {
    fn render(&mut self, _window: &mut Window, _context: &mut Context<Self>) -> impl IntoElement {
        tree(
            &self.tree_state,
            |index, tree_entry, selected, _window, _context| {
                ListItem::new(index)
                    .selected(selected)
                    .pl(px(16.) * tree_entry.depth() as f32 + px(12.))
                    .child(h_flex().gap_2().child(tree_entry.item().label.clone()))
            },
        )
    }
}

pub struct Inspector {
    widget_tree: Entity<WidgetTree>,
}

impl Inspector {
    pub fn new(context: &mut Context<Self>) -> Self {
        Self {
            widget_tree: context.new(|context| WidgetTree::new(context)),
        }
    }
}

impl Render for Inspector {
    fn render(&mut self, _window: &mut Window, _context: &mut Context<Self>) -> impl IntoElement {
        h_resizable("inspector")
            .child(resizable_panel())
            .child(self.widget_tree.clone().into_any_element())
    }
}
