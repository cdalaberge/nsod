use std::time::Duration;
use serde::{Serialize, Deserialize};
use vaultrs::{auth, aws, client::{Client, Identity, VaultClientSettings, VaultClientSettingsBuilder}, kv1, kv2};
use vaultrs::auth::cert::login;
use url::Url;

use std::collections::HashMap; // data type for kv1

use crate::route::SimpleSource;




#[derive(Serialize, Deserialize)]
pub struct VaultSource {

    // Secret Engine (includes varied credential sources).
    pub secret_engine: VaultSecretEngine,

    // Vault credential: either an Identity (in which case NSOD will request a token for each request) or a Token
    pub cred_source: VaultCredentialSource,

    // Non-sensitive components of Vaultrs VaultClientSettings.
    pub address: Url,
    pub timeout: Option<Duration>,
    pub verify: bool,
    pub version: u8, // defaults to "1"
    pub wrapping: bool, // defaults to "false"
    pub namespace: Option<String>,

    // Vault mount
    pub mount: String,
}

#[derive(Serialize, Deserialize)]
pub enum VaultCredentialSource {
    Identity(VaultIdentity),
    Token(SimpleSource),
}

#[derive(Serialize, Deserialize)]
pub struct VaultIdentity {
    pub cert_source: SimpleSource,
    pub cert_name: String,
}

#[derive(Serialize, Deserialize)]
pub enum VaultSecretEngine {
    AWS(AWSCfg),
    KV1(KV1Cfg),
    KV2(KV2Cfg),
}

#[derive(Serialize, Deserialize)]
pub struct AWSCfg {
    pub access_key: SimpleSource,
    pub secret_key: SimpleSource,
}

#[derive(Serialize, Deserialize)]
pub struct KV1Cfg {
    pub path: String,
    pub key: String,
}

#[derive(Serialize, Deserialize)]
pub struct KV2Cfg {
    pub path: String,
}


impl VaultSource {

    // No error handling: if we intercept an open and try to get a secret, we either get the secret or crash.
    // Trying to recover/falling back to libc open will only mask what is probably a configuration error.
    pub async fn get_secret(&self) -> Vec<u8> {

        let mut builder = VaultClientSettingsBuilder::default();

        // Load non-sensitive settings
        builder.address(self.address.clone());
        builder.timeout(self.timeout);
        builder.verify(self.verify);
        builder.version(self.version);
        builder.wrapping(self.wrapping);
        builder.namespace(self.namespace.clone());

        // Load credentials
        match &self.cred_source {
            VaultCredentialSource::Identity(identity) => {
                let cert_bytes = identity.cert_source.get_secret();
                let cert: Identity = Identity::from_pem(cert_bytes.as_slice()).expect("NSOD: Malformed Vault identity. Please use PEM encoding.");
                builder.identity(Some(cert));
            },
            VaultCredentialSource::Token(source) => {
                let token_bytes = source.get_secret();
                let token = String::from_utf8(token_bytes).expect("NSOD: invalid unicode in token.");
                builder.token(token);
            },
        }


        let settings: VaultClientSettings = builder.build().expect("NSOD: Bad client settings");

        let mut client = vaultrs::client::VaultClient::new(settings).expect("NSOD: Bad client settings");


        match &self.cred_source {
            VaultCredentialSource::Token(_) => {
                // Nothing needed?
            },
            VaultCredentialSource::Identity(identity) => {
                let info = login(&client, &self.mount, &identity.cert_name).await.expect("NSOD: Failed to login to Vault."); // Login to receive a token.
                client.set_token(&info.client_token); // Update our token to the new one.
            },
        }

        match &self.secret_engine {
            VaultSecretEngine::AWS(_cfg) => {
                
                //uth::aws::iam_login(client, mount, iam_http_request_method, iam_request_url, iam_request_headers, iam_request_body, role);
                //auth::aws::ec2_login(client, mount, pkcs7, nonce, role);

                //aws::roles::read(client, mount, name);

                return Vec::new();
                
            },
            VaultSecretEngine::KV1(cfg) => {
                let secrets: HashMap<String, String> = kv1::get(&client, &self.mount, &cfg.path).await.expect("NSOD: Vault request failed.");

                let secret_string = secrets.get(&cfg.key).expect("NSOD: key not found in vault.").clone();
                return secret_string.into_bytes();
            },
            VaultSecretEngine::KV2(cfg) => {

                let secret_bytes: Vec<u8> = kv2::read(&client, &self.mount, &cfg.path).await.expect("NSOD: Vault request failed.");
                return secret_bytes;
            }
        }
    }
}