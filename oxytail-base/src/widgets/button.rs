use std::fmt::Display;

use floem::{style::Style, view::View, views::Decorators, widgets::button as upstreambutton};

use crate::GLOBAL_THEME;

use super::common_props::{OxySize, OxyVariant};

#[derive(Default, Clone, Copy)]
pub struct ButtonProps {
    pub variant: OxyVariant,
    pub outlined: bool,
    pub size: OxySize,
}

pub fn button<S: Display + 'static>(
    label: impl Fn() -> S + 'static,
    props: Option<ButtonProps>,
) -> impl View {
    let base_widget = upstreambutton(label);
    let theme = GLOBAL_THEME.get().unwrap();

    let props = props.unwrap_or(ButtonProps::default());

    let styles_enhancer = theme.get_button_style(props);
    let enhanced_style = styles_enhancer(Style::new());

    let styled_button = base_widget.style(move |_| enhanced_style.clone());

    styled_button
}
