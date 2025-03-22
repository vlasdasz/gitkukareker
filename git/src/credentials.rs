use std::path::PathBuf;

use anyhow::Result;
use git2::Cred;
use ssh_key::{PrivateKey, PublicKey};

pub struct Credentials {}

impl Credentials {
    pub fn get() -> Result<Cred> {
        let private_data = std::fs::read(private_path())?;
        let public_data = std::fs::read(public_path())?;

        let private_key = PrivateKey::from_openssh(&private_data)?;
        let public_key = PublicKey::from_openssh(&String::from_utf8(public_data)?)?;

        if private_key.is_encrypted() {
            println!("The SSH key is password-protected.");
        } else {
            println!("The SSH key is not password-protected.");
        }

        let key = Cred::ssh_key(
            "git",
            Some(&public_path()),
            &private_path(),
            Some("molar-psychic-rancho-combat"),
        )?;

        Ok(key)
    }
}

fn ssh_path() -> PathBuf {
    dirs::home_dir().unwrap().join(".ssh")
}

fn public_path() -> PathBuf {
    ssh_path().join("id_ed25519.pub")
}

fn private_path() -> PathBuf {
    ssh_path().join("id_ed25519")
}

#[test]
fn test() {
    Credentials::get().unwrap();
}
