use gpui::{Context, IntoElement, ParentElement, Render, Window, div};
use gpui_component::resizable::{h_resizable, resizable_panel};

pub struct Inspector {}

impl Inspector {
    pub fn new(_context: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for Inspector {
    fn render(&mut self, _window: &mut Window, _context: &mut Context<Self>) -> impl IntoElement {
        h_resizable("inspector")
            .child(resizable_panel().child(div()))
            .child(div().into_any_element())
    }
}
