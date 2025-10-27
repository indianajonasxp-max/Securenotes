use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
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
    in_flight: Arc<Mutex<HashSet<TileCoord>>>,
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
            in_flight: Arc::new(Mutex::new(HashSet::new())),
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
        let tile_path = Self::get_tile_path(&self.cache_dir, &coord);
        if tile_path.exists() {
            if let Ok(img) = image::open(&tile_path) {
                let mut cache = self.cache.lock();
                cache.insert(coord, Some(img.clone()));
                return Some(img);
            }
        } else {
            // Ensure directory exists so background workers can save tiles
            let _ = Self::ensure_tile_dir(&self.cache_dir, &coord);
        }

        // Spawn background loader if not already fetching this tile
        const MAX_IN_FLIGHT: usize = 16;
        let mut should_spawn = false;
        {
            let mut in_flight = self.in_flight.lock();
            if !in_flight.contains(&coord) && in_flight.len() < MAX_IN_FLIGHT {
                in_flight.insert(coord);
                should_spawn = true;
            }
        }

        if should_spawn {
            let cache = Arc::clone(&self.cache);
            let in_flight = Arc::clone(&self.in_flight);
            let cache_dir = self.cache_dir.clone();

            std::thread::spawn(move || {
                let tile = Self::load_or_download_tile(&cache_dir, coord);

                let mut cache_lock = cache.lock();
                match tile {
                    Some(ref img) => {
                        cache_lock.insert(coord, Some(img.clone()));
                    }
                    None => {
                        cache_lock.insert(coord, None);
                    }
                }
                drop(cache_lock);

                let mut in_flight_lock = in_flight.lock();
                in_flight_lock.remove(&coord);
            });
        }

        None
    }

    fn load_or_download_tile(cache_dir: &Path, coord: TileCoord) -> Option<DynamicImage> {
        let tile_path = Self::get_tile_path(cache_dir, &coord);

        if tile_path.exists() {
            match image::open(&tile_path) {
                Ok(img) => return Some(img),
                Err(e) => {
                    eprintln!("Failed to load cached tile {:?}: {}", coord, e);
                    let _ = std::fs::remove_file(&tile_path);
                }
            }
        }

        match Self::download_tile(&coord) {
            Ok(img) => {
                if let Err(e) = Self::save_tile(cache_dir, &coord, &img) {
                    eprintln!("Failed to save tile {:?} to cache: {}", coord, e);
                }
                Some(img)
            }
            Err(e) => {
                eprintln!("Failed to download tile {:?}: {}", coord, e);
                None
            }
        }
    }

    fn download_tile(coord: &TileCoord) -> Result<DynamicImage, Box<dyn std::error::Error>> {
        let url = Self::get_tile_url(coord.x, coord.y, coord.z);

        // Respect OSM tile usage policy - add User-Agent
        let client = reqwest::blocking::Client::builder()
            .user_agent("SecureNotesApp/0.1.0")
            .timeout(std::time::Duration::from_secs(10))
            .build()?;
        
        let response = client.get(&url).send()?;
        let bytes = response.bytes()?;
        let img = image::load_from_memory(&bytes)?;
        
        Ok(img)
    }

    fn get_tile_path(cache_dir: &Path, coord: &TileCoord) -> PathBuf {
        cache_dir
            .join(format!("{}", coord.z))
            .join(format!("{}", coord.x))
            .join(format!("{}.png", coord.y))
    }

    fn ensure_tile_dir(cache_dir: &Path, coord: &TileCoord) -> std::io::Result<()> {
        let dir = cache_dir
            .join(format!("{}", coord.z))
            .join(format!("{}", coord.x));
        std::fs::create_dir_all(dir)
    }

    fn save_tile(
        cache_dir: &Path,
        coord: &TileCoord,
        img: &DynamicImage,
    ) -> Result<(), image::ImageError> {
        if let Err(e) = Self::ensure_tile_dir(cache_dir, coord) {
            eprintln!("Failed to create cache dir for {:?}: {}", coord, e);
        }

        img.save(Self::get_tile_path(cache_dir, coord))
    }
}
