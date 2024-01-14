Creating your own theme in oxytail is meant to be as easy as possible.

1. Create a new lib package with `cargo new <package_name> --lib`
2. Create a public enum with your theme

```rs
#[derive(Default)]
pub enum SuperTheme {
    #[default]
    SuperThemeDark,
}

```

Now, the easiest way to create a theme is to override the `oxytail-theme-defaults` package, which includes what can be described as "sane defaults" for behaviors of the components.
You can specify colors and override those behaviors if needed.

So, install `floem`, `oxytail-theme-defaults` and `oxytail-base`.
```toml
[dependencies]
# Please check README.md for the most up-to-date `rev` value!
floem = { git = "https://github.com/lapce/floem", rev = "e304d30f8771a28788904d64464d8fd192d07439" 
oxytail-base = { path = "../oxytail-base" }
oxytail-theme-defaults = { path = "../oxytail-theme-defaults" }
```


3. Implement `ThemeStyling` trait for your enum

```rs
use floem::{peniko::Color, style::Style};
use oxytail_base::{
    themes::{DefaultThemeProps, ThemeStyling},
    widgets::{
        button::ButtonProps, checkbox::CheckboxProps, common_props::OxyVariant,
    },
};
use oxytail_theme_defaults::DEFAULT_ACCENT;

// These should return a `Color` for each variant.
fn get_variant_colors(oxy_variant: OxyVariant) -> Color {
    // ...
}
fn get_hover_variant_colors(oxy_variant: OxyVariant) -> Color {
    // ...
}
fn get_active_variant_colors(oxy_variant: OxyVariant) -> Color {
    // ...
}

impl ThemeStyling for Theme {
    // This function should return the leading colors for your theme.
    fn theme_defaults(&self) -> DefaultThemeProps {
        DefaultThemeProps {
            get_variant_colors,
            get_hover_variant_colors,
            get_active_variant_colors,
            light_text_color: Color::rgb8(166, 173, 187),
            dark_text_color: Color::rgb8(25, 2, 17),
            __default_accent: DEFAULT_ACCENT,
        }
    }

    // Rest of the interface controls stuff on a per-component basis.
    // Start with returning `oxytail_theme_defaults::ThemeDefault::<func>` for a set of nicely working defaults, and override it if needed!
    fn get_button_style(&self, button_props: ButtonProps) -> Box<dyn Fn(Style) -> Style> {
        oxytail_theme_defaults::ThemeDefault::get_button_style(button_props, self.theme_defaults())
    }

    fn get_checkbox_style(&self, checkbox_props: CheckboxProps) -> Box<dyn Fn(Style) -> Style> {
        oxytail_theme_defaults::ThemeDefault::get_checkbox_style(
            checkbox_props,
            self.theme_defaults(),
        )
    }

    // ...rest of the trait
    // Each function declared in the trait, has a corresponding function with same defaults in `oxytail_theme_default` package. 
}


```

You can also expose other nice-to-haves you want users of your theme to have. A good practice is to expose stuff like variant colors, sizes... so if anyone wants to do something custom using your theme can do that easily.

4. That's it! Now you can use your theme, just like any other. 

```rs
fn main() {
    let window_config = WindowConfig::default()
        .size(Size {
            width: 1200.0,
            height: 500.0,
        })
        // `themed` should be false - we don't want default styles to be interfering with your theme!
        .themed(false);

    // Initialize the theme
    init_theme(SuperTheme::SuperThemeDark);

    // Eveything else is the same as if it was a normal floem app.
    let root_view = app_view();
    let app = Application::new().window(move |_| root_view, Some(window_config));

    app.run();
}
```


Something's still not clear? Please don't hesitate to open an issue! Also, if you need an example on this docs applied in practice, check out `oxytail-theme-dark` package in this repo!