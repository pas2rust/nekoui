use super::tailwind::prelude::*;
use kenzu::Builder;
use leptos::prelude::{Get, Signal};

use crate::components::{prelude::use_memo, provider::tailwind::use_theme_context};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Theme {
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
}

impl Class {
    pub fn create(&self) -> Signal<String> {
        use_memo({
            let dark = self.dark.to_class();
            let light = self.light.to_class();
            let tokyo = self.tokyo.to_class();
            let monokai = self.monokai.to_class();
            let dracula = self.dracula.to_class();
            let custom = self.custom.to_class();
            let carbon = self.carbon.to_class();
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
