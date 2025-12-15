// Main NSOD config environment variable key.
#[macro_export]
macro_rules! cfg_env {
    () => {
        "_NSOD_CFG"
    };
}

// Validate credential sources before launching via the wrapper.
#[macro_export]
macro_rules! validate_sources {
    () => {
        true
    };
}

// Validate each route verbosely.
#[macro_export]
macro_rules! validate_verbose {
    () => {
        true
    };
}