use floem::{
    peniko::Color,
    style::{Background, CursorStyle, Style, Transition},
    style_class,
};

#[derive(Default)]
pub enum Theme {
    #[default]
    Dark,
    // Light,
}

// Each class is the same across themes; only what the class name maps to changes.
style_class!(pub OxyButtonClass);
style_class!(pub OxyCheckboxClass);
style_class!(pub OxyLabeledCheckboxClass);
style_class!(pub OxyTextInputClass);

pub trait StyleEnhancer {
    fn enhance(self, theme: Theme) -> Self;
}

impl StyleEnhancer for Style {
    fn enhance(mut self, theme: Theme) -> Self {
        match theme {
            Theme::Dark => {
                let border = Color::rgb8(140, 140, 140);
                let padding = 5.0;
                let border_radius = 5.0;

                let hover_bg_color = Color::rgb8(20, 25, 30);
                let focus_hover_bg_color = Color::rgb8(20, 25, 30);
                let active_bg_color = Color::rgb8(20, 25, 30);
                let light_hover_bg_color = Color::rgb8(250, 252, 248);
                let light_focus_hover_bg_color = Color::rgb8(250, 249, 251);

                let focus_applied_style = Style::new().border_color(Color::rgb8(114, 74, 140));
                let focus_visible_applied_style = Style::new().outline(3.0);

                let focus_style = Style::new()
                    .outline_color(Color::rgba8(213, 208, 216, 150))
                    .focus(|_| focus_applied_style.clone())
                    .focus_visible(|_| focus_visible_applied_style.clone());

                let border_style = Style::new()
                    .disabled(|s| s.border_color(Color::rgb8(131, 145, 123).with_alpha_factor(0.3)))
                    .border(1.0)
                    .border_color(border)
                    .padding(padding)
                    .border_radius(border_radius)
                    .apply(focus_style.clone());

                let base_checkbox_style = Style::new()
                    .width(20.)
                    .height(20.)
                    .background(Color::WHITE)
                    .active(|s| s.background(active_bg_color))
                    .transition(Background, Transition::linear(0.04))
                    .hover(|s| s.background(hover_bg_color))
                    .focus(|s| s.hover(|s| s.background(focus_hover_bg_color)))
                    .apply(border_style.clone())
                    .apply(focus_style.clone())
                    .disabled(|s| {
                        s.background(Color::rgb8(180, 188, 175).with_alpha_factor(0.3))
                            .color(Color::GRAY)
                    });

                let base_button_style = Style::new()
                    .background(Color::rgb8(25, 30, 36))
                    .hover(|s| s.background(hover_bg_color))
                    .disabled(|s| {
                        s.background(Color::rgb8(180, 188, 175).with_alpha_factor(0.3))
                            .border_color(Color::rgb8(131, 145, 123).with_alpha_factor(0.3))
                            .color(Color::GRAY)
                    })
                    .active(|s| {
                        s.background(active_bg_color)
                            .color(Color::WHITE.with_alpha_factor(0.9))
                    })
                    .transition(Background, Transition::linear(0.04))
                    .focus(|s| s.hover(|s| s.background(focus_hover_bg_color)))
                    .padding(padding)
                    .justify_center()
                    .items_center()
                    .apply(focus_style.clone());

                let base_labeled_checkbox_style = Style::new()
                    .gap(padding, 0.0)
                    .hover(|s| s.background(hover_bg_color))
                    .padding(padding)
                    .transition(Background, Transition::linear(0.04))
                    .border_radius(border_radius)
                    .active(|s| s.class(OxyCheckboxClass, |s| s.background(active_bg_color)))
                    .focus(|s| {
                        s.class(OxyCheckboxClass, |_| focus_applied_style.clone())
                            .hover(|s| s.background(focus_hover_bg_color))
                    })
                    .disabled(|s| {
                        s.color(Color::GRAY).class(OxyCheckboxClass, |s| {
                            s.background(Color::rgb8(180, 188, 175).with_alpha_factor(0.3))
                                .color(Color::GRAY)
                                .hover(|s| {
                                    s.background(Color::rgb8(180, 188, 175).with_alpha_factor(0.3))
                                })
                        })
                    })
                    .apply(focus_style.clone());

                let input_style = Style::new()
                    .background(Color::WHITE)
                    .hover(|s| s.background(light_hover_bg_color))
                    .focus(|s| s.hover(|s| s.background(light_focus_hover_bg_color)))
                    .apply(border_style.clone())
                    .apply(focus_style.clone())
                    .cursor(CursorStyle::Text)
                    .padding_vert(8.0)
                    .disabled(|s| {
                        s.background(Color::rgb8(180, 188, 175).with_alpha_factor(0.3))
                            .color(Color::GRAY)
                    });

                // Below is for eventual font support
                // let mut font_db = floem::cosmic_text::fontdb::Database::new();
                // font_db.load_system_fonts();
                // self = self.font_family(StyleValue::Val("system-ui".to_string()));

                self = self
                    .background(Color::rgb8(29, 35, 42))
                    .color(Color::rgb8(255, 255, 255));

                self = self.class(OxyButtonClass, |_| base_button_style);
                self = self.class(OxyCheckboxClass, |_| base_checkbox_style);
                self = self.class(OxyLabeledCheckboxClass, |_| base_labeled_checkbox_style);
                self = self.class(OxyTextInputClass, |_| input_style);

                self
            }
        }
    }
}
