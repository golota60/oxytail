use std::fmt::Display;

use floem::{
    reactive::ReadSignal,
    style::Style,
    view::View,
    views::{self, container, empty, h_stack, Decorators},
};

use crate::get_current_theme;

use super::common_props::{OxySize, OxyVariant};

#[derive(Default, Clone, Copy)]
pub struct RadioProps {
    pub variant: OxyVariant,
    pub outlined: bool,
    pub size: OxySize,
}

fn radio_button_svg<T>(
    represented_value: T,
    actual_value: ReadSignal<T>,
    inner_style_enhancer: Box<dyn Fn(Style) -> Style>,
    outer_style_enhancer: Box<dyn Fn(Style) -> Style>,
) -> impl View
where
    T: Eq + PartialEq + Clone + 'static,
{
    container(empty().style(move |s| {
        inner_style_enhancer(s).apply_if(actual_value.get() != represented_value, |s| {
            s.display(floem::style::Display::None)
        })
    }))
    .style(move |s| outer_style_enhancer(s))
}

fn upstream_radio_button<T>(
    represented_value: T,
    actual_value: ReadSignal<T>,
    inner_style_enhancer: Box<dyn Fn(Style) -> Style>,
    outer_style_enhancer: Box<dyn Fn(Style) -> Style>,
) -> impl View
where
    T: Eq + PartialEq + Clone + 'static,
{
    radio_button_svg(
        represented_value,
        actual_value,
        inner_style_enhancer,
        outer_style_enhancer,
    )
    .keyboard_navigatable()
}

pub fn radio_button<T>(
    represented_value: T,
    actual_value: ReadSignal<T>,
    props: Option<RadioProps>,
) -> impl View
where
    T: Eq + PartialEq + Clone + 'static,
{
    let theme = get_current_theme();

    let props = props.unwrap_or(RadioProps::default());

    let (inner_style, outer_style) = theme.get_radio_style(props);

    let base_widget =
        upstream_radio_button(represented_value, actual_value, inner_style, outer_style);

    base_widget
}

pub fn labeled_radio_button<T, S: Display + 'static>(
    represented_value: T,
    actual_value: ReadSignal<T>,
    label: impl Fn() -> S + 'static,
    props: Option<RadioProps>,
) -> impl View
where
    T: Eq + PartialEq + Clone + 'static,
{
    h_stack((
        radio_button(represented_value, actual_value, props),
        views::label(label),
    ))
    .style(|s| s.gap(8, 0))
}
