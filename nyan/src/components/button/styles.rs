use crate::components::prelude::*;

pub struct ButtonStyle;

impl ButtonStyle {
    pub fn neon_light() -> TailwindStyles {
        Self::neon()
            .bg_color(Color::Gray(Shade::FourHundred))
            .shadow_color(Color::Gray(Shade::FourHundred))
            .ring_color(Color::Gray(Shade::ThreeHundred))
            .focus(
                TailwindStyles::new()
                    .ring_size(TwUnit::Four)
                    .ring_color(Color::Gray(Shade::ThreeHundred))
                    .outline(Outline::None)
                    .to_box(),
            )
            .hover(
                TailwindStyles::new()
                    .scale(TwUnit::OneHundredTenRaw)
                    .bg_color(Color::Gray(Shade::SevenHundred))
                    .to_box(),
            )
    }

    pub fn neon_dark() -> TailwindStyles {
        Self::neon()
            .bg_color(Color::Slate(Shade::SixHundred))
            .shadow_color(Color::Slate(Shade::FiveHundred))
            .ring_color(Color::Slate(Shade::FiveHundred))
            .focus(
                TailwindStyles::new()
                    .ring_size(TwUnit::Four)
                    .ring_color(Color::Slate(Shade::ThreeHundred))
                    .outline(Outline::None)
                    .to_box(),
            )
            .hover(
                TailwindStyles::new()
                    .scale(TwUnit::OneHundredTenRaw)
                    .bg_color(Color::Slate(Shade::SevenHundred))
                    .to_box(),
            )
    }

    pub fn neon_monokai() -> TailwindStyles {
        Self::neon()
            .bg_color(Color::Fuchsia(Shade::FiveHundred))
            .shadow_color(Color::Fuchsia(Shade::FiveHundred))
            .ring_color(Color::Fuchsia(Shade::FourHundred))
            .focus(
                TailwindStyles::new()
                    .ring_size(TwUnit::Four)
                    .ring_color(Color::Fuchsia(Shade::ThreeHundred))
                    .outline(Outline::None)
                    .to_box(),
            )
            .hover(
                TailwindStyles::new()
                    .scale(TwUnit::OneHundredTenRaw)
                    .bg_color(Color::Fuchsia(Shade::SevenHundred))
                    .to_box(),
            )
    }

    pub fn neon_dracula() -> TailwindStyles {
        Self::neon()
            .bg_color(Color::Pink(Shade::FiveHundred))
            .shadow_color(Color::Pink(Shade::FiveHundred))
            .ring_color(Color::Pink(Shade::FourHundred))
            .focus(
                TailwindStyles::new()
                    .ring_size(TwUnit::Four)
                    .ring_color(Color::Pink(Shade::ThreeHundred))
                    .outline(Outline::None)
                    .to_box(),
            )
            .hover(
                TailwindStyles::new()
                    .scale(TwUnit::OneHundredTenRaw)
                    .bg_color(Color::Pink(Shade::SevenHundred))
                    .to_box(),
            )
    }

    pub fn neon_carbon() -> TailwindStyles {
        Self::neon()
            .bg_color(Color::Slate(Shade::EightHundred))
            .shadow_color(Color::Slate(Shade::FourHundred))
            .ring_color(Color::Slate(Shade::FourHundred))
            .focus(
                TailwindStyles::new()
                    .ring_size(TwUnit::Four)
                    .ring_color(Color::Slate(Shade::ThreeHundred))
                    .outline(Outline::None)
                    .to_box(),
            )
            .hover(
                TailwindStyles::new()
                    .scale(TwUnit::OneHundredTenRaw)
                    .bg_color(Color::Slate(Shade::NineHundredFifty))
                    .ring_color(Color::Slate(Shade::ThreeHundred))
                    .to_box(),
            )
    }

