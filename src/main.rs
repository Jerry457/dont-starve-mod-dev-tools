#[cfg(debug_assertions)]
use gpui::KeyBinding;
use gpui::{App, AppContext, Bounds, WindowBounds, WindowOptions, px, size};
use gpui_component_assets::Assets;

#[cfg(debug_assertions)]
use gpui_component::ToggleInspector;

mod tools;
mod components;

fn main() {
    gpui_platform::application()
        .with_assets(Assets)
        .run(|app: &mut App| {
            gpui_component::init(app);

            let bounds = Bounds::centered(None, size(px(1000.0), px(900.0)), app);

            app.spawn(async move |async_app| {
                async_app
                    .open_window(
                        WindowOptions {
                            window_bounds: Some(WindowBounds::Windowed(bounds)),
                            ..Default::default()
                        },
                        |window, app| {
                            let root_view = app.new(|context| tools::Root::new(context));
                            app.new(|context| {
                                #[cfg(debug_assertions)]
                                context.bind_keys([KeyBinding::new("F12", ToggleInspector, None)]);

                                gpui_component::Root::new(root_view, window, context)
                            })
                        },
                    )
                    .expect("Failed to open window");
            })
            .detach();
        });
}
