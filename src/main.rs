use std::env;
use std::process::exit;

pub mod cfg_struct;
pub mod route;
pub mod ui;
pub mod cli;
pub mod cfg_ui;
pub mod cfg_core;
pub mod cfg_install_path;
pub mod run;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let result: Result<i32, Box<dyn std::error::Error>>;

    if argv.len() >= 2 {
        let mode: &str = &argv[1];
        match mode {
            "configure" => {
                result = cli::__nsod_configure_main();
            }
            "run" => {
                result = run::__nsod_run(&argv); // Will not return unless error occurs.
            }
            "help" => { // Advanced help.
                result = ui::__nsod_help();
            }
            "usage" => { // Basic help.
                result = ui::__nsod_usage();
            }
            _ => { // Print basic help, but with abnormal exit.
                let _r = ui::__nsod_usage();
                result = Ok(1); 
            }
        }
    }
    else {
        let _r = ui::__nsod_usage();
        result = Ok(1); 
    }

    match result {
        Ok(code) => exit(code),
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    }
}