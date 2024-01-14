use floem::{
    reactive::create_signal,
    view::View,
    views::{h_stack, v_stack, Decorators},
};
use oxytail_base::widgets::{
    checkbox::{checkbox, labeled_checkbox, CheckboxProps},
    common_props::{OxySize, OxyVariant},
    text_divider::text_divider,
    text_header::text_header,
};

fn checkbox_with_state(props: Option<CheckboxProps>) -> impl View {
    let (checked, set_checked) = create_signal(true);

    checkbox(checked, props).on_click_stop(move |_| {
        set_checked.update(|checked| *checked = !*checked);
    })
}

pub fn checkboxes_sizes() -> impl View {
    v_stack((
        text_header("Checkbox sizes", None),
        text_divider(),
        h_stack((
            checkbox_with_state(Some(CheckboxProps {
                size: OxySize::Large,
                ..Default::default()
            })),
            checkbox_with_state(None),
            checkbox_with_state(Some(CheckboxProps {
                size: OxySize::Small,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                size: OxySize::Tiny,
                ..Default::default()
            })),
        ))
        .style(|s| s.gap(4., 4.)),
    ))
}

pub fn checkboxes_variants() -> impl View {
    v_stack((
        text_header("Checkbox variants", None),
        text_divider(),
        h_stack((
            checkbox_with_state(None),
            checkbox_with_state(Some(CheckboxProps {
                variant: OxyVariant::Neutral,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                variant: OxyVariant::Primary,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                variant: OxyVariant::Secondary,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                variant: OxyVariant::Accent,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                variant: OxyVariant::Ghost,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                variant: OxyVariant::Link,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                variant: OxyVariant::Info,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                variant: OxyVariant::Success,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                variant: OxyVariant::Warning,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                variant: OxyVariant::Error,
                ..Default::default()
            })),
        ))
        .style(|s| s.gap(4., 4.)),
    ))
}

pub fn labeled_checkboxes() -> impl View {
    let (checked, set_checked) = create_signal(true);
    v_stack((
        text_header("Labeled checkboxes", None),
        text_divider(),
        labeled_checkbox(checked, || "I am the default!", None).on_click_stop(move |_| {
            set_checked.update(|checked| *checked = !*checked);
        }),
        labeled_checkbox(
            checked,
            || "I am primary!",
            Some(CheckboxProps {
                variant: OxyVariant::Primary,
                ..Default::default()
            }),
        )
        .on_click_stop(move |_| {
            set_checked.update(|checked| *checked = !*checked);
        }),
    ))
}
