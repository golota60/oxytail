pub mod themes;
pub mod widgets;

use std::sync::OnceLock;

use themes::ThemeStyling;

pub static GLOBAL_THEME: OnceLock<Box<dyn ThemeStyling + Send + Sync>> = OnceLock::new();

pub fn init_theme(theme: impl ThemeStyling + Send + Sync + 'static) {
    GLOBAL_THEME.get_or_init(|| Box::new(theme));
}
