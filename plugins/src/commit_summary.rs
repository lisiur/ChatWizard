wit_bindgen::generate!("host");

struct MyHost;

impl Host for MyHost {
    fn run() {
        exec("git diff");
    }
}

export_host!(MyHost);
