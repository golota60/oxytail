use floem::{
    style::Style,
    view::View,
    views::{text, v_stack, Decorators},
};
use oxytail_base::widgets::{
    common_props::OxySize,
    text_divider::text_divider,
    text_header::{text_header, HeaderProps},
};

pub fn headers_sizes() -> impl View {
    v_stack((
        text_header(
            "I am a large header",
            Some(HeaderProps {
                size: OxySize::Large,
                ..Default::default()
            }),
        ),
        text_divider(),
        text("The biggest header provided!"),
        text_header("I am a normal header", None),
        text_divider(),
        text("This one is a default size."),
        text_header(
            "I am a small header",
            Some(HeaderProps {
                size: OxySize::Small,
                ..Default::default()
            }),
        ),
        text_divider(),
        text("This one is small."),
        text_header(
            "I am a tiny header",
            Some(HeaderProps {
                size: OxySize::Tiny,
                ..Default::default()
            }),
        ),
        text_divider(),
        text("This one is tiiiny."),
    ))
    .style(|s: Style| s.width_full())
}
