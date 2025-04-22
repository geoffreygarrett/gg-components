
#[macro_export]
macro_rules! cn {
    ($($args:tt)*) => {{
        let composed = clsx::clsx!($($args)*);
        tailwind_fuse::tw_merge!(&composed)
    }};
}

