use gpui::{AppContext, Context, Entity, IntoElement, Render, Window};
use gpui_component::IconName;

use crate::ui::{
    components::tab::{Tab, TabConfig, TabTitleConfig},
    inspector::Inspector,
};

mod components;
pub mod inspector;

pub struct Root {
    tab_view: Entity<Tab>,
}

impl Root {
    pub fn new(context: &mut Context<Self>) -> Self {
        let inspector_page = TabConfig {
            title: TabTitleConfig {
                name: "Inspector",
                icon: IconName::Inspector,
            },
            view: context.new(|context| Inspector::new(context)).into(),
        };

        Self {
            tab_view: context.new(|context| Tab::new(context, vec![inspector_page], 100.0)),
        }
    }
}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, _context: &mut Context<Self>) -> impl IntoElement {
        self.tab_view.clone()
    }
}