    pub fn neon_tokyo() -> TailwindStyles {
        Self::neon()
            .bg_color(Color::Purple(Shade::SixHundred))
            .shadow_color(Color::Purple(Shade::FiveHundred))
            .ring_color(Color::Purple(Shade::FourHundred))
            .focus(
                TailwindStyles::new()
                    .ring_size(TwUnit::Four)
                    .ring_color(Color::Purple(Shade::ThreeHundred))
                    .outline(Outline::None)
                    .to_box(),
            )
            .hover(
                TailwindStyles::new()
                    .scale(TwUnit::OneHundredTenRaw)
                    .bg_color(Color::Purple(Shade::SevenHundred))
                    .to_box(),
            )
    }

     pub fn neon_rust() -> TailwindStyles {
        Self::neon()
            .bg_color(Color::Orange(Shade::SixHundred))
            .shadow_color(Color::Orange(Shade::FiveHundred))
            .ring_color(Color::Orange(Shade::FourHundred))
            .focus(
                TailwindStyles::new()
                    .ring_size(TwUnit::Four)
                    .ring_color(Color::Orange(Shade::ThreeHundred))
                    .outline(Outline::None)
                    .to_box(),
            )
            .hover(
                TailwindStyles::new()
                    .scale(TwUnit::OneHundredTenRaw)
                    .bg_color(Color::Orange(Shade::SevenHundred))
                    .to_box(),
            )
    }

     pub fn neon_neko() -> TailwindStyles {
        Self::neon()
            .bg_color(Color::Indigo(Shade::SixHundred))
            .shadow_color(Color::Indigo(Shade::FiveHundred))
            .ring_color(Color::Indigo(Shade::FourHundred))
            .focus(
                TailwindStyles::new()
                    .ring_size(TwUnit::Four)
                    .ring_color(Color::Indigo(Shade::ThreeHundred))
                    .outline(Outline::None)
                    .to_box(),
            )
            .hover(
                TailwindStyles::new()
                    .scale(TwUnit::OneHundredTenRaw)
                    .bg_color(Color::Indigo(Shade::SevenHundred))
                    .to_box(),
            )
    }

    pub fn fill_light() -> TailwindStyles {
        Self::fill()
            .bg_color(Color::Gray(Shade::FourHundred))
            .hover(
                TailwindStyles::new()
                    .scale(TwUnit::OneHundredTenRaw)
                    .bg_color(Color::Gray(Shade::SevenHundred))
                    .to_box(),
            )
            .focus(
                TailwindStyles::new()
                    .ring_size(TwUnit::Four)
                    .ring_color(Color::Gray(Shade::ThreeHundred))
                    .outline(Outline::None)
                    .to_box(),
            )
    }

    pub fn fill_dark() -> TailwindStyles {
        Self::fill()
            .bg_color(Color::Slate(Shade::SixHundred))
            .hover(
                TailwindStyles::new()
                    .scale(TwUnit::OneHundredTenRaw)
                    .bg_color(Color::Slate(Shade::SevenHundred))
                    .to_box(),
            )
            .focus(
                TailwindStyles::new()
                    .ring_size(TwUnit::Four)
                    .ring_color(Color::Slate(Shade::ThreeHundred))
                    .outline(Outline::None)
                    .to_box(),
            )
    }

    pub fn fill_monokai() -> TailwindStyles {
        Self::fill()
            .bg_color(Color::Fuchsia(Shade::SixHundred))
            .hover(
                TailwindStyles::new()
                    .scale(TwUnit::OneHundredTenRaw)
                    .bg_color(Color::Fuchsia(Shade::SevenHundred))
                    .to_box(),
            )
            .focus(
                TailwindStyles::new()
                    .ring_size(TwUnit::Four)
                    .ring_color(Color::Fuchsia(Shade::ThreeHundred))
                    .outline(Outline::None)
                    .to_box(),
            )
    }

    pub fn fill_dracula() -> TailwindStyles {
        Self::fill()
            .bg_color(Color::Pink(Shade::FiveHundred))
            .hover(
                TailwindStyles::new()
                    .scale(TwUnit::OneHundredTenRaw)
                    .bg_color(Color::Pink(Shade::SevenHundred))
                    .to_box(),
            )
            .focus(
                TailwindStyles::new()
                    .ring_size(TwUnit::Four)
                    .ring_color(Color::Pink(Shade::ThreeHundred))
                    .outline(Outline::None)
                    .to_box(),
            )
    }

