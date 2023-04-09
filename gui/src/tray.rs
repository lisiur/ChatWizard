#[cfg(target_os = "macos")]
use tauri::TitleBarStyle;
use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};
use tauri_plugin_positioner::{Position, WindowExt};

use crate::window::{show_or_create_window, WindowOptions};

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
        SystemTrayEvent::LeftClick { .. } => {
            let window = if let Some(window) = app.get_window("casual-chat") {
                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                    window
                } else {
                    window.show().unwrap();
                    window.unminimize().unwrap();
                    window.set_focus().unwrap();
                    window
                }
            } else {
                let mut window_options = WindowOptions {
                    title: "".to_string(),
                    url: "index.html/#/casual-chat".to_string(),
                    width: 460.0,
                    height: 720.0,
                    always_on_top: true,
                    decorations: Some(false),
                    transparent: Some(true),
                    ..Default::default()
                };
                #[cfg(target_os = "macos")]
                {
                    window_options.title_bar_style = Some(TitleBarStyle::Transparent);
                }
                show_or_create_window(app, "casual-chat", window_options).unwrap()
            };
            #[cfg(target_os = "macos")]
            {
                window.move_window(Position::TopRight).unwrap();
            }
            #[cfg(not(target_os = "macos"))]
            {
                window.move_window(Position::BottomRight).unwrap();
            }
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
            if id == "show" {
                show_main_window(app);
            } else if id == "quit" {
                app.exit(0);
            }
        }
        _ => {}
    }
}

fn show_main_window(app: &AppHandle) {
    let window = app.get_window("main").unwrap();
    window.show().unwrap();
    window.unminimize().unwrap();
    window.set_focus().unwrap();
}
