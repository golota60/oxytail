## Oxytail

INSERT_DEMO_IMG_HERE

Ready to use native components for [Floem](https://github.com/lapce/floem), inspired by [daisyUI](https://daisyui.com/components/button/).

Features:
 - Cross-system support
 - HIGHLY customizable - modify and create [your own themes](YOUR_OWN_THEME.md) easily
 - Based on a proven GUI library

## Installation

`oxytail`  consists of the main "loader" package(`oxytail-loader`), which, well, loads the plugin you want.

Next, you need to choose your theme, and install it. Currently, `oxytail` comes with only one theme, `oxytail-theme-dark`, but if it doesn't suit your needs you can [write your own one](YOUR_OWN_THEME.md).

```
cargo add floem oxytail-loader oxytail-theme-dark
```

Next, simply use `floem` like normal, except with a few quirk regarding initializaiton.

```rs
fn main() {
    let window_config = WindowConfig::default()
        .size(Size {
            width: 1200.0,
            height: 500.0,
        })
        // 1. We don't want any default `floem` styling to be interfering with ours, so we need to disable the default styling.
        .themed(false);

    // 2. We need to initialize our them
    init_theme(Theme::Dark);
    // That's all!

    let root_view = app_view();
    let root_view = root_view.style(|s| {
        s.width_full()
            .background(Color::rgb8(29, 35, 42))
            .color(Color::rgb8(166, 173, 187))
            .padding(16.)
    });

    let app = Application::new().window(move |_| root_view, Some(window_config));

    app.run();
}

```


## Contributing

If you notice any bugs, missing features or nice-to-haves, don't hesistate to open an issue.

If you want to submit a custom theme(or just link your own in this README for other people to know) to be a part of this library, also don't hesitate and simply create a new package in the workspace(see `oxytail-theme-dark` folder for guidance).[writing your own theme](YOUR_OWN_THEME.md)

If you need some components which currently don't exist, please consider submitting them to the upstream `floem` library first.

If you think some of the docs are unclear, also feel free to create an issue!

## Notes

The idea for this project came from my slight frustration when developing this [tauri-based desk manager app](https://github.com/golota60/trayasen). While for that project tauri is an awesome pick, I was looking for a native solution, i.e. not using system WebView. I've chosen `floem` as a base, because of their really friendly and extensible styling.

I'm only aware of one other component library for floem, and it's [floem-ui-kit](https://github.com/pieterdd/floem-ui-kit). I really like the simplicity of it, but I wanted something more customizable.


