use tauri::{AppHandle, Manager, TitleBarStyle, Window, WindowBuilder, WindowUrl};

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
    pub position: Option<[f64; 2]>,
    pub min_size: Option<[f64; 2]>,
    pub max_size: Option<[f64; 2]>,
    pub title_bar_style: Option<TitleBarStyle>,
    pub decorations: Option<bool>,
    pub transparent: Option<bool>,
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
            title_bar_style: Some(TitleBarStyle::Visible),
            decorations: Some(true),
            transparent: Some(false),
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
    handle: &AppHandle,
    label: &str,
    options: WindowOptions,
) -> Result<Window> {
    let window = match handle.get_window(label) {
        Some(win) => {
            win.show()?;
            win.unminimize()?;
            win.set_focus()?;
            log::debug!("show window {}", label);
            win
        }
        None => create_window(handle, label, options)?,
    };

    Ok(window)
}

pub fn create_window(handle: &AppHandle, label: &str, options: WindowOptions) -> Result<Window> {
    let min_size = options.min_size.unwrap_or([0.0, 0.0]);
    let max_size = options.max_size.unwrap_or([f64::MAX, f64::MAX]);

    let url = WindowUrl::App(options.url.into());
    log::debug!("creating window {} with url {}", label, url.to_string());

    let title_bar_style = options.title_bar_style.unwrap_or(TitleBarStyle::Visible);
    let mut builder = WindowBuilder::new(handle, label, url)
        .title(&options.title)
        .always_on_top(options.always_on_top)
        .inner_size(options.width, options.height)
        .min_inner_size(min_size[0], min_size[1])
        .max_inner_size(max_size[0], max_size[1])
        .resizable(options.resizable)
        .title_bar_style(title_bar_style)
        .decorations(options.decorations.unwrap_or(true))
        .transparent(options.transparent.unwrap_or(false))
        .visible(false);

    if let Some(position) = options.position {
        builder = builder.position(position[0], position[1]);
    }

    let window = builder.build()?;

    Ok(window)
}
