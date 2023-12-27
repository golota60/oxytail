use std::fmt::Display;

use floem::{
    reactive::ReadSignal,
    view::View,
    views::{self, h_stack, svg, Decorators},
    widgets::checkbox as upstreamcheckbox,
};

use crate::themes::{OxyCheckboxClass, OxyLabeledCheckboxClass};

// checkbox_svg is not exported, so recreating labeled_checkbox is pretty raw

fn checkbox_svg(checked: ReadSignal<bool>) -> impl View {
    const CHECKBOX_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="-2 -2 16 16"><polygon points="5.19,11.83 0.18,7.44 1.82,5.56 4.81,8.17 10,1.25 12,2.75" /></svg>"#;
    let svg_str = move || if checked.get() { CHECKBOX_SVG } else { "" }.to_string();
    svg(svg_str).class(OxyCheckboxClass)
}

pub fn checkbox(checked: ReadSignal<bool>) -> impl View {
    upstreamcheckbox(checked).class(OxyCheckboxClass)
}

pub fn labeled_checkbox<S: Display + 'static>(
    checked: ReadSignal<bool>,
    label: impl Fn() -> S + 'static,
) -> impl View {
    h_stack((checkbox_svg(checked), views::label(label)))
        .class(OxyLabeledCheckboxClass)
        .base_style(|s| s.items_center().justify_center())
        .keyboard_navigatable()
}