    pub fn fill_carbon() -> TailwindStyles {
        Self::fill()
            .bg_color(Color::Gray(Shade::EightHundred))
            .hover(
                TailwindStyles::new()
                    .scale(TwUnit::OneHundredTenRaw)
                    .bg_color(Color::Gray(Shade::NineHundredFifty))
                    .to_box(),
            )
            .focus(
                TailwindStyles::new()
                    .ring_size(TwUnit::Four)
                    .ring_color(Color::Gray(Shade::ThreeHundred))
                    .outline(Outline::None)
                    .to_box(),
            )
    }

    pub fn fill_tokyo() -> TailwindStyles {
        Self::fill()
            .bg_color(Color::Purple(Shade::FiveHundred))
            .hover(
                TailwindStyles::new()
                    .scale(TwUnit::OneHundredTenRaw)
                    .bg_color(Color::Purple(Shade::SevenHundred))
                    .to_box(),
            )
            .focus(
                TailwindStyles::new()
                    .ring_size(TwUnit::Four)
                    .ring_color(Color::Purple(Shade::SevenHundred))
                    .outline(Outline::None)
                    .to_box(),
            )
    }

    pub fn fill_neko() -> TailwindStyles {
        Self::fill()
            .bg_color(Color::Indigo(Shade::FiveHundred))
            .hover(
                TailwindStyles::new()
                    .scale(TwUnit::OneHundredTenRaw)
                    .bg_color(Color::Indigo(Shade::SevenHundred))
                    .to_box(),
            )
            .focus(
                TailwindStyles::new()
                    .ring_size(TwUnit::Four)
                    .ring_color(Color::Indigo(Shade::SevenHundred))
                    .outline(Outline::None)
                    .to_box(),
            )
    }

    pub fn fill_rust() -> TailwindStyles {
        Self::fill()
            .bg_color(Color::Orange(Shade::FiveHundred))
            .hover(
                TailwindStyles::new()
                    .scale(TwUnit::OneHundredTenRaw)
                    .bg_color(Color::Orange(Shade::SevenHundred))
                    .to_box(),
            )
            .focus(
                TailwindStyles::new()
                    .ring_size(TwUnit::Four)
                    .ring_color(Color::Orange(Shade::SevenHundred))
                    .outline(Outline::None)
                    .to_box(),
            )
    }

    pub fn outline_light() -> TailwindStyles {
        Self::outline()
            .ring_color(Color::Gray(Shade::FourHundred))
            .text_color(Color::Gray(Shade::FiveHundred))
            .hover(
                TailwindStyles::new()
                    .scale(TwUnit::OneHundredTenRaw)
                    .text_color(Color::Gray(Shade::OneHundred))
                    .bg_color(Color::Gray(Shade::FourHundred))
                    .to_box(),
            )
            .focus(
                TailwindStyles::new()
                    .ring_size(TwUnit::Four)
                    .ring_color(Color::Gray(Shade::SevenHundred))
                    .outline(Outline::None)
                    .to_box(),
            )
    }

    pub fn outline_dark() -> TailwindStyles {
        Self::outline()
            .ring_color(Color::Slate(Shade::FiveHundred))
            .text_color(Color::Slate(Shade::ThreeHundred))
            .hover(
                TailwindStyles::new()
                    .scale(TwUnit::OneHundredTenRaw)
                    .text_color(Color::Slate(Shade::OneHundred))
                    .bg_color(Color::Slate(Shade::FiveHundred))
                    .to_box(),
            )
            .focus(
                TailwindStyles::new()
                    .ring_size(TwUnit::Four)
                    .ring_color(Color::Slate(Shade::SevenHundred))
                    .outline(Outline::None)
                    .to_box(),
            )
    }

