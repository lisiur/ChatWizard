use crate::result::Result;
use crate::services::plugin::PluginService;
use host::WasiCtx;
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
}

impl RunningPluginState {
    pub fn new(plugin_service: PluginService) -> Self {
        let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
        Self {
            wasi_ctx,
            plugin_service,
        }
    }
}

#[async_trait::async_trait]
impl Host_Imports for RunningPluginState {
    async fn exec(&mut self, cmd: String) -> wasmtime::Result<(i32, String)> {
        let cmd_and_args = cmd.split_whitespace().collect::<Vec<_>>();
        let cmd = cmd_and_args[0];
        let args = &cmd_and_args[1..];
        let output = std::process::Command::new(cmd)
            .current_dir(".")
            .args(args)
            .output()
            .unwrap();
        let status = output.status.code().unwrap_or_default();
        let output = String::from_utf8_lossy(&output.stdout);
        Ok((status, output.to_string()))
    }

    async fn openai(&mut self, prompt: String) -> wasmtime::Result<(i32, String)> {
        match self.plugin_service.send_message(&prompt).await {
            Ok(reply) => Ok((0, reply)),
            Err(err) => Ok((1, err.to_string())),
        }
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
    use crate::{services::plugin::PluginService, test::establish_connection};

    #[tokio::test]
    async fn it_works() {
        let conn = establish_connection();
        let plugin_service = PluginService::new(conn);
        let state = super::RunningPluginState::new(plugin_service);

        let binary =
            std::fs::read("../plugins/built/chat_wizard_plugin_commit_summary.wasm").unwrap();
        let plugin = super::RunningPlugin::init(&binary, state).await.unwrap();
        plugin.bindings.call_run(plugin.store).await.unwrap();
    }
}
