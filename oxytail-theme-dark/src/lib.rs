use floem::{
    cosmic_text::Weight,
    peniko::Color,
    style::{Background, Style, Transition},
};
use oxytail_base::{
    themes::{Reusables, ThemeStyling},
    widgets::button::ButtonVariant,
};

pub struct CommonThemeProps {
    pub border: Color,
    pub padding: f32,
    pub border_radius: f32,
    pub hover_bg_color: Color,
    pub focus_hover_bg_color: Color,
    pub active_bg_color: Color,
    pub light_hover_bg_color: Color,
    pub light_focus_hover_bg_color: Color,
    pub focus_applied_style: Style,
    pub focus_visible_applied_style: Style,
    pub focus_style: Style,
    pub border_style: Style,
}
pub trait Reusables {
    fn get_reusables(&self) -> CommonThemeProps;
}
impl Reusables for Theme {
    fn get_reusables(&self) -> CommonThemeProps {
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

        CommonThemeProps {
            border,
            padding,
            border_radius,
            hover_bg_color,
            focus_hover_bg_color,
            active_bg_color,
            light_hover_bg_color,
            light_focus_hover_bg_color,
            focus_applied_style,
            focus_visible_applied_style,
            focus_style,
            border_style,
        }
    }
}

#[derive(Default)]
pub enum Theme {
    #[default]
    Dark,
    // Light,
}

impl ThemeStyling for Theme {
    fn get_button_base_style(&self, button_variant: ButtonVariant) -> Style {
        let reusables = self.get_reusables();
        let base_button_style = Style::new()
            .hover(|s| s.background(reusables.hover_bg_color))
            .disabled(|s| {
                s.background(Color::rgb8(180, 188, 175).with_alpha_factor(0.3))
                    .border_color(Color::rgb8(131, 145, 123).with_alpha_factor(0.3))
                    .color(Color::rgb8(166, 173, 187))
            })
            .font_size(14.)
            .line_height(1.)
            .color(Color::rgb8(166, 173, 187))
            .font_weight(Weight::SEMIBOLD)
            .transition(Background, Transition::linear(0.04))
            .focus(|s| s.hover(|s| s.background(reusables.focus_hover_bg_color)))
            .padding_left(16.0)
            .padding_right(16.0)
            .padding_top(20.0)
            .padding_bottom(20.0)
            .border_radius(5.0)
            .justify_center()
            .items_center()
            .apply(reusables.focus_style.clone());

        let variant_enhancer = match button_variant {
            ButtonVariant::Default => |s: Style| s.background(Color::rgb8(25, 30, 36)),

            ButtonVariant::Neutral => |s: Style| s.background(Color::rgb8(42, 50, 60)),
            ButtonVariant::Primary => |s: Style| s.background(Color::rgb8(116, 128, 255)),
            ButtonVariant::Secondary => |s: Style| s.background(Color::rgb8(255, 82, 217)),
            ButtonVariant::Accent => |s: Style| s.background(Color::rgb8(0, 205, 183)),
            ButtonVariant::Ghost => |s: Style| s.background(Color::TRANSPARENT),
            ButtonVariant::Link => |s: Style| s.background(Color::rgb8(117, 130, 255)),

            ButtonVariant::Info => |s: Style| s.background(Color::rgb8(0, 181, 255)),
            ButtonVariant::Success => |s: Style| s.background(Color::rgb8(0, 169, 110)),
            ButtonVariant::Warning => |s: Style| s.background(Color::rgb8(255, 190, 0)),
            ButtonVariant::Error => |s: Style| s.background(Color::rgb8(255, 88, 97)),
        };
        let enhanced_button = variant_enhancer(base_button_style);

        enhanced_button
    }
}
