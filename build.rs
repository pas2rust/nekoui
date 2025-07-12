use nyan::components::prelude::*;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() {
    let styles = vec![
        AvatarContainerStyle::new(),
        AvatarStyle::xs(),
        AvatarStyle::sm(),
        AvatarStyle::md(),
        AvatarStyle::lg(),
        AvatarStyle::xl(),
        AvatarStyle::xxl(),
    ];

    let mut class_set = HashSet::new();
    for style in styles {
        let class_string = style.to_class();
        for class in class_string.split_whitespace() {
            class_set.insert(class.to_string());
        }
    }

    let mut class_list: Vec<_> = class_set.into_iter().collect();
    class_list.sort();
    let class = class_list.join(" ");
    //println!("cargo:warning=✅ Tailwind classes generated: {class}");

    let index_path = Path::new("index.html");
    let mut content = fs::read_to_string(index_path).expect("Failed to read index.html");

    let div_regex = Regex::new(r#"<div\s+id="tailwind-safe-list"[^>]*>"#).unwrap();
    let new_div = format!(r#"<div id="tailwind-safe-list" class="{class}">"#);

    if div_regex.is_match(&content) {
        content = div_regex.replace(&content, new_div).to_string();
    } else {
        content = content.replace("</body>", &format!("  {new_div}</div>\n</body>"));
    }

    fs::write(index_path, content).expect("Failed to write updated index.html");
}
