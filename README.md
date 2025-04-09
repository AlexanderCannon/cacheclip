# CacheClip

A lightweight command-line clipboard history manager written in Rust.

## Features

- Silently monitors and stores clipboard contents
- Quick terminal search with fuzzy matching
- Restore previous clipboard items instantly
- Deduplicate repeated clipboard entries
- Easy to use command-line interface

## Installation

### From source

```bash
git clone https://github.com/yourusername/cacheclip.git
cd cacheclip
cargo install --path .
```

## Usage

Start the daemon to begin tracking clipboard history:

```bash
cacheclip daemon
```

It's recommended to set up the daemon to run on startup for your operating system.

### Commands

List recent clipboard items:
```bash
cacheclip list
```

List a specific number of items:
```bash
cacheclip list --count 20
```

Search clipboard history:
```bash
cacheclip search "api key"
```

Restore a clipboard item by index:
```bash
cacheclip restore 3
```

Clear clipboard history:
```bash
cacheclip clear
```

Show help:
```bash
cacheclip --help
```

## Data Storage

CacheClip stores your clipboard history in:
- Linux: `~/.local/share/cacheclip/history.json`
- macOS: `~/Library/Application Support/com.cacheclip.cacheclip/history.json`
- Windows: `%APPDATA%\cacheclip\cacheclip\data\history.json`

## License

MIT