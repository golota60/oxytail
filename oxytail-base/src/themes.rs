use floem::style::Style;

use crate::widgets::{
    button::ButtonProps, checkbox::CheckboxProps, text_input::InputProps, toggle::ToggleProps,
};

// TODO: LOAD TTF FROM FILE SO THAT ITS CONSISTENT. FONT IDEA: ROBOTO
// Below is for eventual font support
// https://github.com/lapce/floem/issues/76
// let mut font_db = floem::cosmic_text::fontdb::Database::new();
// font_db.load_fonts_dir("fonts");
// let faces = font_db.faces();
// for f in faces {
//     println!("{}", f.post_script_name);
// }
// self = self.font_family(StyleValue::Val("Poppins-Bold.ttf".to_string()));

// All the functions return a function on how to apply the style instead of the style object, cause we need to apply multiple styles to the same object.

/// To be implemented by themes.
pub trait ThemeStyling {
    /// Defines how a button style should look like.
    fn get_button_style(&self, button_props: ButtonProps) -> Box<dyn Fn(Style) -> Style>;
    /// Defines how a checkbox should look like.
    fn get_checkbox_style(&self, checkbox_props: CheckboxProps) -> Box<dyn Fn(Style) -> Style>;
    /// Defines how a input should look like.
    fn get_input_style(&self, checkbox_props: InputProps) -> Box<dyn Fn(Style) -> Style>;
    /// Defines how a toggle should look like.
    fn get_toggle_border_style(&self, toggle_props: ToggleProps) -> Box<dyn Fn(Style) -> Style>;
}

pub struct ButtonStyle<T> {
    pub variant_styles: T,
}
