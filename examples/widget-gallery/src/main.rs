use floem::{
    kurbo::Size,
    peniko::Color,
    reactive::create_signal,
    view::View,
    views::{label, stack, v_stack, Decorators},
    widgets::slider::slider,
    window::WindowConfig,
    Application, EventPropagation,
};
use oxytail_base::{
    init_theme,
    widgets::{
        button::{button, ButtonProps, ButtonSize, ButtonVariant},
        checkbox::checkbox,
        checkbox::labeled_checkbox,
    },
};
use oxytail_theme_dark::Theme;

fn app_view() -> impl View {
    // create a counter reactive signal with initial value 0
    let (counter, set_counter) = create_signal(0);
    let (checked, set_checked) = create_signal(true);

    // create user interface with Floem view functions
    stack((v_stack((
        label(|| "Button variants"),
        stack((
            button(|| "Default", None),
            button(
                || "Neutral",
                Some(ButtonProps {
                    variant: ButtonVariant::Neutral,
                    ..Default::default()
                }),
            ),
            button(
                || "Primary",
                Some(ButtonProps {
                    variant: ButtonVariant::Primary,
                    ..Default::default()
                }),
            ),
            button(
                || "Secondary",
                Some(ButtonProps {
                    variant: ButtonVariant::Secondary,
                    ..Default::default()
                }),
            ),
            button(
                || "Accent",
                Some(ButtonProps {
                    variant: ButtonVariant::Accent,
                    ..Default::default()
                }),
            ),
            button(
                || "Ghost",
                Some(ButtonProps {
                    variant: ButtonVariant::Ghost,
                    ..Default::default()
                }),
            ),
            button(
                || "Link",
                Some(ButtonProps {
                    variant: ButtonVariant::Link,
                    ..Default::default()
                }),
            ),
            button(
                || "Info",
                Some(ButtonProps {
                    variant: ButtonVariant::Info,
                    ..Default::default()
                }),
            ),
            button(
                || "Success",
                Some(ButtonProps {
                    variant: ButtonVariant::Success,
                    ..Default::default()
                }),
            ),
            button(
                || "Warning",
                Some(ButtonProps {
                    variant: ButtonVariant::Warning,
                    ..Default::default()
                }),
            ),
            button(
                || "Error",
                Some(ButtonProps {
                    variant: ButtonVariant::Error,
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
                    size: ButtonSize::Large,
                    ..Default::default()
                }),
            ),
            button(
                || "Normal",
                Some(ButtonProps {
                    size: ButtonSize::Normal,
                    ..Default::default()
                }),
            ),
            button(
                || "Small",
                Some(ButtonProps {
                    size: ButtonSize::Small,
                    ..Default::default()
                }),
            ),
            button(
                || "Tiny",
                Some(ButtonProps {
                    size: ButtonSize::Tiny,
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
                    variant: ButtonVariant::Info,
                    outlined: true,
                    ..Default::default()
                }),
            ),
            button(
                || "Success",
                Some(ButtonProps {
                    variant: ButtonVariant::Success,
                    outlined: true,
                    ..Default::default()
                }),
            ),
            button(
                || "Warning",
                Some(ButtonProps {
                    variant: ButtonVariant::Warning,
                    outlined: true,
                    ..Default::default()
                }),
            ),
            button(
                || "Error",
                Some(ButtonProps {
                    variant: ButtonVariant::Error,
                    outlined: true,
                    ..Default::default()
                }),
            ),
        ))
        .style(|s| s.gap(4., 4.)),
        label(|| "CHECKBOXES(first-floem, second-oxytail)"),
        checkbox(checked).on_click_stop(move |_| {
            set_checked.update(|checked| *checked = !*checked);
        }),
        label(|| "LABELED CHECKBOXES"),
        labeled_checkbox(checked, || "oxytail labeled").on_click_stop(move |_| {
            set_checked.update(|checked| *checked = !*checked);
        }),
        label(|| "SLIDERS"),
        slider(|| 50.0).style(|s| s.height(15).width(200)),
    )),))
}

fn main() {
    let window_config = WindowConfig::default()
        .size(Size {
            width: 1000.0,
            height: 500.0,
        })
        .themed(false);

    init_theme(Theme::Dark);
    let root_view = app_view();
    let root_view = root_view.style(|s| {
        s.width_full()
            .background(Color::rgb8(29, 35, 42))
            .color(Color::rgb8(166, 173, 187))
    });

    let app = Application::new().window(move |_| root_view, Some(window_config));

    app.run();
}
