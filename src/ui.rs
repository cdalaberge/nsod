use std::collections::HashMap;
use std::fs::read;
use std::io::{self, Write};

//use clearscreen::clear;
use crate::{base_ui_dir_path, db_en, db_set, help_path, interact_char, usage_path};

pub fn print_from_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", str::from_utf8(&read(path)?)?);
    return Ok(());
}

pub fn print_interactive(line: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", line);
    print!(interact_char!());
    io::stdout().flush()?;
    return Ok(());
}

pub fn print_from_key(key: &str, db: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    if db.contains_key(key) {
        println!("{}", db.get(key).unwrap());
    }
    return Ok(());
}

pub fn print_from_key_interactive(key: &str, db: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    if db.contains_key(key) {
        println!("{}", db.get(key).unwrap());
        print!(interact_char!());
        io::stdout().flush()?;
    }
    return Ok(());
}

pub fn load_db() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut db: HashMap<String, String> = HashMap::new();

    let basepath = base_ui_dir_path!();
    let lang = db_set!();

    for name in db_en!() {
        let path = format!("{basepath}/{lang}/{name}.txt");
        db.insert(name.to_string(), String::from_utf8(read(path)?)?);
    }

    return Ok(db);
}

pub fn get_input_bool() -> Result<bool, Box<dyn std::error::Error>> {
    let mut r_string = String::new();

    for _ in 0..10 { // Loop a finite number of times to prevent getting stuck if automated incorrectly.
        let _length = io::stdin().read_line(&mut r_string)?;

        match r_string.trim() {
            "y" | "Y" => {
                return Ok(true);
            },
            "n" | "N" => {
                return Ok(false);
            }
            _ => {
                r_string.clear();
            }
        }
    }

    return Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, "invalid input")));
}

pub fn get_input_i32(min: i32, max: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let mut r_string = String::new();

    for _ in 0..10 { // Loop a finite number of times to prevent getting stuck if automated incorrectly.
        let _length = io::stdin().read_line(&mut r_string)?;

        match r_string.trim().parse() {
            Ok(int) => {
                if int >= min && int <= max {
                    return Ok(int); // Only return if user inputs an int.
                }
                else {
                    r_string.clear();
                }
            },
            Err(_) => {
                r_string.clear(); // Otherwise clear input and try again.
            }
        }
    }

    return Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, "input not an integer")));
}

pub fn get_input_string() -> Result<String, Box<dyn std::error::Error>> {
    let mut r = String::new();
    let _length = io::stdin().read_line(&mut r)?;
    return Ok(r.trim().to_string());
}

pub fn __nsod_help () -> Result<i32, Box<dyn std::error::Error>> {
    __nsod_usage()?; // Print basic usage first
    print_from_file(help_path!())?;
    return Ok(0);
}

pub fn __nsod_usage () -> Result<i32, Box<dyn std::error::Error>> {
    print_from_file(usage_path!())?;
    return Ok(0);
}