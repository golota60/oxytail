use floem::{
    cosmic_text::Weight,
    peniko::Color,
    style::{Background, CursorStyle, Display, Style, StyleValue, Transition},
    unit::Pct,
};
use oxytail_base::{
    themes::{DefaultThemeProps, ThemeStyling},
    widgets::{
        button::ButtonProps,
        checkbox::CheckboxProps,
        common_props::{OxySize, OxyVariant},
        radio_button::RadioProps,
        text_header::HeaderProps,
        text_input::InputProps,
        toggle::ToggleProps,
    },
};
use oxytail_theme_defaults::ThemeDefault;

pub struct CommonThemeProps {
    pub light_text_color: Color,
    pub dark_text_color: Color,
    pub gray_default_color: Color,
}
// pub trait Reusables {
//     fn get_reusables(&self) -> CommonThemeProps;
// }
// impl Reusables for Theme<dyn ThemeDefault> {
//     fn get_reusables(&self) -> CommonThemeProps {
//         CommonThemeProps {
//             light_text_color: Color::rgb8(166, 173, 187),
//             dark_text_color: Color::rgb8(25, 2, 17),
//             gray_default_color: Color::rgb8(166, 173, 187),
//         }
//     }
// }

#[derive(Default)]
pub enum Theme {
    #[default]
    Dark,
}

fn get_variant_colors(oxy_variant: OxyVariant) -> Color {
    match oxy_variant {
        OxyVariant::Default => Color::rgb8(25, 30, 36),

        OxyVariant::Neutral => Color::rgb8(42, 50, 60),
        OxyVariant::Primary => Color::rgb8(116, 128, 255),
        OxyVariant::Secondary => Color::rgb8(255, 82, 217),
        OxyVariant::Accent => Color::rgb8(0, 205, 183),
        OxyVariant::Ghost => Color::TRANSPARENT,
        OxyVariant::Link => Color::rgb8(117, 130, 255),

        OxyVariant::Info => Color::rgb8(0, 181, 255),
        OxyVariant::Success => Color::rgb8(0, 169, 110),
        OxyVariant::Warning => Color::rgb8(255, 190, 0),
        OxyVariant::Error => Color::rgb8(255, 88, 97),
    }
}

fn get_hover_variant_colors(oxy_variant: OxyVariant) -> Color {
    match oxy_variant {
        OxyVariant::Default => Color::rgb8(20, 25, 30),

        OxyVariant::Neutral => Color::rgb8(35, 42, 51),
        OxyVariant::Primary => Color::rgb8(100, 110, 228),
        OxyVariant::Secondary => Color::rgb8(239, 71, 188),
        OxyVariant::Accent => Color::rgb8(0, 178, 159),
        OxyVariant::Ghost => Color::rgb8(56, 63, 71),
        OxyVariant::Link => Color::TRANSPARENT,

        OxyVariant::Info => Color::rgb8(0, 157, 228),
        OxyVariant::Success => Color::rgb8(0, 147, 95),
        OxyVariant::Warning => Color::rgb8(231, 165, 0),
        OxyVariant::Error => Color::rgb8(239, 76, 83),
    }
}

fn get_active_variant_colors(oxy_variant: OxyVariant) -> Color {
    Color::BLACK
}

impl ThemeStyling for Theme {
    fn theme_defaults(&self) -> DefaultThemeProps {
        DefaultThemeProps {
            variant_colors: get_variant_colors,
            hover_variant_colors: get_hover_variant_colors,
            active_variant_colors: get_active_variant_colors,
            light_text_color: Color::rgb8(166, 173, 187),
            dark_text_color: Color::rgb8(25, 2, 17),
        }
    }

    fn get_button_style(&self, button_props: ButtonProps) -> Box<dyn Fn(Style) -> Style> {
        oxytail_theme_defaults::ThemeDefault::get_button_style(button_props, self.theme_defaults())
    }

    fn get_checkbox_style(&self, checkbox_props: CheckboxProps) -> Box<dyn Fn(Style) -> Style> {
        oxytail_theme_defaults::ThemeDefault::get_checkbox_style(
            checkbox_props,
            self.theme_defaults(),
        )
    }

    fn get_input_style(&self, input_props: InputProps) -> Box<dyn Fn(Style) -> Style> {
        oxytail_theme_defaults::ThemeDefault::get_input_style(input_props, self.theme_defaults())
    }

    fn get_toggle_style(&self, toggle_props: ToggleProps) -> Box<dyn Fn(Style) -> Style> {
        oxytail_theme_defaults::ThemeDefault::get_toggle_style(toggle_props, self.theme_defaults())
    }

    fn get_radio_style(
        &self,
        radio_props: RadioProps,
    ) -> (Box<dyn Fn(Style) -> Style>, Box<dyn Fn(Style) -> Style>) {
        oxytail_theme_defaults::ThemeDefault::get_radio_style(radio_props, self.theme_defaults())
    }

    fn get_header_style(&self, header_props: HeaderProps) -> Box<dyn Fn(Style) -> Style> {
        oxytail_theme_defaults::ThemeDefault::get_header_style(header_props, self.theme_defaults())
    }

    fn get_divider_style(&self) -> Box<dyn Fn(Style) -> Style> {
        oxytail_theme_defaults::ThemeDefault::get_divider_style(self.theme_defaults())
    }
}
