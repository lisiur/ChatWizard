use std::time::Duration;

#[cfg(target_os = "macos")]
use tauri::TitleBarStyle;
use tauri::{
    AppHandle, LogicalPosition, Manager, PhysicalPosition, PhysicalSize, Window, WindowBuilder,
    WindowUrl,
};
use tokio::time::sleep;

use crate::{result::Result, AppSetting};

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WindowOptions {
    pub title: String,
    pub url: String,
    pub width: f64,
    pub height: f64,
    pub resizable: bool,
    pub always_on_top: bool,
    pub visible: bool,
    pub position: Option<[f64; 2]>,
    pub min_size: Option<[f64; 2]>,
    pub max_size: Option<[f64; 2]>,
    #[cfg(target_os = "macos")]
    pub title_bar_style: Option<TitleBarStyle>,
    pub decorations: Option<bool>,
    pub transparent: Option<bool>,
    pub skip_taskbar: Option<bool>,
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            url: "index.html".to_string(),
            width: 720.0,
            height: 540.0,
            resizable: true,
            always_on_top: false,
            visible: false,
            position: None,
            min_size: None,
            max_size: None,
            #[cfg(target_os = "macos")]
            title_bar_style: Some(TitleBarStyle::Visible),
            decorations: Some(true),
            transparent: Some(false),
            skip_taskbar: Some(false),
        }
    }
}

pub fn show_window(label: &str, window: Window) -> Result<()> {
    if let Some(win) = window.get_window(label) {
        win.show()?;
        log::debug!("show window {}", label);
    }
    Ok(())
}

pub fn show_or_create_window_in_background(
    handle: &AppHandle,
    label: &str,
    options: WindowOptions,
) -> Result<Window> {
    let window = match handle.get_window(label) {
        Some(win) => {
            focus_window(&win);
            log::debug!("show window {}", label);
            win
        }
        None => create_window_in_background(handle, label, options)?,
    };

    Ok(window)
}

pub fn create_window_in_background(
    handle: &AppHandle,
    label: &str,
    options: WindowOptions,
) -> Result<Window> {
    let min_size = options.min_size.unwrap_or([0.0, 0.0]);
    let max_size = options.max_size.unwrap_or([f64::MAX, f64::MAX]);

    let url = WindowUrl::App(options.url.into());
    log::debug!("creating window {} with url {}", label, url.to_string());

    let mut builder = WindowBuilder::new(handle, label, url)
        .title(&options.title)
        .always_on_top(options.always_on_top)
        .inner_size(options.width, options.height)
        .min_inner_size(min_size[0], min_size[1])
        .max_inner_size(max_size[0], max_size[1])
        .resizable(options.resizable)
        .decorations(options.decorations.unwrap_or(true))
        .transparent(options.transparent.unwrap_or(false))
        .skip_taskbar(options.skip_taskbar.unwrap_or(false))
        .visible(false);

    #[cfg(target_os = "macos")]
    {
        if let Some(title_bar_style) = options.title_bar_style {
            builder = builder.title_bar_style(title_bar_style);
        }
    }

    if let Some(position) = options.position {
        builder = builder.position(position[0], position[1]);
    }

    let window = builder.build()?;

    Ok(window)
}

pub async fn show_or_create_main_window(handle: &AppHandle) -> Result<Window> {
    log::debug!("show_or_create_main_window");
    let setting = handle.state::<AppSetting>();
    let hide_taskbar = setting.0.lock().await.hide_taskbar;

    #[cfg(target_os = "macos")]
    {
        let window = show_or_create_window_in_background(
            handle,
            "main",
            WindowOptions {
                title: "".to_string(),
                url: "index.html".to_string(),
                width: 860.0,
                height: 720.0,
                title_bar_style: Some(TitleBarStyle::Overlay),
                skip_taskbar: Some(hide_taskbar),
                ..Default::default()
            },
        )
        .unwrap();

        Ok(window)
    }

    #[cfg(not(target_os = "macos"))]
    {
        let window = show_or_create_window_in_background(
            handle,
            "main",
            WindowOptions {
                title: "ChatWizard".to_string(),
                url: "index.html".to_string(),
                width: 860.0,
                height: 720.0,
                skip_taskbar: Some(hide_taskbar),
                ..Default::default()
            },
        )
        .unwrap();
        Ok(window)
    }
}

pub fn toggle_tray_window(
    handle: &AppHandle,
    tray_position: PhysicalPosition<f64>,
    tray_size: PhysicalSize<f64>,
) -> Result<()> {
    let window = create_tray_window_in_background(handle).unwrap();
    if window.is_visible().unwrap() {
        window.hide().unwrap();
    } else {
        tokio::spawn(async move {
            fixed_tray_window_position(&window, tray_position, tray_size)
                .await
                .unwrap();
            focus_window(&window);
        });
    }

    Ok(())
}

pub fn focus_window(window: &Window) {
    window.show().unwrap();
    window.unminimize().unwrap();
    window.set_focus().unwrap();
}

pub fn create_tray_window_in_background(handle: &AppHandle) -> Result<Window> {
    match handle.get_window("casual-chat") {
        Some(window) => Ok(window),
        None => {
            let options = tray_window_options();
            create_window_in_background(handle, "casual-chat", options)
        }
    }
}

fn tray_window_options() -> WindowOptions {
    let mut window_options = WindowOptions {
        title: "".to_string(),
        url: "index.html/#/casual-chat?background".to_string(),
        width: 460.0,
        height: 720.0,
        always_on_top: true,
        decorations: Some(false),
        transparent: Some(true),
        skip_taskbar: Some(true),
        visible: false,
        ..Default::default()
    };
    #[cfg(target_os = "macos")]
    {
        window_options.title_bar_style = Some(TitleBarStyle::Transparent);
    }
    window_options
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
    let mut window_tray_center_pos_x =
        tray_pos_x + tray_size.width as i32 / 2 - (window.outer_size().unwrap().width / 2) as i32;

    if window_tray_center_pos_x > window_right_pos_x {
        window_tray_center_pos_x = window_right_pos_x;
    }

    let window_top_pos_y = tray_size.height as i32;

    let window_bottom_pos_y =
        tray_monitor.size().height as i32 - window.outer_size().unwrap().height as i32;

    if tray_pos_y < tray_monitor.size().height as i32 / 2 {
        window
            .set_position(PhysicalPosition {
                x: window_tray_center_pos_x,
                y: window_top_pos_y,
            })
            .unwrap();
    } else {
        window
            .set_position(PhysicalPosition {
                x: window_tray_center_pos_x,
                y: window_bottom_pos_y,
            })
            .unwrap();
    }

    Ok(())
}
