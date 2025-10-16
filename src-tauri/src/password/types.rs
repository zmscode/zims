use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordEntry {
    pub id: Uuid,
    pub title: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub tags: Vec<String>,
    pub favorite: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordEntrySummary {
    pub id: Uuid,
    pub title: String,
    pub username: String,
    pub url: Option<String>,
    pub tags: Vec<String>,
    pub favorite: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<PasswordEntry> for PasswordEntrySummary {
    fn from(entry: PasswordEntry) -> Self {
        Self {
            id: entry.id,
            title: entry.title,
            username: entry.username,
            url: entry.url,
            tags: entry.tags,
            favorite: entry.favorite,
            created_at: entry.created_at,
            updated_at: entry.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordOptions {
    pub length: u32,
    pub include_uppercase: bool,
    pub include_lowercase: bool,
    pub include_numbers: bool,
    pub include_symbols: bool,
    pub exclude_ambiguous: bool,
}

impl Default for PasswordOptions {
    fn default() -> Self {
        Self {
            length: 20,
            include_uppercase: true,
            include_lowercase: true,
            include_numbers: true,
            include_symbols: true,
            exclude_ambiguous: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrengthScore {
    pub score: u8, 
    pub entropy: f64,
    pub feedback: Vec<String>,
}

#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct SecureString(String);

impl SecureString {
    pub fn new(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<String> for SecureString {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl std::fmt::Debug for SecureString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[REDACTED]")
    }
}
