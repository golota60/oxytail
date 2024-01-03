use std::fmt::Display;

use floem::{view::View, views::Decorators, widgets::button as upstreambutton};

use crate::GLOBAL_THEME;

#[derive(Default)]
pub enum ButtonSize {
    Large,
    #[default]
    Normal,
    Small,
    Tiny,
}

#[derive(Default)]

pub enum ButtonVariant {
    #[default]
    Default,

    Neutral,
    Primary,
    Secondary,
    Accent,
    Ghost,
    Link,

    Info,
    Success,
    Warning,
    Error,
}

#[derive(Default)]
pub struct ButtonProps {
    pub variant: ButtonVariant,
    pub outlined: bool,
    pub size: ButtonSize,
}

pub fn button<S: Display + 'static>(
    label: impl Fn() -> S + 'static,
    props: Option<ButtonProps>,
) -> impl View {
    let base_component = upstreambutton(label); //.class(OxyButtonClass);
    let theme = GLOBAL_THEME.get().unwrap();

    match props {
        Some(props) => {
            let base_styles = theme.get_button_base_style(props.variant);
            let styled_button = base_component.style(move |_| base_styles.clone());

            styled_button
        }
        None => base_component,
    }
}
