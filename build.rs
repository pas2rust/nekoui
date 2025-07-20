use nyan::components::prelude::*;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let dev_mode = cfg!(debug_assertions);
    let index_path = Path::new("index.html");

    let default_html = |div: String| {
        format!(
            r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <link rel="stylesheet" href="public/tailwind.css">
        <link href="public/favicon.ico" rel="icon" type="image/x-icon" />
        <link data-trunk href="public/" rel="copy-dir" />
        <title>NekoノUI・Nyan</title>
    </head>
    <body class="bg-slate-950 text-white">
        {div}
    </body>
</html>"#
        )
    };
    if dev_mode {
        let styles = ButtonStyle::build().into_iter().chain(AvatarStyle::build());

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
        println!("cargo:warning=✅ Tailwind classes generated: {class}");
        fs::write(
            index_path,
            default_html(format!(
                r#"<div id="tailwind-safe-list" style="display:none" class="{class}"></div>"#
            )),
        )
        .expect("Failed to write default index.html");
        let status = Command::new("tailwindcss")
            .args(&[
                "-i",
                "./main.css",
                "-o",
                "./public/tailwind.css",
                "--minify",
            ])
            .status()
            .expect("failed to execute tailwindcss");

        if status.success() {
            println!("cargo:warning=✅ Tailwind CSS generated successfully.");
        } else {
            eprintln!("cargo:warning=❌ Failed to generate Tailwind CSS.");
        }
    } else {
        fs::write(index_path, default_html("".into())).expect("Failed to write default index.html");
    }
}
