use floem::{reactive::RwSignal, views::TextInput, widgets::text_input as upstream_text_input};

pub fn text_input(buffer: RwSignal<String>) -> TextInput {
    upstream_text_input(buffer) //.class(OxyTextInputClass)
}
