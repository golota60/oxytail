use floem::{view::View, views::v_stack};
use oxytail_base::widgets::{
    common_props::OxySize,
    header::{header, HeaderProps},
};

pub fn headers_sizes() -> impl View {
    v_stack((
        header(
            || "I am Large",
            Some(HeaderProps {
                size: OxySize::Large,
                ..Default::default()
            }),
        ),
        header(|| "I am Normal", None),
        header(
            || "I am Small",
            Some(HeaderProps {
                size: OxySize::Small,
                ..Default::default()
            }),
        ),
        header(
            || "I am Tiny",
            Some(HeaderProps {
                size: OxySize::Tiny,
                ..Default::default()
            }),
        ),
    ))
}
