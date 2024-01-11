use btn::{btn_outlines, btn_sizes, btn_variants};
use checkboxes::{checkboxes_sizes, checkboxes_variants, labeled_checkboxes};
use floem::{
    event::{Event, EventListener},
    keyboard::{Key, NamedKey},
    kurbo::Size,
    peniko::Color,
    reactive::create_signal,
    style::{Background, CursorStyle, Transition},
    unit::UnitExt,
    view::View,
    views::{
        container, container_box, h_stack, label, scroll, stack, tab, v_stack, virtual_stack,
        ContainerBox, Decorators, VirtualDirection, VirtualItemSize,
    },
    window::WindowConfig,
    Application, EventPropagation,
};

use headers::headers_sizes;
use inputs::{text_input_sizes, text_input_variants};
use oxytail_base::{init_theme, widgets::button::button};
use oxytail_theme_dark::Theme;
use radio_buttons::{
    disabled_labeled_radio_variants, labeled_radio_variants, radio_sizes, radio_variants,
};
use toggles::{toggle_sizes, toggle_variants};

mod btn;
mod checkboxes;
pub mod headers;
mod inputs;
mod radio_buttons;
mod toggles;

fn padded_container_box(child: impl View + 'static) -> ContainerBox {
    container_box(child).style(|s| s.padding(10.))
}

fn app_view() -> impl View {
    let tabs: im::Vector<&str> = vec![
        "Intro", "Header", "Button", "Radio", "Checkbox", "Input", "Toggle",
    ]
    .into_iter()
    .collect();
    let (tabs, _set_tabs) = create_signal(tabs);
    let (active_tab, set_active_tab) = create_signal(0);

    let left_nav_list = container(
        scroll({
            virtual_stack(
                VirtualDirection::Vertical,
                VirtualItemSize::Fixed(Box::new(|| 36.0)),
                move || tabs.get(),
                move |item| *item,
                move |item| {
                    let index = tabs
                        .get_untracked()
                        .iter()
                        .position(|it| *it == item)
                        .unwrap();
                    stack((label(move || item).style(|s| s.font_size(18.0)),))
                        .on_click_stop(move |_| {
                            set_active_tab.update(|v: &mut usize| {
                                *v = tabs
                                    .get_untracked()
                                    .iter()
                                    .position(|it| *it == item)
                                    .unwrap();
                            });
                        })
                        .on_event(EventListener::KeyDown, move |e| {
                            if let Event::KeyDown(key_event) = e {
                                let active = active_tab.get();
                                if key_event.modifiers.is_empty() {
                                    match key_event.key.logical_key {
                                        Key::Named(NamedKey::ArrowUp) => {
                                            if active > 0 {
                                                set_active_tab.update(|v| *v -= 1)
                                            }
                                            EventPropagation::Stop
                                        }
                                        Key::Named(NamedKey::ArrowDown) => {
                                            if active < tabs.get().len() - 1 {
                                                set_active_tab.update(|v| *v += 1)
                                            }
                                            EventPropagation::Stop
                                        }
                                        _ => EventPropagation::Continue,
                                    }
                                } else {
                                    EventPropagation::Continue
                                }
                            } else {
                                EventPropagation::Continue
                            }
                        })
                        .keyboard_navigatable()
                        .draggable()
                        .style(move |s| {
                            s.flex_row()
                                .padding(5.0)
                                .width(100.pct())
                                .height(36.0)
                                .transition(Background, Transition::linear(0.4))
                                .items_center()
                                .border_bottom(1.0)
                                .border_color(Color::LIGHT_GRAY)
                                .apply_if(index == active_tab.get(), |s| {
                                    s.background(Color::GRAY.with_alpha_factor(0.6))
                                })
                                .focus_visible(|s| s.border(2.).border_color(Color::BLUE))
                                .hover(|s| {
                                    s.background(Color::LIGHT_GRAY)
                                        .apply_if(index == active_tab.get(), |s| {
                                            s.background(Color::GRAY)
                                        })
                                        .cursor(CursorStyle::Pointer)
                                })
                        })
                },
            )
            .style(|s| s.flex_col().width(140.0))
        })
        .style(|s| {
            s.flex_col()
                .width(140.0)
                .flex_grow(1.0)
                .min_height(0)
                .flex_basis(0)
        }),
    )
    .style(|s| {
        s.border(1.0)
            .border_color(Color::GRAY)
            .flex_grow(1.0)
            .min_height(0)
    });

    let id = left_nav_list.id();
    let inspector = button(|| "Open Inspector", None)
        .on_click_stop(move |_| {
            id.inspect();
        })
        .style(|s| s);

    let left = v_stack((left_nav_list, inspector)).style(|s| s.height_full().gap(0.0, 5.0));

    let tab = tab(
        move || active_tab.get(),
        move || tabs.get(),
        |it| *it,
        |it| match it {
            "Intro" => padded_container_box(v_stack(
                (
                    label(|| "This is a demo of widgets provided by oxytail. These widgets are just an extension on top of floem."),
                    label(|| "Floem itself, in addition to widgets also provides more components. You can mix&match them as you please."))
            )),
            "Header" => padded_container_box(headers_sizes()),
            "Button" => padded_container_box(v_stack((btn_variants(), btn_outlines(), btn_sizes()))),
            "Radio" => padded_container_box(v_stack((radio_variants(),radio_sizes(),labeled_radio_variants(),disabled_labeled_radio_variants()))),
            "Checkbox" => padded_container_box(v_stack((
                checkboxes_variants(),
                checkboxes_sizes(),
                labeled_checkboxes(),
            ))),
            "Input" => padded_container_box(v_stack((text_input_variants(), text_input_sizes()))),
            "Toggle" => padded_container_box(v_stack((toggle_variants(), toggle_sizes()))),
            _ => padded_container_box(label(|| "Not implemented".to_owned())),
        },
    )
    .style(|s| s.flex_col().items_start());

    let tab = scroll(tab).style(|s| s.flex_basis(0).min_width(0).flex_grow(1.0));

    let view = h_stack((left, tab))
        .style(|s| s.padding(5.0).width_full().height_full().gap(5.0, 0.0))
        .window_title(|| "Widget Gallery".to_owned());

    let id = view.id();
    view.on_event_stop(EventListener::KeyUp, move |e| {
        if let Event::KeyUp(e) = e {
            if e.key.logical_key == Key::Named(NamedKey::F11) {
                id.inspect();
            }
        }
    })
}

fn main() {
    let window_config = WindowConfig::default()
        .size(Size {
            width: 1200.0,
            height: 800.0,
        })
        .apply_default_theme(false);

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
