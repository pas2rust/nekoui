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
        format!("{dark}{light}{tokyo}{carbon}{dracula}{monokai}{custom}{neko}")
    }
}

impl Class {
    pub fn to_box(self) -> Box<Self> {
        Box::new(self)
    }
    pub fn apply(self, variant: TailwindStyles) -> Self {
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
            md,
            lg,
            sm,
            xxl,
            xl,
            ..
        } = variant;

        Self::new()
            .carbon(
                self.carbon
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
                    .xxl(xxl.clone()),
            )
            .dark(
                self.dark
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
                    .xxl(xxl.clone()),
            )
            .light(
                self.light
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
                    .xxl(xxl.clone()),
            )
            .monokai(
                self.monokai
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
                    .xxl(xxl.clone()),
            )
            .tokyo(
                self.tokyo
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
                    .xxl(xxl.clone()),
            )
            .dracula(
                self.dracula
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
                    .xxl(xxl.clone()),
            )
            .custom(
                self.custom
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
                    .xxl(xxl.clone()),
            )
            .neko(
                self.neko
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
                    .sm(sm)
                    .md(md)
                    .lg(lg)
                    .xl(xl)
                    .xxl(xxl),
            )
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
