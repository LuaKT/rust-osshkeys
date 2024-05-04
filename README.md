# osshkeys

[![Crates](https://img.shields.io/crates/v/osshkeys.svg)](https://crates.io/crates/osshkeys)
[![Docs](https://docs.rs/osshkeys/badge.svg)](https://docs.rs/osshkeys)
[![dependency status](https://deps.rs/repo/github/Leo1003/rust-osshkeys/status.svg)](https://deps.rs/repo/github/Leo1003/rust-osshkeys)
![minimum rustc version](https://img.shields.io/badge/rustc-1.63+-blue.svg)
[![GitHub license](https://img.shields.io/github/license/Leo1003/rust-osshkeys)](https://github.com/Leo1003/rust-osshkeys/blob/master/LICENSE)
[![GitHub issues](https://img.shields.io/github/issues/Leo1003/rust-osshkeys?logo=github)](https://github.com/Leo1003/rust-osshkeys/issues)

## Description
A Rust library to handle OpenSSH key and other common SSH key

The main function of this library is to read, write different formats of SSH keys.
Also, it provides the ability to generate a key, sign and verify data.

## Current Status
The library's basic features are implemented.

~~Since I don't have too much time to work on the additional features currently,
this project may not be as active as usual.~~
We have the crates needed to implement those features in the Rust ecosystem today.
I will use my spare time to add new features.

But I still maintain the dependencies and release new versions.

It's welcome to submit PRs or issues on this project if you have ideas or questiones.

## Minimum Supported Rust Version (MSRV)
Rust 1.63 or higher.

This repository keeps track of the minimum Rust version which required to compile in CI.

However, the minimum supported Rust version can be changed in the future
and is not considered as a breaking change(exempt from SemVer).

## Example
```rust
#[macro_use]
extern crate hex_literal;
use osshkeys::{KeyPair, KeyType, Key as _, PublicParts as _, PrivateParts as _};
use osshkeys::keys::FingerprintHash;

fn main() {
    let keyfile = std::fs::read_to_string("assets/openssh_ed25519_enc").unwrap();
    let keypair = KeyPair::from_keystr(&keyfile, Some(b"12345678")).unwrap();

    // Get the public key
    let publickey = keypair.clone_public_key().unwrap();

    // Get the key type
    assert_eq!(keypair.keytype(), KeyType::ED25519);

    // Get the fingerprint
    assert_eq!(keypair.fingerprint(FingerprintHash::MD5).unwrap(), hex!("d29552b0c87d7ff1acb3c2229e783321"));

    // Sign some data
    const SOME_DATA: &[u8] = b"8Kn9PPQV";
    let sign = keypair.sign(SOME_DATA).unwrap();

    assert_eq!(sign.as_slice(), hex!("7206f04ef062ec35f8fb9f9e8a17ec023070ecf5f6e1021ea2af73137b1b832bba08766e5ad95fdca81af37b27898428f9a7dbeb044dd550afeb46efb94fe808").as_ref());
    assert!(publickey.verify(SOME_DATA, &sign).unwrap());
}
```

## Cargo Features
- `openssl-cipher`: [default] Using OpenSSL as symmetric cipher
- `rustcrypto-cipher`: Using RustCrypto as symmetric cipher
    - Choosing this does not remove the `openssl` dependency, since many places still require to use OpenSSL.
- `openssl-vendored`: Build with `openssl/vendored` feature

## Roadmap
- Core Features
    - Key Types
        - RSA
        - DSA
        - EcDSA
        - Ed25519
        - [ ] Supporting FIDO keys
            - ecdsa-sk
            - ed25519-sk
    - [x] Documentation
        - [x] Descriptions
        - [x] Examples in README
        - [ ] More examples in `examples/` directory
    - [x] Key generation
    - [x] Public key formats
        - [x] Openssh
        - [x] PEM
    - [x] Private keys
        - [x] PEM (Using OpenSSL)
        - [x] PEM (Encrypted) (Using OpenSSL)
        - [x] PKCS#8 (Using OpenSSL)
        - [x] PKCS#8 (Encrypted) (Using OpenSSL)
        - [x] Openssh v2
        - [x] Openssh v2 (Encrypted)
- Additional Features
    - [x] Draw the ASCII art (the picture shown when you generate a key)
        ```
        +---[RSA 2048]----+
        |       .++       |
        |       .+..     .|
        |     . .   . . ..|
        |    . .     .E.. |
        |     ...S     .  |
        |      o+.        |
        |     +..o        |
        |  o B .o.        |
        | . + +..         |
        +------[MD5]------+
        ```
        - [ ] Make the ASCII art as an object
    - [ ] Supporting the experimental XMSS keys
    - [ ] Supporting read/write Putty key format(.ppk)
    - [ ] Supporting more ciphers
        - [ ] AES GCM mode
        - [ ] ChaCha20-Poly1305
    - [ ] Supporting keys with certificates
    - [ ] Without using openssl (Become pure Rust library) (if there exists required cryptography crates and being mature enough)
        - Currently missing:
            - [x] DSA library
                - Now has [dsa](https://crates.io/crates/dsa) crate
            - [x] EcDSA library
                - Now has [ecdsa](https://crates.io/crates/ecdsa) crate
