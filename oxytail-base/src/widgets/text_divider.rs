use floem::{
    view::View,
    views::{empty, Decorators},
};

use crate::get_current_theme;

pub fn text_divider() -> impl View {
    let base_widget = empty();
    let theme = get_current_theme();

    let styles_enhancer = theme.get_divider_style();

    let styled_button = base_widget.style(move |s| styles_enhancer(s));

    styled_button
}
