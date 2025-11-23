use std::env;
use serde::{Serialize, Deserialize};

use crate::vault::{VaultSource};

#[derive(Serialize, Deserialize)]
pub struct NsodRoute {
    pub path: String,
    pub source: NsodSource,
}

#[derive(Serialize, Deserialize)]
pub enum NsodSource {
    Simple(SimpleSource),
    Vault(VaultSource),
}


#[derive(Serialize, Deserialize)]
pub enum SimpleSource {
    EnvironmentVariable(String),
    File(String), // This is just a regular open again, BUT the target can be any file (including tmpfs, which is RAM-only). Useful when a path is hardcoded.
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

    pub async fn get_secret(&self) -> Vec<u8> {
        match &self {
            NsodSource::Simple(source) => return source.get_secret(),
            NsodSource::Vault(source) => return source.get_secret().await,
        }
    }
}






