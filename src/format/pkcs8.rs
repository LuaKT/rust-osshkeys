use crate::error::*;
use crate::keys::*;

//TODO: Not to depend on openssl to parse pem file in the future
pub fn stringify_pkcs8_privkey(keypair: &KeyPair, passphrase: Option<&[u8]>) -> OsshResult<String> {
    let pem = if let Some(passphrase) = passphrase {
        let cipher = openssl::symm::Cipher::aes_128_cbc();
        keypair
            .ossl_pkey()?
            .private_key_to_pem_pkcs8_passphrase(cipher, passphrase)?
    } else {
        keypair.ossl_pkey()?.private_key_to_pem_pkcs8()?
    };

    Ok(String::from_utf8(pem).map_err(|e| Error::with_failure(ErrorKind::InvalidPemFormat, e))?)
}
