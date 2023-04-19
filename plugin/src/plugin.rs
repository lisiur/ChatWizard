use std::process::exit;

use wasmtime::*;

pub struct Plugin {
    code: Vec<u8>,
}

impl Plugin {
    pub fn new(code: Vec<u8>) -> Self {
        Self { code }
    }

    pub fn activate(&self) -> anyhow::Result<RunningPlugin> {
        RunningPlugin::init(self)
    }
}

pub struct RunningPlugin {
    store: Store<()>,
}

impl RunningPlugin {
    pub fn init(plugin: &Plugin) -> anyhow::Result<Self> {
        let mut store = Store::default();

        let exec_command = Func::wrap(
            &mut store,
            |mut caller: Caller<'_, ()>, cmd_prt: i32, cmd_len: i32| {
                let Some(Extern::Memory(mem)) = caller.get_export("memory") else {
                    exit(1);
                };

                let data = mem
                    .data(&caller)
                    .get(cmd_prt as usize..)
                    .and_then(|arr| arr.get(..cmd_len as usize))
                    .unwrap();

                let cmd = String::from_utf8_lossy(data);
                let cmd_and_args = cmd.split_whitespace().collect::<Vec<_>>();
                let cmd = cmd_and_args[0];
                let args = &cmd_and_args[1..];

                let output = std::process::Command::new(cmd)
                    .current_dir(".")
                    .args(args)
                    .output()
                    .unwrap();
                let output = String::from_utf8_lossy(&output.stdout);

                println!("{}", output);
            },
        );

        let module = Module::from_binary(store.engine(), &plugin.code)?;

        let instance = Instance::new(&mut store, &module, &[exec_command.into()])?;

        let run = instance.get_typed_func::<(), ()>(&mut store, "run")?;

        run.call(&mut store, ())?;

        Ok(Self { store })
    }
}
