use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::str::FromStr;

fn main() {
    let out_dir = OsString::from_str("./src/hello").unwrap();
    let dest_path = Path::new(&out_dir).join("hello.rs");
    fs::write(
        &dest_path,
        "pub fn message() -> &'static str { \"Hello, World!\" }",
    )
    .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}