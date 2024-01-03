use std::fmt::Display;

use floem::{
    peniko::Color, style::Style, view::View, views::Decorators, widgets::button as upstreambutton,
};

use crate::{themes::OxyButtonClass, GLOBAL_THEME};

#[derive(Default)]
pub enum ButtonSize {
    Large,
    #[default]
    Normal,
    Small,
    Tiny,
}

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
    let base_component = upstreambutton(label); //.class(OxyButtonClass);

    match props {
        Some(props) => base_component.style(
            GLOBAL_THEME
                .get()
                .unwrap()
                .get_button_base_style(props.variant),
        ),
        None => base_component,
    }
}
