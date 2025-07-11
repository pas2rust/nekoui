use super::prelude::*;
use kenzu::Builder;

#[derive(Builder, Default, Debug, Clone)]
pub struct TailwindStyles {
    pub text_size: TwUnit,
    pub max_width: TwUnit,
    pub min_width: TwUnit,
    pub rounded: TwUnit,
    pub border_size: TwUnit,
    pub padding: TwUnit,
    pub px: TwUnit,
    pub py: TwUnit,
    pub margin: TwUnit,
    pub duration: TwUnit,
    pub gap: TwUnit,
    pub space_x: TwUnit,
    pub outline: Outline,
    pub space_y: TwUnit,
    pub animation: Animation,
    pub top: TwUnit,
    pub bg_opacity: TwUnit,
    pub bottom: TwUnit,
    pub left: TwUnit,
    pub right: TwUnit,
    pub tracking: Tracking,
    pub z_index: TwUnit,
    pub inset: TwUnit,
    pub border_color: Color,
    pub width: TwUnit,
    pub height: TwUnit,
    pub font_weight: FontWeight,
    pub overflow: Overflow,
    pub bg_color: Color,
    pub text_color: Color,
    pub ring_size: TwUnit,
    pub ring_color: Color,
    pub shadow_color: Color,
    pub fill: Color,
    pub shadow_size: TwUnit,
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub flex_grow: FlexGrow,
    pub object_fit: ObjectFit,
    pub flex_shrink: FlexShrink,
    pub scale: TwUnit,
    pub position: Position,
    pub display: Display,
    pub align_items: AlignItems,
    pub text_align: TextAlign,
    pub cursor: Cursor,
    pub backdrop_blur: TwUnit,
    pub text_transform: TextTransform,
    pub text_decoration: TextDecoration,
    pub font_style: FontStyle,
    pub translate_x: TwUnit,
    pub translate_y: TwUnit,
    pub opacity: TwUnit,
    pub justify_content: JustifyContent,
    pub rounded_tl: TwUnit,
    pub gradient_colors: GradientColors,
    pub gradient_direction: GradientDirection,
    pub rounded_tr: TwUnit,
    pub rounded_bl: TwUnit,
    pub rounded_br: TwUnit,
    pub transition: Transition,
    pub hover: Option<Box<Self>>,
    pub focus: Option<Box<Self>>,
    pub active: Option<Box<Self>>,
    pub disabled: Option<Box<Self>>,
    pub visited: Option<Box<Self>>,
    pub checked: Option<Box<Self>>,
    pub group_hover: Option<Box<Self>>,
    pub group_focus: Option<Box<Self>>,
    pub peer_hover: Option<Box<Self>>,
    pub peer_checked: Option<Box<Self>>,
    pub sm: Option<Box<Self>>,
    pub md: Option<Box<Self>>,
    pub lg: Option<Box<Self>>,
    pub xl: Option<Box<Self>>,
    pub x2l: Option<Box<Self>>,
}

macro_rules! push_classes {
    ($self:ident, $classes:expr,
     $( ($field:ident, $default:expr, $prefix:expr) ),* $(,)? ) => {
        $(
            Self::push_enum(&$self.$field, $default, $prefix, $classes);
        )*
    };
}

macro_rules! push_variants {
    ($self:ident, $classes:expr,
     $( ($prefix:expr, $field:ident) ),* $(,)? ) => {
        $(
            $self.push_variant($prefix, &$self.$field, $classes);
        )*
    };
}

