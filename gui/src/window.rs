use tauri::{AppHandle, Manager, Window, WindowBuilder, WindowUrl};

use crate::result::Result;

#[derive(serde::Deserialize, Debug)]
pub struct WindowOptions {
    title: Option<String>,
    url: Option<String>,
    width: Option<f64>,
    height: Option<f64>,
    resizable: Option<bool>,
}

pub fn show_window_lazy(
    label: String,
    options: Option<WindowOptions>,
    window: Window,
    handle: AppHandle,
) -> Result<()> {
    match window.get_window(&label) {
        Some(win) => {
            win.show().unwrap();
            log::debug!("show window {}", label);
        }
        None => {
            let title = options
                .as_ref()
                .and_then(|o| o.title.clone())
                .unwrap_or_default();
            let url = options
                .as_ref()
                .and_then(|o| o.url.clone())
                .unwrap_or_default();
            let resizable = options.as_ref().and_then(|o| o.resizable).unwrap_or(false);

            let url = WindowUrl::App(format!("index.html{}", url).into());
            log::debug!("creating window {} with url {}", label, url.to_string());

            let mut builder = WindowBuilder::new(&handle, &label, url)
                .title(&title)
                .resizable(resizable);

            #[cfg(target_os = "macos")]
            {
                builder = builder
                    .title("")
                    .title_bar_style(tauri::TitleBarStyle::Overlay);
            }

            if let (Some(width), Some(height)) = (
                options.as_ref().and_then(|o| o.width),
                options.as_ref().and_then(|o| o.height),
            ) {
                builder = builder.inner_size(width, height);
            }
            builder.visible(false).build().unwrap();
        }
    }

    Ok(())
}
