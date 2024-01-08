use floem::{
    reactive::{create_signal, ReadSignal},
    view::View,
    views::{container, dyn_container, svg, Decorators},
};

use crate::get_current_theme;

use super::common_props::{OxySize, OxyVariant};

#[derive(Default, Clone, Copy)]
pub struct ToggleProps {
    pub variant: OxyVariant,
    pub outlined: bool,
    pub size: OxySize,
}

fn toggle_ball_svg(enabled: ReadSignal<bool>, props: Option<ToggleProps>) -> impl View {
    const BALL_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg">
        <circle cx="50" cy="50" r="50" />
     </svg>"#;
    let svg_str = move || if enabled.get() { BALL_SVG } else { BALL_SVG }.to_string();

    let base_widget = svg(svg_str);
    let theme = get_current_theme();
    let props = props.unwrap_or(ToggleProps::default());

    let styles_enhancer = theme.get_toggle_ball_style(props, enabled.get());

    let styled_toggle = base_widget.style(move |s| styles_enhancer(s));

    styled_toggle
}

pub fn toggle(enabled: ReadSignal<bool>, props: Option<ToggleProps>) -> impl View {
    // We need to wrap the toggle in a dynamic container, just so it can re-render on `enabled` change.
    let base_widget = dyn_container(
        move || enabled.get(),
        move |en| {
            let base_widget = container(toggle_ball_svg(create_signal(en).0, props));

            let base_widget = base_widget.keyboard_navigatable();
            let theme = get_current_theme();
            let props = props.unwrap_or(ToggleProps::default());

            let styles_enhancer = theme.get_toggle_border_style(props, en);

            let styled_widget = base_widget.style(move |s| styles_enhancer(s));

            Box::new(styled_widget)
        },
    );

    base_widget
}
