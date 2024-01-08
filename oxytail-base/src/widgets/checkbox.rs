use std::fmt::Display;

use floem::{
    reactive::ReadSignal,
    view::View,
    views::{self, h_stack, svg, Decorators},
};

use crate::get_current_theme;

use super::common_props::{OxySize, OxyVariant};

#[derive(Default, Clone, Copy)]
pub struct CheckboxProps {
    pub variant: OxyVariant,
    pub size: OxySize,
}

// checkbox_svg is not exported, so recreating labeled_checkbox is pretty raw
fn checkbox_svg(checked: ReadSignal<bool>, props: Option<CheckboxProps>) -> impl View {
    const CHECKBOX_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="-2 -2 16 16"><polygon points="5.19,11.83 0.18,7.44 1.82,5.56 4.81,8.17 10,1.25 12,2.75" /></svg>"#;
    let svg_str = move || if checked.get() { CHECKBOX_SVG } else { "" }.to_string();
    let base_widget = svg(svg_str);
    let theme = get_current_theme();

    let props = props.unwrap_or(CheckboxProps::default());

    let styles_enhancer = theme.get_checkbox_style(props);

    let styled_checkbox = base_widget.style(move |s| styles_enhancer(s));

    styled_checkbox
}

pub fn checkbox(checked: ReadSignal<bool>, props: Option<CheckboxProps>) -> impl View {
    checkbox_svg(checked, props).keyboard_navigatable()
}

pub fn labeled_checkbox<S: Display + 'static>(
    checked: ReadSignal<bool>,
    label: impl Fn() -> S + 'static,
    props: Option<CheckboxProps>,
) -> impl View {
    h_stack((checkbox_svg(checked, props), views::label(label)))
        .style(|s| s.items_center().justify_center())
        .keyboard_navigatable()
}
