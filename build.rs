use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get the output directory
    let out_dir = env::var("OUT_DIR").unwrap();
    
    // Create templates directory in the output directory
    let templates_dir = Path::new(&out_dir).join("templates");
    fs::create_dir_all(&templates_dir).unwrap();
    
    // Copy email templates
    let source_dir = "src/features/mail/templates";
    for entry in fs::read_dir(source_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap();
            fs::copy(&path, templates_dir.join(file_name)).unwrap();
        }
    }
    
    // Tell cargo to rerun this if the templates change
    println!("cargo:rerun-if-changed=src/features/mail/templates");
} 