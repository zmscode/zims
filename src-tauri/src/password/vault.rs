use chrono::Utc;
use rand::Rng;
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};
use uuid::Uuid;
use zeroize::Zeroizing;

use super::crypto::{decrypt, derive_key, encrypt, generate_salt};
use super::storage::{self, VaultData};
use super::types::{PasswordEntry, PasswordEntrySummary, PasswordOptions, SecureString, StrengthScore};

pub struct VaultState {
    pub is_unlocked: bool,
    pub encryption_key: Option<[u8; 32]>,
    pub salt: Option<[u8; 32]>,
    pub entries: Vec<PasswordEntry>,
}

impl VaultState {
    pub fn new() -> Self {
        Self {
            is_unlocked: false,
            encryption_key: None,
            salt: None,
            entries: Vec::new(),
        }
    }

    pub fn lock(&mut self) {
        self.is_unlocked = false;
        self.encryption_key = None;
        self.entries.clear();
    }
}

pub type VaultStateMutex = Mutex<VaultState>;

#[tauri::command]
pub async fn initialize_vault(
    app: AppHandle,
    master_password: String,
    state: State<'_, VaultStateMutex>,
) -> Result<(), String> {
    if storage::vault_exists(&app) {
        return Err("Vault already exists".to_string());
    }

    let salt = generate_salt();

    let key = derive_key(&master_password, &salt).map_err(|e| e.to_string())?;

    let vault_data = VaultData {
        version: 1,
        salt,
        entries: Vec::new(),
    };

    let json = serde_json::to_vec(&vault_data).map_err(|e| e.to_string())?;
    let encrypted = encrypt(&json, &key).map_err(|e| e.to_string())?;

    storage::save_vault(&app, &encrypted).map_err(|e| e.to_string())?;

    let key_hash = hex::encode(key);
    storage::store_master_key(&key_hash).map_err(|e| e.to_string())?;

    let mut vault_state = state.lock().unwrap();
    vault_state.is_unlocked = true;
    vault_state.encryption_key = Some(key);
    vault_state.salt = Some(salt);
    vault_state.entries = Vec::new();

    Ok(())
}

#[tauri::command]
pub async fn unlock_vault(
    app: AppHandle,
    master_password: String,
    state: State<'_, VaultStateMutex>,
) -> Result<bool, String> {
    let encrypted = storage::load_vault(&app).map_err(|e| e.to_string())?;

    Err("Unlock not fully implemented - need to refactor salt storage".to_string())
}

#[tauri::command]
pub async fn lock_vault(state: State<'_, VaultStateMutex>) -> Result<(), String> {
    let mut vault_state = state.lock().unwrap();
    vault_state.lock();
    Ok(())
}

#[tauri::command]
pub async fn vault_exists(app: AppHandle) -> Result<bool, String> {
    Ok(storage::vault_exists(&app))
}

#[tauri::command]
pub async fn is_vault_unlocked(state: State<'_, VaultStateMutex>) -> Result<bool, String> {
    let vault_state = state.lock().unwrap();
    Ok(vault_state.is_unlocked)
}

#[tauri::command]
pub async fn get_all_passwords(
    state: State<'_, VaultStateMutex>,
) -> Result<Vec<PasswordEntrySummary>, String> {
    let vault_state = state.lock().unwrap();

    if !vault_state.is_unlocked {
        return Err("Vault is locked".to_string());
    }

    let summaries: Vec<PasswordEntrySummary> = vault_state
        .entries
        .iter()
        .map(|e| e.clone().into())
        .collect();

    Ok(summaries)
}

#[tauri::command]
pub async fn get_password(
    id: String,
    state: State<'_, VaultStateMutex>,
) -> Result<PasswordEntry, String> {
    let vault_state = state.lock().unwrap();

    if !vault_state.is_unlocked {
        return Err("Vault is locked".to_string());
    }

    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    vault_state
        .entries
        .iter()
        .find(|e| e.id == uuid)
        .cloned()
        .ok_or_else(|| "Password entry not found".to_string())
}

