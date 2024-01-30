use std::fmt::Display;

use floem::{
    style::FlexDirection,
    unit::PxPctAuto,
    view::View,
    views::{container, empty, h_stack, Decorators},
};

use crate::get_current_theme;

use super::common_props::{OxySize, OxyVariant};

#[derive(Default, Clone, Copy)]
pub struct ProgressProps {
    pub variant: OxyVariant,
    pub size: OxySize,
}

/// The progress element. You unfortunately, you need to define the width yourself for this one to work.
/// `value` should be a pixel/percentage value.
/// `max_value` should be the max_value.
/// value should be less or equal to max_value.
pub fn progress(
    value: impl Into<PxPctAuto> + Copy + 'static,
    max_value: impl Into<PxPctAuto> + Copy + 'static,
    props: Option<ProgressProps>,
) -> impl View {
    let props = props.unwrap_or(ProgressProps::default());

    let theme = get_current_theme();
    let (inner_enhancer, outer_enhancer, ball_enhancer) = theme.get_progress_style(props);

    let ball_widget = empty();
    let ball_widget = ball_widget.style(move |s| ball_enhancer(s));
    let inner_widget = container(ball_widget).style(move |s| {
        s.width(value)
            .max_width(value)
            .flex()
            .flex_direction(FlexDirection::RowReverse)
    });
    let inner_widget = inner_widget.style(move |s| inner_enhancer(s));
    let outline_widget = container(inner_widget).style(move |s| s.width(max_value));

    let styled_progress = outline_widget.style(move |s| outer_enhancer(s));

    styled_progress
}
