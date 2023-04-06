use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

pub fn system_tray() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show", "Show"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit", "Quit"));

    SystemTray::new().with_menu(tray_menu)
}

pub fn on_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            if id == "show" {
                show_main_window(app);
            } else if id == "quit" {
                app.exit(0);
            }
        }
        SystemTrayEvent::LeftClick { ..} => {},
        SystemTrayEvent::RightClick { ..} => {},
        _ => {}
    }
}

fn show_main_window(app: &AppHandle) {
    let window = app.get_window("main").unwrap();
    window.show().unwrap();
    window.unminimize().unwrap();
    window.set_focus().unwrap();
}
