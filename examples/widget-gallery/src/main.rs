use floem::{
    kurbo::Size,
    peniko::Color,
    reactive::create_signal,
    view::View,
    views::{h_stack, label, stack, v_stack, Decorators},
    widgets::slider::slider,
    window::WindowConfig,
    Application, EventPropagation,
};
use oxytail_base::{
    init_theme,
    widgets::{
        button::{button, ButtonProps},
        checkbox::labeled_checkbox,
        checkbox::{checkbox, CheckboxProps},
        common_props::{OxySize, OxyVariant},
    },
};
use oxytail_theme_dark::Theme;

fn checkbox_with_state(props: Option<CheckboxProps>) -> impl View {
    let (checked, set_checked) = create_signal(true);

    checkbox(checked, props).on_click_stop(move |_| {
        set_checked.update(|checked| *checked = !*checked);
    })
}

fn app_view() -> impl View {
    // create a counter reactive signal with initial value 0
    let (counter, set_counter) = create_signal(0);
    let (checked, set_checked) = create_signal(true);

    // create user interface with Floem view functions
    stack((v_stack((
        label(|| "Buttons").style(|s| {
            s.width_full()
                .font_size(20.)
                .items_center()
                .justify_center()
        }),
        label(|| "Button variants"),
        stack((
            button(|| "Default", None),
            button(
                || "Neutral",
                Some(ButtonProps {
                    variant: OxyVariant::Neutral,
                    ..Default::default()
                }),
            ),
            button(
                || "Primary",
                Some(ButtonProps {
                    variant: OxyVariant::Primary,
                    ..Default::default()
                }),
            ),
            button(
                || "Secondary",
                Some(ButtonProps {
                    variant: OxyVariant::Secondary,
                    ..Default::default()
                }),
            ),
            button(
                || "Accent",
                Some(ButtonProps {
                    variant: OxyVariant::Accent,
                    ..Default::default()
                }),
            ),
            button(
                || "Ghost",
                Some(ButtonProps {
                    variant: OxyVariant::Ghost,
                    ..Default::default()
                }),
            ),
            button(
                || "Link",
                Some(ButtonProps {
                    variant: OxyVariant::Link,
                    ..Default::default()
                }),
            ),
            button(
                || "Info",
                Some(ButtonProps {
                    variant: OxyVariant::Info,
                    ..Default::default()
                }),
            ),
            button(
                || "Success",
                Some(ButtonProps {
                    variant: OxyVariant::Success,
                    ..Default::default()
                }),
            ),
            button(
                || "Warning",
                Some(ButtonProps {
                    variant: OxyVariant::Warning,
                    ..Default::default()
                }),
            ),
            button(
                || "Error",
                Some(ButtonProps {
                    variant: OxyVariant::Error,
                    ..Default::default()
                }),
            ),
        ))
        .style(|s| s.gap(4., 4.)),
        label(|| "Button sizes"),
        stack((
            button(
                || "Large",
                Some(ButtonProps {
                    size: OxySize::Large,
                    ..Default::default()
                }),
            ),
            button(
                || "Normal",
                Some(ButtonProps {
                    size: OxySize::Normal,
                    ..Default::default()
                }),
            ),
            button(
                || "Small",
                Some(ButtonProps {
                    size: OxySize::Small,
                    ..Default::default()
                }),
            ),
            button(
                || "Tiny",
                Some(ButtonProps {
                    size: OxySize::Tiny,
                    ..Default::default()
                }),
            ),
        ))
        .style(|s| s.gap(4., 4.)),
        label(|| "Outlined buttons"),
        stack((
            button(
                || "Info",
                Some(ButtonProps {
                    variant: OxyVariant::Info,
                    outlined: true,
                    ..Default::default()
                }),
            ),
            button(
                || "Success",
                Some(ButtonProps {
                    variant: OxyVariant::Success,
                    outlined: true,
                    ..Default::default()
                }),
            ),
            button(
                || "Warning",
                Some(ButtonProps {
                    variant: OxyVariant::Warning,
                    outlined: true,
                    ..Default::default()
                }),
            ),
            button(
                || "Error",
                Some(ButtonProps {
                    variant: OxyVariant::Error,
                    outlined: true,
                    ..Default::default()
                }),
            ),
        ))
        .style(|s| s.gap(4., 4.)),
        label(|| "Checkboxes").style(|s| {
            s.width_full()
                .font_size(20.)
                .items_center()
                .justify_center()
        }),
        label(|| "Checkbox sizes"),
        h_stack((
            checkbox_with_state(Some(CheckboxProps {
                size: OxySize::Large,
                ..Default::default()
            })),
            checkbox_with_state(None),
            checkbox_with_state(Some(CheckboxProps {
                size: OxySize::Small,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                size: OxySize::Tiny,
                ..Default::default()
            })),
        ))
        .style(|s| s.gap(4., 4.)),
        label(|| "Checkbox variants"),
        h_stack((
            checkbox_with_state(None),
            checkbox_with_state(Some(CheckboxProps {
                variant: OxyVariant::Neutral,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                variant: OxyVariant::Primary,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                variant: OxyVariant::Secondary,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                variant: OxyVariant::Accent,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                variant: OxyVariant::Ghost,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                variant: OxyVariant::Link,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                variant: OxyVariant::Info,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                variant: OxyVariant::Success,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                variant: OxyVariant::Warning,
                ..Default::default()
            })),
            checkbox_with_state(Some(CheckboxProps {
                variant: OxyVariant::Error,
                ..Default::default()
            })),
        ))
        .style(|s| s.gap(4., 4.)),
        // label(|| "LABELED CHECKBOXES"),
        // labeled_checkbox(checked, || "oxytail labeled", None).on_click_stop(move |_| {
        //     set_checked.update(|checked| *checked = !*checked);
        // }),
        // label(|| "SLIDERS"),
        // slider(|| 50.0).style(|s| s.height(15).width(200)),
    )),))
}

fn main() {
    let window_config = WindowConfig::default()
        .size(Size {
            width: 1200.0,
            height: 500.0,
        })
        .themed(false);

    init_theme(Theme::Dark);
    let root_view = app_view();
    let root_view = root_view.style(|s| {
        s.width_full()
            .background(Color::rgb8(29, 35, 42))
            .color(Color::rgb8(166, 173, 187))
            .padding(16.)
    });

    let app = Application::new().window(move |_| root_view, Some(window_config));

    app.run();
}
