use gpui::{App, AppContext, Bounds, WindowBounds, WindowOptions, px, size};

mod app;

fn main() {
    gpui_platform::application().run(|cx: &mut App| {
        gpui_component::init(cx);

        let bounds = Bounds::centered(None, size(px(1000.0), px(900.0)), cx);

        cx.spawn(async move |async_app| {
            async_app
                .open_window(
                    WindowOptions {
                        window_bounds: Some(WindowBounds::Windowed(bounds)),
                        ..Default::default()
                    },
                    |window, app| {
                        let root_view = app.new(|_| app::Root::default());

                        app.new(|context| gpui_component::Root::new(root_view, window, context))
                    },
                )
                .expect("Failed to open window");
        })
        .detach();
    });
}
