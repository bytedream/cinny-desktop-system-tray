use tauri::{
    CustomMenuItem, GlobalWindowEvent, Manager, SystemTray, SystemTrayEvent, SystemTrayHandle,
    SystemTrayMenu, SystemTrayMenuItem, Window, WindowEvent,
};

pub const TRAY_LABEL: &'static str = "main-tray";

pub fn window_event_handler<R: tauri::Runtime>(event: GlobalWindowEvent<R>) {
    match event.event() {
        // Prevent Cinny from closing, instead hide it and let it be
        // reopened through the tray.
        WindowEvent::CloseRequested { api, .. } => {
            api.prevent_close();

            let window = event.window().clone();
            let tray_handle = window.app_handle().tray_handle_by_id(TRAY_LABEL).unwrap();
            toggle_window_state(window, tray_handle);
        }
        _ => {}
    }
}

/// Build the system tray object
pub fn system_tray() -> SystemTray {
    let toggle = CustomMenuItem::new("toggle".to_owned(), "Hide Cinny");
    let quit = CustomMenuItem::new("quit".to_owned(), "Quit");
    let menu = SystemTrayMenu::new()
        .add_item(toggle)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new()
        .with_menu(menu)
        .with_id(TRAY_LABEL.to_owned())
}

pub fn toggle_window_state<R: tauri::Runtime>(window: Window<R>, tray_handle: SystemTrayHandle<R>) {
    // Hide the window if it's visible, show it if not
    // `is_visible` returns true for minimized state for whatever reason
    if window.is_visible().unwrap() {
        window.hide().unwrap();
        tray_handle
            .get_item("toggle")
            .set_title("Show Cinny")
            .unwrap();
    } else {
        window.unminimize().unwrap();
        window.show().unwrap();
        window.set_focus().unwrap();
        tray_handle
            .get_item("toggle")
            .set_title("Hide Cinny")
            .unwrap();
    };
}

pub fn system_tray_handler<R: tauri::Runtime>(app: &tauri::AppHandle<R>, event: SystemTrayEvent) {
    let tray_handle = match app.tray_handle_by_id(TRAY_LABEL) {
        Some(h) => h,
        None => return,
    };
    let window = app.get_window("main").unwrap();

    match event {
        SystemTrayEvent::LeftClick { .. } => {
            toggle_window_state(window, tray_handle);
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                app.exit(0);
            }
            "toggle" => toggle_window_state(window, tray_handle),
            _ => {}
        },
        _ => {}
    }
}
