use std::fs::{File, read};
use serde::{Serialize, Deserialize};
use std::error::Error;


use crate::route::{NsodRoute};

#[derive(Serialize, Deserialize)]
pub struct NsodCfg {
    pub inject: NsodCfgInject,
    pub wrapper: NsodCfgWrapper,
}

#[derive(Serialize, Deserialize)]
pub struct NsodCfgInject {
    pub routes: Vec<NsodRoute>,
}

#[derive(Serialize, Deserialize)]
pub struct NsodCfgWrapper {
    pub bin: String,
}

impl NsodCfg {
    pub fn from_import(cfg_name: &str, dir_path: &str) -> Result<NsodCfg, Box<dyn Error>> {
        let path_inj = format!("{dir_path}/{cfg_name}_inject.json");
        let path_wrap = format!("{dir_path}/{cfg_name}_wrapper.json");

        let inj_bytes = read(&path_inj)?;
        let wrap_bytes = read(&path_wrap)?;

        let cfg = NsodCfg {
            inject: serde_json::from_slice(inj_bytes.as_slice())?,
            wrapper: serde_json::from_slice(wrap_bytes.as_slice())?,
        };

        return Ok(cfg);
    }

    pub fn export(&self, cfg_name: &str, dir_path: &str) -> Result<(), Box<dyn Error>> {
        let path_inj = format!("{dir_path}/{cfg_name}_inject.json");
        let path_wrap = format!("{dir_path}/{cfg_name}_wrapper.json");

        let file_inj = File::create(path_inj)?;
        let file_wrap = File::create(path_wrap)?;

        serde_json::to_writer_pretty(file_inj, &self.inject)?;
        serde_json::to_writer_pretty(file_wrap, &self.wrapper)?;
        
        return Ok(());
    }

    pub fn new() -> NsodCfg {
        return NsodCfg {
            inject: NsodCfgInject { routes: Vec::new() },
            wrapper: NsodCfgWrapper { bin: String::new() },
        };
    }
}

impl NsodCfgInject {
    pub fn secret_query(&self, path: &str) -> Option<Vec<u8>> {

        for route in self.routes.iter() {
            if route.path == path {
                return Some(route.source.get_secret()); // Path in cfg: get secret from source.
            }
        }

        return None; // Path not in cfg: do not intercept.
    }
}

