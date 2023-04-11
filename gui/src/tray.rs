use crate::window::{show_or_create_main_window, toggle_tray_window};
use tauri::{
    AppHandle, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

pub fn system_tray() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show", "Main Window"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit", "Quit"));

    SystemTray::new().with_menu(tray_menu)
}

pub fn on_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    tauri_plugin_positioner::on_tray_event(app, &event);

    match event {
        SystemTrayEvent::LeftClick { position, size, .. } => {
            toggle_tray_window(app, position, size).unwrap();
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
            if id == "show" {
                let handle = app.clone();
                tokio::spawn(async move {
                    show_or_create_main_window(&handle).await.unwrap();
                });
            } else if id == "quit" {
                app.exit(0);
            }
        }
        _ => {}
    }
}
