use std::env;
use winresource::WindowsResource;


fn main() {
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        WindowsResource::new()
            .set_icon("assets/escoba.ico")
            .compile().unwrap();
    }
}
