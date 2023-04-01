use tauri::{AppHandle, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

pub fn system_tray() -> SystemTray {
    let tray_menu = SystemTrayMenu::new();

    SystemTray::new().with_menu(tray_menu)
}

pub fn on_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    if let SystemTrayEvent::LeftClick { .. } = event {
        let window = app.get_window("main").unwrap();
        window.show().unwrap();
        window.set_focus().unwrap();
    }
}
