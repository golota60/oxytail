pub mod themes;
pub mod widgets;

use std::sync::OnceLock;

use themes::{ButtonVariantStyles, ThemeStyles};

pub static GLOBAL_THEME: OnceLock<Box<dyn ButtonVariantStyles + Send + Sync>> = OnceLock::new();

pub fn init_theme(theme: impl ButtonVariantStyles + Send + Sync + 'static) {
    GLOBAL_THEME.get_or_init(|| Box::new(theme));
}
