use floem::{
    reactive::ReadSignal,
    view::View,
    views::{svg, Decorators},
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
    const OFF_SVG: &str = r#"<svg viewBox="0 0 24 12" xmlns="http://www.w3.org/2000/svg" fill="none">
    <rect x="0.5" y="0.5" width="23" height="11" rx="6" stroke="currentColor" stroke-width="1" />
    <circle cx="6" cy="6" r="4.5" fill="currentColor"/>
    </svg>
    "#;
    const ON_SVG: &str = r#"<svg viewBox="0 0 24 12" xmlns="http://www.w3.org/2000/svg" fill="none">
    <rect x="0.5" y="0.5" width="23" height="11" rx="6" stroke="currentColor" stroke-width="1" />
    <circle cx="18" cy="6" r="4.5" fill="currentColor"/>
    </svg>
    "#;

    let svg_str = move || if enabled.get() { ON_SVG } else { OFF_SVG }.to_string();

    let base_widget = svg(svg_str);
    let theme = get_current_theme();
    let props = props.unwrap_or(ToggleProps::default());

    let styles_enhancer = theme.get_toggle_style(props);

    let styled_toggle = base_widget.style(move |s| styles_enhancer(s));

    styled_toggle
}

pub fn toggle(enabled: ReadSignal<bool>, props: Option<ToggleProps>) -> impl View {
    toggle_ball_svg(enabled, props)
}
