pub mod themes;
pub mod widgets;

use std::{any::Any, sync::OnceLock};

use floem::{
    reactive::ReadSignal,
    view::View,
    views::{container, label, v_stack},
};
use themes::ThemeStyling;
use widgets::modal::modal;

pub static GLOBAL_THEME: OnceLock<Box<dyn ThemeStyling + Send + Sync>> = OnceLock::new();
// App head. We need a global reference to it, to render topmost level stuff.
pub static TOPMOST_HOOK: OnceLock<Box<dyn View + Send + Sync>> = OnceLock::new();

pub(crate) fn get_current_theme() -> &'static Box<dyn ThemeStyling + Send + Sync> {
    GLOBAL_THEME.get().expect("Oxytail widget received no theme while trying to render. Did you forget to run `init_oxytail(theme, root_component)`")
}

pub(crate) fn get_current_topmost() -> &'static Box<dyn View + Send + Sync> {
    TOPMOST_HOOK.get().expect("Oxytail widget received no root_component while trying to render. Did you forget to run `init_oxytail(theme, root_component)`")
}

pub fn init_theme(theme: impl ThemeStyling + Send + Sync + 'static) {
    GLOBAL_THEME.get_or_init(|| Box::new(theme));
}

pub fn init_topmost_hook(
    topmost: impl View + Send + Sync + 'static,
) -> impl View + Send + Sync + 'static {
    TOPMOST_HOOK.get_or_init(|| Box::new(topmost));
    topmost
}

pub fn init_oxytail(
    theme: impl ThemeStyling + Send + Sync + 'static,
    topmost: impl View + Send + Sync + 'static,
) -> impl View {
    init_theme(theme);
    init_topmost_hook(topmost);
    topmost
}

pub fn mut_topmost_hook() {}

pub fn oxytail_wrapper<V: View + 'static>(child: V) -> impl View {
    let modall = label(|| "ASd");
    v_stack((modall, child))
}
