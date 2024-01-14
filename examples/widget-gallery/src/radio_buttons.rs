use std::fmt::Display;

use floem::{
    reactive::create_signal,
    view::View,
    views::{h_stack, label, v_stack, Decorators},
};
use oxytail_base::widgets::{
    common_props::{OxySize, OxyVariant},
    radio_button::{labeled_radio_button, radio_button, RadioProps},
    text_divider::text_divider,
    text_header::text_header,
};

#[derive(PartialEq, Eq, Clone)]
enum RgbSpace {
    Red,
    Green,
    Blue,
}

impl Display for RgbSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            RgbSpace::Red => write!(f, "Red"),
            RgbSpace::Green => write!(f, "Green"),
            RgbSpace::Blue => write!(f, "Blue"),
        }
    }
}

fn labeled_radio_variant_with_state(variant: OxyVariant, size: OxySize) -> impl View {
    let (color, set_color) = create_signal(RgbSpace::Green);

    v_stack((
        labeled_radio_button(
            RgbSpace::Red,
            color,
            || RgbSpace::Red,
            Some(RadioProps {
                variant,
                size,
                ..Default::default()
            }),
        )
        .on_click_stop(move |_| {
            set_color.set(RgbSpace::Red);
        }),
        labeled_radio_button(
            RgbSpace::Green,
            color,
            || RgbSpace::Green,
            Some(RadioProps {
                variant,
                size,
                ..Default::default()
            }),
        )
        .on_click_stop(move |_| {
            set_color.set(RgbSpace::Green);
        }),
        labeled_radio_button(
            RgbSpace::Blue,
            color,
            || RgbSpace::Blue,
            Some(RadioProps {
                variant,
                size,
                ..Default::default()
            }),
        )
        .on_click_stop(move |_| {
            set_color.set(RgbSpace::Blue);
        }),
    ))
}

fn radio_variant_with_state(variant: OxyVariant, size: OxySize) -> impl View {
    let (color, set_color) = create_signal(RgbSpace::Green);

    v_stack((
        radio_button(
            RgbSpace::Red,
            color,
            Some(RadioProps {
                variant,
                size,
                ..Default::default()
            }),
        )
        .on_click_stop(move |_| {
            set_color.set(RgbSpace::Red);
        }),
        radio_button(
            RgbSpace::Green,
            color,
            Some(RadioProps {
                variant,
                size,
                ..Default::default()
            }),
        )
        .on_click_stop(move |_| {
            set_color.set(RgbSpace::Green);
        }),
        radio_button(
            RgbSpace::Blue,
            color,
            Some(RadioProps {
                variant,
                size,
                ..Default::default()
            }),
        )
        .on_click_stop(move |_| {
            set_color.set(RgbSpace::Blue);
        }),
    ))
}

pub fn radio_variants() -> impl View {
    v_stack((
        text_header("Radio button variants", None),
        text_divider(),
        h_stack((
            radio_variant_with_state(OxyVariant::Default, OxySize::Normal),
            radio_variant_with_state(OxyVariant::Neutral, OxySize::Normal),
            radio_variant_with_state(OxyVariant::Primary, OxySize::Normal),
            radio_variant_with_state(OxyVariant::Secondary, OxySize::Normal),
            radio_variant_with_state(OxyVariant::Accent, OxySize::Normal),
            radio_variant_with_state(OxyVariant::Ghost, OxySize::Normal),
            radio_variant_with_state(OxyVariant::Link, OxySize::Normal),
            radio_variant_with_state(OxyVariant::Info, OxySize::Normal),
            radio_variant_with_state(OxyVariant::Success, OxySize::Normal),
            radio_variant_with_state(OxyVariant::Warning, OxySize::Normal),
            radio_variant_with_state(OxyVariant::Error, OxySize::Normal),
        ))
        .style(|s| s.gap(10, 10)),
    ))
}
pub fn radio_sizes() -> impl View {
    v_stack((
        label(|| "Radio button sizes"),
        h_stack((
            radio_variant_with_state(OxyVariant::Primary, OxySize::Large),
            radio_variant_with_state(OxyVariant::Primary, OxySize::Normal),
            radio_variant_with_state(OxyVariant::Primary, OxySize::Small),
            radio_variant_with_state(OxyVariant::Primary, OxySize::Tiny),
        ))
        .style(|s| s.gap(10, 10)),
    ))
}
pub fn labeled_radio_variants() -> impl View {
    v_stack((
        text_header("Labeled radio buttons", None),
        text_divider(),
        h_stack((
            labeled_radio_variant_with_state(OxyVariant::Primary, OxySize::Normal),
            labeled_radio_variant_with_state(OxyVariant::Secondary, OxySize::Normal),
        ))
        .style(|s| s.gap(10, 10)),
    ))
}
pub fn disabled_labeled_radio_variants() -> impl View {
    v_stack((
        text_header("Disabled states", None),
        text_divider(),
        h_stack((
            labeled_radio_variant_with_state(OxyVariant::Primary, OxySize::Normal)
                .disabled(|| true),
            labeled_radio_variant_with_state(OxyVariant::Secondary, OxySize::Normal)
                .disabled(|| true),
        ))
        .style(|s| s.gap(10, 10)),
    ))
}
