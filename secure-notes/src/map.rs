use crate::note::GeoLocation;
use crate::tile_loader::{TileCoord, TileLoader};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const TILE_SIZE: f32 = 256.0;
const TILE_SIZE_I: i32 = 256;

#[derive(Clone)]
pub struct MapView {
    pub center_lat: f64,
    pub center_lon: f64,
    pub zoom: u8,
    pub width: f32,
    pub height: f32,
    pub tile_loader: Option<std::sync::Arc<TileLoader>>,
    pub selected_point: Option<GeoLocation>,
}

impl MapView {
    pub fn new() -> Self {
        // Default to center between Copenhagen and Hamburg
        Self {
            center_lat: 54.5,
            center_lon: 11.0,
            zoom: 7,
            width: 800.0,
            height: 600.0,
            tile_loader: Some(std::sync::Arc::new(TileLoader::new())),
            selected_point: None,
        }
    }

    pub fn zoom_in(&mut self) {
        if self.zoom < 18 {
            self.zoom += 1;
        }
    }

    pub fn zoom_out(&mut self) {
        if self.zoom > 2 {
            self.zoom -= 1;
        }
    }

    pub fn pan(&mut self, dx: f32, dy: f32) {
        let scale = self.pixels_per_degree() as f32;
        self.center_lon -= (dx / scale) as f64;
        self.center_lat += (dy / scale) as f64;
        
        // Clamp latitude
        self.center_lat = self.center_lat.clamp(-85.0, 85.0);
        
        // Wrap longitude
        while self.center_lon > 180.0 {
            self.center_lon -= 360.0;
        }
        while self.center_lon < -180.0 {
            self.center_lon += 360.0;
        }
    }

    pub fn set_center(&mut self, lat: f64, lon: f64) {
        self.center_lat = lat.clamp(-85.0, 85.0);
        self.center_lon = lon;
    }

    fn pixels_per_degree(&self) -> f64 {
        let tiles_per_side = 2_f64.powi(self.zoom as i32);
        (tiles_per_side * TILE_SIZE as f64) / 360.0
    }

    pub fn screen_to_geo(&self, screen_x: f32, screen_y: f32) -> (f64, f64) {
        let center_pixel = self.lat_lon_to_pixel(self.center_lat, self.center_lon, self.zoom);
        
        let pixel_x = center_pixel.0 + (screen_x - self.width / 2.0) as f64;
        let pixel_y = center_pixel.1 + (screen_y - self.height / 2.0) as f64;
        
        self.pixel_to_lat_lon(pixel_x, pixel_y, self.zoom)
    }
    
    pub fn lat_lon_to_pixel(&self, lat: f64, lon: f64, zoom: u8) -> (f64, f64) {
        let n = 2_f64.powi(zoom as i32);
        let x = (lon + 180.0) / 360.0 * n * TILE_SIZE as f64;
        let lat_rad = lat.to_radians();
        let y = (1.0 - lat_rad.tan().asinh() / std::f64::consts::PI) / 2.0 * n * TILE_SIZE as f64;
        (x, y)
    }
    
    pub fn pixel_to_lat_lon(&self, pixel_x: f64, pixel_y: f64, zoom: u8) -> (f64, f64) {
        let n = 2_f64.powi(zoom as i32);
        let lon = pixel_x / (n * TILE_SIZE as f64) * 360.0 - 180.0;
        let lat_rad = ((1.0 - 2.0 * pixel_y / (n * TILE_SIZE as f64)) * std::f64::consts::PI).sinh().atan();
        let lat = lat_rad.to_degrees();
        (lat, lon)
    }

    pub fn lat_lon_to_tile(&self, lat: f64, lon: f64, zoom: u8) -> TileCoord {
        let n = 2_f64.powi(zoom as i32);
        let x = ((lon + 180.0) / 360.0 * n).floor() as i32;
        let lat_rad = lat.to_radians();
        let y = ((1.0 - lat_rad.tan().asinh() / std::f64::consts::PI) / 2.0 * n).floor() as i32;
        TileCoord { x, y, z: zoom }
    }

    pub fn get_visible_tiles(&self) -> Vec<TileCoord> {
        let center_tile = self.lat_lon_to_tile(self.center_lat, self.center_lon, self.zoom);
        let tiles_x = (self.width / TILE_SIZE).ceil() as i32 + 2;
        let tiles_y = (self.height / TILE_SIZE).ceil() as i32 + 2;
        
        let mut tiles = Vec::new();
        for dy in -(tiles_y / 2)..=(tiles_y / 2) {
            for dx in -(tiles_x / 2)..=(tiles_x / 2) {
                tiles.push(TileCoord {
                    x: center_tile.x + dx,
                    y: center_tile.y + dy,
                    z: self.zoom,
                });
            }
        }
        tiles
    }
    
