use floem::{
    reactive::RwSignal,
    style::Style,
    views::{Decorators, TextInput},
    widgets::text_input as upstream_text_input,
};

use crate::get_current_theme;

use super::common_props::{OxySize, OxyVariant};

#[derive(Default, Clone, Copy)]
pub struct InputProps {
    pub variant: OxyVariant,
    pub size: OxySize,
}

pub fn text_input(buffer: RwSignal<String>, props: Option<InputProps>) -> TextInput {
    let base_widget = upstream_text_input(buffer);

    let theme = get_current_theme();

    let props = props.unwrap_or(InputProps::default());

    let styles_enhancer = theme.get_input_style(props);
    let enhanced_style = styles_enhancer(Style::new());

    // .style() does not exist on an input
    let styled_input = base_widget.base_style(move |_| enhanced_style.clone());

    styled_input
}
