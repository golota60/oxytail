use std::fmt::Display;

use floem::{view::View, views::Decorators, widgets::button as upstreambutton};

use crate::get_current_theme;

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
    let theme = get_current_theme();

    let props = props.unwrap_or(ButtonProps::default());

    let styles_enhancer = theme.get_button_style(props);

    let styled_button = base_widget.style(move |s| styles_enhancer(s));

    styled_button
}
