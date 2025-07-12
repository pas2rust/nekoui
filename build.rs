use nyan::components::prelude::*;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

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
        let styles = vec![
            AvatarContainerStyle::new(),
            AvatarStyle::xs(),
            AvatarStyle::sm(),
            AvatarStyle::md(),
            AvatarStyle::lg(),
            AvatarStyle::xl(),
            AvatarStyle::xxl(),
            AvatarDotStyle::xs(),
            AvatarDotStyle::sm(),
            AvatarDotStyle::md(),
            AvatarDotStyle::lg(),
            AvatarDotStyle::xl(),
            AvatarDotStyle::xxl(),
            ButtonStyle::fill(),
            ButtonStyle::neon(),
            ButtonStyle::glass(),
            ButtonStyle::gradient(),
            ButtonStyle::outline(),
            ToggleContainerStyle::new(),
            ToggleStyle::xs_checked(),
            ToggleStyle::xs_unchecked(),
            ToggleStyle::sm_checked(),
            ToggleStyle::sm_unchecked(),
            ToggleStyle::md_checked(),
            ToggleStyle::md_unchecked(),
            ToggleStyle::lg_checked(),
            ToggleStyle::lg_unchecked(),
            ToggleStyle::xl_checked(),
            ToggleStyle::xl_unchecked(),
            ToggleStyle::x2l_checked(),
            ToggleStyle::x2l_unchecked(),
            ToggleThumbStyle::xs_checked(),
            ToggleThumbStyle::xs_unchecked(),
            ToggleThumbStyle::sm_checked(),
            ToggleThumbStyle::sm_unchecked(),
            ToggleThumbStyle::md_checked(),
            ToggleThumbStyle::md_unchecked(),
            ToggleThumbStyle::lg_checked(),
            ToggleThumbStyle::lg_unchecked(),
            ToggleThumbStyle::xl_checked(),
            ToggleThumbStyle::xl_unchecked(),
            ToggleThumbStyle::x2l_checked(),
            ToggleThumbStyle::x2l_unchecked(),
            SpinnerStyle::xs(),
            SpinnerStyle::sm(),
            SpinnerStyle::md(),
            SpinnerStyle::lg(),
            SpinnerStyle::xl(),
            ChartAreaStyle::container(),
            ChartAreaStyle::card(),
            ChartCanvasStyle::normal(),
            ChartCanvasStyle::tall(),
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
        println!("cargo:warning=✅ Tailwind classes generated: {class}");
        fs::write(
            index_path,
            default_html(format!(
                r#"<div id="tailwind-safe-list" style="display:none" class="{class}"></div>"#
            )),
        )
        .expect("Failed to write default index.html");
    } else {
        fs::write(index_path, default_html("".into())).expect("Failed to write default index.html");
    }
}
