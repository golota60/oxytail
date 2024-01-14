use floem::{
    style::Style,
    view::View,
    views::{label, static_label, v_stack, Decorators},
};
use oxytail_base::widgets::{
    button::button,
    common_props::{OxySize, OxyVariant},
    tooltip::{tooltip, TooltipProps},
};

pub fn button_tooltips() -> impl View {
    tooltip(
        button(|| "I'm a button(hover over me!)", None),
        || static_label("And I'm the button tooltip!"),
        None,
    )
}

pub fn label_tooltips() -> impl View {
    v_stack((
        tooltip(
            label(|| "Default label tooltip(hover on me!)"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                variant: OxyVariant::Default,
                ..Default::default()
            }),
        ),
        tooltip(
            label(|| "Neutral label tooltip(hover on me!)"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                variant: OxyVariant::Neutral,
                ..Default::default()
            }),
        ),
        tooltip(
            label(|| "Primary label tooltip(hover on me!)"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                variant: OxyVariant::Primary,
                ..Default::default()
            }),
        ),
        tooltip(
            label(|| "Secondary label tooltip(hover on me!)"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                variant: OxyVariant::Secondary,
                ..Default::default()
            }),
        ),
        tooltip(
            label(|| "Accent label tooltip(hover on me!)"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                variant: OxyVariant::Accent,
                ..Default::default()
            }),
        ),
        tooltip(
            label(|| "Ghost label tooltip(hover on me!)"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                variant: OxyVariant::Ghost,
                ..Default::default()
            }),
        ),
        tooltip(
            label(|| "Link label tooltip(hover on me!)"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                variant: OxyVariant::Link,
                ..Default::default()
            }),
        ),
        tooltip(
            label(|| "Info label tooltip(hover on me!)"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                variant: OxyVariant::Info,
                ..Default::default()
            }),
        ),
        tooltip(
            label(|| "Success label tooltip(hover on me!)"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                variant: OxyVariant::Success,
                ..Default::default()
            }),
        ),
        tooltip(
            label(|| "Warning label tooltip(hover on me!)"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                variant: OxyVariant::Warning,
                ..Default::default()
            }),
        ),
        tooltip(
            label(|| "Error label tooltip(hover on me!)"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                variant: OxyVariant::Error,
                ..Default::default()
            }),
        ),
    ))
    .style(|s: Style| s.gap(10., 10.))
}

pub fn sizes_tooltips() -> impl View {
    v_stack((
        tooltip(
            label(|| "Large tooltip"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                size: OxySize::Large,
                ..Default::default()
            }),
        ),
        tooltip(
            label(|| "Normal tooltip"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                size: OxySize::Normal,
                ..Default::default()
            }),
        ),
        tooltip(
            label(|| "Small tooltip"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                size: OxySize::Small,
                ..Default::default()
            }),
        ),
        tooltip(
            label(|| "Tiny tooltip"),
            || static_label("And I'm the label tooltip!"),
            Some(TooltipProps {
                size: OxySize::Tiny,
                ..Default::default()
            }),
        ),
    ))
    .style(|s| s.gap(10., 10.))
}
