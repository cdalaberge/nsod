#[macro_export]
macro_rules! cfg_ld_path {
    () => {
        ".nsod/lib/libnsod_open_hook.so"
    };
}

#[macro_export]
macro_rules! cfg_ld_abs_path {
    () => {
        false
    };
}