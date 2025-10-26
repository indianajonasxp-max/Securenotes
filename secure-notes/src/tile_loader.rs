use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use parking_lot::Mutex;
use image::DynamicImage;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct TileCoord {
    pub x: i32,
    pub y: i32,
    pub z: u8,
}

#[derive(Debug)]
pub struct TileLoader {
    cache: Arc<Mutex<HashMap<TileCoord, Option<DynamicImage>>>>,
    cache_dir: PathBuf,
}

impl TileLoader {
    pub fn new() -> Self {
        let cache_dir = directories::ProjectDirs::from("com", "secnotes", "SecureNotes")
            .expect("Failed to get cache directory")
            .cache_dir()
            .join("tiles");
        
        std::fs::create_dir_all(&cache_dir).ok();
        
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            cache_dir,
        }
    }

    pub fn get_tile_url(x: i32, y: i32, z: u8) -> String {
        // OpenStreetMap tile server
        format!("https://tile.openstreetmap.org/{}/{}/{}.png", z, x, y)
    }

    pub fn get_tile(&self, coord: TileCoord) -> Option<DynamicImage> {
        // Check memory cache
        {
            let cache = self.cache.lock();
            if let Some(tile) = cache.get(&coord) {
                return tile.clone();
            }
        }

        // Try to load from disk cache
        let tile_path = self.get_tile_path(&coord);
        if tile_path.exists() {
            if let Ok(img) = image::open(&tile_path) {
                let mut cache = self.cache.lock();
                cache.insert(coord, Some(img.clone()));
                return Some(img);
            }
        }

        // Try to download (blocking for simplicity)
        if let Ok(img) = self.download_tile(&coord) {
            // Ensure directory exists before saving
            self.ensure_tile_dir(&coord);
            
            // Save to disk
            if let Err(e) = img.save(&tile_path) {
                eprintln!("Failed to save tile to cache: {}", e);
            }
            
            let mut cache = self.cache.lock();
            cache.insert(coord, Some(img.clone()));
            return Some(img);
        }

        // Mark as failed in cache
        let mut cache = self.cache.lock();
        cache.insert(coord, None);
        None
    }

    fn download_tile(&self, coord: &TileCoord) -> Result<DynamicImage, Box<dyn std::error::Error>> {
        let url = Self::get_tile_url(coord.x, coord.y, coord.z);
        
        // Respect OSM tile usage policy - add User-Agent
        let client = reqwest::blocking::Client::builder()
            .user_agent("SecureNotesApp/0.1.0")
            .timeout(std::time::Duration::from_secs(10))
            .build()?;
        
        let response = client.get(&url).send()?;
        let bytes = response.bytes()?;
        let img = image::load_from_memory(&bytes)?;
        
        // Ensure directory exists before saving
        self.ensure_tile_dir(coord);
        
        Ok(img)
    }

    fn get_tile_path(&self, coord: &TileCoord) -> PathBuf {
        self.cache_dir
            .join(format!("{}", coord.z))
            .join(format!("{}", coord.x))
            .join(format!("{}.png", coord.y))
    }

    pub fn ensure_tile_dir(&self, coord: &TileCoord) {
        let dir = self.cache_dir
            .join(format!("{}", coord.z))
            .join(format!("{}", coord.x));
        std::fs::create_dir_all(dir).ok();
    }
}
