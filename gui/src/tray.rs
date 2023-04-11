use crate::window::{show_or_create_about_window, show_or_create_main_window, toggle_tray_window};
use tauri::{
    AppHandle, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

pub fn system_tray() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("about", "About"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("show", "Main Window"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit", "Quit"));

    SystemTray::new().with_menu(tray_menu)
}

pub fn on_system_tray_event(handle: &AppHandle, event: SystemTrayEvent) {
    tauri_plugin_positioner::on_tray_event(handle, &event);

    match event {
        SystemTrayEvent::LeftClick { position, size, .. } => {
            toggle_tray_window(handle, position, size).unwrap();
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_ref() {
            "show" => {
                let handle = handle.clone();
                tokio::spawn(async move {
                    show_or_create_main_window(&handle).await.unwrap();
                });
            }
            "about" => {
                show_or_create_about_window(handle).unwrap();
            }
            "quit" => {
                handle.exit(0);
            }
            _ => {}
        },
        _ => {}
    }
}
