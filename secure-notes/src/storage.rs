use crate::crypto;
use crate::note::Note;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct EncryptedData {
    salt: String,
    data: String, // Base64 encoded encrypted data
}

pub struct SecureStorage {
    notes: HashMap<String, Note>,
    file_path: PathBuf,
    encryption_key: Option<[u8; 32]>,
    salt: String,
    is_unlocked: bool,
}

impl SecureStorage {
    pub fn new() -> Self {
        let data_dir = directories::ProjectDirs::from("com", "secnotes", "SecureNotes")
            .expect("Failed to get data directory");
        
        let data_path = data_dir.data_dir();
        fs::create_dir_all(data_path).ok();
        
        let file_path = data_path.join("notes.enc");
        
        let salt = if file_path.exists() {
            // Load existing salt
            if let Ok(data) = fs::read_to_string(&file_path) {
                if let Ok(encrypted) = serde_json::from_str::<EncryptedData>(&data) {
                    encrypted.salt
                } else {
                    crypto::generate_salt()
                }
            } else {
                crypto::generate_salt()
            }
        } else {
            crypto::generate_salt()
        };

        Self {
            notes: HashMap::new(),
            file_path,
            encryption_key: None,
            salt,
            is_unlocked: false,
        }
    }

    pub fn is_unlocked(&self) -> bool {
        self.is_unlocked
    }

    pub fn has_existing_data(&self) -> bool {
        self.file_path.exists()
    }

    pub fn unlock(&mut self, password: &str) -> Result<(), String> {
        let key = crypto::derive_key(password, &self.salt)?;
        
        if self.file_path.exists() {
            let file_content = fs::read_to_string(&self.file_path)
                .map_err(|e| format!("Failed to read file: {}", e))?;
            
            let encrypted_data: EncryptedData = serde_json::from_str(&file_content)
                .map_err(|e| format!("Failed to parse encrypted data: {}", e))?;
            
            let encrypted_bytes = crypto::decode_base64(&encrypted_data.data)?;
            let decrypted = crypto::decrypt(&encrypted_bytes, &key)?;
            
            let notes: Vec<Note> = serde_json::from_slice(&decrypted)
                .map_err(|e| format!("Failed to parse notes: {}", e))?;
            
            self.notes = notes.into_iter().map(|n| (n.id.clone(), n)).collect();
        }
        
        self.encryption_key = Some(key);
        self.is_unlocked = true;
        Ok(())
    }

    pub fn save(&self) -> Result<(), String> {
        if !self.is_unlocked {
            return Err("Storage is locked".to_string());
        }

        let key = self.encryption_key.as_ref()
            .ok_or("No encryption key available")?;
        
        let notes: Vec<&Note> = self.notes.values().collect();
        let json = serde_json::to_vec(&notes)
            .map_err(|e| format!("Failed to serialize notes: {}", e))?;
        
        let encrypted = crypto::encrypt(&json, key)?;
        let encrypted_base64 = crypto::encode_base64(&encrypted);
        
        let encrypted_data = EncryptedData {
            salt: self.salt.clone(),
            data: encrypted_base64,
        };
        
        let json_string = serde_json::to_string_pretty(&encrypted_data)
            .map_err(|e| format!("Failed to serialize encrypted data: {}", e))?;
        
        fs::write(&self.file_path, json_string)
            .map_err(|e| format!("Failed to write file: {}", e))?;
        
        Ok(())
    }

    pub fn add_note(&mut self, note: Note) -> Result<(), String> {
        if !self.is_unlocked {
            return Err("Storage is locked".to_string());
        }
        self.notes.insert(note.id.clone(), note);
        self.save()
    }

    pub fn update_note(&mut self, note: Note) -> Result<(), String> {
        if !self.is_unlocked {
            return Err("Storage is locked".to_string());
        }
        self.notes.insert(note.id.clone(), note);
        self.save()
    }

    pub fn delete_note(&mut self, id: &str) -> Result<(), String> {
        if !self.is_unlocked {
            return Err("Storage is locked".to_string());
        }
        self.notes.remove(id);
        self.save()
    }

    pub fn get_note(&self, id: &str) -> Option<&Note> {
        if !self.is_unlocked {
            return None;
        }
        self.notes.get(id)
    }

    pub fn get_all_notes(&self) -> Vec<&Note> {
        if !self.is_unlocked {
            return Vec::new();
        }
        let mut notes: Vec<&Note> = self.notes.values().collect();
        notes.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
        notes
    }

    pub fn search_notes(&self, query: &str) -> Vec<&Note> {
        if !self.is_unlocked {
            return Vec::new();
        }
        let query_lower = query.to_lowercase();
        self.notes
            .values()
            .filter(|note| {
                note.title.to_lowercase().contains(&query_lower)
                    || note.content.to_lowercase().contains(&query_lower)
                    || note.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .collect()
    }
}
