use floem::{
    cosmic_text::Weight,
    peniko::Color,
    style::{Background, CursorStyle, Style, Transition},
    style_class,
};

use crate::{widgets::button::ButtonVariant, GLOBAL_THEME};
use strum_macros::EnumString;

// IDEA: Allow any style that implements ThemeStyle to be a valid style?
// So that external styles can be created
// TODO: Figure out a nice "stylesheet" that would create a theme

/*
This should work like the following

1. User ".enhance(Theme)"s their style
2. fn enhance() reads all the functions declared below to determine the stylesheets and how they should look like

*/

/*
CURRENT LIMITATION: all the stylesheets need to have pre-defined types, so the "stylesheet" provided will have to use the same naming etc.
Classes is the same across themes; only what the class name maps to changes.
This should be ok tho. There needs to be a line somewhere.
*/

// BUTTON CLASSES
style_class!(pub OxyButtonClass);
//

style_class!(pub OxyCheckboxClass);
style_class!(pub OxyLabeledCheckboxClass);
style_class!(pub OxyTextInputClass);

// pub trait StyleEnhancer {
//     fn enhance(self) -> Self;
// }

// impl StyleEnhancer for Style {
//     fn enhance(mut self) -> Self {
//         let selected_theme = GLOBAL_THEME.get().expect("The theme is uninitialized. Did you forget to run `init_theme(Theme)` in your `main` function?");
//         match selected_theme {
//             Theme::Dark => {
//                 let border = Color::rgb8(140, 140, 140);
//                 let padding = 5.0;
//                 let border_radius = 5.0;

//                 let hover_bg_color = Color::rgb8(20, 25, 30);
//                 let focus_hover_bg_color = Color::rgb8(20, 25, 30);
//                 let active_bg_color = Color::rgb8(20, 25, 30);
//                 let light_hover_bg_color = Color::rgb8(250, 252, 248);
//                 let light_focus_hover_bg_color = Color::rgb8(250, 249, 251);

//                 let focus_applied_style = Style::new().border_color(Color::rgb8(114, 74, 140));
//                 let focus_visible_applied_style = Style::new().outline(3.0);

//                 let focus_style = Style::new()
//                     .outline_color(Color::rgba8(213, 208, 216, 150))
//                     .focus(|_| focus_applied_style.clone())
//                     .focus_visible(|_| focus_visible_applied_style.clone());

//                 let border_style = Style::new()
//                     .disabled(|s| s.border_color(Color::rgb8(131, 145, 123).with_alpha_factor(0.3)))
//                     .border(1.0)
//                     .border_color(border)
//                     .padding(padding)
//                     .border_radius(border_radius)
//                     .apply(focus_style.clone());

//                 let base_button_style = Style::new()
//                     .hover(|s| s.background(hover_bg_color))
//                     .disabled(|s| {
//                         s.background(Color::rgb8(180, 188, 175).with_alpha_factor(0.3))
//                             .border_color(Color::rgb8(131, 145, 123).with_alpha_factor(0.3))
//                             .color(Color::rgb8(166, 173, 187))
//                     })
//                     .font_size(14.)
//                     .line_height(1.)
//                     .color(Color::rgb8(166, 173, 187))
//                     .font_weight(Weight::SEMIBOLD)
//                     .transition(Background, Transition::linear(0.04))
//                     .focus(|s| s.hover(|s| s.background(focus_hover_bg_color)))
//                     .padding_left(16.0)
//                     .padding_right(16.0)
//                     .padding_top(20.0)
//                     .padding_bottom(20.0)
//                     .border_radius(5.0)
//                     .justify_center()
//                     .items_center()
//                     .apply(focus_style.clone());

//                 let base_checkbox_style = Style::new()
//                     .width(20.)
//                     .height(20.)
//                     .background(Color::WHITE)
//                     .active(|s| s.background(active_bg_color))
//                     .transition(Background, Transition::linear(0.04))
//                     .hover(|s| s.background(hover_bg_color))
//                     .focus(|s| s.hover(|s| s.background(focus_hover_bg_color)))
//                     .apply(border_style.clone())
//                     .apply(focus_style.clone())
//                     .disabled(|s| {
//                         s.background(Color::rgb8(180, 188, 175).with_alpha_factor(0.3))
//                             .color(Color::GRAY)
//                     });

//                 // let base_button_style = selected_theme.button_style();
//                 // let get_variant_style = base_button_style.get_variant_style;
//                 // let variant_styles = get_variant_style(ButtonVariant::Primary);

//                 let base_labeled_checkbox_style = Style::new()
//                     .gap(padding, 0.0)
//                     .hover(|s| s.background(hover_bg_color))
//                     .padding(padding)
//                     .transition(Background, Transition::linear(0.04))
//                     .border_radius(border_radius)
//                     .active(|s| s.class(OxyCheckboxClass, |s| s.background(active_bg_color)))
//                     .focus(|s| {
//                         s.class(OxyCheckboxClass, |_| focus_applied_style.clone())
//                             .hover(|s| s.background(focus_hover_bg_color))
//                     })
//                     .disabled(|s| {
//                         s.color(Color::GRAY).class(OxyCheckboxClass, |s| {
//                             s.background(Color::rgb8(180, 188, 175).with_alpha_factor(0.3))
//                                 .color(Color::GRAY)
//                                 .hover(|s| {
//                                     s.background(Color::rgb8(180, 188, 175).with_alpha_factor(0.3))
//                                 })
//                         })
//                     })
//                     .apply(focus_style.clone());

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

pub struct ButtonData {
    get_initial: Box<dyn Fn(Style) -> Style>,
    get_active: Box<dyn Fn(Style) -> Style>,
}

pub trait ThemeStyling {
    fn get_button_base_style(&self, button_variant: ButtonVariant) -> Box<dyn Fn(Style) -> Style>;
}

pub struct ButtonStyle<T> {
    pub variant_styles: T,
}
