use std::collections::HashMap;
use std::fs::{self, remove_file};
use std::str::FromStr;

//use url::Url;

use crate::{base_cfg_dir_path, ui};
use crate::cfg_struct::{NsodCfg};
use crate::route::{NsodRoute, NsodSource, SimpleSource};


pub fn __nsod_configure_main () -> Result<i32, Box<dyn std::error::Error>> {

    let db_owned = ui::load_db()?;
    let db = &db_owned;

    loop {
        ui::print_from_key_interactive("cfg_start", db)?;
        match ui::get_input_i32(0, 2)? {
            0 => return Ok(0),
            1 => {
                __nsod_create_cfg(db)?;
            },
            2 => {
                __nsod_delete_cfg(db)?;
            },
            _ => {
            },
        }
    }
}


pub fn __nsod_delete_cfg(db: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    let names = __nsod_list_cfg(db)?;

    ui::print_from_key_interactive("cfg_delete", db)?;
    
    let mut delete_name = ui::get_input_string()?;
    loop {
        if delete_name.contains(".") || delete_name.contains("/") {
            ui::print_from_key_interactive("cfg_bad_name", db)?;
            delete_name = ui::get_input_string()?;
            continue;
        }
        else {
            break;
        }
    }

    let cfg_dir = base_cfg_dir_path!();

    for name in names {
        if name == delete_name {
            remove_file(format!("{cfg_dir}/{delete_name}_inject.json"))?;
            remove_file(format!("{cfg_dir}/{delete_name}_wrapper.json"))?;
            break;
        }
    }

    return Ok(());
}


pub fn __nsod_list_cfg(db: &HashMap<String, String>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    ui::print_from_key("cfg_list", db)?;
    
    let files = fs::read_dir(&base_cfg_dir_path!())?;
    let mut names: Vec<String> = Vec::new();

    for file in files {
        if file.as_ref().unwrap().path().is_file() {
            let path = file.unwrap().path();
            let filename = path.file_name().unwrap().to_str().unwrap();
            let filename_len = filename.len();

            let t1 = &filename[filename_len - 12..filename_len];

            if t1 == "_inject.json" {
                names.push(String::from_str(&filename[0..filename_len-12])?);
            }
        }
    }

    for name in &names {
        println!("{}", name);
    }

    return Ok(names);
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
    
    loop {
        route.path = ui::get_input_string()?;
        
        let this_path = std::path::Path::new(&route.path);
            if !this_path.is_absolute() {
                ui::print_from_key_interactive("path_not_absolute", db)?;
            }
            else {
                break;
            }
    }

    // get source
    route.source = __nsod_get_source(db)?;
    

    cfg.inject.routes.push(route);
    return Ok(());
}

pub fn __nsod_create_cfg(db: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    ui::print_from_key_interactive("cfg_create", db)?;

    let mut name = ui::get_input_string()?;
    loop {
        if name.contains(".") || name.contains("/") {
            ui::print_from_key_interactive("cfg_bad_name", db)?;
            name = ui::get_input_string()?;
            continue;
        }
        else {
            break;
        }
    }

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

    cfg.export(&name, &crate::base_cfg_dir_path!())?;

    return Ok(());
}

/*
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
*/