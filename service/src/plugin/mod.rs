use crate::result::Result;
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};

bindgen!();

#[derive(Default)]
pub struct RunningPluginState {}

impl RunningPluginState {}

impl Host_Imports for RunningPluginState {
    fn exec(&mut self, cmd: String) -> wasmtime::Result<(i32, String)> {
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
}

pub struct RunningPlugin {
    store: Store<RunningPluginState>,
    bindings: Host_,
}

impl RunningPlugin {
    fn init(code: &[u8], state: RunningPluginState) -> Result<Self> {
        let mut config = Config::new();
        config.wasm_component_model(true);
        let engine = Engine::new(&config)?;
        let component = Component::from_binary(&engine, code)?;

        let mut linker = Linker::new(&engine);
        Host_::add_to_linker(&mut linker, |state: &mut RunningPluginState| state)?;

        let mut store = Store::new(&engine, state);

        let (bindings, _) = Host_::instantiate(&mut store, &component, &linker)?;

        Ok(RunningPlugin { bindings, store })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let binary = std::fs::read("../plugins/test.wasm").unwrap();
        let state = super::RunningPluginState::default();
        let plugin = super::RunningPlugin::init(&binary, state).unwrap();
        plugin.bindings.call_run(plugin.store).unwrap();
    }
}
