# 🎯 Key Features Overview

## 🔐 Security Features
- **ChaCha20Poly1305 Encryption**: Military-grade AEAD cipher for note encryption
- **Argon2 Key Derivation**: Secure password-based key generation
- **Zero-Knowledge Architecture**: Password never leaves your device
- **Encrypted at Rest**: All notes encrypted on disk
- **Local-Only Storage**: No cloud, no servers, complete privacy

## 📝 Note Management
- **Create/Edit/Delete**: Full CRUD operations on notes
- **Markdown Support**: 
  - Headings (H1, H2, H3)
  - Bold and italic text
  - Bullet lists
  - Live preview mode
- **Search**: Real-time search across all note content, titles, and tags
- **Tags**: Organize notes with multiple tags
- **Timestamps**: Automatic creation and modification tracking

## 🗺️ Map & Location Features
- **Interactive Map Viewer**: 
  - Pan and zoom controls
  - Dynamic map rendering
  - Location markers
- **Route Planning**: 
  - Denmark to Germany route visualization
  - 7 major cities on route
  - Distance calculation using Haversine formula
- **Location-Based Notes**:
  - Attach GPS coordinates to notes
  - Quick location presets (Copenhagen, Berlin, Hamburg)
  - View note locations on map

## 🌍 Denmark → Germany Route
Predefined route through major cities:
1. **Copenhagen, Denmark** (55.6761°N, 12.5683°E)
2. **Odense, Denmark** (55.4038°N, 10.4024°E)
3. **Kolding, Denmark** (55.0583°N, 9.9167°E)
4. **Flensburg, Germany** (54.7818°N, 9.4386°E)
5. **Kiel, Germany** (54.3233°N, 10.1228°E)
6. **Hamburg, Germany** (53.5511°N, 9.9937°E)
7. **Berlin, Germany** (52.5200°N, 13.4050°E)

Total approximate distance: ~450 km

## 🎨 User Interface
- **Modern GUI**: Built with egui for cross-platform support
- **Responsive Design**: Adapts to different window sizes
- **Intuitive Navigation**: 
  - Sidebar for note list
  - Main panel for editing/viewing
  - Map view toggle
- **Visual Feedback**: Icons and emojis for better UX
- **Keyboard Shortcuts**: Enter to unlock, quick navigation

## 🔧 Technical Stack
- **Language**: Rust (Edition 2021)
- **GUI Framework**: eframe/egui 0.28
- **Encryption**: 
  - chacha20poly1305 0.10
  - argon2 0.5
- **Markdown**: pulldown-cmark 0.11
- **Serialization**: serde 1.0 + serde_json
- **Date/Time**: chrono 0.4

## 📦 Project Structure
```
secure-notes/
├── src/
│   ├── main.rs      # Application entry point
│   ├── crypto.rs    # Encryption/decryption (ChaCha20, Argon2)
│   ├── note.rs      # Note data structures
│   ├── storage.rs   # Encrypted file storage
│   ├── map.rs       # OpenStreetMap integration
│   └── ui.rs        # GUI implementation (egui)
```

## 🚀 Quick Start
```bash
# Build the app
cargo build --release

# Run the app
cargo run --release

# Or use the batch file (Windows)
run.bat
```

## 💾 Data Storage Location
- **Windows**: `C:\Users\<Username>\AppData\Roaming\secnotes\SecureNotes\notes.enc`
- **Linux**: `~/.local/share/secnotes/SecureNotes/notes.enc`
- **macOS**: `~/Library/Application Support/com.secnotes.SecureNotes/notes.enc`

## 🔒 Security Guarantees
✅ End-to-end encryption  
✅ Local-only storage  
✅ No network requests (except future OSM tiles)  
✅ Password-based key derivation  
✅ Nonce-based encryption (no key reuse)  
✅ AEAD cipher (authenticated encryption)  

## 🎯 Use Cases
- **Personal Journal**: Keep encrypted daily notes
- **Travel Planning**: Plan trips with map integration
- **Study Notes**: Markdown support for formatted notes
- **Location Diary**: Document places you visit
- **Secure Ideas**: Store sensitive information locally
- **Route Documentation**: Track and plan travel routes

## 🌟 What Makes This Special
1. **True E2EE**: Unlike cloud-based notes apps, your password never leaves your device
2. **Rust Security**: Memory-safe implementation reduces vulnerability surface
3. **Map Integration**: Unique combination of notes and geographic data
4. **Offline First**: Works completely offline, no internet required
5. **Cross-Platform**: Runs on Windows, Linux, and macOS
6. **Lightweight**: Small binary size, fast startup
7. **Open Architecture**: All code is visible and auditable
