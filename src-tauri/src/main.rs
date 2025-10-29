#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[cfg(target_os = "macos")]
mod menu;
mod tray;

use tauri::{utils::config::AppUrl, Manager, WindowUrl};

fn main() {
    let port = 44548;

    let mut context = tauri::generate_context!();
    let url = format!("http://localhost:{}", port).parse().unwrap();
    let window_url = WindowUrl::External(url);
    // rewrite the config so the IPC is enabled on this URL
    context.config_mut().build.dist_dir = AppUrl::Url(window_url.clone());
    context.config_mut().build.dev_path = AppUrl::Url(window_url.clone());
    let builder = tauri::Builder::default();

    let builder = builder
        .system_tray(tray::system_tray())
        .on_system_tray_event(tray::system_tray_handler);

    #[cfg(target_os = "macos")]
    let builder = builder.menu(menu::menu());

    builder
        .plugin(tauri_plugin_localhost::Builder::new(port).build())
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            let window = app.get_window("main").unwrap();
            let tray_handle = match app.tray_handle_by_id(tray::TRAY_LABEL) {
                Some(h) => h,
                None => return,
            };
            tray::toggle_window_state(window, tray_handle)
        }))
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .on_window_event(tray::window_event_handler)
        .run(context)
        .expect("error while building tauri application")
}
