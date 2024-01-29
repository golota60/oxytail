use floem::{
    style::Style,
    view::View,
    views::{h_stack, Decorators},
};
use oxytail_base::widgets::{
    badge::{badge, BadgeProps},
    common_props::{OxySize, OxyVariant},
};

fn variant_badge(name: &'static str, variant: OxyVariant) -> impl View {
    badge(
        name,
        Some(BadgeProps {
            variant,
            ..Default::default()
        }),
    )
}

pub fn badges_variants() -> impl View {
    h_stack((
        variant_badge("default", OxyVariant::Default),
        variant_badge("neutral", OxyVariant::Neutral),
        variant_badge("primary", OxyVariant::Primary),
        variant_badge("secondary", OxyVariant::Secondary),
        variant_badge("accent", OxyVariant::Accent),
        variant_badge("ghost", OxyVariant::Ghost),
        variant_badge("link", OxyVariant::Link),
        variant_badge("info", OxyVariant::Info),
        variant_badge("success", OxyVariant::Success),
        variant_badge("warning", OxyVariant::Warning),
        variant_badge("error", OxyVariant::Error),
    ))
    .style(|s: Style| s.gap(4., 4.))
}

fn size_badge(name: &'static str, size: OxySize) -> impl View {
    badge(
        name,
        Some(BadgeProps {
            size,
            ..Default::default()
        }),
    )
}

pub fn badges_sizes() -> impl View {
    h_stack((
        size_badge("large", OxySize::Large),
        size_badge("normal", OxySize::Normal),
        size_badge("small", OxySize::Small),
        size_badge("tiny", OxySize::Tiny),
    ))
    .style(|s: Style| s.gap(4., 4.))
}

pub fn badges_outlines() -> impl View {
    h_stack((
        badge(
            "outlined default",
            Some(BadgeProps {
                outlined: true,
                ..Default::default()
            }),
        ),
        badge(
            "outlined primary",
            Some(BadgeProps {
                outlined: true,
                variant: OxyVariant::Primary,
                ..Default::default()
            }),
        ),
        badge(
            "outlined secondary",
            Some(BadgeProps {
                outlined: true,
                variant: OxyVariant::Secondary,
                ..Default::default()
            }),
        ),
    ))
    .style(|s: Style| s.gap(4., 4.))
}
