use gpui::{Context, IntoElement, Render, Window, div};
use gpui_component::ToggleInspector;

mod widget_tree;

#[derive(Default)]
pub struct Root {}

impl Root {}

impl Render for Root {
    fn render(&mut self, _: &mut Window, context: &mut Context<Self>) -> impl IntoElement {
        #[cfg(debug_assertions)]
        context.bind_keys([gpui::KeyBinding::new("F12", ToggleInspector, None)]);

        div()
    }
}
