use floem::{
    view::View,
    views::{self, Decorators},
};

use crate::get_current_theme;

use super::common_props::OxySize;

#[derive(Default, Copy, Clone)]
pub struct HeaderProps {
    pub size: OxySize,
}

pub fn text_header(label: &str, props: Option<HeaderProps>) -> impl View {
    let base_widget = views::text(label);
    let theme = get_current_theme();

    let props = props.unwrap_or(HeaderProps::default());

    let styles_enhancer = theme.get_header_style(props);

    let styled_button = base_widget.style(move |s| styles_enhancer(s));

    styled_button
}
