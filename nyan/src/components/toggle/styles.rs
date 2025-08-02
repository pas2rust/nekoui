use crate::components::prelude::*;

pub struct ToggleContainerStyle;
pub struct ToggleButtonStyle;
pub struct ToggleThumbStyle;

impl ToggleContainerStyle {
    pub fn new() -> TailwindStyles {
        TailwindStyles::new()
            .align_items(AlignItems::Center)
            .cursor(Cursor::Pointer)
            .display(Display::InlineFlex)
            .margin(TwUnit::Four)
    }
}

impl ToggleButtonStyle {
    pub fn base() -> TailwindStyles {
        TailwindStyles::new()
            .position(Position::Relative)
            .rounded(TwUnit::Full)
            .transition(Transition::All)
            .width(TwUnit::Twelve)
            .height(TwUnit::Seven)
            .padding(TwUnit::One)
    }

    pub fn checked() -> Class {
        Class::new()
            .light(
                Self::checked_base()
                    .ring_color(Color::Gray(Shade::ThreeHundred))
                    .bg_color(Color::Gray(Shade::FiveHundred)),
            )
            .dark(
                Self::checked_base()
                    .ring_color(Color::Slate(Shade::ThreeHundred))
                    .bg_color(Color::Slate(Shade::FiveHundred)),
            )
            .dracula(
                Self::checked_base()
                    .ring_color(Color::Pink(Shade::ThreeHundred))
                    .bg_color(Color::Pink(Shade::FiveHundred)),
            )
            .carbon(
                Self::checked_base()
                    .ring_color(Color::Slate(Shade::ThreeHundred))
                    .bg_color(Color::Slate(Shade::EightHundred)),
            )
            .monokai(
                Self::checked_base()
                    .ring_color(Color::Fuchsia(Shade::ThreeHundred))
                    .bg_color(Color::Fuchsia(Shade::FiveHundred)),
            )
            .neko(
                Self::checked_base()
                    .ring_color(Color::Indigo(Shade::ThreeHundred))
                    .bg_color(Color::Indigo(Shade::FiveHundred)),
            )
            .rust(
                Self::checked_base()
                    .ring_color(Color::Orange(Shade::ThreeHundred))
                    .bg_color(Color::Orange(Shade::FiveHundred)),
            )
            .tokyo(
                Self::checked_base()
                    .ring_color(Color::Purple(Shade::ThreeHundred))
                    .bg_color(Color::Purple(Shade::FiveHundred)),
            )
    }

    pub fn unchecked() -> Class {
        Class::new()
            .light(Self::unchecked_base())
            .dark(Self::unchecked_base())
            .dracula(Self::unchecked_base())
            .carbon(Self::unchecked_base())
            .monokai(Self::unchecked_base())
            .neko(Self::unchecked_base())
            .rust(Self::unchecked_base())
            .tokyo(Self::unchecked_base())
    }

    pub fn checked_base() -> TailwindStyles {
        Self::base()
            .ring_size(TwUnit::Four)
            .ring_color(Color::Blue(Shade::ThreeHundred))
            .bg_color(Color::Blue(Shade::SixHundred))
    }

    pub fn unchecked_base() -> TailwindStyles {
        Self::base()
            .ring_size(TwUnit::Inherit)
            .ring_color(Color::Transparent)
            .bg_color(Color::Gray(Shade::ThreeHundred))
    }

    pub fn xs_checked() -> TailwindStyles {
        TailwindStyles::new()
            .width(TwUnit::Seven)
            .height(TwUnit::Four)
    }

    pub fn xs_unchecked() -> TailwindStyles {
        TailwindStyles::new()
            .width(TwUnit::Seven)
            .height(TwUnit::Four)
    }

    pub fn sm_checked() -> TailwindStyles {
        TailwindStyles::new()
            .width(TwUnit::Nine)
            .height(TwUnit::Five)
    }

    pub fn sm_unchecked() -> TailwindStyles {
        TailwindStyles::new()
            .width(TwUnit::Nine)
            .height(TwUnit::Five)
    }

    pub fn md_checked() -> TailwindStyles {
        TailwindStyles::new()
            .width(TwUnit::Twelve)
            .height(TwUnit::Five)
    }

    pub fn md_unchecked() -> TailwindStyles {
        TailwindStyles::new()
            .width(TwUnit::Twelve)
            .height(TwUnit::Five)
    }

    pub fn lg_checked() -> TailwindStyles {
        TailwindStyles::new()
            .width(TwUnit::Fourteen)
            .height(TwUnit::Eight)
    }

