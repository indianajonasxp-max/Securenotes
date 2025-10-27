# 🔒 Secure Notes - E2EE Notes App with OpenStreetMap

A local, end-to-end encrypted notes application written in Rust with integrated OpenStreetMap support, Markdown rendering, and route planning capabilities.

## ✨ Features

### 🔐 Security
- **End-to-End Encryption (E2EE)**: All notes are encrypted using ChaCha20Poly1305 AEAD cipher
- **Password-based Key Derivation**: Uses Argon2 for secure password-based encryption keys
- **Local Storage**: All data stored locally on your machine, never sent to any server
- **Encrypted at Rest**: Notes are encrypted when saved to disk

### 📝 Notes Management
- Create, edit, and delete notes
- Full Markdown support with live preview
- Search functionality across all notes
- Add tags to organize notes
- Attach geographic locations to notes
- Track creation and modification timestamps

### 🗺️ OpenStreetMap Integration
- Interactive map viewer with real OSM tiles
- Location-based notes
- **Real road routing** using OSRM API (follows actual roads!)
- Click-to-route: Set start/end points anywhere on the map
- Distance calculation using Haversine formula
- Pan and zoom controls
- City presets for quick navigation:
  - Copenhagen, Denmark
  - Berlin, Germany
  - London, UK
  - Paris, France
  - Madrid, Spain

### 📄 Markdown Support
Features include:
- Headings (H1, H2, H3)
- Bold and italic text
- Bullet lists
- Code blocks
- Live preview mode

## 🚀 Getting Started

### Prerequisites
- Rust 1.70 or higher
- Cargo package manager

### Installation

1. Clone the repository to ensure you have the latest features:
```bash
git clone https://github.com/<your-username>/Securenotes.git
cd Securenotes
```

2. Build the project with Cargo:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run --release
```

### First Launch

On first launch, you'll be prompted to create a password. This password will be used to encrypt all your notes. 

**⚠️ IMPORTANT**: Keep your password safe! If you lose it, there's no way to recover your notes as they are end-to-end encrypted.

## 📖 Usage

### Creating Notes
1. Click the "➕ New Note" button in the toolbar
2. Edit the title and content
3. Use Markdown syntax for formatting
4. Click "💾 Save" to save your changes

### Adding Locations
1. Open a note
2. Click one of the quick location buttons (Copenhagen, Berlin, Hamburg)
3. Or add custom coordinates
4. View the location on the map by clicking "View on Map"

### Viewing Maps
1. Click the "🗺️ Map" button in the toolbar
2. Use the zoom controls or drag to navigate
3. Click "Show Route DK→DE" to see a sample route from Denmark to Germany
4. The map displays major cities and calculates the total distance

### Markdown Tips
```markdown
# Heading 1
## Heading 2
### Heading 3

**Bold text**
*Italic text*

- Bullet point 1
- Bullet point 2

Code: `inline code`
```

### Searching Notes
Use the search box in the toolbar to filter notes by title, content, or tags.

## 🏗️ Architecture

### Modules
- **crypto**: Encryption/decryption using ChaCha20Poly1305 and Argon2
- **note**: Note data structures and management
- **storage**: Encrypted storage with automatic save/load
- **map**: OpenStreetMap integration and routing
- **ui**: GUI built with egui framework

### Data Storage
Notes are stored in an encrypted file at:
- **Windows**: `C:\Users\<Username>\AppData\Roaming\secnotes\SecureNotes\notes.enc`
- **Linux**: `~/.local/share/secnotes/SecureNotes/notes.enc`
- **macOS**: `~/Library/Application Support/com.secnotes.SecureNotes/notes.enc`

### Encryption Details
- **Cipher**: ChaCha20Poly1305 (AEAD)
- **Key Derivation**: Argon2id
- **Key Size**: 256 bits
- **Nonce**: 96 bits (randomly generated per encryption)

## 🔧 Development

### Project Structure
```
dd/
├── Cargo.toml           # Workspace configuration
├── README.md            # This file
└── secure-notes/
    ├── Cargo.toml       # Package configuration
    └── src/
        ├── main.rs      # Application entry point
        ├── crypto.rs    # Encryption/decryption
        ├── note.rs      # Note data structures
        ├── storage.rs   # Encrypted storage
        ├── map.rs       # OpenStreetMap integration
        └── ui.rs        # GUI implementation
```

### Dependencies
- **eframe/egui**: Cross-platform GUI framework
- **chacha20poly1305**: AEAD encryption
- **argon2**: Password hashing
- **pulldown-cmark**: Markdown parsing
- **chrono**: Date/time handling
- **serde**: Serialization
- **reqwest**: HTTP client (for future OSM tile loading)

### Building for Release
```bash
cargo build --release
```

The compiled binary will be in `target/release/secure-notes` (or `secure-notes.exe` on Windows).

## 🛣️ Roadmap

Completed features:
- [x] Real OpenStreetMap tile loading and caching
- [x] OSRM integration for actual route planning

Future improvements could include:
- [ ] Export notes to encrypted backup files
- [ ] Import/export functionality
- [ ] Rich text editor with more Markdown features
- [ ] Custom location picker with geocoding
- [ ] Note attachments (images, files)
- [ ] Multiple notebooks/folders
- [ ] Dark/light theme toggle
- [ ] Sync across devices (with E2EE)
- [ ] Multiple routing profiles (car, bike, walking)

## 🔒 Security Considerations

- This app stores the encryption key in memory while unlocked
- Network requests are made only for:
  - OpenStreetMap tile downloads (cached locally)
  - OSRM routing API (only coordinates, no personal data)
- All note data remains local to your machine
- Use a strong, unique password
- The app locks when closed - you must enter password again
- Consider backing up the encrypted `notes.enc` file

## 📝 License

This project is provided as-is for educational and personal use.

## 🙏 Acknowledgments

- OpenStreetMap contributors for map data
- egui framework for the excellent GUI toolkit
- Rust cryptography working group for crypto libraries
