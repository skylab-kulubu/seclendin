# Seclendin

## Build Environment

### [Install Rustup](https://rustup.rs/)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Add Target

`seclendin` targets `Windows` machines primarly, however you can build your own instance for `*nix` targets as well as `Windows` yet It may not work properly.

```bash
rustup target add x86_64-pc-windows-gnu
```

## Building

### .env

```env

KEYBOARD_LAYOUT=0000041F
WALLPAPER_URL=https://github.com/farukerdem34/catppuccin-wallpapers/blob/main/flatppuccin/glow-skylab.png?raw=true
SCREEN_RES_W=1024
SCREEN_RES_H=768
TARGET_LANG=tr-tr
TARGET_LANG_REG=041F
TARGET_LANG_NUM=0
FLIP_FUNC_URL="https://github.com/skylab-kulubu/seclendin/raw/refs/heads/main/ps1/flip_screen.ps1"
```

#### Trouble Shooting Some issues

##### Exit Code 2 On Changing Resulation

You have te declare valid resulation ratio that target machine has. Eg. if the target machine doesn't have `800x600` resulation option, it will return `exit code 2`.

### Command

```bash
export $(grep -v '^#' .env | xargs)
cargo build --target x86_64-pc-windows-gnu --release
```

## TO DO

- self-destruction on virtual machines, running on wine, if IDA/Ghidra/Radare2 etc. installed on system
