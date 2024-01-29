use floem::{
    view::View,
    views::{label as upstreamlabel, Decorators},
};

use crate::get_current_theme;

use super::common_props::{OxySize, OxyVariant};

#[derive(Default, Copy, Clone)]
pub struct BadgeProps {
    pub variant: OxyVariant,
    pub size: OxySize,
    pub outlined: bool,
}

pub fn badge(label: &'static str, props: Option<BadgeProps>) -> impl View {
    let base_widget = upstreamlabel(move || label);
    let theme = get_current_theme();

    let props = props.unwrap_or(BadgeProps::default());
    let styles_enhancer = theme.get_badge_style(props);

    let styled_badge = base_widget.style(move |s| styles_enhancer(s));

    styled_badge
}
