use std::fmt::Display;

use floem::{style::Style, view::View, views::Decorators, widgets::button as upstreambutton};

use crate::GLOBAL_THEME;

#[derive(Default, Clone, Copy)]
pub enum ButtonSize {
    Large,
    #[default]
    Normal,
    Small,
    Tiny,
}

#[derive(Default, Clone, Copy)]
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

#[derive(Default, Clone, Copy)]
pub struct ButtonProps {
    pub variant: ButtonVariant,
    pub outlined: bool,
    pub size: ButtonSize,
}

pub fn button<S: Display + 'static>(
    label: impl Fn() -> S + 'static,
    props: Option<ButtonProps>,
) -> impl View {
    let base_component = upstreambutton(label);
    let theme = GLOBAL_THEME.get().unwrap();

    let props = props.unwrap_or(ButtonProps::default());

    let styles_enhancer = theme.get_button_style(props);
    let enhanced_style = styles_enhancer(Style::new());

    let styled_button = base_component.style(move |_| enhanced_style.clone());

    styled_button
}
