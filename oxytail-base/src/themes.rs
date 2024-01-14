use floem::{peniko::Color, style::Style};

use crate::widgets::{
    button::ButtonProps, checkbox::CheckboxProps, common_props::OxyVariant,
    radio_button::RadioProps, text_header::HeaderProps, text_input::InputProps,
    toggle::ToggleProps,
};

// TODO: LOAD TTF FROM FILE SO THAT ITS CONSISTENT.
// Below is for eventual font support
// https://github.com/lapce/floem/issues/76
// let mut font_db = floem::cosmic_text::fontdb::Database::new();
// font_db.load_fonts_dir("fonts");
// let faces = font_db.faces();
// for f in faces {
//     println!("{}", f.post_script_name);
// }
// self = self.font_family(StyleValue::Val("Poppins-Bold.ttf".to_string()));

pub struct DefaultThemeProps {
    /// Leading colors for variants.
    pub get_variant_colors: fn(OxyVariant) -> Color,
    /// Hover colors for variants.
    pub get_hover_variant_colors: fn(OxyVariant) -> Color,
    /// Active colors for variants.
    pub get_active_variant_colors: fn(OxyVariant) -> Color,
    /// Text color to be used in dark contexts(i.e. light text color on dark background)
    pub light_text_color: Color,
    /// Text color to be used in light contexts(i.e. dark text color on light background)
    pub dark_text_color: Color,
    // TODO: Decide whether to expose disabled states. Is it even needed?
    // Disabled colors for variants. If not provided, will just dim the default variant color.
    // -------
    /// Default variant is a little bit "special".
    /// For any other variant usually the background is the same for every widget(ex. button's color is the same as checkbox's background color).
    /// 
    /// 
    /// But for `OxyVariant::Default`, checkbox's background for example, does not share the same background color as button. Same thing for input outline and bunch of other things.
    /// This prop controls the `Color` of those.
    /// 
    /// 
    /// It's marked with double floor `__` to indicate that this controls an edge-case behavior.
    pub __default_accent: Color,
}

/// To be implemented by themes.
pub trait ThemeStyling {
    /// Defines a set of colors to be used by the theme.
    /// This can be adjusted within specific widget function later if needed, but for a vast majority of themes the colors are reused.
    fn theme_defaults(&self) -> DefaultThemeProps;

    /// Defines how a button style should look like.
    fn get_button_style(&self, button_props: ButtonProps) -> Box<dyn Fn(Style) -> Style + '_>;
    /// Defines how a checkbox should look like.
    fn get_checkbox_style(&self, checkbox_props: CheckboxProps)
        -> Box<dyn Fn(Style) -> Style + '_>;
    /// Defines how a input should look like.
    fn get_input_style(&self, checkbox_props: InputProps) -> Box<dyn Fn(Style) -> Style + '_>;
    /// Defines how a toggle should look like.
    fn get_toggle_style(&self, toggle_props: ToggleProps) -> Box<dyn Fn(Style) -> Style + '_>;
    /// Defined how a radio button should look like.
    /// Returns a tuple, where first argument styles the "dot" of the active radio and the second one is the "outer circle", containing the default state of the radio.
    fn get_radio_style(
        &self,
        radio_props: RadioProps,
    ) -> (
        Box<dyn Fn(Style) -> Style + '_>,
        Box<dyn Fn(Style) -> Style + '_>,
    );

    /// Defined how a text_header should look like.
    fn get_header_style(&self, header_props: HeaderProps) -> Box<dyn Fn(Style) -> Style + '_>;

    /// Defined how a text_divider should look like.
    fn get_divider_style(&self) -> Box<dyn Fn(Style) -> Style + '_>;
}
