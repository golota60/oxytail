pub mod themes;
pub mod widgets;
use std::sync::OnceLock;

use themes::Theme;

pub static GLOBAL_THEME: OnceLock<Theme> = OnceLock::new();

pub fn init_theme(theme: Theme) {
    GLOBAL_THEME.get_or_init(|| theme);
}