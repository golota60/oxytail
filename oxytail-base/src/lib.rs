pub mod themes;
pub mod widgets;

use std::sync::OnceLock;

use themes::ThemeStyling;

pub static GLOBAL_THEME: OnceLock<Box<dyn ThemeStyling + Send + Sync>> = OnceLock::new();

pub(crate) fn get_current_theme() -> &'static Box<dyn ThemeStyling + Send + Sync> {
    GLOBAL_THEME.get().expect("Oxytail widget received no theme while trying to render. Did you forget to run `init_theme(theme)`")
}

pub fn init_theme(theme: impl ThemeStyling + Send + Sync + 'static) {
    GLOBAL_THEME.get_or_init(|| Box::new(theme));
}
