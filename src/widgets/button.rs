use std::fmt::Display;

use floem::{
    peniko::Color, style::Style, view::View, views::Decorators, widgets::button as upstreambutton,
};

use crate::themes::OxyButtonClass;

#[derive(Default)]
pub enum ButtonVariant {
    #[default]
    Default,

    Neutral,
    Primary,
    Secondary,
    Accent,
    Ghost,
    Link,

    Info,
    Success,
    Warning,
    Error,
}

impl ButtonVariant {
    fn get_style(&self) -> impl Fn(Style) -> Style {
        match self {
            ButtonVariant::Default => |s: Style| s.background(Color::rgb8(25, 30, 36)),

            ButtonVariant::Neutral => |s: Style| s.background(Color::rgb8(42, 50, 60)),
            ButtonVariant::Primary => |s: Style| s.background(Color::rgb8(116, 128, 255)),
            ButtonVariant::Secondary => |s: Style| s.background(Color::rgb8(255, 82, 217)),
            ButtonVariant::Accent => |s: Style| s.background(Color::rgb8(42, 50, 60)),
            ButtonVariant::Ghost => |s: Style| s.background(Color::rgb8(42, 50, 60)),
            ButtonVariant::Link => |s: Style| s.background(Color::rgb8(42, 50, 60)),

            ButtonVariant::Info => |s: Style| s.background(Color::rgb8(42, 50, 60)),
            ButtonVariant::Success => |s: Style| s.background(Color::rgb8(42, 50, 60)),
            ButtonVariant::Warning => |s: Style| s.background(Color::rgb8(42, 50, 60)),
            ButtonVariant::Error => |s: Style| s.background(Color::rgb8(42, 50, 60)),
        }
    }
}

#[derive(Default)]
pub enum ButtonSize {
    Large,
    #[default]
    Normal,
    Small,
    Tiny,
}

#[derive(Default)]
pub struct ButtonProps {
    pub variant: ButtonVariant,
    pub outlined: bool,
    pub size: ButtonSize,
}

pub fn button<S: Display + 'static>(
    label: impl Fn() -> S + 'static,
    props: Option<ButtonProps>,
) -> impl View {
    let base_component = upstreambutton(label).class(OxyButtonClass);

    match props {
        Some(props) => base_component.style(props.variant.get_style()),
        None => base_component,
    }
}
