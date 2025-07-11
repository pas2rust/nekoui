use crate::components::_shared::prelude::*;

pub struct ButtonStyle;

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
            .scale(TwUnit::OneHundred)
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
                    .scale(TwUnit::OneHundredTen)
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
            .hover(TailwindStyles::new().opacity(TwUnit::SeventyTwo).to_box())
    }

    pub fn glass() -> TailwindStyles {
        Self::fill()
            .bg_color(Color::Purple(Shade::FiveHundred))
            .backdrop_blur(TwUnit::ThreeXl)
            .shadow_size(TwUnit::Lg)
            .shadow_color(Color::Black)
            .bg_opacity(TwUnit::Forty)
    }
}
