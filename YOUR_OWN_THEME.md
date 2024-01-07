Creating your own theme in oxytail is mean to be as easy as possible.

1. Create a new lib package with `cargo new <package_name> --lib`
2. Create a public enum with your theme

```rs
#[derive(Default)]
pub enum SuperTheme {
    #[default]
    SuperThemeDark,
}

```

3. Implement `ThemeStyling` trait for your enum

```rs
use floem::style::Style;
use oxytail_base::{
    themes::ThemeStyling,
    widgets::button::{ButtonProps},
};

impl ThemeStyling for Theme {
    fn get_button_style(&self, button_props: ButtonProps) -> Box<dyn Fn(Style) -> Style> {
        // Return a boxed function describing how your style should behave based on button props
        Box::new(|s: Style| {
            // I want my text BIG for every button
            s.font_size(100.0)
        }) 
    }
    //...
}

```

You can also expose other nice-to-haves you want users of your theme to have. A good practice is to expose stuff like variant colors, sizes... so if anyone wants to do something custom using your theme can do that easily.

4. That's it! Now users can use your theme.

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