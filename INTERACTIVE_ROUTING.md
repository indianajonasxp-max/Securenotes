# 🗺️ Interactive Routing Guide

## Overview
The app now supports **interactive routing between any two locations** on the map! You can route from any country or city to anywhere else in the world.

## How to Use

### Method 1: Click-to-Route (Interactive)

1. **Open the Map View**
   - Click the "🗺️ Map" button in the toolbar

2. **Set Your Starting Point**
   - Click the "📍 Set Start Point" button
   - Click anywhere on the map to set your starting location
   - A blue marker will appear with "📍 START"

3. **Set Your Destination**
   - Click the "🎯 Set End Point" button  
   - Click another location on the map
   - An orange marker will appear with "🎯 END"

4. **Calculate the Route**
   - Click the "🧭 Calculate Route" button
   - The app will draw a route between your two points
   - You'll see the total distance in kilometers

5. **Clear and Start Over**
   - Click "🗑️ Clear Route" to remove all markers and start fresh

### Method 2: Quick City Navigation

Use the preset city buttons to quickly jump to major locations:

- **🇩🇰 Copenhagen** - Denmark (55.6761°N, 12.5683°E)
- **🇩🇪 Berlin** - Germany (52.5200°N, 13.4050°E)
- **🇬🇧 London** - United Kingdom (51.5074°N, -0.1278°W)
- **🇫🇷 Paris** - France (48.8566°N, 2.3522°E)
- **🇪🇸 Madrid** - Spain (40.4168°N, -3.7038°W)

### Method 3: Sample Route

Click "🌍 Sample: DK→DE" to see a pre-defined route from Denmark to Germany through 7 major cities.

## Map Controls

- **🖱️ Drag**: Click and drag to pan the map
- **➕ Zoom In**: Increase map detail
- **➖ Zoom Out**: Decrease map detail
- **Click**: When in selection mode, click to set start/end points

## Example Use Cases

### 1. Plan a Road Trip
```
1. Click "📍 Set Start Point"
2. Click on your home location (e.g., Paris)
3. Click "🎯 Set End Point"
4. Click on your destination (e.g., Rome)
5. Click "🧭 Calculate Route"
6. View distance and waypoints
```

### 2. Measure Distance Between Cities
```
1. Use city preset buttons to jump to first city
2. Click "📍 Set Start Point" and click the map
3. Use another city preset button
4. Click "🎯 Set End Point" and click the map
5. Calculate route to see distance
```

### 3. Explore Any Location
```
1. Click a city preset button to jump there
2. Use zoom controls to get closer
3. Drag to explore the area
4. Set points to plan routes
```

## Features

✅ **Any-to-Any Routing**: Route between any two points on Earth  
✅ **Visual Feedback**: Clear START (blue) and END (orange) markers  
✅ **Distance Calculation**: Haversine formula for accurate distances  
✅ **Smooth Routes**: 10 waypoints calculated between start and end  
✅ **Interactive**: Click directly on the map - no typing required  
✅ **Quick Navigation**: Preset buttons for major European cities  
✅ **Pan & Zoom**: Explore any area of the world  

## Visual Indicators

| Marker | Meaning | Color |
|--------|---------|-------|
| 📍 START | Your starting point | Blue |
| 🎯 END | Your destination | Orange |
| Yellow dots | Route waypoints | Yellow |
| Blue line | Calculated route | Blue |

## Tips

💡 **Tip 1**: Zoom in closer for more precise point selection  
💡 **Tip 2**: Use city presets to quickly navigate to known locations  
💡 **Tip 3**: The route automatically centers between your start and end points  
💡 **Tip 4**: Distance is calculated using the Haversine formula (great circle distance)  
💡 **Tip 5**: Create notes with locations and view them on the map  

## Technical Details

- **Projection**: Web Mercator (EPSG:3857)
- **Distance Calculation**: Haversine formula for spherical Earth
- **Waypoints**: 10 intermediate points per route
- **Coordinates**: WGS84 (latitude/longitude)
- **Map**: OpenStreetMap visualization

## Future Enhancements

Coming soon:
- 🛣️ Real road routing using OSRM
- 🗺️ Actual OpenStreetMap tile rendering
- 🔍 Location search by name
- 📍 Save favorite locations
- 🌐 Support for more coordinate systems
- 📊 Elevation profiles
- ⏱️ Estimated travel time

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| Mouse Drag | Pan the map |
| Click | Set point (when mode active) |

Enjoy exploring and routing anywhere in the world! 🌍✈️
