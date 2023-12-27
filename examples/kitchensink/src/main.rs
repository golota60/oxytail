use floem::{
    kurbo::Size,
    peniko::Color,
    reactive::create_signal,
    style::Style,
    view::View,
    views::{label, stack, v_stack, Decorators},
    widgets::button,
    window::WindowConfig,
    Application, EventPropagation,
};
use oxytail::{
    themes::{get_theme_styles, OxyButtonClass, StyleEnhancer, Theme, ValidOxyclasses},
    widgets::button::button as oxybutton,
};

fn app_view() -> impl View {
    // create a counter reactive signal with initial value 0
    let (counter, set_counter) = create_signal(0);

    // create user interface with Floem view functions
    stack((
        label(move || format!("Value: {}", counter.get())),
        v_stack((
            button(|| "FLOEM").on_click(move |_| {
                set_counter.update(|value| *value += 1);
                EventPropagation::Stop
            }),
            oxybutton(|| "OXYTAIL").on_click(move |_| {
                set_counter.update(|value| *value -= 1);
                EventPropagation::Stop
            }),
        )),
    ))
}

fn main() {
    let window_config = WindowConfig::default()
        .size(Size {
            width: 1000.0,
            height: 500.0,
        })
        .themed(false);

    let root_view = app_view();
    let root_view = root_view.style(|_| {
        let current_theme_styles = get_theme_styles(Theme::Light);
        Style::new().enhance(current_theme_styles)
    });

    let app = Application::new().window(move |_| root_view, Some(window_config));

    app.run();
}

// let border = Color::rgb8(140, 140, 140);
// let padding = 5.0;
// let border_radius = 5.0;

// let hover_bg_color = Color::rgba8(228, 237, 216, 160);
// let focus_hover_bg_color = Color::rgb8(234, 230, 236);
// let active_bg_color = Color::rgb8(160, 160, 160);
// let focus_applied_style = Style::new().border_color(Color::rgb8(114, 74, 140));
// let focus_visible_applied_style = Style::new().outline(3.0);

// let focus_style = Style::new()
//     .outline_color(Color::rgba8(213, 208, 216, 150))
//     .focus(|_| focus_applied_style.clone())
//     .focus_visible(|_| focus_visible_applied_style.clone());

// let border_style = Style::new()
//     .disabled(|s| s.border_color(Color::rgb8(131, 145, 123).with_alpha_factor(0.3)))
//     .border(1.0)
//     .border_color(border)
//     .padding(padding)
//     .border_radius(border_radius)
//     .apply(focus_style.clone());

// let mut init_style = Style::new().enhance(current_theme_styles);

// let init_style = |_| {
//     Style::new().class(OxyButtonClass, |_| {
//         Style::new().background(Color::rgb8(0, 124, 0))
//     })
// };

// let root_view = root_view.style(move |_| {
//     Style::new()
//         .background(Color::rgb8(240, 240, 240))
//         .disabled(|s| {
//             s.background(Color::rgb8(180, 188, 175).with_alpha_factor(0.3))
//                 .border_color(Color::rgb8(131, 145, 123).with_alpha_factor(0.3))
//                 .color(Color::GRAY)
//         })
//         .active(|s| {
//             s.background(active_bg_color)
//                 .color(Color::WHITE.with_alpha_factor(0.9))
//         })
//         .transition(Background, Transition::linear(0.04))
//         .focus(|s| s.hover(|s| s.background(focus_hover_bg_color)))
//         .hover(|s| s.background(hover_bg_color))
//         .padding(padding)
//         .justify_center()
//         .items_center()
//         .apply(focus_style.clone())
//         .apply(border_style.clone())
//         .color(Color::rgb8(40, 40, 40))
// });
