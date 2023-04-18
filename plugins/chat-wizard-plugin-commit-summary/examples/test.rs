use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;

fn main() -> Result<()> {
    let engine = Engine::default();

    let mut linker = Linker::new(&engine);

    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();

    let mut store = Store::new(&engine, wasi);

    let module = Module::from_file(
        &engine,
        "target/wasm32-wasi/debug/chat-wizard-plugin-commit-summary.wasm",
    )?;
    linker.func_wrap("host", "foo", || {
        let output = std::process::Command::new("git")
            .current_dir("../../")
            .arg("diff")
            .output()
            .unwrap();
        let output = String::from_utf8_lossy(&output.stdout);

        println!("{}", output);
    })?;
    linker.module(&mut store, "", &module)?;

    linker
        .get_default(&mut store, "")?
        .typed::<(), ()>(&store)?
        .call(&mut store, ())?;

    Ok(())
}
