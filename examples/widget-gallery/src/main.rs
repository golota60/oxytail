use btn::{btn_outlines, btn_sizes, btn_variants};
use checkboxes::{checkboxes_sizes, checkboxes_variants};
use floem::{
    kurbo::Size,
    peniko::Color,
    reactive::create_signal,
    view::View,
    views::{label, v_stack, Decorators},
    window::WindowConfig,
    Application,
};

use inputs::{text_input_sizes, text_input_variants};
use oxytail_base::{
    init_theme,
    widgets::{
        checkbox::labeled_checkbox, checkbox::CheckboxProps, common_props::OxyVariant,
        toggle::toggle,
    },
};
use oxytail_theme_dark::Theme;
use toggles::{toggle_sizes, toggle_variants};

mod btn;
mod checkboxes;
mod inputs;
mod toggles;

fn app_view() -> impl View {
    let (checked, set_checked) = create_signal(true);
    v_stack((
        label(|| "Buttons").style(|s| {
            s.width_full()
                .font_size(20.)
                .items_center()
                .justify_center()
        }),
        btn_variants(),
        btn_sizes(),
        btn_outlines(),
        label(|| "Checkboxes").style(|s| {
            s.width_full()
                .font_size(20.)
                .items_center()
                .justify_center()
        }),
        checkboxes_sizes(),
        checkboxes_variants(),
        v_stack((
            label(|| "Labeled checkboxes"),
            labeled_checkbox(checked, || "I am the default!", None).on_click_stop(move |_| {
                set_checked.update(|checked| *checked = !*checked);
            }),
            labeled_checkbox(
                checked,
                || "I am primary!",
                Some(CheckboxProps {
                    variant: OxyVariant::Primary,
                    ..Default::default()
                }),
            )
            .on_click_stop(move |_| {
                set_checked.update(|checked| *checked = !*checked);
            }),
        ))
        .style(|s| s.justify_start().items_start().gap(5., 5.)),
        label(|| "Text inputs").style(|s| {
            s.width_full()
                .font_size(20.)
                .items_center()
                .justify_center()
        }),
        text_input_variants(),
        text_input_sizes().style(|s| s.margin_top(20.)),
        toggle(create_signal(true).0, None),
        toggle_variants(),
        toggle_sizes(),
        toggle_sizes(),
        // toggle_sizes(), // labeled_checkbox(checked, || "oxytail labeled", None).on_click_stop(move |_| {
                        //     set_checked.update(|checked| *checked = !*checked);
                        // }),
                        // label(|| "SLIDERS"),
                        // slider(|| 50.0).style(|s| s.height(15).width(200)),
    ))
}

fn main() {
    let window_config = WindowConfig::default()
        .size(Size {
            width: 1200.0,
            height: 800.0,
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
