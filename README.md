# FedEx Base64 Decoder

A desktop application built with Tauri and React that decodes base64 strings from FedEx Ship API and saves them as files with automatic type detection.

## Features

- Decode base64 strings from FedEx Ship API
- Automatic file type detection (PDF, PNG, ZPLII, EPL2)
- Save files with timestamp-based filenames (YYYYMMDD-HHMMSS.ext)
- Clean, modern UI
- Single executable file (no installer required)

## Supported File Types

- **PDF** - Portable Document Format
- **PNG** - Portable Network Graphics
- **ZPLII** - Zebra Programming Language II
- **EPL2** - Eltron Programming Language 2

## Development

### Prerequisites

- Node.js (v18 or higher)
- Rust (latest stable)
- Tauri CLI: `npm install -g @tauri-apps/cli`

### Setup

1. Install dependencies:
```bash
npm install
```

2. Run in development mode:
```bash
npm run tauri dev
```

### Build

Build the executable:
```bash
npm run tauri build
```

The executable will be in `src-tauri/target/release/` (or `src-tauri/target/release/bundle/` for installer).

## Usage

1. Launch the application
2. Paste your base64 encoded string from FedEx Ship API into the input box
3. Click "Decode & Save"
4. Choose a location to save the file in the dialog
5. The file will be saved with a timestamp-based filename

