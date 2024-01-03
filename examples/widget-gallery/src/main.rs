use floem::{
    kurbo::Size,
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
        button::{button, ButtonProps, ButtonVariant},
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
    stack((
        label(move || format!("Value: {}", counter.get())),
        v_stack((
            label(|| "BUTTONS"),
            stack((
                button(|| "Default", None).on_click(move |_| {
                    set_counter.update(|value| *value += 1);
                    EventPropagation::Stop
                }),
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
            )),
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
        )),
    ))
}

fn main() {
    let window_config = WindowConfig::default().size(Size {
        width: 1000.0,
        height: 500.0,
    });

    init_theme(Theme::Dark);
    let root_view = app_view();
    let root_view = root_view.style(|s| s.width_full());

    let app = Application::new().window(move |_| root_view, Some(window_config));

    app.run();
}
