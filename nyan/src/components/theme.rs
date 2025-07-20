use super::tailwind::prelude::*;
use kenzu::Builder;
use leptos::prelude::{Get, Signal};

use crate::components::{prelude::use_memo, provider::tailwind::use_theme_context};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    Neko,
    Dark,
    #[default]
    Light,
    Tokyo,
    Monokai,
    Dracula,
    Custom,
    Carbon,
    Rust,
}

#[derive(Builder, Default, Debug, Clone)]
pub struct Class {
    pub dark: TailwindStyles,
    pub light: TailwindStyles,
    pub monokai: TailwindStyles,
    pub tokyo: TailwindStyles,
    pub dracula: TailwindStyles,
    pub custom: TailwindStyles,
    pub carbon: TailwindStyles,
    pub neko: TailwindStyles,
    pub rust: TailwindStyles,
}

impl ToClass for Class {
    fn to_class(&self) -> String {
        let dark = self.dark.to_class();
        let light = self.light.to_class();
        let tokyo = self.tokyo.to_class();
        let monokai = self.monokai.to_class();
        let dracula = self.dracula.to_class();
        let custom = self.custom.to_class();
        let carbon = self.carbon.to_class();
        let neko = self.neko.to_class();
        let rust = self.rust.to_class();
        format!(" {dark} {light} {tokyo} {carbon} {dracula} {monokai} {custom} {neko} {rust} ")
    }
}

macro_rules! apply_variant {
    ($self:ident, $theme:ident, $variant:expr) => {{
        let TailwindStyles {
            padding,
            margin,
            px,
            py,
            pb,
            pt,
            pr,
            pl,
            width,
            height,
            mb,
            mt,
            mr,
            ml,
            left,
            ref md,
            ref lg,
            ref sm,
            ref xxl,
            ref xl,
            ..
        } = $variant;

        $self
            .$theme
            .padding(padding)
            .margin(margin)
            .px(px)
            .py(py)
            .width(width)
            .height(height)
            .pb(pb)
            .pl(pl)
            .pt(pt)
            .pr(pr)
            .mb(mb)
            .mt(mt)
            .mr(mr)
            .ml(ml)
            .sm(sm.clone())
            .md(md.clone())
            .lg(lg.clone())
            .xl(xl.clone())
            .xxl(xxl.clone())
            .left(left)
    }};
}

impl Class {
    pub fn to_box(self) -> Box<Self> {
        Box::new(self)
    }
    pub fn apply(self, variant: TailwindStyles) -> Self {
        Self::new()
            .carbon(apply_variant!(self, carbon, variant))
            .dark(apply_variant!(self, dark, variant))
            .light(apply_variant!(self, light, variant))
            .monokai(apply_variant!(self, monokai, variant))
            .tokyo(apply_variant!(self, tokyo, variant))
            .dracula(apply_variant!(self, dracula, variant))
            .custom(apply_variant!(self, custom, variant))
            .neko(apply_variant!(self, neko, variant))
            .rust(apply_variant!(self, rust, variant))
    }
    pub fn create(&self) -> Signal<String> {
        use_memo({
            let dark = self.dark.to_class();
            let light = self.light.to_class();
            let tokyo = self.tokyo.to_class();
            let monokai = self.monokai.to_class();
            let dracula = self.dracula.to_class();
            let custom = self.custom.to_class();
            let carbon = self.carbon.to_class();
            let rust = self.rust.to_class();
            let neko = self.neko.to_class();
            move || {
                let default_theme = |th: &String| -> String {
                    if th.is_empty() {
                        light.clone()
                    } else {
                        th.clone()
                    }
                };
                let theme = use_theme_context();
                match theme.get() {
                    Theme::Neko => default_theme(&neko),
                    Theme::Rust => default_theme(&rust),
                    Theme::Light => light.clone(),
                    Theme::Dark => default_theme(&dark),
                    Theme::Dracula => default_theme(&dracula),
                    Theme::Custom => default_theme(&custom),
                    Theme::Tokyo => default_theme(&tokyo),
                    Theme::Monokai => default_theme(&monokai),
                    Theme::Carbon => default_theme(&carbon),
                }
            }
        })
    }
}

impl Theme {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::Neko => "neko",
            Self::Dark => "dark",
            Self::Light => "light",
            Self::Tokyo => "tokyo",
            Self::Monokai => "monokai",
            Self::Dracula => "dracula",
            Self::Custom => "custom",
            Self::Carbon => "carbon",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "rust" => Self::Rust,
            "neko" => Self::Neko,
            "dark" => Self::Dark,
            "light" => Self::Light,
            "tokyo" => Self::Tokyo,
            "monokai" => Self::Monokai,
            "dracula" => Self::Dracula,
            "custom" => Self::Custom,
            "carbon" => Self::Carbon,
            _ => Self::Light,
        }
    }
}
