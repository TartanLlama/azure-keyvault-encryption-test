use azure_identity::token_credentials::{ClientSecretCredential, TokenCredentialOptions};
use azure_key_vault::key::{DecryptParameters, DecryptParametersEncryption};
use azure_key_vault::key::{EncryptionAlgorithm, RsaDecryptParameters};
use azure_key_vault::KeyClient;

use std::num::ParseIntError;
use std::{env, io};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_id =
        env::var("AZURE_CLIENT_ID").expect("Missing AZURE_CLIENT_ID environment variable.");
    let client_secret =
        env::var("AZURE_CLIENT_SECRET").expect("Missing AZURE_CLIENT_SECRET environment variable.");
    let tenant_id =
        env::var("AZURE_TENANT_ID").expect("Missing AZURE_TENANT_ID environment variable.");
    let keyvault_uri =
        env::var("AZURE_KEY_VAULT_URI").expect("Missing AZURE_KEY_VAULT_URI environment variable.");
    let key_name =
        env::var("AZURE_KEY_NAME").expect("Missing AZURE_KEY_NAME environment variable.");

    let creds = ClientSecretCredential::new(
        tenant_id.into(),
        client_id.into(),
        client_secret.into(),
        TokenCredentialOptions::default(),
    );
    let mut client = KeyClient::new(&keyvault_uri, &creds)?;

    loop {
        println!("Provide ciphertext:");
        let mut ciphertext = String::new();
        io::stdin().read_line(&mut ciphertext)?;

        let decrypt_parameters_encryption = DecryptParametersEncryption::Rsa(
            RsaDecryptParameters::new(EncryptionAlgorithm::Rsa15).unwrap(),
        );
        let ciphertext = decode_hex(&ciphertext.trim())?;
        let params = DecryptParameters {
            decrypt_parameters_encryption,
            ciphertext,
        };
        let secret = client.decrypt(&key_name, None, params).await?;

        let msg = String::from_utf8_lossy(secret.result());
        println!("Very secret message: {}\n", msg);
    }
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}