impl TailwindStyles {
    pub fn to_class(&self) -> String {
        let mut classes = vec![];

        push_classes!(
            self,
            &mut classes,
            (animation, Animation::Inherit, "animate"),
            (object_fit, ObjectFit::Inherit, "object-fit"),
            (duration, TwUnit::Inherit, "duration"),
            (backdrop_blur, TwUnit::Inherit, "backdrop-blur"),
            (gradient_colors, GradientColors::Inherit, ""),
            (
                gradient_direction,
                GradientDirection::Inherit,
                "bg-gradient"
            ),
            (tracking, Tracking::Inherit, "tracking"),
            (display, Display::Inherit, ""),
            (text_transform, TextTransform::Inherit, "text"),
            (text_decoration, TextDecoration::Inherit, "text"),
            (outline, Outline::Inherit, "outline"),
            (font_style, FontStyle::Inherit, ""),
            (scale, TwUnit::Inherit, "scale"),
            (text_align, TextAlign::Inherit, "text"),
            (font_weight, FontWeight::Inherit, "font"),
            (cursor, Cursor::Inherit, "cursor"),
            (ring_size, TwUnit::Inherit, "ring"),
            (opacity, TwUnit::Inherit, "opacity"),
            (bg_opacity, TwUnit::Inherit, "bg-opacity"),
            (transition, Transition::Inherit, "transition"),
            (shadow_color, Color::Inherit, "shadow"),
            (justify_content, JustifyContent::Inherit, "justify"),
            (shadow_size, TwUnit::Inherit, "shadow"),
            (translate_x, TwUnit::Inherit, "translate-x"),
            (translate_y, TwUnit::Inherit, "translate-y"),
            (text_size, TwUnit::Inherit, "text"),
            (max_width, TwUnit::Inherit, "max-w"),
            (min_width, TwUnit::Inherit, "min-w"),
            (position, Position::Inherit, ""),
            (margin, TwUnit::Inherit, "m"),
            (padding, TwUnit::Inherit, "p"),
            (gap, TwUnit::Inherit, "gap"),
            (rounded, TwUnit::Inherit, "rounded"),
            (width, TwUnit::Inherit, "w"),
            (height, TwUnit::Inherit, "h"),
            (overflow, Overflow::Inherit, "overflow"),
            (border_size, TwUnit::Inherit, "border"),
            (border_color, Color::Inherit, "border"),
            (bg_color, Color::Inherit, "bg"),
            (fill, Color::Inherit, "fill"),
            (text_color, Color::Inherit, "text"),
            (ring_color, Color::Inherit, "ring"),
            (space_x, TwUnit::Inherit, "space-x"),
            (space_y, TwUnit::Inherit, "space-y"),
            (px, TwUnit::Inherit, "px"),
            (py, TwUnit::Inherit, "py"),
            (top, TwUnit::Inherit, "top"),
            (bottom, TwUnit::Inherit, "bottom"),
            (left, TwUnit::Inherit, "left"),
            (right, TwUnit::Inherit, "right"),
            (z_index, TwUnit::Inherit, "z"),
            (inset, TwUnit::Inherit, "inset"),
            (flex_direction, FlexDirection::Inherit, "flex"),
            (flex_wrap, FlexWrap::Inherit, "flex"),
            (flex_grow, FlexGrow::Inherit, "grow"),
            (flex_shrink, FlexShrink::Inherit, "shrink"),
            (align_items, AlignItems::Inherit, "items"),
        );

        push_variants!(
            self,
            &mut classes,
            ("hover", hover),
            ("focus", focus),
            ("active", active),
            ("disabled", disabled),
            ("visited", visited),
            ("checked", checked),
            ("group-hover", group_hover),
            ("group-focus", group_focus),
            ("peer-hover", peer_hover),
            ("peer-checked", peer_checked),
            ("sm", sm),
            ("md", md),
            ("lg", lg),
            ("xl", xl),
            ("2xl", x2l),
        );
        classes.join(" ")
    }

    fn push_variant(&self, prefix: &str, variant: &Option<Box<Self>>, classes: &mut Vec<String>) {
        if let Some(v) = variant {
            for class in v.to_class().split_whitespace() {
                classes.push(format!("{prefix}:{class}"));
            }
        }
    }

    fn push_enum<T: PartialEq>(value: &T, none_value: T, prefix: &str, classes: &mut Vec<String>)
    where
        T: ToStr,
    {
        if value != &none_value {
            let suffix = value.to_str();
            let class = if prefix.is_empty() {
                suffix.to_string()
            } else {
                format!("{prefix}-{suffix}")
            };
            classes.push(class);
        }
    }

    pub fn to_box(self) -> Box<Self> {
        Box::new(self)
    }
}
