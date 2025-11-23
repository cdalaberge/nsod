use std::collections::HashMap;
use std::str::FromStr;

use clearscreen::clear;
use url::Url;

use crate::ui;
use crate::nsod_cfg::{NsodCfg};
use crate::route::{NsodRoute, NsodSource, SimpleSource};



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
    match ui::get_input_i32(1, 2)? {
        1 => { // ENV
            ui::print_from_key_interactive("input_src_env", db)?;
            return Ok(NsodSource::Simple(SimpleSource::EnvironmentVariable(ui::get_input_string()?)));
        },
        2 => { // FILE
            ui::print_from_key_interactive("input_src_file", db)?;
            return Ok(NsodSource::Simple(SimpleSource::File(ui::get_input_string()?)));
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