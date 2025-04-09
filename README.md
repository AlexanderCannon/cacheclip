# CacheClip

A lightweight command-line clipboard history manager written in Rust.

## Features

- Silently monitors and stores clipboard contents
- Quick terminal search with fuzzy matching
- Restore previous clipboard items instantly
- Deduplicate repeated clipboard entries
- Easy to use command-line interface

## Installation

### Download pre-compiled binary

#### Linux/macOS (x86_64)
```bash
# Download the latest release for your platform
curl -L https://github.com/alexandercannon/cacheclip/releases/download/v0.1.0/cacheclip-v0.1.0-x86_64-linux.tar.gz -o cacheclip.tar.gz
# or for macOS
# curl -L https://github.com/alexandercannon/cacheclip/releases/download/v0.1.0/cacheclip-v0.1.0-x86_64-macos.tar.gz -o cacheclip.tar.gz

# Extract the archive
tar -xzf cacheclip.tar.gz

# Make the binary executable
chmod +x cacheclip

# Move to a directory in your PATH
sudo mv cacheclip /usr/local/bin/
```

#### Linux/macOS (ARM64/Apple Silicon)
```bash
# Download the latest release for your platform
curl -L https://github.com/alexandercannon/cacheclip/releases/download/v0.1.0/cacheclip-v0.1.0-arm64-linux.tar.gz -o cacheclip.tar.gz
# or for macOS with Apple Silicon
# curl -L https://github.com/alexandercannon/cacheclip/releases/download/v0.1.0/cacheclip-v0.1.0-arm64-macos.tar.gz -o cacheclip.tar.gz

# Extract the archive
tar -xzf cacheclip.tar.gz

# Make the binary executable
chmod +x cacheclip

# Move to a directory in your PATH
sudo mv cacheclip /usr/local/bin/
```

#### Windows
1. Download the latest release for your architecture (x86_64 or arm64) from [https://github.com/alexandercannon/cacheclip/releases](https://github.com/alexandercannon/cacheclip/releases)
2. Extract the ZIP file
3. Add the extracted directory to your PATH or move the executable to a directory that's already in your PATH

### From source

```bash
git clone https://github.com/alexandercannon/cacheclip.git
cd cacheclip
cargo install --path .
```

## Building from source for different architectures

The repository includes a build script to create binaries for multiple architectures:

```bash
# Make the script executable
chmod +x scripts/build.sh

# Run the build script
./scripts/build.sh

# Binaries will be placed in the dist/ directory
```

## Usage

Start the daemon to begin tracking clipboard history:

```bash
cacheclip daemon
```

It's recommended to set up the daemon to run on startup for your operating system.

### Setting up to run on startup

#### Linux (systemd)
Create a systemd service file:

```bash
cat > ~/.config/systemd/user/cacheclip.service << EOL
[Unit]
Description=CacheClip clipboard manager daemon
After=default.target

[Service]
ExecStart=/usr/local/bin/cacheclip daemon
Restart=always

[Install]
WantedBy=default.target
EOL

# Enable and start the service
systemctl --user enable cacheclip.service
systemctl --user start cacheclip.service
```

#### macOS
Create a launch agent:

```bash
cat > ~/Library/LaunchAgents/com.cacheclip.daemon.plist << EOL
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.cacheclip.daemon</string>
    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/bin/cacheclip</string>
        <string>daemon</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
</dict>
</plist>
EOL

# Load the launch agent
launchctl load ~/Library/LaunchAgents/com.cacheclip.daemon.plist
```

#### Windows
Create a shortcut to `cacheclip.exe daemon` in your startup folder (`%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup`).

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
- Linux: `~/.local/share/cacheclip/cacheclip/history.json`
- macOS: `~/Library/Application Support/com.cacheclip.cacheclip/history.json`
- Windows: `%APPDATA%\cacheclip\cacheclip\data\history.json`

## License

MIT