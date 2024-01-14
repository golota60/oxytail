use floem::{
    view::View,
    views::{self, container, Decorators},
};

use crate::get_current_theme;

use super::common_props::{OxySize, OxyVariant};

#[derive(Default, Clone, Copy)]
pub struct TooltipProps {
    pub variant: OxyVariant,
    pub size: OxySize,
}

pub fn tooltip<V: View + 'static, T: View + 'static>(
    child: V,
    tip: impl Fn() -> T + 'static,
    props: Option<TooltipProps>,
) -> impl View {
    let theme = get_current_theme();
    let props = props.unwrap_or(TooltipProps::default());

    let base_widget = views::tooltip(child, move || {
        let styles_enhancer = theme.get_tooltip_style(props);
        container(tip().style(move |s| styles_enhancer(s)))
    });

    base_widget
}
