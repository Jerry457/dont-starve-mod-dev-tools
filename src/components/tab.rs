use gpui::{
    AnyView, AppContext, Context, Div, Entity, EventEmitter, InteractiveElement, IntoElement,
    ParentElement, Render, Stateful, StatefulInteractiveElement, Styled, Window, div, px,
};
use gpui_component::{ActiveTheme, IconName, StyledExt};

pub struct TabTitleConfig {
    pub name: &'static str,
    pub icon: IconName,
}

pub struct TabConfig {
    pub title: TabTitleConfig,
    pub view: AnyView,
}

pub enum TabBarEvent {
    TabSelected(usize), // 携带被选中的索引
}

pub struct TabBar {
    pub configs: Vec<TabTitleConfig>,

    tab_width: f32,

    selected_index: usize,
    underline_current_offset: f32,
    underline_target_offset: f32,
}

impl TabBar {
    pub fn new(configs: Vec<TabTitleConfig>, tab_width: f32) -> Self {
        Self {
            configs,
            tab_width,
            selected_index: 0,
            underline_current_offset: 0.0,
            underline_target_offset: 0.0,
        }
    }

    fn update_underline(&mut self, window: &mut Window, context: &mut Context<Self>) {
        self.underline_target_offset = self.selected_index as f32 * self.tab_width;
        if (self.underline_current_offset - self.underline_target_offset).abs() <= 0.5 {
            self.underline_current_offset = self.underline_target_offset;
        } else {
            self.underline_current_offset +=
                (self.underline_target_offset - self.underline_current_offset) * 0.18;

            context.on_next_frame(window, move |this, window, context| {
                this.update_underline(window, context);
                context.notify();
            });
        }
    }

    fn underline(&self, context: &mut Context<Self>) -> Div {
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

    fn tab_title(
        &self,
        context: &mut Context<Self>,
        config: &TabTitleConfig,
        index: usize,
    ) -> Stateful<Div> {
        let theme = context.theme();
        let selected = self.selected_index == index;
        div()
            .id(config.name)
            // style
            .flex()
            .items_center()
            .justify_center()
            .w(px(self.tab_width))
            .h_full()
            .px_4()
            .gap_1()
            .cursor_pointer()
            .border_b_2()
            .text_sm()
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
            .child(config.icon.clone())
            .child(config.name)
            // events
            .on_click(context.listener(move |this, _event, window, context| {
                this.selected_index = index;
                this.update_underline(window, context);
                context.emit(TabBarEvent::TabSelected(index));
            }))
    }
}

impl Render for TabBar {
    fn render(&mut self, _window: &mut Window, context: &mut Context<Self>) -> impl IntoElement {
        let tabs = self.configs.iter().enumerate();
        Self::bar(context)
            .children(tabs.map(|(index, config)| self.tab_title(context, config, index)))
            .child(
                self.underline(context)
                    .left(gpui::px(self.underline_current_offset)),
            )
    }
}

impl EventEmitter<TabBarEvent> for TabBar {}

pub struct Tab {
    views: Vec<AnyView>,
    bar: Entity<TabBar>,

    selected_tab: usize,
}

impl Tab {
    pub fn new(context: &mut Context<Self>, configs: Vec<TabConfig>, tab_width: f32) -> Self {
        let (title_configs, views) = configs
            .into_iter()
            .map(|config| (config.title, config.view))
            .unzip();

        let bar = context.new(|_context| TabBar::new(title_configs, tab_width));

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
            views,
            bar,
        }
    }

    pub fn view(&self) -> impl IntoElement {
        self.views[self.selected_tab].clone()
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
