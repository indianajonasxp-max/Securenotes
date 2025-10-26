use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub location: Option<GeoLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub name: String,
}

impl Note {
    pub fn new(title: String, content: String) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            content,
            created_at: now,
            modified_at: now,
            tags: Vec::new(),
            location: None,
        }
    }

    pub fn update_content(&mut self, content: String) {
        self.content = content;
        self.modified_at = Utc::now();
    }

    pub fn update_title(&mut self, title: String) {
        self.title = title;
        self.modified_at = Utc::now();
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.modified_at = Utc::now();
        }
    }

    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
        self.modified_at = Utc::now();
    }

    pub fn set_location(&mut self, location: GeoLocation) {
        self.location = Some(location);
        self.modified_at = Utc::now();
    }

    pub fn remove_location(&mut self) {
        self.location = None;
        self.modified_at = Utc::now();
    }
}

impl GeoLocation {
    pub fn new(latitude: f64, longitude: f64, name: String) -> Self {
        Self {
            latitude,
            longitude,
            name,
        }
    }

    // Copenhagen, Denmark coordinates
    pub fn copenhagen() -> Self {
        Self::new(55.6761, 12.5683, "Copenhagen, Denmark".to_string())
    }

    // Berlin, Germany coordinates
    pub fn berlin() -> Self {
        Self::new(52.5200, 13.4050, "Berlin, Germany".to_string())
    }

    // Hamburg, Germany coordinates
    pub fn hamburg() -> Self {
        Self::new(53.5511, 9.9937, "Hamburg, Germany".to_string())
    }
}
