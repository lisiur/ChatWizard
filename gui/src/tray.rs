use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

pub fn system_tray() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show".to_string(), "Show"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));

    SystemTray::new().with_menu(tray_menu)
}

pub fn on_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            tray_id, position, ..
        } => {
            println!("left click on tray {} in {:?}", tray_id, position);
        }
        SystemTrayEvent::RightClick {
            tray_id, position, ..
        } => {
            println!("right click on tray {} in {:?}", tray_id, position);
        }
        SystemTrayEvent::DoubleClick {
            tray_id, position, ..
        } => {
            println!("double click on tray {} in {:?}", tray_id, position);
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "show" => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            "quit" => app.exit(0),
            _ => {}
        },
        _ => {}
    }
}
