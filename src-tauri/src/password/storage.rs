use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;

use super::types::PasswordEntry;

#[derive(Debug, Serialize, Deserialize)]
pub struct VaultData {
    pub version: u32,
    pub salt: [u8; 32],
    pub entries: Vec<PasswordEntry>,
}

pub fn get_vault_path(app: &AppHandle) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;

    fs::create_dir_all(&app_data_dir)?;

    Ok(app_data_dir.join("vault.encrypted"))
}

pub fn vault_exists(app: &AppHandle) -> bool {
    match get_vault_path(app) {
        Ok(path) => path.exists(),
        Err(_) => false,
    }
}

pub fn save_vault(
    app: &AppHandle,
    encrypted_data: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let vault_path = get_vault_path(app)?;
    fs::write(vault_path, encrypted_data)?;
    Ok(())
}

pub fn load_vault(app: &AppHandle) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let vault_path = get_vault_path(app)?;

    if !vault_path.exists() {
        return Err("Vault file does not exist".into());
    }

    let data = fs::read(vault_path)?;
    Ok(data)
}

pub fn delete_vault(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let vault_path = get_vault_path(app)?;

    if vault_path.exists() {
        fs::remove_file(vault_path)?;
    }

    Ok(())
}

const SERVICE_NAME: &str = "com.zmscode.zims";
const USERNAME: &str = "master";

pub fn store_master_key(key_hash: &str) -> Result<(), Box<dyn std::error::Error>> {
    let entry = keyring::Entry::new(SERVICE_NAME, USERNAME)?;
    entry.set_password(key_hash)?;
    Ok(())
}

pub fn get_master_key() -> Result<String, Box<dyn std::error::Error>> {
    let entry = keyring::Entry::new(SERVICE_NAME, USERNAME)?;
    let password = entry.get_password()?;
    Ok(password)
}

pub fn delete_master_key() -> Result<(), Box<dyn std::error::Error>> {
    let entry = keyring::Entry::new(SERVICE_NAME, USERNAME)?;
    entry.delete_credential()?;
    Ok(())
}

pub fn master_key_exists() -> bool {
    match keyring::Entry::new(SERVICE_NAME, USERNAME) {
        Ok(entry) => entry.get_password().is_ok(),
        Err(_) => false,
    }
}
