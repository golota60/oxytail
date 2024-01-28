use floem::{
    cosmic_text::Weight,
    peniko::Color,
    style::{Background, CursorStyle, Display, Style, StyleValue, Transition},
    unit::{Pct, Px},
};
use oxytail_base::{
    themes::DefaultThemeProps,
    widgets::{
        badge::BadgeProps,
        button::ButtonProps,
        checkbox::CheckboxProps,
        common_props::{OxySize, OxyVariant},
        radio_button::RadioProps,
        text_header::HeaderProps,
        text_input::InputProps,
        toggle::ToggleProps,
        tooltip::TooltipProps,
    },
};

/// A default common accent color that can be used in `OxyVariant::Default` variant. Can be overridden via `DefaultThemeProps` -> `__default_accent`.
/// `DefaultThemeProps` also contains better docs explaining what that prop is.
pub const DEFAULT_ACCENT: Color = Color::rgb8(166, 173, 187);

pub struct ThemeDefault;

impl ThemeDefault {
    pub fn get_button_style(
        button_props: ButtonProps,
        theme_defaults: DefaultThemeProps,
    ) -> Box<dyn Fn(Style) -> Style> {
        let style_creator = move |s: Style| {
            let base_button_style = {
                s.hover(|s| {
                    s.background((theme_defaults.get_hover_variant_colors)(
                        button_props.variant,
                    ))
                })
                .disabled(|s| {
                    // TODO: figure out disabled styling and how it should integrate with the API.
                    s.background(Color::rgb8(180, 188, 175).with_alpha_factor(0.3))
                        .border_color(Color::rgb8(131, 145, 123).with_alpha_factor(0.3))
                        .color(Color::rgb8(166, 173, 187))
                })
                .font_size(14.)
                .line_height(1.)
                .color(theme_defaults.light_text_color)
                .font_weight(Weight::SEMIBOLD)
                .transition(Background, Transition::linear(0.04))
                .border_radius(5.0)
                .justify_center()
                .items_center()
                .cursor(StyleValue::Val(floem::style::CursorStyle::Pointer))
            };

            let curr_variant_color = (theme_defaults.get_variant_colors)(button_props.variant);
            let variant_enhancer = |s: Style| match button_props.variant {
                OxyVariant::Default => s.background(curr_variant_color),
                OxyVariant::Neutral => s.background(curr_variant_color),
                OxyVariant::Ghost => s,
                // TODO: Links should be underlined
                OxyVariant::Link => s.color(curr_variant_color),
                _ => s
                    .background(curr_variant_color)
                    .color(theme_defaults.dark_text_color),
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
                        .hover(|s| s.color(theme_defaults.dark_text_color)),
                }
            } else {
                enhanced_button_style
            };

            enhanced_button_style
        };
        Box::new(style_creator)
    }

    pub fn get_checkbox_style(
        checkbox_props: CheckboxProps,
        theme_defaults: DefaultThemeProps,
    ) -> Box<dyn Fn(Style) -> Style> {
        let defaults = theme_defaults;

        let style_creator = move |s: Style| {
            let curr_variant_color = (defaults.get_variant_colors)(checkbox_props.variant);
            let base_checkbox_style = s
                .transition(Background, Transition::linear(0.04))
                .disabled(|s| {
                    s.background(Color::rgb8(180, 188, 175).with_alpha_factor(0.3))
                        .color(Color::GRAY)
                })
                .border_radius(8)
                .color(Color::rgb8(29, 35, 42));

            let variant_enhancer = |s: Style| match checkbox_props.variant {
                OxyVariant::Default => s.background(defaults.__default_accent),
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

    pub fn get_input_style(
        input_props: InputProps,
        theme_defaults: DefaultThemeProps,
    ) -> Box<dyn Fn(Style) -> Style> {
        let defaults = theme_defaults;

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
            let curr_variant_color = (defaults.get_variant_colors)(input_props.variant);

            let variant_enhancer = |s: Style| match input_props.variant {
                OxyVariant::Default => s.outline_color(defaults.__default_accent),
                _ => s.outline_color(curr_variant_color),
            };
            let size_enhancer = |s: Style| match input_props.size {
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

    pub fn get_toggle_style(
        toggle_props: ToggleProps,
        theme_defaults: DefaultThemeProps,
    ) -> Box<dyn Fn(Style) -> Style> {
        let defaults = theme_defaults;
        let style_creator = move |s: Style| {
            let curr_variant_color = (defaults.get_variant_colors)(toggle_props.variant);
            let base_toggle_style = s.border_radius(31);

            let size_enhancer = |s: Style| match toggle_props.size {
                // These sizes are completely arbitrary
                OxySize::Large => s.min_width(64).min_height(32),
                OxySize::Normal => s.min_width(48).min_height(24),
                OxySize::Small => s.min_width(32).min_height(16),
                OxySize::Tiny => s.min_width(25.6).min_height(16),
            };

            let variant_enhancer = |s: Style| match toggle_props.variant {
                OxyVariant::Default => s.color(defaults.__default_accent),
                _ => s.color(curr_variant_color),
            };

            let enhanced_style = size_enhancer(base_toggle_style);
            let enhanced_style = variant_enhancer(enhanced_style);

            enhanced_style
        };

        Box::new(style_creator)
    }

    pub fn get_radio_style(
        radio_props: RadioProps,
        theme_defaults: DefaultThemeProps,
    ) -> (Box<dyn Fn(Style) -> Style>, Box<dyn Fn(Style) -> Style>) {
        let defaults = theme_defaults;
        let inner_dot_style_creator = move |s: Style| {
            let curr_variant_color = (defaults.get_variant_colors)(radio_props.variant);
            let base_toggle_style = s.border_radius(Pct(100.)).disabled(|s| {
                s.background(Color::rgb(0.5, 0.5, 0.5))
                    .hover(|s| s.background(Color::rgb(0.5, 0.5, 0.5)))
            });

            let size_enhancer = |s: Style| match radio_props.size {
                // These sizes are completely arbitrary, daisyUI does not have sizes for radios
                OxySize::Large => s.width(18.5).height(18.5),
                OxySize::Normal => s.width(14).height(14),
                OxySize::Small => s.width(10.5).height(10.5),
                OxySize::Tiny => s.width(8).height(8),
            };

            let variant_enhancer = |s: Style| match radio_props.variant {
                OxyVariant::Default => s.background(defaults.__default_accent),
                _ => s.background(curr_variant_color),
            };

            let enhanced_style = size_enhancer(base_toggle_style);
            let enhanced_style = variant_enhancer(enhanced_style);

            enhanced_style
        };

        let outer_dot_style_creator = move |s: Style| {
            let curr_variant_color = (defaults.get_variant_colors)(radio_props.variant);
            let base_toggle_style = s
                .background(Color::TRANSPARENT)
                .border(1)
                .border_radius(Pct(100.))
                .justify_center()
                .items_center()
                .disabled(|s| {
                    s.background(Color::rgb8(180, 188, 175).with_alpha_factor(0.3))
                        .color(Color::GRAY)
                        .border_color(Color::rgb(0.5, 0.5, 0.5))
                });

            let size_enhancer = |s: Style| match radio_props.size {
                // These sizes are completely arbitrary, daisyUI does not have sizes for radios
                OxySize::Large => s.width(32).height(32),
                OxySize::Normal => s.width(24).height(24),
                OxySize::Small => s.width(18).height(18),
                OxySize::Tiny => s.width(14).height(14),
            };

            let variant_enhancer = |s: Style| match radio_props.variant {
                OxyVariant::Default => s.border_color(defaults.__default_accent),
                _ => s.border_color(curr_variant_color),
            };

            let enhanced_style = size_enhancer(base_toggle_style);
            let enhanced_style = variant_enhancer(enhanced_style);

            enhanced_style
        };

        (
            Box::new(inner_dot_style_creator),
            Box::new(outer_dot_style_creator),
        )
    }

    pub fn get_header_style(
        header_props: HeaderProps,
        _theme_defaults: DefaultThemeProps,
    ) -> Box<dyn Fn(Style) -> Style> {
        let style_creator = move |s: Style| {
            let size_enhancer = |s: Style| match header_props.size {
                OxySize::Large => s.font_size(2.25 * 16.).font_bold(),
                OxySize::Normal => s.font_size(1.5 * 16.).font_bold(),
                OxySize::Small => s.font_size(1.25 * 16.).font_bold(),
                OxySize::Tiny => s.font_bold(),
            };

            let enhanced_style = size_enhancer(s);

            enhanced_style
        };

        Box::new(style_creator)
    }

    pub fn get_divider_style(_theme_defaults: DefaultThemeProps) -> Box<dyn Fn(Style) -> Style> {
        let styles_creator = |s: Style| {
            s.width_full()
                .min_width_full()
                .height(1)
                .background(Color::GRAY)
                .display(Display::Flex)
                .flex_grow(1.)
                .margin_vert(10.)
        };

        Box::new(styles_creator)
    }

    pub fn get_tooltip_style(
        tooltip_props: TooltipProps,
        theme_defaults: DefaultThemeProps,
    ) -> Box<dyn Fn(Style) -> Style> {
        let styles_creator = move |s: Style| {
            let base_style = s.padding(8.);

            let curr_variant_color = (theme_defaults.get_variant_colors)(tooltip_props.variant);
            let variant_enhancer = |s: Style| match tooltip_props.variant {
                OxyVariant::Default => s
                    .background((theme_defaults.get_variant_colors)(OxyVariant::Neutral))
                    .color(theme_defaults.light_text_color),
                OxyVariant::Neutral => s
                    .background(curr_variant_color)
                    .color(theme_defaults.light_text_color),
                _ => s.background(curr_variant_color),
            };

            let size_enhancer = |s: Style| match tooltip_props.size {
                OxySize::Large => s.font_size(1.5 * 16.),
                OxySize::Normal => s,
                OxySize::Small => s.font_size(0.8 * 16.),
                OxySize::Tiny => s.font_size(0.6 * 16.),
            };

            let enhanced_style = variant_enhancer(base_style);
            let enhanced_style = size_enhancer(enhanced_style);

            enhanced_style
        };

        Box::new(styles_creator)
    }

    /// Defines how a `badge` should look like.
    pub fn get_badge_style(
        badge_props: BadgeProps,
        theme_defaults: DefaultThemeProps,
    ) -> Box<dyn Fn(Style) -> Style> {
        let styles_creator = move |s: Style| {
            let base_style = s
                .padding_vert(2.)
                .padding_horiz(9.)
                .font_size(13.)
                .border_radius(Px(14.))
                .items_center()
                .justify_center()
                .height(18.);

            let curr_variant_color = (theme_defaults.get_variant_colors)(badge_props.variant);

            let variant_enhancer = |s: Style| match badge_props.variant {
                OxyVariant::Default => s
                    .background((theme_defaults.get_variant_colors)(OxyVariant::Neutral))
                    .color(theme_defaults.light_text_color),
                OxyVariant::Neutral => s
                    .background(curr_variant_color)
                    .color(theme_defaults.light_text_color),
                _ => s
                    .background(curr_variant_color)
                    .color(theme_defaults.dark_text_color),
            };

            let size_enhancer = |s: Style| match badge_props.size {
                OxySize::Large => s.font_size(16.).height(22),
                OxySize::Normal => s,
                OxySize::Small => s.font_size(10.).height(15),
                OxySize::Tiny => s.font_size(8.).height(11),
            };

            let outlined_enhancer = |s: Style| match badge_props.outlined {
                true => s
                    .border_color(curr_variant_color)
                    .border(0.5)
                    .background(Color::TRANSPARENT)
                    .color(curr_variant_color),
                false => s,
            };

            let enhanced_style = variant_enhancer(base_style);
            let enhanced_style = size_enhancer(enhanced_style);
            let enhanced_style = outlined_enhancer(enhanced_style);

            enhanced_style
        };

        Box::new(styles_creator)
    }
}
