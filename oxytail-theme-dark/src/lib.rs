use floem::{
    cosmic_text::Weight,
    peniko::Color,
    style::{Background, CursorStyle, Display, FlexDirection, Style, StyleValue, Transition},
    unit::Pct,
};
use oxytail_base::{
    themes::ThemeStyling,
    widgets::{
        button::ButtonProps,
        checkbox::CheckboxProps,
        common_props::{OxySize, OxyVariant},
        text_input::InputProps,
        toggle::ToggleProps,
    },
};

pub struct CommonThemeProps {
    pub light_text_color: Color,
    pub dark_text_color: Color,
    pub gray_default_color: Color,
}
pub trait Reusables {
    fn get_reusables(&self) -> CommonThemeProps;
}
impl Reusables for Theme {
    fn get_reusables(&self) -> CommonThemeProps {
        CommonThemeProps {
            light_text_color: Color::rgb8(166, 173, 187),
            dark_text_color: Color::rgb8(25, 2, 17),
            gray_default_color: Color::rgb8(166, 173, 187),
        }
    }
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
                s.hover(|s| s.background(get_hover_variant_colors(button_props.variant)))
                    .disabled(|s| {
                        s.background(Color::rgb8(180, 188, 175).with_alpha_factor(0.3))
                            .border_color(Color::rgb8(131, 145, 123).with_alpha_factor(0.3))
                            .color(Color::rgb8(166, 173, 187))
                    })
                    .font_size(14.)
                    .line_height(1.)
                    .color(reusables.light_text_color)
                    .font_weight(Weight::SEMIBOLD)
                    .transition(Background, Transition::linear(0.04))
                    .border_radius(5.0)
                    .justify_center()
                    .items_center()
                    .cursor(StyleValue::Val(floem::style::CursorStyle::Pointer))
            };

            let curr_variant_color = get_variant_colors(button_props.variant);
            let variant_enhancer = |s: Style| match button_props.variant {
                OxyVariant::Default => s.background(curr_variant_color),
                OxyVariant::Neutral => s.background(curr_variant_color),
                OxyVariant::Ghost => s,
                // TODO: Links should be underlined
                OxyVariant::Link => s.color(curr_variant_color),
                _ => s
                    .background(curr_variant_color)
                    .color(reusables.dark_text_color),
            };

            let size_enhancer = match button_props.size {
                OxySize::Large => |s: Style| {
                    s.min_height(4 * 16)
                        .height(4 * 16)
                        .padding_horiz(1.5 * 16.)
                        .font_size(1.125 * 16.)
                },
                OxySize::Normal => {
                    |s: Style| s.min_height(3 * 16).height(3 * 16).padding_horiz(1.5 * 16.)
                }
                OxySize::Small => |s: Style| {
                    s.min_height(2 * 16)
                        .height(2 * 16)
                        .padding_horiz(0.75 * 16.)
                        .font_size(0.875 * 16.)
                },
                OxySize::Tiny => |s: Style| {
                    s.min_height(1.5 * 16.)
                        .height(1.5 * 16.)
                        .padding_horiz(0.5 * 16.)
                        .font_size(0.75 * 16.)
                        .padding_vert(0)
                },
            };

            let cloned = curr_variant_color.clone();

            let enhanced_button_style = variant_enhancer(base_button_style);
            let enhanced_button_style = size_enhancer(enhanced_button_style);
            // Outline handling
            let enhanced_button_style = if button_props.outlined {
                let outline_style = enhanced_button_style
                    .outline(0.5)
                    .outline_color(cloned)
                    .background(Color::TRANSPARENT);
                match button_props.variant {
                    OxyVariant::Default => outline_style,
                    OxyVariant::Neutral => outline_style,
                    OxyVariant::Ghost => outline_style,
                    _ => outline_style
                        .color(curr_variant_color)
                        .hover(|s| s.color(Color::rgb8(25, 2, 17))),
                }
            } else {
                enhanced_button_style
            };

            enhanced_button_style
        };
        Box::new(style_creator)
    }

    fn get_checkbox_style(&self, checkbox_props: CheckboxProps) -> Box<dyn Fn(Style) -> Style> {
        let reusables = self.get_reusables();

        let style_creator = move |s: Style| {
            let curr_variant_color = get_variant_colors(checkbox_props.variant);
            let base_checkbox_style = s
                .background(Color::rgb8(166, 174, 188))
                .transition(Background, Transition::linear(0.04))
                .disabled(|s| {
                    s.background(Color::rgb8(180, 188, 175).with_alpha_factor(0.3))
                        .color(Color::GRAY)
                })
                .border_radius(8)
                .color(Color::rgb8(29, 35, 42));

            let variant_enhancer = |s: Style| match checkbox_props.variant {
                OxyVariant::Default => s.background(reusables.gray_default_color),
                OxyVariant::Neutral => s.background(reusables.gray_default_color),
                _ => s.background(curr_variant_color).color(Color::BLACK),
            };

            let size_enhancer = |s: Style| match checkbox_props.size {
                OxySize::Large => s.width(2. * 16.).height(2. * 16.),
                OxySize::Normal => s.width(1.5 * 16.).height(1.5 * 16.),
                OxySize::Small => s.width(1.25 * 16.).height(1.25 * 16.),
                OxySize::Tiny => s.width(1. * 16.).height(1. * 16.),
            };

            let enhanced_widget = variant_enhancer(base_checkbox_style);
            let enhanced_widget = size_enhancer(enhanced_widget);

            enhanced_widget
        };

        Box::new(style_creator)
    }

    fn get_input_style(&self, checkbox_props: InputProps) -> Box<dyn Fn(Style) -> Style> {
        let reusables = self.get_reusables();

        let style_creator = move |s: Style| {
            let input_style = s
                .cursor(CursorStyle::Text)
                .outline(1)
                .border_radius(0.5 * 16.)
                .disabled(|s| {
                    s.background(Color::rgb8(180, 188, 175).with_alpha_factor(0.3))
                        .color(Color::GRAY)
                })
                .items_center()
                .justify_center();
            let curr_variant_color = get_variant_colors(checkbox_props.variant);

            let variant_enhancer = |s: Style| match checkbox_props.variant {
                OxyVariant::Default => s.outline_color(reusables.gray_default_color),
                OxyVariant::Neutral => s.outline_color(reusables.gray_default_color),
                _ => s.outline_color(curr_variant_color),
            };
            let size_enhancer = |s: Style| match checkbox_props.size {
                OxySize::Large => s
                    .height(4 * 16)
                    .font_size(16. * 1.125)
                    .padding_horiz(1.5 * 16.),
                OxySize::Normal => s.height(3 * 16).padding_horiz(1 * 16),
                OxySize::Small => s
                    .height(2 * 16)
                    .font_size(16. * 0.875)
                    .padding_horiz(0.75 * 16.),
                OxySize::Tiny => s
                    .height(1.5 * 16.)
                    .font_size(16. * 0.75)
                    .padding_horiz(0.5 * 16.),
            };

            let enhanced_style = variant_enhancer(input_style);
            let enhanced_style = size_enhancer(enhanced_style);

            enhanced_style
        };

        Box::new(style_creator)
    }

    fn get_toggle_border_style(
        &self,
        toggle_props: ToggleProps,
        enabled: bool,
    ) -> Box<dyn Fn(Style) -> Style> {
        let reusables = self.get_reusables();
        let style_creator = move |s: Style| {
            let curr_variant_color = get_variant_colors(toggle_props.variant);
            let base_toggle_style = s
                .border(1)
                .padding(2)
                .border_color(curr_variant_color)
                .border_radius(31);

            let size_enhancer = |s: Style| match toggle_props.size {
                // These sizes are completely arbitrary
                OxySize::Large => s.width(61).height(32),
                OxySize::Normal => s.width(46.4).height(24),
                OxySize::Small => s.width(30.4).height(20),
                OxySize::Tiny => s.padding(1).width(25.6).height(16),
            };

            let enabled_enhancer = |s: Style| match enabled {
                true => s
                    .display(Display::Flex)
                    .flex_direction(FlexDirection::RowReverse),
                false => s.display(Display::Flex).flex_direction(FlexDirection::Row),
            };

            let variant_enhancer = |s: Style| match toggle_props.variant {
                OxyVariant::Default => s.border_color(reusables.gray_default_color),
                _ => s.border_color(curr_variant_color),
            };

            let enhanced_style = size_enhancer(base_toggle_style);
            let enhanced_style = enabled_enhancer(enhanced_style);
            let enhanced_style = variant_enhancer(enhanced_style);

            enhanced_style
        };

        Box::new(style_creator)
    }

    fn get_toggle_ball_style(
        &self,
        toggle_props: ToggleProps,
        _enabled: bool,
    ) -> Box<dyn Fn(Style) -> Style> {
        let reusables = self.get_reusables();
        let style_creator = move |s: Style| {
            let curr_variant_color = get_variant_colors(toggle_props.variant);
            let base_ball_style = s.color(curr_variant_color).border_radius(Pct(50.));
            let size_enhancer = |s: Style| match toggle_props.size {
                // These sizes are completely arbitrary
                OxySize::Large => s.width(25.28).height(25.28),
                OxySize::Normal => s.width(17.28).height(17.28),
                OxySize::Small => s.width(13.44).height(13.44),
                OxySize::Tiny => s.width(10.72).height(10.72),
            };

            let enhanced_style = size_enhancer(base_ball_style);

            let variant_enhancer = |s: Style| match toggle_props.variant {
                OxyVariant::Default => s.color(reusables.gray_default_color),
                _ => s.color(curr_variant_color),
            };

            let enhanced_style = variant_enhancer(enhanced_style);

            enhanced_style
        };

        Box::new(style_creator)
    }
}
