use floem::{
    reactive::create_signal,
    view::View,
    views::{h_stack, label, v_stack, Decorators},
};
use oxytail_base::widgets::{
    common_props::{OxySize, OxyVariant},
    toggle::{toggle, ToggleProps},
};

fn toggle_with_state(props: Option<ToggleProps>) -> impl View {
    let (checked, set_checked) = create_signal(true);

    toggle(checked, props).on_click_stop(move |_| {
        set_checked.update(|checked| *checked = !*checked);
    })
}

pub fn toggle_sizes() -> impl View {
    v_stack((
        label(|| "Toggle sizes"),
        h_stack((
            toggle_with_state(Some(ToggleProps {
                size: OxySize::Large,
                ..Default::default()
            })),
            toggle_with_state(None),
            toggle_with_state(Some(ToggleProps {
                size: OxySize::Small,
                ..Default::default()
            })),
            toggle_with_state(Some(ToggleProps {
                size: OxySize::Tiny,
                ..Default::default()
            })),
        ))
        .style(|s| s.gap(4., 4.)),
    ))
}

pub fn toggle_variants() -> impl View {
    v_stack((
        label(|| "Toggle variants"),
        h_stack((
            toggle_with_state(None),
            toggle_with_state(Some(ToggleProps {
                variant: OxyVariant::Neutral,
                ..Default::default()
            })),
            toggle_with_state(Some(ToggleProps {
                variant: OxyVariant::Primary,
                ..Default::default()
            })),
            toggle_with_state(Some(ToggleProps {
                variant: OxyVariant::Secondary,
                ..Default::default()
            })),
            toggle_with_state(Some(ToggleProps {
                variant: OxyVariant::Accent,
                ..Default::default()
            })),
            toggle_with_state(Some(ToggleProps {
                variant: OxyVariant::Ghost,
                ..Default::default()
            })),
            toggle_with_state(Some(ToggleProps {
                variant: OxyVariant::Link,
                ..Default::default()
            })),
            toggle_with_state(Some(ToggleProps {
                variant: OxyVariant::Info,
                ..Default::default()
            })),
            toggle_with_state(Some(ToggleProps {
                variant: OxyVariant::Success,
                ..Default::default()
            })),
            toggle_with_state(Some(ToggleProps {
                variant: OxyVariant::Warning,
                ..Default::default()
            })),
            toggle_with_state(Some(ToggleProps {
                variant: OxyVariant::Error,
                ..Default::default()
            })),
        ))
        .style(|s| s.gap(4., 4.)),
    ))
}
