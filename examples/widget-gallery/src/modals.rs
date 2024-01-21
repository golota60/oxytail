use floem::{
    reactive::create_signal,
    view::View,
    views::{label, v_stack, Decorators},
    EventPropagation,
};
use oxytail_base::widgets::{button::button, modal::modal};

pub fn modal_demo() -> impl View {
    let (open, set_open) = create_signal(true);

    v_stack((
        button(|| "open modal", None).on_click(move |_| {
            println!("clicked");
            set_open.update(|open| *open = !*open);

            EventPropagation::Stop
        }),
        modal(label(|| "modal content"), open),
    ))
}
