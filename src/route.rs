use std::env;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct NsodRoute {
    pub path: String,
    pub source: NsodSource,
}

#[derive(Serialize, Deserialize)]
pub enum NsodSource {
    Simple(SimpleSource),
}


#[derive(Serialize, Deserialize)]
pub enum SimpleSource {
    EnvironmentVariable(String),
    File(String),
}


impl SimpleSource {

    // No error handling: if we intercept an open and try to get a secret, we either get the secret or crash.
    // Trying to recover/falling back to libc open will only mask what is probably a configuration error.
    pub fn get_secret(&self) -> Vec<u8> {
        match &self {
            SimpleSource::EnvironmentVariable(key) => {
                let secret = env::var(key).expect("NSOD: Secret not found at environment variable.");
                return secret.into_bytes();
            }
            SimpleSource::File(path) => {
                let secret = std::fs::read(path).expect("NSOD: Error reading secret from file."); // Ditto
                return secret;
            },
        }
    }
}

impl NsodSource {

    pub fn get_secret(&self) -> Vec<u8> {
        match &self {
            NsodSource::Simple(source) => return source.get_secret(),
        }
    }
}






