use floem::{view::View, views::v_stack};
use oxytail_base::widgets::{
    common_props::OxyVariant,
    progress::{progress, ProgressProps},
};

pub fn progresses_variants() -> impl View {
    v_stack((
        progress(
            20,
            80,
            Some(ProgressProps {
                variant: OxyVariant::Primary,
                ..Default::default()
            }),
        ),
        progress(
            40,
            80,
            Some(ProgressProps {
                variant: OxyVariant::Primary,
                ..Default::default()
            }),
        ),
        progress(
            60,
            80,
            Some(ProgressProps {
                variant: OxyVariant::Primary,
                ..Default::default()
            }),
        ),
    ))
}
