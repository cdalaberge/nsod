use std::ffi::{CStr, c_char};
use std::process::exit;
use std::env;
use std::io::Write;
use std::fs::File;
use std::error::Error;
use std::os::fd::FromRawFd;

use crate::cfg_struct::NsodCfgInject;

pub mod cfg_core;
pub mod cfg_struct;
pub mod route;

// Deserialize data from the NSOD config environment variable into an NsodCfgInject data structure.
fn __nsod_get_cfg() -> Result<NsodCfgInject, Box<dyn Error>> {

    let cfg_string: String = env::var(cfg_env!())?; // Treat absence of cfg variable as a killswitch for the hook: fallback to libc open.

    match serde_json::from_slice(cfg_string.as_bytes()) {
        Ok(cfg) => return Ok(cfg),
        Err(e) => {
            println!("NSOD: Failed to parse config:");
            println!("{}", e);
            exit(1); // Bad cfg **might** be survivable, but at least by crashing here the user knows the problem.
        }
    }
}


// Entry point to the NSOD rust component. 
// Compares path_raw against a vector of routes in NSOD's config environment variable.
// If a match is found, fd is populated with data from the corresponding source, also in the same config.
// Returns zero if the open call was intercepted, non-zero to fallback to libc.
#[unsafe(no_mangle)]
pub extern "C" fn __nsod_rust_request(path_raw: *const c_char, fd: i32) -> i32 {

    match __nsod_get_cfg() {
        Ok(cfg) => {
            unsafe {

                // Clone our raw path pointer to a safe, owned Rust String.
                // If the raw pointer were bad, this mitigates undefined behaviour by crashing here.
                let path_wrap: &CStr = CStr::from_ptr(path_raw);
                let path: String = path_wrap.to_str().expect("NSOD: invalid unicode in path violates safety contract.").to_string();

                let mut pipe_file = File::from_raw_fd(fd);

                match cfg.secret_query(&path) {
                    Some(secret) => {
                        pipe_file.write(secret.as_slice()).expect("NSOD: invalid fd violates safety contract");
                        return 0; // Normal return.
                    }
                    None => {
                        return -1; // Non-normal return: occurs when path not intercepted. Fallback to libc open.
                    }
                }
            }
        },
        Err(_) => {
            return -1; // Non-normal return: occurs when cfg is absent. Fallback to libc open.
        }
    }
}