#[tauri::command]
pub async fn create_password(
    app: AppHandle,
    title: String,
    username: String,
    password: String,
    url: Option<String>,
    notes: Option<String>,
    tags: Vec<String>,
    state: State<'_, VaultStateMutex>,
) -> Result<String, String> {
    let mut vault_state = state.lock().unwrap();

    if !vault_state.is_unlocked {
        return Err("Vault is locked".to_string());
    }

    let entry = PasswordEntry {
        id: Uuid::new_v4(),
        title,
        username,
        password,
        url,
        notes,
        tags,
        favorite: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let id = entry.id.to_string();
    vault_state.entries.push(entry);

    save_vault_internal(&app, &vault_state).map_err(|e| e.to_string())?;

    Ok(id)
}

#[tauri::command]
pub async fn update_password(
    app: AppHandle,
    id: String,
    title: String,
    username: String,
    password: String,
    url: Option<String>,
    notes: Option<String>,
    tags: Vec<String>,
    favorite: bool,
    state: State<'_, VaultStateMutex>,
) -> Result<(), String> {
    let mut vault_state = state.lock().unwrap();

    if !vault_state.is_unlocked {
        return Err("Vault is locked".to_string());
    }

    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    let entry = vault_state
        .entries
        .iter_mut()
        .find(|e| e.id == uuid)
        .ok_or_else(|| "Password entry not found".to_string())?;

    entry.title = title;
    entry.username = username;
    entry.password = password;
    entry.url = url;
    entry.notes = notes;
    entry.tags = tags;
    entry.favorite = favorite;
    entry.updated_at = Utc::now();

    save_vault_internal(&app, &vault_state).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn delete_password(
    app: AppHandle,
    id: String,
    state: State<'_, VaultStateMutex>,
) -> Result<(), String> {
    let mut vault_state = state.lock().unwrap();

    if !vault_state.is_unlocked {
        return Err("Vault is locked".to_string());
    }

    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    let index = vault_state
        .entries
        .iter()
        .position(|e| e.id == uuid)
        .ok_or_else(|| "Password entry not found".to_string())?;

    vault_state.entries.remove(index);

    save_vault_internal(&app, &vault_state).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn generate_password(options: PasswordOptions) -> Result<String, String> {
    let mut charset = String::new();

    if options.include_lowercase {
        charset.push_str(if options.exclude_ambiguous {
            "abcdefghjkmnpqrstuvwxyz"
        } else {
            "abcdefghijklmnopqrstuvwxyz"
        });
    }

    if options.include_uppercase {
        charset.push_str(if options.exclude_ambiguous {
            "ABCDEFGHJKLMNPQRSTUVWXYZ"
        } else {
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        });
    }

    if options.include_numbers {
        charset.push_str(if options.exclude_ambiguous {
            "23456789"
        } else {
            "0123456789"
        });
    }

    if options.include_symbols {
        charset.push_str("!@#$%^&*()-_=+[]{}|;:,.<>?");
    }

    if charset.is_empty() {
        return Err("No character types selected".to_string());
    }

    let charset: Vec<char> = charset.chars().collect();
    let mut rng = rand::thread_rng();

    let password: String = (0..options.length)
        .map(|_| charset[rng.gen_range(0..charset.len())])
        .collect();

    Ok(password)
}

#[tauri::command]
pub async fn check_password_strength(password: String) -> Result<StrengthScore, String> {
    let length = password.len();
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_numbers = password.chars().any(|c| c.is_numeric());
    let has_symbols = password.chars().any(|c| !c.is_alphanumeric());

    let mut charset_size = 0;
    if has_lowercase {
        charset_size += 26;
    }
    if has_uppercase {
        charset_size += 26;
    }
    if has_numbers {
        charset_size += 10;
    }
    if has_symbols {
        charset_size += 32;
    }

    let entropy = (length as f64) * (charset_size as f64).log2();

    let score = if entropy < 28.0 {
        0 
    } else if entropy < 36.0 {
        1
    } else if entropy < 60.0 {
        2 
    } else if entropy < 128.0 {
        3 
    } else {
        4 
    };

    let mut feedback = Vec::new();
    if length < 8 {
        feedback.push("Use at least 8 characters".to_string());
    }
    if !has_uppercase {
        feedback.push("Add uppercase letters".to_string());
    }
    if !has_lowercase {
        feedback.push("Add lowercase letters".to_string());
    }
    if !has_numbers {
        feedback.push("Add numbers".to_string());
    }
    if !has_symbols {
        feedback.push("Add symbols".to_string());
    }
    if length < 12 && feedback.is_empty() {
        feedback.push("Consider using 12+ characters for better security".to_string());
    }

    Ok(StrengthScore {
        score,
        entropy,
        feedback,
    })
}

fn save_vault_internal(
    app: &AppHandle,
    vault_state: &VaultState,
) -> Result<(), Box<dyn std::error::Error>> {
    let key = vault_state
        .encryption_key
        .as_ref()
        .ok_or("No encryption key available")?;

    let salt = vault_state
        .salt
        .as_ref()
        .ok_or("No salt available")?;

    let vault_data = VaultData {
        version: 1,
        salt: *salt,
        entries: vault_state.entries.clone(),
    };

    let json = serde_json::to_vec(&vault_data)?;
    let encrypted = encrypt(&json, key)?;

    storage::save_vault(app, &encrypted)?;

    Ok(())
}
