use anyhow::Result;

#[link(wasm_import_module = "host")]
extern "C" {
    fn foo();
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    unsafe {
        run().await?;
    }

    Ok(())
}

async unsafe fn run() -> Result<()> {
    foo();
    Ok(())
}
