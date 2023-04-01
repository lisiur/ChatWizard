use tauri::{AppHandle, Manager, Window, WindowBuilder, WindowUrl};

use crate::result::Result;

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
    pub min_size: Option<[f64; 2]>,
    pub max_size: Option<[f64; 2]>,
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            url: "".to_string(),
            width: 720.0,
            height: 540.0,
            resizable: true,
            always_on_top: false,
            visible: false,
            min_size: None,
            max_size: None,
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

pub fn show_or_create_window(
    label: &str,
    window: Window,
    handle: AppHandle,
    options: WindowOptions,
) -> Result<Window> {
    let window = match window.get_window(label) {
        Some(win) => {
            win.show()?;
            log::debug!("show window {}", label);
            win
        }
        None => create_window(label, options, handle)?,
    };

    Ok(window)
}

pub fn create_window(label: &str, options: WindowOptions, handle: AppHandle) -> Result<Window> {
    let min_size = options.min_size.unwrap_or([0.0, 0.0]);
    let max_size = options.max_size.unwrap_or([f64::MAX, f64::MAX]);

    let url = WindowUrl::App(format!("index.html{}", options.url).into());
    log::debug!("creating window {} with url {}", label, url.to_string());

    let mut builder = WindowBuilder::new(&handle, label, url)
        .title(&options.title)
        .always_on_top(options.always_on_top)
        .inner_size(options.width, options.height)
        .min_inner_size(min_size[0], min_size[1])
        .max_inner_size(max_size[0], max_size[1])
        .resizable(options.resizable)
        .skip_taskbar(true)
        .visible(false);

    #[cfg(target_os = "macos")]
    {
        builder = builder
            .title("")
            .title_bar_style(tauri::TitleBarStyle::Overlay);
    }

    let window = builder.build()?;

    Ok(window)
}