    pub fn outline_monokai() -> TailwindStyles {
        Self::outline()
            .ring_color(Color::Fuchsia(Shade::FiveHundred))
            .text_color(Color::Fuchsia(Shade::FourHundred))
            .hover(
                TailwindStyles::new()
                    .scale(TwUnit::OneHundredTenRaw)
                    .text_color(Color::Fuchsia(Shade::OneHundred))
                    .bg_color(Color::Fuchsia(Shade::FiveHundred))
                    .to_box(),
            )
            .focus(
                TailwindStyles::new()
                    .ring_size(TwUnit::Four)
                    .ring_color(Color::Fuchsia(Shade::SevenHundred))
                    .outline(Outline::None)
                    .to_box(),
            )
    }

    pub fn outline_dracula() -> TailwindStyles {
        Self::outline()
            .ring_color(Color::Pink(Shade::FiveHundred))
            .text_color(Color::Pink(Shade::FourHundred))
            .hover(
                TailwindStyles::new()
                    .scale(TwUnit::OneHundredTenRaw)
                    .text_color(Color::Pink(Shade::OneHundred))
                    .bg_color(Color::Pink(Shade::FiveHundred))
                    .to_box(),
            )
            .focus(
                TailwindStyles::new()
                    .ring_size(TwUnit::Four)
                    .ring_color(Color::Pink(Shade::SevenHundred))
                    .outline(Outline::None)
                    .to_box(),
            )
    }

    pub fn outline_carbon() -> TailwindStyles {
        Self::outline()
            .ring_color(Color::Gray(Shade::EightHundred))
            .text_color(Color::Gray(Shade::EightHundred))
            .hover(
                TailwindStyles::new()
                    .scale(TwUnit::OneHundredTenRaw)
                    .text_color(Color::Gray(Shade::OneHundred))
                    .bg_color(Color::Gray(Shade::EightHundred))
                    .to_box(),
            )
            .focus(
                TailwindStyles::new()
                    .ring_size(TwUnit::Four)
                    .ring_color(Color::Gray(Shade::SevenHundred))
                    .outline(Outline::None)
                    .to_box(),
            )
    }

    pub fn outline_tokyo() -> TailwindStyles {
        Self::outline()
            .ring_color(Color::Purple(Shade::FiveHundred))
            .text_color(Color::Purple(Shade::FourHundred))
            .hover(
                TailwindStyles::new()
                    .scale(TwUnit::OneHundredTenRaw)
                    .text_color(Color::Purple(Shade::OneHundred))
                    .bg_color(Color::Purple(Shade::FiveHundred))
                    .to_box(),
            )
            .focus(
                TailwindStyles::new()
                    .ring_size(TwUnit::Four)
                    .ring_color(Color::Purple(Shade::SevenHundred))
                    .outline(Outline::None)
                    .to_box(),
            )
    }

     pub fn outline_neko() -> TailwindStyles {
        Self::outline()
            .ring_color(Color::Indigo(Shade::FiveHundred))
            .text_color(Color::Indigo(Shade::FourHundred))
            .hover(
                TailwindStyles::new()
                    .scale(TwUnit::OneHundredTenRaw)
                    .text_color(Color::Indigo(Shade::OneHundred))
                    .bg_color(Color::Indigo(Shade::FiveHundred))
                    .to_box(),
            )
            .focus(
                TailwindStyles::new()
                    .ring_size(TwUnit::Four)
                    .ring_color(Color::Indigo(Shade::SevenHundred))
                    .outline(Outline::None)
                    .to_box(),
            )
    }

     pub fn outline_rust() -> TailwindStyles {
        Self::outline()
            .ring_color(Color::Orange(Shade::FiveHundred))
            .text_color(Color::Orange(Shade::FourHundred))
            .hover(
                TailwindStyles::new()
                    .scale(TwUnit::OneHundredTenRaw)
                    .text_color(Color::Orange(Shade::OneHundred))
                    .bg_color(Color::Orange(Shade::FiveHundred))
                    .to_box(),
            )
            .focus(
                TailwindStyles::new()
                    .ring_size(TwUnit::Four)
                    .ring_color(Color::Orange(Shade::SevenHundred))
                    .outline(Outline::None)
                    .to_box(),
            )
    }

