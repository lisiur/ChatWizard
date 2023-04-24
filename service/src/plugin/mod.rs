use std::time::Duration;

use crate::result::Result;
use crate::services::plugin::PluginService;
use host::WasiCtx;
use indicatif::{ProgressBar, ProgressStyle};
use wasi_cap_std_sync::WasiCtxBuilder;
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};

bindgen!({
    path: "./src/plugin/wit",
    async: true,
});

pub struct RunningPluginState {
    wasi_ctx: WasiCtx,
    plugin_service: PluginService,
    loading_bar: Option<ProgressBar>,
}

impl RunningPluginState {
    pub fn new(plugin_service: PluginService) -> Self {
        let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
        Self {
            wasi_ctx,
            plugin_service,
            loading_bar: None,
        }
    }

    pub fn show_loading(&mut self) {
        self.stop_loading();
        let lb = ProgressBar::new_spinner();
        lb.enable_steady_tick(Duration::from_millis(500));
        lb.set_style(
            ProgressStyle::with_template("{msg}{spinner:.blue}")
                .unwrap()
                .tick_strings(&["   ", ".  ", ".. ", "...", ""]),
        );
        lb.set_message("Thinking");
        self.loading_bar = Some(lb);
    }

    pub fn stop_loading(&mut self) {
        if let Some(loading_bar) = self.loading_bar.take() {
            loading_bar.finish_and_clear();
        }
    }

    pub fn select(&mut self, options: Vec<&str>) -> Option<String> {
        match inquire::Select::new("Select an option: ", options).prompt() {
            Ok(choice) => Some(choice.to_string()),
            Err(_) => None,
        }
    }
}

#[async_trait::async_trait]
impl Host_Imports for RunningPluginState {
    async fn host_exec(&mut self, cmd: String, args: Vec<String>) -> wasmtime::Result<(i32, String)> {
        let output = std::process::Command::new(cmd)
            .current_dir(".")
            .args(args)
            .output()
            .unwrap();
        let success = output.status.success();
        let status = output.status.code().unwrap_or_default();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        Ok((
            status,
            if success {
                stdout.to_string()
            } else {
                stderr.to_string()
            },
        ))
    }

    async fn host_openai(&mut self, prompt: String) -> wasmtime::Result<(i32, String)> {
        match self.plugin_service.send_message(&prompt).await {
            Ok(reply) => Ok((0, reply)),
            Err(err) => Ok((1, err.to_string())),
        }
    }

    async fn host_loading(&mut self, loading: bool) -> wasmtime::Result<()> {
        if loading {
            self.show_loading();
        } else {
            self.stop_loading();
        }
        Ok(())
    }

    async fn host_select(&mut self, options: Vec<String>) -> wasmtime::Result<Option<String>> {
        Ok(self.select(options.iter().map(|x| x.as_str()).collect()))
    }
}

pub struct RunningPlugin {
    pub store: Store<RunningPluginState>,
    pub bindings: Host_,
}

impl RunningPlugin {
    pub async fn init(code: &[u8], state: RunningPluginState) -> Result<Self> {
        let mut config = Config::new();
        config.wasm_component_model(true).async_support(true);
        let engine = Engine::new(&config)?;
        let component = Component::from_binary(&engine, code)?;

        let mut linker = Linker::new(&engine);
        host::wasi::filesystem::add_to_linker(&mut linker, |x: &mut RunningPluginState| {
            &mut x.wasi_ctx
        })?;
        host::wasi::streams::add_to_linker(&mut linker, |x| &mut x.wasi_ctx)?;
        host::wasi::environment::add_to_linker(&mut linker, |x| &mut x.wasi_ctx)?;
        host::wasi::preopens::add_to_linker(&mut linker, |x| &mut x.wasi_ctx)?;
        host::wasi::exit::add_to_linker(&mut linker, |x| &mut x.wasi_ctx)?;
        Host_::add_to_linker(&mut linker, |state: &mut RunningPluginState| state)?;

        let mut store = Store::new(&engine, state);

        let (bindings, _) = Host_::instantiate_async(&mut store, &component, &linker).await?;

        Ok(RunningPlugin { bindings, store })
    }

    pub async fn run(&mut self) -> Result<()> {
        self.bindings.call_run(&mut self.store).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use crate::{services::plugin::PluginService, test::establish_connection};

    #[test]
    fn test_loading() {
        let conn = establish_connection();
        let plugin_service = PluginService::new(conn);
        let mut state = super::RunningPluginState::new(plugin_service);

        state.show_loading();
        thread::sleep(std::time::Duration::from_secs(3));
        state.stop_loading();
    }

    #[tokio::test]
    async fn it_works() {
        let conn = establish_connection();
        let plugin_service = PluginService::new(conn);
        let state = super::RunningPluginState::new(plugin_service);

        let binary = std::fs::read(
            "../../chat-wizard-plugins/plugins/commit-summary/built/commit_summary.wasm",
        )
        .unwrap();
        let plugin = super::RunningPlugin::init(&binary, state).await.unwrap();
        plugin.bindings.call_run(plugin.store).await.unwrap();
    }
}
