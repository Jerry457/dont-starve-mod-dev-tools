use gpui::{
    AppContext, Context, Div, Entity, EventEmitter, InteractiveElement, IntoElement, ParentElement,
    Render, Stateful, StatefulInteractiveElement, Styled, Window, div, px,
};
use gpui_component::{ActiveTheme, StyledExt};

pub enum TabBarEvent {
    TabSelected(usize), // 携带被选中的索引
}

pub struct TabBar {
    tabs: Vec<&'static str>,

    tab_width: f32,

    selected_index: usize,
    under_line_current_offset: f32,
    under_line_target_offset: f32,
}

impl TabBar {
    pub fn new(tabs: Vec<&'static str>, tab_width: f32) -> Self {
        Self {
            tabs,
            tab_width,
            selected_index: 0,
            under_line_current_offset: 0.0,
            under_line_target_offset: 0.0,
        }
    }

    fn update_under_line(&mut self, window: &mut Window, context: &mut Context<Self>) {
        self.under_line_target_offset = self.selected_index as f32 * self.tab_width;
        if (self.under_line_current_offset - self.under_line_target_offset).abs() <= 0.5 {
            self.under_line_current_offset = self.under_line_target_offset;
        } else {
            self.under_line_current_offset +=
                (self.under_line_target_offset - self.under_line_current_offset) * 0.18;

            context.on_next_frame(window, move |this, window, context| {
                this.update_under_line(window, context);
                context.notify();
            });
        }
    }

    fn under_line(&self, context: &mut Context<Self>) -> Div {
        div()
            .h(px(2.0))
            .w(px(self.tab_width))
            .absolute()
            .bottom_0()
            .bg(context.theme().tab_active_foreground)
    }

    fn bar(context: &mut Context<Self>) -> Div {
        let theme = context.theme();
        div()
            .flex()
            .items_center()
            .h_8()
            .relative()
            .bg(theme.tab_bar)
            .border_b_1()
            .border_color(theme.border)
    }

    fn tab_title(&self, context: &mut Context<Self>, label: String, index: usize) -> Stateful<Div> {
        let theme = context.theme();
        let selected = self.selected_index == index;
        div()
            .id(label.to_string())
            // style
            .flex()
            .items_center()
            .justify_center()
            .w(px(self.tab_width))
            .h_full()
            .px_4()
            .cursor_pointer()
            .border_b_2()
            .text_xs()
            .hover(|style| {
                style
                    .bg(theme.button_hover)
                    .text_color(theme.tab_active_foreground)
            })
            .active(|style| style.bg(theme.button_active))
            .text_color(match selected {
                true => theme.tab_active_foreground,
                false => theme.tab_foreground,
            })
            // children
            .child(label)
            // events
            .on_click(context.listener(move |this, _event, window, context| {
                this.selected_index = index;
                this.update_under_line(window, context);
                context.emit(TabBarEvent::TabSelected(index));
            }))
    }
}

impl Render for TabBar {
    fn render(&mut self, _window: &mut Window, context: &mut Context<Self>) -> impl IntoElement {
        let tabs = self.tabs.iter().enumerate();
        Self::bar(context)
            .children(tabs.map(|(index, &label)| self.tab_title(context, label.to_string(), index)))
            .child(
                self.under_line(context)
                    .left(gpui::px(self.under_line_current_offset)),
            )
    }
}

impl EventEmitter<TabBarEvent> for TabBar {}

pub struct Tab {
    selected_tab: usize,
    bar: Entity<TabBar>,
}

impl Tab {
    pub fn new(context: &mut Context<Self>, tabs: Vec<&'static str>, tab_width: f32) -> Self {
        let bar = context.new(|_context| TabBar::new(tabs, tab_width));

        context
            .subscribe(&bar, |this, _tab_bar, event, context| match event {
                TabBarEvent::TabSelected(index) => {
                    this.selected_tab = *index;
                    context.notify();
                }
            })
            .detach();

        Self {
            selected_tab: 0,
            bar,
        }
    }

    pub fn view(&self) -> impl IntoElement {
        match self.selected_tab {
            0 => div().p_4().child("Widgets"),
            1 => div().p_4().child("Console"),
            _ => div(),
        }
    }
}

impl Render for Tab {
    fn render(&mut self, _window: &mut Window, _context: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .size_full()
            .child(self.bar.clone())
            .child(self.view())
    }
}
