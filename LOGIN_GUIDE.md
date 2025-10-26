# 🔐 Login Screen Guide

## Two Separate Buttons

The app now has **distinct buttons** for creating new storage vs unlocking existing notes!

## 🆕 First Time Use

When you launch the app for the first time (no existing encrypted data):

```
┌─────────────────────────────────────┐
│     🔒 Secure Notes                 │
│                                     │
│  End-to-End Encrypted Local Notes  │
│  with OpenStreetMap Integration    │
│                                     │
│  Password: [______________]        │
│                                     │
│  ┌─────────────────────────┐       │
│  │ ✨ Create New Storage   │       │
│  └─────────────────────────┘       │
│                                     │
│  🆕 No existing notes found        │
│  Enter a password to create new    │
│  encrypted storage                 │
└─────────────────────────────────────┘
```

### Steps:
1. **Enter a strong password** (you choose it)
2. **Click "✨ Create New Storage"**
3. Your encrypted storage is created!

**⚠️ IMPORTANT**: Remember this password! It's the only way to decrypt your notes.

## 🔓 Returning User

When you launch the app and encrypted data exists:

```
┌─────────────────────────────────────┐
│     🔒 Secure Notes                 │
│                                     │
│  End-to-End Encrypted Local Notes  │
│  with OpenStreetMap Integration    │
│                                     │
│  Password: [______________]        │
│                                     │
│  ┌─────────────────────────┐       │
│  │ 🔓 Unlock Existing Notes│       │
│  └─────────────────────────┘       │
│                                     │
│  📁 Existing encrypted notes found │
│  Enter your password to unlock     │
└─────────────────────────────────────┘
```

### Steps:
1. **Enter your password**
2. **Click "🔓 Unlock Existing Notes"** (or press Enter)
3. Your notes are decrypted and loaded!

## 🎯 Key Differences

| Scenario | Button | Icon | Message |
|----------|--------|------|---------|
| **First Time** | Create New Storage | ✨ | "No existing notes found" |
| **Has Data** | Unlock Existing Notes | 🔓 | "Existing encrypted notes found" |

## ⌨️ Keyboard Shortcut

You can press **Enter** after typing your password instead of clicking the button!

## 🔴 Error Messages

If you see errors:

**"Failed to unlock: ..."**
- Wrong password entered
- Corrupted data file
- Solution: Try correct password or check data file

## 🔒 Security Notes

### Password Requirements
- **No minimum length** (your choice)
- **No complexity requirements** (your choice)
- **Recommendation**: Use a strong, memorable password

### What Happens Behind the Scenes

**Creating New Storage:**
1. App generates a random salt
2. Derives encryption key from your password using Argon2
3. Creates encrypted storage file
4. Saves to: `AppData\Roaming\secnotes\SecureNotes\notes.enc`

**Unlocking Existing Storage:**
1. Loads salt from existing file
2. Derives key from your password + salt
3. Attempts to decrypt notes
4. If successful, loads your notes
5. If failed, shows error

## 📁 Data Location

Your encrypted notes are stored at:

**Windows:**
```
C:\Users\<YourName>\AppData\Roaming\secnotes\SecureNotes\notes.enc
```

**Linux:**
```
~/.local/share/secnotes/SecureNotes/notes.enc
```

**macOS:**
```
~/Library/Application Support/com.secnotes.SecureNotes/notes.enc
```

## 💡 Tips

**Tip 1**: Choose a password you'll remember - there's no password recovery!

**Tip 2**: The button text tells you exactly what will happen

**Tip 3**: Press Enter after typing password for quick login

**Tip 4**: Your password never leaves your device - true E2EE!

**Tip 5**: Back up your `notes.enc` file to preserve your notes

## 🆘 Troubleshooting

**Q: I forgot my password!**
- A: Unfortunately, there's no way to recover encrypted notes without the password
- This is by design - true end-to-end encryption

**Q: Can I change my password?**
- A: Not yet - this feature may be added in future versions
- Workaround: Export notes, delete storage, create new with different password

**Q: The button says "Create" but I have notes!**
- A: Your `notes.enc` file might have been deleted
- Check the data location above

**Q: The button says "Unlock" but this is my first time!**
- A: An old `notes.enc` file exists
- Either unlock it or delete it to start fresh

## 🎉 That's It!

Simple, clear buttons:
- ✨ **Create** for new users
- 🔓 **Unlock** for returning users

Your data stays encrypted and secure! 🔒
