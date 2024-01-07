use floem::{
    view::View,
    views::{label, stack, v_stack, Decorators},
};
use oxytail_base::widgets::{
    button::{button, ButtonProps},
    common_props::{OxySize, OxyVariant},
};

pub fn btn_variants() -> impl View {
    v_stack((
        label(|| "Button variants"),
        stack((
            button(|| "Default", None),
            button(
                || "Neutral",
                Some(ButtonProps {
                    variant: OxyVariant::Neutral,
                    ..Default::default()
                }),
            ),
            button(
                || "Primary",
                Some(ButtonProps {
                    variant: OxyVariant::Primary,
                    ..Default::default()
                }),
            ),
            button(
                || "Secondary",
                Some(ButtonProps {
                    variant: OxyVariant::Secondary,
                    ..Default::default()
                }),
            ),
            button(
                || "Accent",
                Some(ButtonProps {
                    variant: OxyVariant::Accent,
                    ..Default::default()
                }),
            ),
            button(
                || "Ghost",
                Some(ButtonProps {
                    variant: OxyVariant::Ghost,
                    ..Default::default()
                }),
            ),
            button(
                || "Link",
                Some(ButtonProps {
                    variant: OxyVariant::Link,
                    ..Default::default()
                }),
            ),
            button(
                || "Info",
                Some(ButtonProps {
                    variant: OxyVariant::Info,
                    ..Default::default()
                }),
            ),
            button(
                || "Success",
                Some(ButtonProps {
                    variant: OxyVariant::Success,
                    ..Default::default()
                }),
            ),
            button(
                || "Warning",
                Some(ButtonProps {
                    variant: OxyVariant::Warning,
                    ..Default::default()
                }),
            ),
            button(
                || "Error",
                Some(ButtonProps {
                    variant: OxyVariant::Error,
                    ..Default::default()
                }),
            ),
        ))
        .style(|s| s.gap(4., 4.)),
    ))
}

pub fn btn_sizes() -> impl View {
    v_stack((
        label(|| "Button sizes"),
        stack((
            button(
                || "Large",
                Some(ButtonProps {
                    size: OxySize::Large,
                    ..Default::default()
                }),
            ),
            button(
                || "Normal",
                Some(ButtonProps {
                    size: OxySize::Normal,
                    ..Default::default()
                }),
            ),
            button(
                || "Small",
                Some(ButtonProps {
                    size: OxySize::Small,
                    ..Default::default()
                }),
            ),
            button(
                || "Tiny",
                Some(ButtonProps {
                    size: OxySize::Tiny,
                    ..Default::default()
                }),
            ),
        ))
        .style(|s| s.gap(4., 4.)),
    ))
}

pub fn btn_outlines() -> impl View {
    v_stack((
        label(|| "Outlined buttons"),
        stack((
            button(
                || "Info",
                Some(ButtonProps {
                    variant: OxyVariant::Info,
                    outlined: true,
                    ..Default::default()
                }),
            ),
            button(
                || "Success",
                Some(ButtonProps {
                    variant: OxyVariant::Success,
                    outlined: true,
                    ..Default::default()
                }),
            ),
            button(
                || "Warning",
                Some(ButtonProps {
                    variant: OxyVariant::Warning,
                    outlined: true,
                    ..Default::default()
                }),
            ),
            button(
                || "Error",
                Some(ButtonProps {
                    variant: OxyVariant::Error,
                    outlined: true,
                    ..Default::default()
                }),
            ),
        ))
        .style(|s| s.gap(4., 4.)),
    ))
}