    pub fn geo_to_screen(&self, lat: f64, lon: f64) -> (f32, f32) {
        let pixel = self.lat_lon_to_pixel(lat, lon, self.zoom);
        let center_pixel = self.lat_lon_to_pixel(self.center_lat, self.center_lon, self.zoom);
        
        let screen_x = (pixel.0 - center_pixel.0) as f32 + self.width / 2.0;
        let screen_y = (pixel.1 - center_pixel.1) as f32 + self.height / 2.0;
        
        (screen_x, screen_y)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutePoint {
    pub lat: f64,
    pub lon: f64,
}

// OSRM API response structures
#[derive(Debug, Deserialize)]
struct OsrmResponse {
    routes: Vec<OsrmRoute>,
}

#[derive(Debug, Deserialize)]
struct OsrmRoute {
    geometry: OsrmGeometry,
    distance: f64,  // in meters
    duration: f64,  // in seconds
}

#[derive(Debug, Deserialize)]
struct OsrmGeometry {
    coordinates: Vec<Vec<f64>>,  // [lon, lat] pairs
}

pub struct Router;

impl Router {
    /// Calculate route using OSRM (Open Source Routing Machine) API
    /// Falls back to straight line if API is unavailable
    pub fn calculate_route(from: &GeoLocation, to: &GeoLocation) -> Vec<RoutePoint> {
        // Try OSRM first
        match Self::calculate_osrm_route(from, to) {
            Ok(route) if !route.is_empty() => {
                println!("✅ Successfully fetched OSRM road route with {} waypoints", route.len());
                route
            }
            Ok(_) => {
                println!("⚠️ OSRM returned empty route, using straight line");
                Self::calculate_straight_route(from, to)
            }
            Err(e) => {
                println!("⚠️ OSRM API error ({}), using straight line route", e);
                Self::calculate_straight_route(from, to)
            }
        }
    }

    /// Fetch real road route from OSRM API
    fn calculate_osrm_route(from: &GeoLocation, to: &GeoLocation) -> Result<Vec<RoutePoint>, Box<dyn std::error::Error>> {
        // OSRM expects coordinates as lon,lat (not lat,lon)
        let url = format!(
            "http://router.project-osrm.org/route/v1/driving/{},{};{},{}?overview=full&geometries=geojson",
            from.longitude, from.latitude,
            to.longitude, to.latitude
        );
        
        // Make HTTP request with timeout
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;
        
        let response = client.get(&url)
            .header("User-Agent", "SecureNotes/1.0")
            .send()?;
        
        if !response.status().is_success() {
            return Err(format!("OSRM API returned status: {}", response.status()).into());
        }
        
        let osrm_data: OsrmResponse = response.json()?;
        
        if osrm_data.routes.is_empty() {
            return Err("No routes found".into());
        }
        
        // Extract coordinates from the first route
        let route = &osrm_data.routes[0];
        let mut route_points = Vec::new();
        
        for coord in &route.geometry.coordinates {
            if coord.len() >= 2 {
                // OSRM returns [lon, lat], we need lat, lon
                route_points.push(RoutePoint {
                    lon: coord[0],
                    lat: coord[1],
                });
            }
        }
        
        Ok(route_points)
    }

    /// Fallback: Simple straight-line routing between two points
    fn calculate_straight_route(from: &GeoLocation, to: &GeoLocation) -> Vec<RoutePoint> {
        let steps = 20;
        let mut route = Vec::new();
        
        for i in 0..=steps {
            let t = i as f64 / steps as f64;
            let lat = from.latitude + (to.latitude - from.latitude) * t;
            let lon = from.longitude + (to.longitude - from.longitude) * t;
            route.push(RoutePoint { lat, lon });
        }
        
        route
    }

    /// Calculate distance between two points in kilometers using Haversine formula
    pub fn distance_km(from: &GeoLocation, to: &GeoLocation) -> f64 {
        let r = 6371.0; // Earth's radius in km
        
        let lat1 = from.latitude.to_radians();
        let lat2 = to.latitude.to_radians();
        let delta_lat = (to.latitude - from.latitude).to_radians();
        let delta_lon = (to.longitude - from.longitude).to_radians();
        
        let a = (delta_lat / 2.0).sin().powi(2)
            + lat1.cos() * lat2.cos() * (delta_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        
        r * c
    }

    /// Get sample locations for Denmark to Germany route
    pub fn get_sample_route() -> Vec<GeoLocation> {
        vec![
            GeoLocation::copenhagen(),
            GeoLocation::new(55.4038, 10.4024, "Odense, Denmark".to_string()),
            GeoLocation::new(55.0583, 9.9167, "Kolding, Denmark".to_string()),
            GeoLocation::new(54.7818, 9.4386, "Flensburg, Germany".to_string()),
            GeoLocation::new(54.3233, 10.1228, "Kiel, Germany".to_string()),
            GeoLocation::hamburg(),
            GeoLocation::berlin(),
        ]
    }
}
