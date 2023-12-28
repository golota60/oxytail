use floem::{
    kurbo::Size,
    reactive::create_signal,
    view::View,
    views::{label, stack, v_stack, Decorators},
    widgets::{button, checkbox, labeled_checkbox},
    window::WindowConfig,
    Application, EventPropagation,
};
use oxytail::{
    themes::{StyleEnhancer, Theme},
    widgets::{
        button::button as oxy_button, checkbox::checkbox as oxy_checkbox,
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
            button(|| "FLOEM").on_click(move |_| {
                set_counter.update(|value| *value += 1);
                EventPropagation::Stop
            }),
            oxy_button(|| "OXYTAIL").on_click(move |_| {
                set_counter.update(|value| *value -= 1);
                EventPropagation::Stop
            }),
            label(|| "CHECKBOXES(first-floem, second-oxytail)"),
            checkbox(checked).on_click_stop(move |_| {
                set_checked.update(|checked| *checked = !*checked);
            }),
            oxy_checkbox(checked).on_click_stop(move |_| {
                set_checked.update(|checked| *checked = !*checked);
            }),
            label(|| "LABELED CHECKBOXES"),
            labeled_checkbox(checked, || "floem labeled").on_click_stop(move |_| {
                set_checked.update(|checked| *checked = !*checked);
            }),
            oxy_labeled_checkbox(checked, || "oxytail labeled").on_click_stop(move |_| {
                set_checked.update(|checked| *checked = !*checked);
            }),
        )),
    ))
}

fn main() {
    let window_config = WindowConfig::default().size(Size {
        width: 1000.0,
        height: 500.0,
    });

    let root_view = app_view();
    let root_view = root_view.style(|s| s.enhance(Theme::Light));

    let app = Application::new().window(move |_| root_view, Some(window_config));

    app.run();
}
