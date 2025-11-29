# ZeroVault

> **Encrypt first. Store anywhere. Only you can unlock.**

ZeroVault is a **zero-knowledge encrypted storage tool** built in **Rust**.  
It lets you securely upload and manage files across multiple cloud platforms — starting with Google Drive — while keeping **full cryptographic control**.

Cloud providers become **blind storage**:  
they store your files, but cannot read or analyze them.

<br>

## Why ZeroVault?

Modern cloud services are convenient—but they aren’t private.  
Companies scan, index, and classify files for analytics or AI.  
If accounts get hacked or devices are stolen, raw data is exposed.

ZeroVault fixes that:

- Files are encrypted **locally** before upload.
- Encryption keys **never leave your device**.
- Even with full access to your cloud storage, attackers only see **ciphertext**.

<br>

## Key Features (Planned)

- 🔐 **Strong Encryption**
  - AES-256-GCM / XChaCha20-Poly1305
  - Argon2 or scrypt for password-derived keys

- ☁️ **Cloud Provider Support**
  - Google Drive (first target)
  - OneDrive
  - Dropbox
  - Amazon S3 / WebDAV (future roadmap)

- 💻 **Cross-Platform CLI**
  - Windows, Linux, macOS
  - Portable single binary (no runtime required)

- 📦 **Encrypted Metadata & Filenames**
  - Optional mode to hide folder structure and file names

- 📁 **Streaming Encryption**
  - Handles large files efficiently without loading everything into memory

- 🛡 Zero-Knowledge Design
  - No telemetry, tracking, analytics, or cloud processing

<br>

## Philosophy

> **Your cloud is just storage — your security stays with you.**

ZeroVault is built around the idea of **user ownership and independence**.
Cloud services shouldn't require trust, only space.

<br>

## Status

   **Under active development — not ready for real data yet.**  
This project is being built feature-by-feature in public.

<br>


## Project Roadmap

| Stage | Feature | Status |
|-------|---------|--------|
| 🔹 1 | Project setup & core encryption format | ⏳ planned |
| 🔹 2 | Local encrypt/decrypt CLI commands | ⏳ planned |
| 🔹 3 | Google Drive integration | ⏳ planned |
| 🔹 4 | Metadata encryption | ⏳ planned |
| 🔹 5 | Multi-cloud support | ⏳ future |
| 🔹 6 | Binary distribution & installers | ⏳ future |

<br>

## Development Requirements

- **Rust** (latest stable)
- (later) provider SDKs and optional runtime dependencies depending on backends

<br>

## Security Note

> If you lose your password, **your data is permanently unrecoverable** — there is no reset, no recovery service, and no backdoor.

This is a security tool, not a convenience tool.

<br>

## License

License: MIT — free to use, modify, and distribute.

<br>

## Contributing

Right now the project is in early design and implementation.  
Contributions, ideas, and feedback are welcome once the initial structure settles.

<br>

## Support the Project

If you like the idea, star ⭐ the repo to follow development.
