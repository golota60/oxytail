use std::fmt::Display;

use floem::{
    view::View,
    views::{self, container, Decorators},
};

use crate::themes::OxyButtonClass;

pub fn button<S: Display + 'static>(label: impl Fn() -> S + 'static) -> impl View {
    container(views::label(label))
        .keyboard_navigatable()
        .class(OxyButtonClass)
}
