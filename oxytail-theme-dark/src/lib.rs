use floem::{peniko::Color, style::Style};
use oxytail_base::{themes::ThemeStyling, widgets::button::ButtonVariant};

#[derive(Default)]
pub enum Theme {
    #[default]
    Dark,
    // Light,
}

impl ThemeStyling for Theme {
    fn get_button_base_style(&self, button_variant: ButtonVariant) -> Box<dyn Fn(Style) -> Style> {
        match button_variant {
            ButtonVariant::Default => Box::new(|s: Style| s.background(Color::rgb8(25, 30, 36))),

            ButtonVariant::Neutral => Box::new(|s: Style| s.background(Color::rgb8(42, 50, 60))),
            ButtonVariant::Primary => Box::new(|s: Style| s.background(Color::rgb8(116, 128, 255))),
            ButtonVariant::Secondary => {
                Box::new(|s: Style| s.background(Color::rgb8(255, 82, 217)))
            }
            ButtonVariant::Accent => Box::new(|s: Style| s.background(Color::rgb8(0, 205, 183))),
            ButtonVariant::Ghost => Box::new(|s: Style| s.background(Color::TRANSPARENT)),
            ButtonVariant::Link => Box::new(|s: Style| s.background(Color::rgb8(117, 130, 255))),

            ButtonVariant::Info => Box::new(|s: Style| s.background(Color::rgb8(0, 181, 255))),
            ButtonVariant::Success => Box::new(|s: Style| s.background(Color::rgb8(0, 169, 110))),
            ButtonVariant::Warning => Box::new(|s: Style| s.background(Color::rgb8(255, 190, 0))),
            ButtonVariant::Error => Box::new(|s: Style| s.background(Color::rgb8(255, 88, 97))),
        }
    }
}
