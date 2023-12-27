use std::fmt::Display;

use floem::{view::View, views::Decorators, widgets::button as upstreambutton};

use crate::themes::OxyButtonClass;

pub fn button<S: Display + 'static>(label: impl Fn() -> S + 'static) -> impl View {
    upstreambutton(label).class(OxyButtonClass)
}
