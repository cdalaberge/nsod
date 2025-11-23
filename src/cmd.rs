use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;

use clearscreen::clear;
use url::Url;

use crate::ui;
use crate::nsod_cfg::{NsodCfg};
use crate::route::{NsodRoute, NsodSource, SimpleSource};
use crate::vault::{KV1Cfg, KV2Cfg, VaultCredentialSource, VaultIdentity, VaultSecretEngine, VaultSource};



pub fn __nsod_configure_main () -> Result<i32, Box<dyn std::error::Error>> {

    let db_owned = ui::load_db()?;
    let db = &db_owned;

    clear()?;
    loop {
        ui::print_from_key_interactive("cfg_start", db)?;
        match ui::get_input_i32(0, 3)? {
            0 => return Ok(0),
            1 => {
                __nsod_create_cfg(db)?;
            },
            2 => {
                //__nsod_ui_edit_cfg()?;
            },
            3 => {
                //__nsod_ui_delete_cfg()?;
            },
            _ => {
            },
        }
        clear()?;
    }
}


pub fn __nsod_get_source(db: &HashMap<String, String>) -> Result<NsodSource, Box<dyn std::error::Error>> {
    ui::print_from_key_interactive("cfg_source", db)?;
    match ui::get_input_i32(1, 3)? {
        1 => {
            ui::print_from_key_interactive("input_src_env", db)?;
            return Ok(NsodSource::Simple(SimpleSource::EnvironmentVariable(ui::get_input_string()?)));
        },
        2 => {

            ui::print_from_key_interactive("vault_src_start", db)?;
            let _ = ui::get_input_string()?; // just wait for the user to press 'enter'.

            ui::print_from_key_interactive("vault_src_addr", db)?;
            let addr = __nsod_get_addr(db)?;

            ui::print_from_key_interactive("vault_src_verify", db)?;
            let verify = ui::get_input_bool()?;

            let mut timeout: Option<Duration> = None;
            ui::print_from_key_interactive("vault_src_timeout_yn", db)?;
            if ui::get_input_bool()? {
                ui::print_from_key_interactive("vault_src_timeout_s", db)?;
                timeout = Some(Duration::from_secs(ui::get_input_i32(1, std::i32::MAX)?.try_into()?));
            }

            let mut namespace: Option<String> = None;
            ui::print_from_key_interactive("vault_src_namespace_yn", db)?;
            if ui::get_input_bool()? {
                ui::print_from_key_interactive("vault_src_namespace_str", db)?;
                namespace= Some(ui::get_input_string()?);
            }

            ui::print_from_key_interactive("kv_mount", db)?;
            let mount = ui::get_input_string()?;

            ui::print_from_key_interactive("vault_src_engine", db)?;
            match ui::get_input_i32(1, 3)? {
            //    1 => { // AWS
            //        
            //    },


                2 => { // KV1
                    ui::print_from_key_interactive("kv_path", db)?;
                    let path = ui::get_input_string()?;

                    ui::print_from_key_interactive("kv_key", db)?;
                    let key = ui::get_input_string()?;

                    let source = VaultSource {
                        secret_engine: VaultSecretEngine::KV1(
                            KV1Cfg {
                                path: path,
                                key: key
                            }
                        ),
                        cred_source: __nsod_vault_get_cred_and_source(db)?,
                        address: addr,
                        timeout: timeout,
                        verify: verify,
                        version: 1, // Client versions other than 1 are not currently supported by anyone. If this changes, it can be edited via config files.
                        wrapping: false, // Wrapping is probably not needed, can be edited via config but not ui.
                        namespace: namespace,
                        mount: mount,                   
                    };

                    return Ok(NsodSource::Vault(source));
                },


                3 => { // KV2
                    ui::print_from_key_interactive("kv_path", db)?;
                    let path = ui::get_input_string()?;

                    let source = VaultSource {
                        secret_engine: VaultSecretEngine::KV2(
                            KV2Cfg {
                                path: path,
                            }
                        ),
                        cred_source: __nsod_vault_get_cred_and_source(db)?,
                        address: addr,
                        timeout: timeout,
                        verify: verify,
                        version: 1, // Client versions other than 1 are not currently supported by anyone. If this changes, it can be edited via config files.
                        wrapping: false, // Wrapping is probably not needed, can be edited via config but not ui.
                        namespace: namespace,
                        mount: mount,                        
                    };

                    return Ok(NsodSource::Vault(source));
                }


                _ => {
                    panic!("impossible"); // not possible given constraints of get_input
                }
            };
        },
        3 => {
            ui::print_from_key_interactive("input_src_file", db)?;
            return Ok(NsodSource::Simple(SimpleSource::File(ui::get_input_string()?)));
        },
        _ => {
            panic!("impossible"); // not possible
        },
    }
}


