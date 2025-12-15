#[macro_export]
macro_rules! db_en {
    () => {
        [
            "cfg_start",
            "cfg_create",
            "cfg_bad_name",
            "cfg_edit",
            "cfg_delete",
            "cfg_bin",
            "cfg_route",
            "cfg_add_route",
            "cfg_source",
            "cfg_list",
            "path_not_absolute",

            "input_src_env",
            "input_src_file",
        ]
    };
}

#[macro_export]
macro_rules! db_set {
    () => {
        "en"
    };
}

    

#[macro_export]
macro_rules! interact_char {
    () => {
        ">"
    };
}