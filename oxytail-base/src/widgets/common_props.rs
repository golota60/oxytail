#[derive(Default, Clone, Copy)]
pub enum OxySize {
    Large,
    #[default]
    Normal,
    Small,
    Tiny,
}

#[derive(Default, Clone, Copy)]
pub enum OxyVariant {
    #[default]
    Default,

    Neutral,
    Primary,
    Secondary,
    Accent,
    Ghost,
    Link,

    Info,
    Success,
    Warning,
    Error,
}
