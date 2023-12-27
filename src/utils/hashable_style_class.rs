#[macro_export]
macro_rules! hashable_style_class {
    ($v:vis $name:ident) => {
        #[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
        $v struct $name;
        impl floem::style::StyleClass for $name {
            fn class_ref() -> floem::style::StyleClassRef {
                static INFO: floem::style::StyleClassInfo = floem::style::StyleClassInfo::new::<$name>();
                floem::style::StyleClassRef { info: &INFO }
            }
        }
    };
}
