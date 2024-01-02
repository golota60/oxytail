use floem::{
    kurbo::Size,
    reactive::create_signal,
    view::View,
    views::{label, stack, v_stack, Decorators},
    widgets::slider::slider,
    window::WindowConfig,
    Application, EventPropagation,
};
use oxytail::{
    init_theme,
    themes::{StyleEnhancer, Theme},
    widgets::{
        button::{button as oxy_button, ButtonProps, ButtonVariant},
        checkbox::checkbox as oxy_checkbox,
        checkbox::labeled_checkbox as oxy_labeled_checkbox,
    },
};

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
                oxy_button(|| "Default", None).on_click(move |_| {
                    set_counter.update(|value| *value += 1);
                    EventPropagation::Stop
                }),
                oxy_button(
                    || "Neutral",
                    Some(ButtonProps {
                        variant: ButtonVariant::Neutral,
                        ..Default::default()
                    }),
                )
                .on_click(move |_| {
                    set_counter.update(|value| *value += 1);
                    EventPropagation::Stop
                }),
                oxy_button(
                    || "Primary",
                    Some(ButtonProps {
                        variant: ButtonVariant::Primary,
                        ..Default::default()
                    }),
                )
                .on_click(move |_| {
                    set_counter.update(|value| *value -= 1);
                    EventPropagation::Stop
                }),
                oxy_button(
                    || "Secondary",
                    Some(ButtonProps {
                        variant: ButtonVariant::Secondary,
                        ..Default::default()
                    }),
                )
                .on_click(move |_| {
                    set_counter.update(|value| *value -= 1);
                    EventPropagation::Stop
                }),
            )),
            label(|| "CHECKBOXES(first-floem, second-oxytail)"),
            oxy_checkbox(checked).on_click_stop(move |_| {
                set_checked.update(|checked| *checked = !*checked);
            }),
            label(|| "LABELED CHECKBOXES"),
            oxy_labeled_checkbox(checked, || "oxytail labeled").on_click_stop(move |_| {
                set_checked.update(|checked| *checked = !*checked);
            }),
            label(|| "SLIDERS"),
            slider(|| 50.0).style(|s| s.height(15).width(200)),
        )),
    ))
}

fn main() {
    let selected_theme = Theme::Dark;

    let window_config = WindowConfig::default().size(Size {
        width: 1000.0,
        height: 500.0,
    });

    init_theme(selected_theme);
    let root_view = app_view();
    let root_view = root_view.style(|s| s.width_full().enhance());

    let app = Application::new().window(move |_| root_view, Some(window_config));

    app.run();
}
