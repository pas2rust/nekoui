use glob::glob;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tailwind::components::prelude::*;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut class_list = Vec::new();

    for entry in glob("**/styles.rs").expect("Failed to read glob pattern") {
        if let Ok(path) = entry {
            if let Ok(content) = fs::read_to_string(&path) {
                for line in content.lines() {
                    if line.contains("TailwindStyles::new()") || line.contains(".") {
                        class_list.push(line.trim().to_string());
                    }
                }
            }
        }
    }

    let html = format!(
        r#"<div style="display:none" class="{}"></div>"#,
        class_list.join(" ")
    );

    let out_path = Path::new(&out_dir).join("tailwind_inject.html");
    let mut file = File::create(out_path).unwrap();
    file.write_all(html.as_bytes()).unwrap();
}
