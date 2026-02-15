# hlm-wad-tools

[![Crates.io Version](https://img.shields.io/crates/v/hlm-wad-tools?style=for-the-badge)](https://crates.io/crates/hlm-wad-tools)

Tools for extracting and repacking Hotline Miami's `.wad` archive files.

## Quick Start

```bash
cargo install hlm-wad-tools

# Extract WAD file
hlm-wad-tools extract original.wad extracted/

# Make your modifications to files in extracted/

# Repack WAD file
hlm-wad-tools repack extracted/ modified.wad
```

## WAD File Format

```txt
┌─────────────────────────────────┐
│ Header (4 bytes)                │  File count
├─────────────────────────────────┤
│ File Table                      │  Metadata for all files
│  - Entry 1                      │
│  - Entry 2                      │
│  - ...                          │
├─────────────────────────────────┤  ← table_end
│ Data Section                    │  Actual file contents
│  - File 1 data                  │
│  - File 2 data                  │
│  - ...                          │
└─────────────────────────────────┘
```

Each entry is structured as:

| Size | Field | Type | Description |
|------|-------|------|-------------|
| 4 bytes | Name Length | `u32` (LE) | Length of the filename string |
| N bytes | Filename | UTF-8 string | Path to file (using `/` separators) |
| 4 bytes | File Size | `u32` (LE) | Size of the file data in bytes |
| 4 bytes | Padding | `0x00` | Reserved (zeros) |
| 4 bytes | File Offset | `u32` (LE) | Offset relative to table end |
| 4 bytes | Padding | `0x00` | Reserved (zeros) |
