use floem::{
    reactive::RwSignal,
    views::{Decorators, TextInput},
    widgets::text_input as upstream_text_input,
};

use crate::themes::OxyTextInputClass;

pub fn text_input(buffer: RwSignal<String>) -> TextInput {
    upstream_text_input(buffer).class(OxyTextInputClass)
}
