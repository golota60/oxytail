<div align="center">
	<h1>Oxytail</h1>
	<p>
		<b>Styled widget library built on top of <a href="https://github.com/lapce/floem">floem</a>, inspired by <a href="https://daisyui.com/components/button/">daisyUI</a></b>
	</p>
	<br>
</div>

Features:
 - Cross-system support
 - HIGHLY customizable - modify and create [your own themes](YOUR_OWN_THEME.md) easily
 - Supports dynamic theme switching(example needed!)
 - Built on top of [Floem](https://github.com/lapce/floem)
 - Loosely inspired by [daisyUI](https://daisyui.com/components/button/)


Showcase:

<img src="https://github.com/golota60/oxytail/blob/main/img/demo.png">

## Installation

`oxytail` consists of the main "base" package(`oxytail-base`), which loads the theme you want, and theme packages(currently only `oxytail-theme-dark`. More coming soon!).

Next, you need to choose your theme, and install it. Currently, `oxytail` comes with only one theme, `oxytail-theme-dark`, but if it doesn't suit your needs you can [write your own one](YOUR_OWN_THEME.md). It's still early days for this project, so some widgets are missing.

First, add floem(floem is not on crates.io just yet, so we need to add it as a git dependency) dependency to your `Cargo.toml`.

Next, add `oxytail-base`(to load a theme) and `oxytail-theme-dark`(or any other theme) also as a git dependency, since we rely on `floem`, which is not published.


```toml
[dependencies]

floem = { git = "https://github.com/lapce/floem", rev = "e304d30f8771a28788904d64464d8fd192d07439" }
oxytail-theme-dark = { git = "https://github.com/golota60/oxytail"}
oxytail-base = { git = "https://github.com/golota60/oxytail"}
```

Done! Now, initialize the theme, and simply use `floem` like normal, *except* import your widgets from `oxytail-base` instead of `floem`!

```rs
use floem::kurbo::Size;
use floem::peniko::Color;
use floem::reactive::create_signal;
use floem::view::View;
use floem::views::{h_stack, label, v_stack, Decorators};
use floem::window::WindowConfig;
use floem::Application;
use oxytail_base::{init_theme, widgets::button::button};
use oxytail_theme_dark::Theme;

fn app_view() -> impl View {
    // Create a reactive signal with a counter value, defaulting to 0
    let (counter, set_counter) = create_signal(0);

    // Create a vertical layout
    v_stack((
        // The counter value updates automatically, thanks to reactivity
        label(move || format!("Value: {}", counter.get())),
        // Create a horizontal layout
        h_stack((
            button(|| "Increment", None).on_click_stop(move |_| {
                set_counter.update(|value| *value += 1);
            }),
            button(|| "Decrement", None).on_click_stop(move |_| {
                set_counter.update(|value| *value -= 1);
            }),
        )),
    ))
}

fn main() {
    let window_config = WindowConfig::default()
        .size(Size {
            width: 1200.0,
            height: 500.0,
        })
        // 1. We don't want any default `floem` styling to be interfering with ours,
        // so we need to disable the default styling.
        .apply_default_theme(false);

    // 2. We need to initialize our theme
    init_theme(Theme::Dark);
    // That's all! Now import some widgets from `oxytail_base` and you're using oxytail!

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

## Doumentation/Examples

The best showcase is `examples/widget-gallery` package. Run it with `cargo run -p widget-gallery`. Docs.rs soon. 

## Contributing

If you notice any bugs, missing features or nice-to-haves, don't hesistate to open an issue.

If you want to submit a custom theme(or just link your own in this README for other people to know) to be a part of this library, also don't hesitate and simply create a new package in the workspace(see `oxytail-theme-dark` folder for guidance).[writing your own theme](YOUR_OWN_THEME.md)

If you need some widgets which currently don't exist, please consider submitting them to the upstream `floem` library first.

If you think some of the docs are unclear, also feel free to create an issue!


## Roadmap

Obviously, we do not have many widgets which are present in daisyUI. The most immediate roadmap focuses on two things: More widgets, more prop support and more themes.

Expect breaking changes.

Short term roadmap(primitives only):

- [x] Toggles(`toggle`)
- [x] Visual Dividers(`text_divider`)
- [x] Radio buttons(`radio_button`)
- [x] H1/H2/H3/H4/H5 Equivalents(`text_header`)
- [ ] Dropdowns/Selects
- [ ] Tooltips
- [ ] Badges
- [ ] Progress bars
- [ ] [your suggestion](https://github.com/golota60/oxytail/issues/new)

## Notes

The idea for this project came from my slight frustration when developing this [tauri-based desk manager app](https://github.com/golota60/trayasen). While for that project tauri is an awesome pick, I was looking for a native solution, i.e. not using system WebView. I've chosen `floem` as a base, because of their really friendly and extensible styling.

I'm only aware of one other widget library for floem, and it's [floem-ui-kit](https://github.com/pieterdd/floem-ui-kit). I really like the simplicity of it, but I wanted something more customizable.


