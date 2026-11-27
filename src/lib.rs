#![forbid(unsafe_code)]
//! SigmaOS KeePass integration: password database lifecycle management.
use thiserror::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Error)]
pub enum KeepassError {
    #[error("database not found: {0}")] DatabaseNotFound(String),
    #[error("entry not found: {0}")] EntryNotFound(String),
    #[error("database locked")] DatabaseLocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordEntry { pub uuid: String, pub title: String, pub username: String, pub url: Option<String> }
impl PasswordEntry {
    pub fn new(uuid: &str, title: &str, username: &str) -> Self {
        Self { uuid: uuid.into(), title: title.into(), username: username.into(), url: None }
    }
    pub fn with_url(mut self, url: &str) -> Self { self.url = Some(url.into()); self }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DbState { Locked, Unlocked }

pub struct KeepassDatabase {
    pub path: String, pub state: DbState, entries: Vec<PasswordEntry>,
}
impl KeepassDatabase {
    pub fn new(path: &str) -> Self { Self { path: path.into(), state: DbState::Locked, entries: vec![] } }
    pub fn unlock(&mut self) { self.state = DbState::Unlocked; }
    pub fn lock(&mut self) { self.state = DbState::Locked; self.entries.clear(); }
    pub fn add_entry(&mut self, e: PasswordEntry) -> Result<(), KeepassError> {
        if self.state == DbState::Locked { return Err(KeepassError::DatabaseLocked); }
        self.entries.push(e); Ok(())
    }
    pub fn get_entry(&self, uuid: &str) -> Result<&PasswordEntry, KeepassError> {
        if self.state == DbState::Locked { return Err(KeepassError::DatabaseLocked); }
        self.entries.iter().find(|e| e.uuid == uuid).ok_or_else(|| KeepassError::EntryNotFound(uuid.into()))
    }
    pub fn entry_count(&self) -> usize { self.entries.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_unlock_and_add_entry_aaa() {
        // Arrange
        let mut db = KeepassDatabase::new("/home/user/passwords.kdbx");
        db.unlock();
        let e = PasswordEntry::new("uuid-1", "GitHub", "user").with_url("https://github.com");
        // Act
        db.add_entry(e).unwrap();
        // Assert
        assert_eq!(db.entry_count(), 1);
        assert_eq!(db.get_entry("uuid-1").unwrap().title, "GitHub");
    }
    #[test]
    fn test_add_while_locked_aaa() {
        // Arrange
        let mut db = KeepassDatabase::new("/home/user/pass.kdbx");
        // Act
        let result = db.add_entry(PasswordEntry::new("u1", "Site", "user"));
        // Assert
        assert!(matches!(result, Err(KeepassError::DatabaseLocked)));
    }
    #[test]
    fn test_lock_clears_entries_aaa() {
        // Arrange
        let mut db = KeepassDatabase::new("/home/user/work.kdbx");
        db.unlock();
        db.add_entry(PasswordEntry::new("u1", "Work", "admin")).unwrap();
        // Act
        db.lock();
        // Assert
        assert_eq!(db.state, DbState::Locked);
        assert_eq!(db.entry_count(), 0);
    }
    #[test]
    fn test_entry_with_url_aaa() {
        // Arrange
        let e = PasswordEntry::new("u2", "Google", "me").with_url("https://google.com");
        // Act + Assert
        assert_eq!(e.url.as_deref(), Some("https://google.com"));
    }
}