    pub fn gradient_light() -> TailwindStyles {
        Self::gradient().gradient_colors(GradientColors::new(
            Color::Gray(Shade::FiveHundred),
            Color::Slate(Shade::FiveHundred),
            Color::Gray(Shade::FiveHundred),
        ))
    }

    pub fn gradient_dark() -> TailwindStyles {
        Self::gradient().gradient_colors(GradientColors::new(
            Color::Slate(Shade::SevenHundred),
            Color::Slate(Shade::SixHundred),
            Color::Slate(Shade::SevenHundred),
        ))
    }

    pub fn gradient_monokai() -> TailwindStyles {
        Self::gradient().gradient_colors(GradientColors::new(
            Color::Fuchsia(Shade::FourHundred),
            Color::Fuchsia(Shade::FiveHundred),
            Color::Fuchsia(Shade::SixHundred),
        ))
    }

    pub fn gradient_dracula() -> TailwindStyles {
        Self::gradient().gradient_colors(GradientColors::new(
            Color::Pink(Shade::FourHundred),
            Color::Pink(Shade::FiveHundred),
            Color::Pink(Shade::SixHundred),
        ))
    }

    pub fn gradient_carbon() -> TailwindStyles {
        Self::gradient().gradient_colors(GradientColors::new(
            Color::Gray(Shade::EightHundred),
            Color::Gray(Shade::SixHundred),
            Color::Gray(Shade::FourHundred),
        ))
    }

    pub fn gradient_tokyo() -> TailwindStyles {
        Self::gradient().gradient_colors(GradientColors::new(
            Color::Purple(Shade::FourHundred),
            Color::Purple(Shade::FiveHundred),
            Color::Purple(Shade::SixHundred),
        ))
    }

    pub fn gradient_neko() -> TailwindStyles {
        Self::gradient().gradient_colors(GradientColors::new(
            Color::Indigo(Shade::FourHundred),
            Color::Indigo(Shade::FiveHundred),
            Color::Indigo(Shade::SixHundred),
        ))
    }

    pub fn gradient_rust() -> TailwindStyles {
        Self::gradient().gradient_colors(GradientColors::new(
            Color::Orange(Shade::FourHundred),
            Color::Orange(Shade::FiveHundred),
            Color::Orange(Shade::SixHundred),
        ))
    }
}

impl ButtonStyle {
    pub fn neko_fill() -> Class {
        Class::new()
            .light(Self::fill_light())
            .dark(Self::fill_dark())
            .monokai(Self::fill_monokai())
            .dracula(Self::fill_dracula())
            .carbon(Self::fill_carbon())
            .tokyo(Self::fill_tokyo())
            .rust(Self::fill_rust())
            .neko(Self::fill_neko())
    }

    pub fn neko_outline() -> Class {
        Class::new()
            .light(Self::outline_light())
            .dark(Self::outline_dark())
            .monokai(Self::outline_monokai())
            .dracula(Self::outline_dracula())
            .carbon(Self::outline_carbon())
            .tokyo(Self::outline_tokyo())
            .rust(Self::outline_rust())
            .neko(Self::outline_neko())
    }

    pub fn neko_gradient() -> Class {
        Class::new()
            .light(Self::gradient_light())
            .dark(Self::gradient_dark())
            .monokai(Self::gradient_monokai())
            .dracula(Self::gradient_dracula())
            .carbon(Self::gradient_carbon())
            .tokyo(Self::gradient_tokyo())
            .rust(Self::gradient_rust())
            .neko(Self::gradient_neko())
    }

    pub fn neko_neon() -> Class {
        Class::new()
            .light(Self::neon_light())
            .dark(Self::neon_dark())
            .monokai(Self::neon_monokai())
            .dracula(Self::neon_dracula())
            .carbon(Self::neon_carbon())
            .tokyo(Self::neon_tokyo())
            .rust(Self::neon_rust())
            .neko(Self::neon_neko())
    }
}

