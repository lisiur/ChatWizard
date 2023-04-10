use std::time::Duration;

use crate::result::Result;
use crate::window::{show_or_create_main_window, toggle_tray_window};
use tauri::{
    AppHandle, CustomMenuItem, LogicalPosition, PhysicalPosition, PhysicalSize, SystemTray,
    SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, Window,
};
use tokio::time::sleep;

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
            let window = toggle_tray_window(app).unwrap();
            tokio::spawn(async move {
                fixed_tray_window_position(&window, position, size)
                    .await
                    .unwrap();
            });
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

async fn fixed_tray_window_position(
    window: &Window,
    tray_position: PhysicalPosition<f64>,
    tray_size: PhysicalSize<f64>,
) -> Result<()> {
    let monitors = window.available_monitors()?;

    let tray_pos_y = tray_position.y as i32;
    let tray_pos_x = tray_position.x as i32;
    let mut tray_monitor = &monitors[0];
    for monitor in &monitors {
        let position = monitor.position();
        let size = monitor.size();
        if tray_pos_x >= position.x && tray_pos_x <= position.x + size.width as i32 {
            tray_monitor = monitor;
            break;
        }
    }

    // It's weird that we need to set the window position twice to make it work.
    // The first time we set the window to the monitor's top left corner. (Because this operation will always do the right thing.)
    // And then set the window to the top right corner. (Because this operation will encounter wrong display if we don't do the first movement.)
    let window_pos_x = tray_monitor.position().x;
    window
        .set_position(LogicalPosition {
            x: window_pos_x,
            y: 0,
        })
        .unwrap();
    // And we also need to wait for a while to make sure the window is moved to the right position.
    sleep(Duration::from_micros(1)).await;

    let window_right_pos_x = tray_monitor.position().x + tray_monitor.size().width as i32
        - window.outer_size().unwrap().width as i32;
    let window_top_pos_y = tray_size.height as i32;

    let window_bottom_pos_y =
        tray_monitor.size().height as i32 - window.outer_size().unwrap().height as i32;

    if tray_pos_y < tray_monitor.size().height as i32 / 2 {
        window
            .set_position(PhysicalPosition {
                x: window_right_pos_x,
                y: window_top_pos_y,
            })
            .unwrap();
    } else {
        window
            .set_position(PhysicalPosition {
                x: window_right_pos_x,
                y: window_bottom_pos_y,
            })
            .unwrap();
    }

    Ok(())
}
