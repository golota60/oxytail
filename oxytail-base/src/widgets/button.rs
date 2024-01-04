use std::fmt::Display;

use floem::{style::Style, view::View, views::Decorators, widgets::button as upstreambutton};

use crate::GLOBAL_THEME;

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
    let base_component = upstreambutton(label);
    let theme = GLOBAL_THEME.get().unwrap();

    let props = props.unwrap_or(ButtonProps::default());

    let base_styles_enhancer = theme.get_button_base_style(props.variant);
    let size_styles_enhancer = theme.get_button_size_style(props.size);

    let enhanced_style = base_styles_enhancer(Style::new());
    let enhanced_style = size_styles_enhancer(enhanced_style);

    let styled_button = base_component.style(move |_| enhanced_style.clone());

    styled_button
}
