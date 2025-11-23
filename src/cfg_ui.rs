#[macro_export]
macro_rules! db_en {
    () => {
        [
            "cfg_start",
            "cfg_create",
            "cfg_edit",
            "cfg_delete",
            "cfg_bin",
            "cfg_route",
            "cfg_add_route",
            "cfg_source",
            "cfg_source_simple",
            "input_src_env",
            "input_src_file",

            "vault_src_start",
            "vault_src_engine",
            "vault_src_addr",
            "vault_src_verify",
            "vault_src_timeout_yn",
            "vault_src_timeout_s",
            "vault_src_namespace_yn",
            "vault_src_namespace_str",

            "kv_mount",
            "kv_key",
            "kv_path",

            "vault_cred",
            "vault_identity_name",

            "invalid_addr",
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


#[macro_export]
macro_rules! base_ui_dir_path {
    () => {
        "/home/cdal/nsod/misc"
    };
}

#[macro_export]
macro_rules! usage_path {
    () => {
        "./misc/en/usage.txt"
    };
}

#[macro_export]
macro_rules! help_path {
    () => {
        "./misc/en/README_en.txt"
    };
}