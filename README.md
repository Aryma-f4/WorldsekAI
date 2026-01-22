# Portable Malware Scanner

Powered with supervised learning.

## Components

### 1. Admin Web (`admin-web`)
- **Stack:** ElysiaJS (Node.js/Bun), PostgreSQL.
- **Purpose:** Manages users, malware uploads, and model versioning.

### 2. Scanner Shell (`scanner-shell`)
- **Stack:** Rust.
- **Purpose:** Cross-platform CLI tool (Linux, Windows, macOS) to scan directories and files using the trained model.

### 3. ML Engine (`ml-engine`)
- **Stack:** Python.
- **Purpose:** Trains models based on uploaded malware samples.
