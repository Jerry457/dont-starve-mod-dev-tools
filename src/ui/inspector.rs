use gpui::{Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div};
use gpui_component::{list::ListItem, resizable::{h_resizable, resizable_panel}, tree::{TreeItem, TreeState, tree}, v_flex};

struct SearchableTree {
    tree_state: Entity<TreeState>,
    original_items: Vec<TreeItem>,
    search_query: String,
}

impl SearchableTree {
    fn filter_tree(&mut self, query: &str, cx: &mut Context<Self>) {
        self.search_query = query.to_string();

        let filtered_items = if query.is_empty() {
            self.original_items.clone()
        } else {
            filter_tree_items(&self.original_items, query)
        };

        self.tree_state.update(cx, |state, cx| {
            state.set_items(filtered_items, cx);
        });
    }
}

impl Render for SearchableTree {
    fn render(&mut self, _window: &mut Window, context: &mut Context<Self>) -> impl IntoElement {
        let tree_state = self.tree_state.clone();

        v_flex()
            .size_full()
            .gap_2()
            // .child(
            //     // 搜索输入框示例（根据您使用的具体输入框组件进行绑定）
            //     // 需在 text_change / on_input 回调中调用 self.filter_tree(value, cx)
            //     /* text_input_component */
            // )
            .child(
                // 渲染过滤后的树
                tree(&tree_state, |ix, entry, selected, _window, _cx| {
                    ListItem::new(ix)
                        .selected(selected)
                        .child(entry.item().label.clone())
                })
            )
    }
}

fn filter_tree_items(items: &[TreeItem], query: &str) -> Vec<TreeItem> {
    items.iter()
        .filter_map(|item| {
            if item.label.to_lowercase().contains(&query.to_lowercase()) {
                Some(item.clone().expanded(true)) // Auto-expand matches
            } else {
                // Check if any children match
                let filtered_children = filter_tree_items(&item.children, query);
                if !filtered_children.is_empty() {
                    Some(item.clone()
                        .children(filtered_children)
                        .expanded(true))
                } else {
                    None
                }
            }
        })
        .collect()
}


pub struct Inspector {
    // widget_tree: Entity<SearchableTree>,
}

impl Inspector {
    pub fn new(_context: &mut Context<Self>) -> Self {
        Self {
            // widget_tree: SearchableTree{
            //     tree_state: todo!(),
            //     original_items: [],
            //     search_query: "",
            // }
        }
    }
}

impl Render for Inspector {
    fn render(&mut self, _window: &mut Window, _context: &mut Context<Self>) -> impl IntoElement {
        h_resizable("inspector")
            // .child(resizable_panel().child(SearchableTree))
            .child(div().into_any_element())
    }
}
