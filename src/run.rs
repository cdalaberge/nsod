use std::ffi::CString;
use std::str::FromStr;

use nix::unistd::execve;

use crate::ui::{ __nsod_usage};
use crate::cfg_struct::{NsodCfg};

// Exec target with cfg matching argv[2].
// Does not return on success.
pub fn __nsod_run (argv: &Vec<String>) -> Result<i32, Box<dyn std::error::Error>> {
    
    let min_argv = 3; // argv[0], run, target.
    if argv.len() < min_argv {
        __nsod_usage()?;
        return Ok(1);
    }

    let cfg = NsodCfg::from_import(&argv[2], &crate::base_cfg_dir_path!())?;

    if !cfg.validate() {
        println!("Exiting...");
        std::process::exit(1);
    }

    let exec_path: CString = CString::new(cfg.wrapper.bin)?;

    // Setup argv.
    let mut exec_argv: Vec<CString> = Vec::new();
    exec_argv.push(exec_path.clone()); // set argv[0] to same bin path

    for i in min_argv..argv.len() {
        exec_argv.push(CString::new(argv[i].clone())?);
    }

    // Setup initial env.
    let mut exec_envp: Vec<CString> = Vec::new();

    for (key, value) in std::env::vars() {
        exec_envp.push(CString::new(format!("{key}={value}"))?);
    }

    // Inject NSOD config
    let cfg_string = serde_json::to_string(&cfg.inject)?;

    let cfg_key = String::from_str(crate::cfg_env!())?;
    exec_envp.push(CString::new(format!("{cfg_key}={cfg_string}"))?);


    let ld_path = crate::so_path!();
    exec_envp.push(CString::new(format!("LD_PRELOAD={ld_path}"))?);

    execve(&exec_path, exec_argv.as_slice(), exec_envp.as_slice())?;
    return Ok(1);
}