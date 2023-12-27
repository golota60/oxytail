use std::collections::HashMap;

use floem::{peniko::Color, style::Style};

use crate::hashable_style_class;

#[derive(Default)]
pub enum Theme {
    #[default]
    Light,
    // Dark,
}

// Each class is the same across themes; only what the class name maps to changes.
hashable_style_class!(pub OxyButtonClass);

// Having the existing structs handled as enum values is a bit awkward.
#[derive(PartialEq, Eq, Hash)]
pub enum ValidOxyclasses {
    OxyButton(OxyButtonClass),
}

pub fn get_theme_styles(th: Theme) -> HashMap<ValidOxyclasses, fn(s: Style) -> Style> {
    // Each theme has a hashmap with styles for every existing widget
    // TODO: implement per-style styles
    match th {
        Theme::Light => {
            let mut hash: HashMap<ValidOxyclasses, fn(s: Style) -> Style> = HashMap::new();

            hash.insert(ValidOxyclasses::OxyButton(OxyButtonClass), |_| {
                Style::new().background(Color::rgb8(255, 0, 0))
            });

            hash
        }
    }
}
pub trait StyleEnhancer {
    fn enhance(self, current_theme_styles: HashMap<ValidOxyclasses, fn(s: Style) -> Style>)
        -> Self;
}

impl StyleEnhancer for Style {
    fn enhance(
        mut self,
        current_theme_styles: HashMap<ValidOxyclasses, fn(s: Style) -> Style>,
    ) -> Self {
        for (classes, e) in current_theme_styles {
            let classname = match classes {
                ValidOxyclasses::OxyButton(asd) => asd,
            };

            self = self.class(classname, e);
        }

        self
    }
}
