use floem::{
    cosmic_text::Weight,
    peniko::Color,
    style::{Background, Style, Transition},
};
use oxytail_base::{
    themes::ThemeStyling,
    widgets::button::{ButtonProps, ButtonSize, ButtonVariant},
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

fn get_variant_colors(button_variant: ButtonVariant) -> Color {
    match button_variant {
        ButtonVariant::Default => Color::rgb8(25, 30, 36),

        ButtonVariant::Neutral => Color::rgb8(42, 50, 60),
        ButtonVariant::Primary => Color::rgb8(116, 128, 255),
        ButtonVariant::Secondary => Color::rgb8(255, 82, 217),
        ButtonVariant::Accent => Color::rgb8(0, 205, 183),
        ButtonVariant::Ghost => Color::TRANSPARENT,
        ButtonVariant::Link => Color::rgb8(117, 130, 255),

        ButtonVariant::Info => Color::rgb8(0, 181, 255),
        ButtonVariant::Success => Color::rgb8(0, 169, 110),
        ButtonVariant::Warning => Color::rgb8(255, 190, 0),
        ButtonVariant::Error => Color::rgb8(255, 88, 97),
    }
}

#[derive(Default)]
pub enum Theme {
    #[default]
    Dark,
    // Light,
}

impl ThemeStyling for Theme {
    fn get_button_style(&self, button_props: ButtonProps) -> Box<dyn Fn(Style) -> Style> {
        let reusables = self.get_reusables();

        let style_creator = move |s: Style| {
            let base_button_style = {
                s.hover(|s| s.background(reusables.hover_bg_color))
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
                    .apply(reusables.focus_style.clone())
            };

            let curr_variant_color = get_variant_colors(button_props.variant);
            let variant_enhancer = |s: Style| s.background(curr_variant_color);
            let variant_text_enhancer = |s: Style| match button_props.variant {
                ButtonVariant::Default => s,
                ButtonVariant::Neutral => s,
                ButtonVariant::Ghost => s,
                _ => s.color(Color::rgb8(25, 2, 17)),
            };

            let size_enhancer = match button_props.size {
                ButtonSize::Large => |s: Style| {
                    s.min_height(4 * 16)
                        .height(4 * 16)
                        .padding_horiz(1.5 * 16.)
                        .font_size(1.125 * 16.)
                },
                ButtonSize::Normal => {
                    |s: Style| s.min_height(3 * 16).height(3 * 16).padding_horiz(1.5 * 16.)
                }
                ButtonSize::Small => |s: Style| {
                    s.min_height(2 * 16)
                        .height(2 * 16)
                        .padding_horiz(0.75 * 16.)
                        .font_size(0.875 * 16.)
                },
                ButtonSize::Tiny => |s: Style| {
                    s.min_height(1.5 * 16.)
                        .height(1.5 * 16.)
                        .padding_horiz(0.5 * 16.)
                        .font_size(0.75 * 16.)
                        .padding_vert(0)
                },
            };

            let cloned = curr_variant_color.clone();

            let enhanced_button_style = variant_enhancer(base_button_style);
            let enhanced_button_style = variant_text_enhancer(enhanced_button_style);
            let enhanced_button_style = size_enhancer(enhanced_button_style);
            // Outline handling
            let enhanced_button_style = if button_props.outlined {
                let outline_style = enhanced_button_style
                    .outline(0.5)
                    .outline_color(cloned)
                    .background(Color::TRANSPARENT);
                match button_props.variant {
                    ButtonVariant::Default => outline_style,
                    ButtonVariant::Neutral => outline_style,
                    ButtonVariant::Ghost => outline_style,
                    _ => outline_style.color(curr_variant_color),
                }
            } else {
                enhanced_button_style
            };
            // let enhanced_button = outline_enhancer(enhanced_button);

            enhanced_button_style
        };
        Box::new(style_creator)
    }
}
