use std::env;
use std::fs::File;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../target/debug/print_schema");
    let schema_file = AsRef::<Path>::as_ref(&env::var("OUT_DIR").unwrap()).join("server.json");
    let schema_file = File::create(schema_file).unwrap();
    Command::new("../target/debug/print_schema")
        .current_dir("../server")
        .stdout(schema_file)
        .status()
        .unwrap();
}
