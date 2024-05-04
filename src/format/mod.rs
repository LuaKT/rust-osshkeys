use crate::error::*;
use crate::keys::*;

pub mod ossh_privkey;
pub mod ossh_pubkey;
pub mod pem;
pub mod pkcs8;

pub fn parse_keystr(pem: &[u8], passphrase: Option<&str>) -> OsshResult<KeyPair> {
    let pemdata = ::pem::parse(pem)?;

    match pemdata.tag() {
        "OPENSSH PRIVATE KEY" => {
            // Openssh format
            ossh_privkey::decode_ossh_priv(pemdata.contents(), passphrase)
        }
        "PRIVATE KEY" => {
            // PKCS#8 format
            pem::parse_pem_privkey(pem, passphrase)
        }
        "ENCRYPTED PRIVATE KEY" => {
            // PKCS#8 format
            pem::parse_pem_privkey(pem, passphrase)
        }
        "DSA PRIVATE KEY" => {
            // Openssl DSA Key
            pem::parse_pem_privkey(pem, passphrase)
        }
        "RSA PRIVATE KEY" => {
            // Openssl RSA Key
            pem::parse_pem_privkey(pem, passphrase)
        }
        "EC PRIVATE KEY" => {
            // Openssl EC Key
            pem::parse_pem_privkey(pem, passphrase)
        }
        "BEGIN PRIVATE KEY" => {
            // Openssl Ed25519 Key
            pem::parse_pem_privkey(pem, passphrase)
        }
        _ => Err(ErrorKind::UnsupportType.into()),
    }
}
