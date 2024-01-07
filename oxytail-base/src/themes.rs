use floem::style::Style;

use crate::widgets::{button::ButtonProps, checkbox::CheckboxProps, text_input::InputProps};

//                 let input_style = Style::new()
//                     .background(Color::WHITE)
//                     .hover(|s| s.background(light_hover_bg_color))
//                     .focus(|s| s.hover(|s| s.background(light_focus_hover_bg_color)))
//                     .apply(border_style.clone())
//                     .apply(focus_style.clone())
//                     .cursor(CursorStyle::Text)
//                     .padding_vert(8.0)
//                     .disabled(|s| {
//                         s.background(Color::rgb8(180, 188, 175).with_alpha_factor(0.3))
//                             .color(Color::GRAY)
//                     });

//                 // TODO: LOAD TTF FROM FILE SO THAT ITS CONSISTENT. FONT IDEA: ROBOTO
//                 // Below is for eventual font support
//                 // https://github.com/lapce/floem/issues/76
//                 // let mut font_db = floem::cosmic_text::fontdb::Database::new();
//                 // font_db.load_fonts_dir("fonts");
//                 // let faces = font_db.faces();
//                 // for f in faces {
//                 //     println!("{}", f.post_script_name);
//                 // }
//                 // self = self.font_family(StyleValue::Val("Poppins-Bold.ttf".to_string()));

//                 self = self
//                     .background(Color::rgb8(29, 35, 42))
//                     .color(Color::rgb8(255, 255, 255));

//                 // self = self.class(OxyButtonClass, |_| base_button_style);
//                 // self = self.class(OxyCheckboxClass, |_| base_checkbox_style);
//                 // self = self.class(OxyLabeledCheckboxClass, |_| base_labeled_checkbox_style);
//                 // self = self.class(OxyTextInputClass, |_| input_style);

//                 self
//             }
//         }
//     }
// }

// All the functions return a function on how to apply the style instead of the style object, cause we need to apply multiple styles to the same object.
// E.g. button needs to have `base_style` applied, and then `size_style`. If those return two different Style instances, one is going to overwrite the other.
pub trait ThemeStyling {
    /// To be implemented by themes; Defines how a button style should look like.  
    fn get_button_style(&self, button_props: ButtonProps) -> Box<dyn Fn(Style) -> Style>;
    fn get_checkbox_style(&self, checkbox_props: CheckboxProps) -> Box<dyn Fn(Style) -> Style>;
    fn get_input_style(&self, checkbox_props: InputProps) -> Box<dyn Fn(Style) -> Style>;
}

pub struct ButtonStyle<T> {
    pub variant_styles: T,
}