pub fn __nsod_vault_get_cred_and_source(db: &HashMap<String, String>) -> Result<VaultCredentialSource, Box<dyn std::error::Error>> {
    ui::print_from_key_interactive("vault_cred", db)?;
    match ui::get_input_i32(1, 2)? {
        1 => { // Identity
            ui::print_from_key_interactive("vault_identity_name", db)?;
            let cert_name = ui::get_input_string()?;
            return Ok(VaultCredentialSource::Identity(VaultIdentity{cert_name: cert_name, cert_source: __nsod_get_simple_source(db)?}));
        },

        2 => { // Token
            return Ok(VaultCredentialSource::Token(__nsod_get_simple_source(db)?));
        },

        _ => {
            panic!("impossible"); // not possible
        },
    }
}

pub fn __nsod_get_simple_source(db: &HashMap<String, String>) -> Result<SimpleSource, Box<dyn std::error::Error>> {
    ui::print_from_key_interactive("cfg_source_simple", db)?;
    match ui::get_input_i32(1, 2)? {
        1 => {
            ui::print_from_key_interactive("input_src_env", db)?;
            return Ok(SimpleSource::EnvironmentVariable(ui::get_input_string()?));
        },
        2 => {
            ui::print_from_key_interactive("input_src_file", db)?;
            return Ok(SimpleSource::File(ui::get_input_string()?));
        },
        _ => {
            panic!("impossible"); // not possible
        },
    }
}

pub fn __nsod_add_route(cfg: &mut NsodCfg, db: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    
    let mut route = NsodRoute {
        path: String::new(),
        source: NsodSource::Simple(SimpleSource::EnvironmentVariable(String::new())), // placeholder
    };
    
    // get path
    ui::print_from_key_interactive("cfg_route", db)?;
    route.path = ui::get_input_string()?;

    // get source
    route.source = __nsod_get_source(db)?;
    

    cfg.inject.routes.push(route);
    return Ok(());
}

pub fn __nsod_create_cfg(db: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    ui::print_from_key_interactive("cfg_create", db)?;
    let name = ui::get_input_string()?;

    let mut cfg = NsodCfg::new();
    
    ui::print_from_key_interactive("cfg_bin", db)?;
    cfg.wrapper.bin = ui::get_input_string()?;

    __nsod_add_route(&mut cfg, db)?;

    loop {
        ui::print_from_key_interactive("cfg_add_route", db)?;
        if ui::get_input_bool()? {
            __nsod_add_route(&mut cfg, db)?;       
        }
        else {
            break;
        }
    }

    cfg.export(&name, crate::base_cfg_dir_path!())?;

    clear()?; // clearscreen
    return Ok(());
}

pub fn __nsod_get_addr(db: &HashMap<String, String>) -> Result<Url, Box<dyn std::error::Error>> {
    loop {
        let addr_raw = Url::from_str(ui::get_input_string()?.as_str());
        if addr_raw.is_ok() {
            return Ok(addr_raw.expect("is ok"));

        } else {
            ui::print_from_key_interactive("invalid_addr", db)?;
        }
    }
}