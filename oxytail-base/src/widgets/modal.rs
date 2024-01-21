use std::fmt::Display;

use floem::{
    action::set_window_title,
    peniko::Color,
    reactive::{ReadSignal, RwSignal},
    style::{Position, Style},
    view::View,
    views::{container, empty, Decorators},
    window,
};

use crate::get_current_theme;

use super::common_props::{OxySize, OxyVariant};

#[derive(Default, Clone, Copy)]
pub struct ModalProps {
    pub open: bool,
}

pub fn modal<V: View + 'static>(content: V, open: ReadSignal<bool>) -> impl View {
    let base_widget = container(content);
    let theme = get_current_theme();

    // let props = props.unwrap_or(ModalProps::default());

    let styles_enhancer = theme.get_modal_style(open.get());

    let styled_modal = base_widget.style(move |s| styles_enhancer(s));
    let styled_modal = styled_modal.style(move |s| {
        s.display(floem::style::Display::None)
            .apply_if(open.get(), |s| {
                s.display(floem::style::Display::Flex)
                    .position(Position::Absolute)
                    .inset_top(0)
                    .inset_left(0)
                    .margin_left(-50)
                    .z_index(9999999)
                    .background(Color::WHITE)
            })
    });

    styled_modal
}