    pub fn lg_unchecked() -> TailwindStyles {
        TailwindStyles::new()
            .width(TwUnit::Fourteen)
            .height(TwUnit::Eight)
    }

    pub fn xl_checked() -> TailwindStyles {
        TailwindStyles::new()
            .width(TwUnit::Sixteen)
            .height(TwUnit::Nine)
    }

    pub fn xl_unchecked() -> TailwindStyles {
        TailwindStyles::new()
            .width(TwUnit::Sixteen)
            .height(TwUnit::Nine)
    }

    pub fn xxl_checked() -> TailwindStyles {
        TailwindStyles::new()
            .width(TwUnit::Twenty)
            .height(TwUnit::Eleven)
    }

    pub fn xxl_unchecked() -> TailwindStyles {
        TailwindStyles::new()
            .width(TwUnit::Twenty)
            .height(TwUnit::Eleven)
    }
}

impl ToggleThumbStyle {
    pub fn base() -> TailwindStyles {
        TailwindStyles::new()
            .top(TwUnit::ZeroPointFive)
            .bg_color(Color::White)
            .rounded(TwUnit::Full)
            .transition(Transition::Transform)
    }

    pub fn xs_checked() -> TailwindStyles {
        Self::base()
            .width(TwUnit::Four)
            .height(TwUnit::Four)
            .top(TwUnit::Zero)
            .left(TwUnit::Zero)
            .translate_x(TwUnit::Three)
    }

    pub fn xs_unchecked() -> TailwindStyles {
        Self::base()
            .top(TwUnit::Zero)
            .left(TwUnit::Zero)
            .width(TwUnit::Four)
            .height(TwUnit::Four)
    }

    pub fn sm_checked() -> TailwindStyles {
        Self::base()
            .width(TwUnit::Four)
            .height(TwUnit::Four)
            .translate_x(TwUnit::Four)
    }

    pub fn sm_unchecked() -> TailwindStyles {
        Self::base().width(TwUnit::Four).height(TwUnit::Four)
    }

    pub fn md_checked() -> TailwindStyles {
        Self::base()
            .width(TwUnit::Five)
            .height(TwUnit::Five)
            .translate_x(TwUnit::Five)
    }

    pub fn md_unchecked() -> TailwindStyles {
        Self::base().width(TwUnit::Five).height(TwUnit::Five)
    }

    pub fn lg_checked() -> TailwindStyles {
        Self::base()
            .width(TwUnit::Seven)
            .height(TwUnit::Seven)
            .translate_x(TwUnit::Seven)
    }

    pub fn lg_unchecked() -> TailwindStyles {
        Self::base().width(TwUnit::Seven).height(TwUnit::Seven)
    }

    pub fn xl_checked() -> TailwindStyles {
        Self::base()
            .width(TwUnit::Eight)
            .height(TwUnit::Eight)
            .translate_x(TwUnit::Eight)
    }

    pub fn xl_unchecked() -> TailwindStyles {
        Self::base().width(TwUnit::Eight).height(TwUnit::Eight)
    }

    pub fn xxl_checked() -> TailwindStyles {
        Self::base()
            .width(TwUnit::Ten)
            .height(TwUnit::Ten)
            .translate_x(TwUnit::Ten)
    }

    pub fn xxl_unchecked() -> TailwindStyles {
        Self::base().width(TwUnit::Ten).height(TwUnit::Ten)
    }
}

impl BuildClass for ToggleButtonStyle {
    fn build() -> Vec<Box<dyn ToClass>> {
        vec![
            ToggleButtonStyle::checked().to_box(),
            ToggleButtonStyle::unchecked().to_box(),
            ToggleThumbStyle::xs_checked().to_box(),
            ToggleThumbStyle::xs_unchecked().to_box(),
            ToggleThumbStyle::sm_checked().to_box(),
            ToggleThumbStyle::sm_unchecked().to_box(),
            ToggleThumbStyle::md_checked().to_box(),
            ToggleThumbStyle::md_unchecked().to_box(),
            ToggleThumbStyle::lg_checked().to_box(),
            ToggleThumbStyle::lg_unchecked().to_box(),
            ToggleThumbStyle::xl_checked().to_box(),
            ToggleThumbStyle::xl_unchecked().to_box(),
            ToggleThumbStyle::xxl_checked().to_box(),
            ToggleThumbStyle::xxl_unchecked().to_box(),
            ToggleContainerStyle::new().to_box(),
        ]
    }
}
