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

impl Class {
    pub fn p(self, padding: TwUnit) -> Self {
        Self::new()
            .carbon(self.carbon.padding(padding))
            .dark(self.dark.padding(padding))
            .light(self.light.padding(padding))
            .monokai(self.monokai.padding(padding))
            .tokyo(self.tokyo.padding(padding))
            .dracula(self.dracula.padding(padding))
            .custom(self.custom.padding(padding))
            .neko(self.neko.padding(padding))
    }

    pub fn m(self, margin: TwUnit) -> Self {
        Self::new()
            .carbon(self.carbon.margin(margin))
            .dark(self.dark.margin(margin))
            .light(self.light.margin(margin))
            .monokai(self.monokai.margin(margin))
            .tokyo(self.tokyo.margin(margin))
            .dracula(self.dracula.margin(margin))
            .custom(self.custom.margin(margin))
            .neko(self.neko.margin(margin))
    }

    pub fn to_class(&self) -> String {
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
