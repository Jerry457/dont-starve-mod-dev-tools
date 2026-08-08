use gpui::{App, AppContext, BorrowAppContext, Bounds, WindowBounds, WindowOptions, px, size};
use gpui_component_assets::Assets;

#[cfg(debug_assertions)]
use gpui::KeyBinding;

#[cfg(debug_assertions)]
use gpui_component::ToggleInspector;

mod app_state;
mod logger;
mod server;
mod ui;

const ADDRESS: &str = "127.0.0.1:45754";

fn main() -> anyhow::Result<()> {
    logger::init()?;

    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel::<()>();
    let app_state = app_state::AppState::new(sender);

    let server_app_state = app_state.clone();
    let (_serde_jsonshutdown_sender, shutdown_receiver) = tokio::sync::oneshot::channel::<()>();
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("create Tokio runtime");
        if let Err(error) =
            runtime.block_on(server::serve(ADDRESS, shutdown_receiver, server_app_state))
        {
            log::error!("HTTP service stopped: {error}");
        }
    });

    gpui_platform::application()
        .with_assets(Assets)
        .run(|app: &mut App| {
            gpui_component::init(app);
            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(1000.0), px(900.0)),
                    app,
                ))),
                ..Default::default()
            };

            app.spawn(async move |async_app| {
                async_app
                    .open_window(options, |window, app| {
                        app.set_global(app_state.clone());

                        let root_view = app.new(|context| ui::Root::new(context));
                        app.new(|context| {
                            #[cfg(debug_assertions)]
                            context.bind_keys([KeyBinding::new("F12", ToggleInspector, None)]);

                            gpui_component::Root::new(root_view, window, context)
                        })
                    })
                    .expect("Failed to open window");

                while let Some(()) = receiver.recv().await {
                    async_app.update(|app| {
                        // 触发 AppState 改变事件
                        app.update_global::<app_state::AppState, _>(|_app_state, _app| {});
                    });
                }
            })
            .detach();
        });

    Ok(())
}