impl ButtonStyle {
    pub fn fill() -> TailwindStyles {
        TailwindStyles::new()
            .tracking(Tracking::Widest)
            .font_weight(FontWeight::Black)
            .text_align(TextAlign::Center)
            .align_items(AlignItems::Center)
            .margin(TwUnit::Three)
            .cursor(Cursor::Pointer)
            .display(Display::Flex)
            .scale(TwUnit::OneHundredRaw)
            .px(TwUnit::Eight)
            .py(TwUnit::TwoHalf)
            .rounded(TwUnit::Full)
            .bg_color(Color::Purple(Shade::FiveHundred))
            .transition(Transition::All)
            .focus(
                TailwindStyles::new()
                    .ring_size(TwUnit::Four)
                    .ring_color(Color::Purple(Shade::ThreeHundred))
                    .outline(Outline::None)
                    .to_box(),
            )
            .hover(
                TailwindStyles::new()
                    .scale(TwUnit::OneHundredTenRaw)
                    .bg_color(Color::Purple(Shade::SevenHundred))
                    .to_box(),
            )
    }

    pub fn outline() -> TailwindStyles {
        Self::fill()
            .text_color(Color::Purple(Shade::FiveHundred))
            .bg_color(Color::Transparent)
            .ring_size(TwUnit::Four)
            .ring_color(Color::Purple(Shade::FiveHundred))
            .hover(
                TailwindStyles::new()
                    .scale(TwUnit::OneHundredTenRaw)
                    .transition(Transition::All)
                    .text_color(Color::White)
                    .bg_color(Color::Purple(Shade::FiveHundred))
                    .to_box(),
            )
    }

    pub fn neon() -> TailwindStyles {
        Self::fill()
            .shadow_color(Color::Purple(Shade::FiveHundred))
            .shadow_size(TwUnit::Xl)
            .ring_size(TwUnit::Four)
            .ring_color(Color::Purple(Shade::FiveHundred))
    }

    pub fn gradient() -> TailwindStyles {
        Self::fill()
            .bg_color(Color::Inherit)
            .gradient_direction(GradientDirection::ToR)
            .gradient_colors(GradientColors::new(
                Color::Purple(Shade::FiveHundred),
                Color::Fuchsia(Shade::FiveHundred),
                Color::Pink(Shade::FiveHundred),
            ))
    }

    pub fn xs() -> TailwindStyles {
        TailwindStyles::new()
            .margin(TwUnit::Three)
            .px(TwUnit::Two)
            .py(TwUnit::TwoHalf)
    }

    pub fn sm() -> TailwindStyles {
        TailwindStyles::new()
            .margin(TwUnit::Three)
            .px(TwUnit::Four)
            .py(TwUnit::TwoHalf)
    }

    pub fn md() -> TailwindStyles {
        TailwindStyles::new()
            .margin(TwUnit::Three)
            .px(TwUnit::Eight)
            .py(TwUnit::TwoHalf)
    }

    pub fn lg() -> TailwindStyles {
        TailwindStyles::new()
            .margin(TwUnit::Three)
            .px(TwUnit::Ten)
            .py(TwUnit::TwoHalf)
    }

    pub fn xl() -> TailwindStyles {
        TailwindStyles::new()
            .margin(TwUnit::Three)
            .px(TwUnit::Twelve)
            .py(TwUnit::TwoHalf)
    }

    pub fn xxl() -> TailwindStyles {
        TailwindStyles::new()
            .margin(TwUnit::Three)
            .px(TwUnit::ThirtyTwo)
            .py(TwUnit::TwoHalf)
    }
}

impl BuildClass for ButtonStyle {
    fn build() -> Vec<Box<dyn ToClass>> {
        vec![
            Self::neko_fill().to_box(),
            Self::neko_outline().to_box(),
            Self::neko_gradient().to_box(),
            Self::neko_neon().to_box(),
            Self::xs().to_box(),
            Self::sm().to_box(),
            Self::md().to_box(),
            Self::lg().to_box(),
            Self::xl().to_box(),
            Self::xxl().to_box(),
        ]
    }
}
