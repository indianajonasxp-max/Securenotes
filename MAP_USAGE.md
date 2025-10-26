# 🗺️ OpenStreetMap Integration - Now Live!

## ✅ What's Working

Your notes app now displays **real OpenStreetMap tiles** directly on the interactive map!

### Features

- **Real Map Tiles**: Actual OpenStreetMap data from `tile.openstreetmap.org`
- **Automatic Caching**: Tiles are cached locally for faster loading
- **Interactive Navigation**: Click and drag to pan, zoom controls
- **Click-to-Route**: Set start/end points anywhere on the actual map
- **Real Road Routing**: OSRM-powered routing that follows actual roads
- **Distance Calculation**: Real distances between any two points on Earth

## 🚀 How It Works

### Tile Loading
1. The app determines which map tiles are visible based on your zoom level and position
2. Tiles are downloaded from OpenStreetMap servers
3. Images are converted to GPU textures for smooth rendering
4. Tiles are cached in: `AppData\Roaming\secnotes\SecureNotes\cache\tiles\`

### Map Projection
- **System**: Web Mercator (EPSG:3857)
- **Tile Size**: 256×256 pixels
- **Zoom Levels**: 2-18 supported
- **Coordinates**: WGS84 (latitude/longitude)

## 📍 Interactive Routing

### Setting Up a Route

1. **Navigate to your starting location**:
   - Use city preset buttons (Copenhagen, Berlin, London, Paris, Madrid)
   - Or drag the map to any location

2. **Set Start Point**:
   - Click "📍 Set Start Point" button
   - Click on the map where you want to start
   - Blue marker appears

3. **Set End Point**:
   - Click "🎯 Set End Point" button
   - Click on the map where you want to go
   - Orange marker appears

4. **Calculate Route**:
   - Click "🧭 Calculate Route"
   - Route line appears with distance

### Examples

#### Route from Copenhagen to Berlin
```
1. Click "🇩🇰 Copenhagen" preset button
2. Click "📍 Set Start Point", then click on Copenhagen
3. Click "🇩🇪 Berlin" preset button
4. Click "🎯 Set End Point", then click on Berlin
5. Click "🧭 Calculate Route"
Result: ~355 km route displayed
```

#### Route from Any City to Any City
```
1. Zoom out to see both cities
2. Click "📍 Set Start Point"
3. Click on your starting city
4. Click "🎯 Set End Point"
5. Click on your destination
6. Click "🧭 Calculate Route"
```

## 🎮 Controls

| Action | How To |
|--------|--------|
| **Pan Map** | Click and drag |
| **Zoom In** | Click "➕ Zoom In" button |
| **Zoom Out** | Click "➖ Zoom Out" button |
| **Set Start** | Click button, then click map |
| **Set End** | Click button, then click map |
| **Clear Route** | Click "🗑️ Clear Route" |
| **Sample Route** | Click "🌍 Sample: DK→DE" |

## 📊 Quick Reference Cities

| City | Country | Coordinates |
|------|---------|-------------|
| Copenhagen | 🇩🇰 Denmark | 55.68°N, 12.57°E |
| Berlin | 🇩🇪 Germany | 52.52°N, 13.41°E |
| London | 🇬🇧 UK | 51.51°N, 0.13°W |
| Paris | 🇫🇷 France | 48.86°N, 2.35°E |
| Madrid | 🇪🇸 Spain | 40.42°N, 3.70°W |

## ⚡ Performance

### Tile Caching
- **First Load**: Downloads tiles from OpenStreetMap (requires internet)
- **Subsequent Loads**: Uses local cache (works offline for visited areas)
- **Cache Location**: `%APPDATA%\secnotes\SecureNotes\cache\tiles\`

### Network Usage
- Tiles are only downloaded once
- Cached tiles persist between app restarts
- Respects OpenStreetMap usage policy with proper User-Agent

## 🌍 Coverage

The map covers **the entire world**! You can:
- Navigate to any country
- Route between any two cities
- Explore any location on Earth
- Zoom in to street level (zoom 18)
- Zoom out to world view (zoom 2)

## 💡 Tips

**Tip 1**: Let tiles load before panning quickly - they download in the background

**Tip 2**: Zoom in closer for precise point selection when routing

**Tip 3**: Use the city preset buttons to quickly jump to major locations

**Tip 4**: The app caches tiles, so revisiting areas is instant

**Tip 5**: Drag the map while routing to explore the route path

## 🔧 Technical Details

### Tile URL Format
```
https://tile.openstreetmap.org/{z}/{x}/{y}.png
```

### Coordinate Conversion
```rust
// Latitude/Longitude → Tile Coordinates
x = floor((lon + 180) / 360 * 2^zoom)
y = floor((1 - asinh(tan(lat)) / π) / 2 * 2^zoom)

// Tile Coordinates → Latitude/Longitude
lon = x / 2^zoom * 360 - 180
lat = atan(sinh(π * (1 - 2 * y / 2^zoom)))
```

### Distance Calculation
Uses the Haversine formula for great circle distance on a sphere:
```
a = sin²(Δlat/2) + cos(lat1) * cos(lat2) * sin²(Δlon/2)
c = 2 * atan2(√a, √(1−a))
distance = R * c  (where R = 6371 km)
```

## 📝 OpenStreetMap Attribution

This application uses map data from © OpenStreetMap contributors.

**OpenStreetMap License**: Open Database License (ODbL)
**More Info**: https://www.openstreetmap.org/copyright

### Usage Policy
- Tiles are cached locally to minimize server load
- User-Agent identifies the application
- Tile downloads respect server limits

## 🆘 Troubleshooting

**Problem**: Map tiles not loading
- **Solution**: Check internet connection (first time only)
- **Solution**: Wait a few seconds for downloads

**Problem**: Map is just blue
- **Solution**: Tiles are loading in background, wait and pan slightly

**Problem**: Can't see my route
- **Solution**: Make sure both start and end points are set
- **Solution**: Click "🧭 Calculate Route" after setting points

**Problem**: Route goes through oceans
- **Solution**: ✅ Real road routing is now implemented using OSRM!
- **Solution**: Routes follow actual roads whenever possible
- **Fallback**: Straight-line route shown if OSRM API is unavailable

## 🎯 Coming Soon

Future enhancements:
- 🔍 Location search by name/address
- 📍 Save and manage favorite locations
- 🌐 Offline map bundles
- 🗺️ Alternative map styles
- 📊 Elevation profiles
- ⏱️ Travel time estimates
- 🚗 Multiple routing profiles (car, bike, walking)

---

**Enjoy exploring the world with your encrypted notes!** 🌍✈️🔒
