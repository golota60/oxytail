use floem::{window::WindowConfig, Application};
use themes::Theme;

pub mod themes;
pub mod utils;
pub mod widgets;

pub struct ExtendedWindowConfig {
    window_config: WindowConfig,
    selected_theme: Theme,
}
