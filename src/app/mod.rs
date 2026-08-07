use gpui::{AppContext, Context, Entity, IntoElement, Render, Window};

use crate::components::tab::Tab;

pub struct Root {
    tab_view: Entity<Tab>,
}

impl Root {
    pub fn new(context: &mut Context<Self>) -> Self {
        Self {
            tab_view: context.new(|context| Tab::new(context, vec!["Widgets", "Console"], 96.0)),
        }
    }
}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, _context: &mut Context<Self>) -> impl IntoElement {
        self.tab_view.clone()
    }
}
