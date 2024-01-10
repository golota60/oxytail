use floem::{
    reactive::create_rw_signal,
    view::View,
    views::{h_stack, label, v_stack, Decorators},
};
use oxytail_base::widgets::{
    common_props::{OxySize, OxyVariant},
    text_input::{text_input, InputProps},
};

pub fn text_input_variants() -> impl View {
    let default_text = create_rw_signal(String::from("I am default!"));
    let primary_text = create_rw_signal(String::from("I am primary!"));
    let secondary_text = create_rw_signal(String::from("I am secondary!"));

    v_stack((
        label(|| "Text input variants(same as buttons, only a few shown)"),
        h_stack((
            text_input(default_text, None),
            text_input(
                primary_text,
                Some(InputProps {
                    variant: OxyVariant::Primary,
                    ..Default::default()
                }),
            ),
            text_input(
                secondary_text,
                Some(InputProps {
                    variant: OxyVariant::Secondary,
                    ..Default::default()
                }),
            ),
        ))
        .style(|s| s.gap(5., 5.)),
    ))
    .style(|s| s.justify_start().items_start().gap(5., 5.))
}

pub fn text_input_sizes() -> impl View {
    let large_text = create_rw_signal(String::from("I am large!"));
    let normal_text = create_rw_signal(String::from("I am normal!"));
    let small_text = create_rw_signal(String::from("I am small!"));
    let tiny_text = create_rw_signal(String::from("I am tiny!"));

    v_stack((
        label(|| "Text input sizes"),
        h_stack((
            text_input(
                large_text,
                Some(InputProps {
                    size: OxySize::Large,
                    ..Default::default()
                }),
            ),
            text_input(normal_text, None),
            text_input(
                small_text,
                Some(InputProps {
                    size: OxySize::Small,
                    ..Default::default()
                }),
            ),
            text_input(
                tiny_text,
                Some(InputProps {
                    size: OxySize::Tiny,
                    ..Default::default()
                }),
            ),
        ))
        .style(|s| s.gap(5., 5.)),
    ))
}
