# netviz

[![Release](https://img.shields.io/github/v/release/IchBinLeoon/netviz?style=flat-square)](https://github.com/IchBinLeoon/netviz/releases)
[![Downloads](https://img.shields.io/crates/d/netviz?style=flat-square)](https://crates.io/crates/netviz)
[![Lint](https://img.shields.io/github/actions/workflow/status/IchBinLeoon/netviz/lint.yml?style=flat-square&label=lint)](https://github.com/IchBinLeoon/netviz/actions/workflows/lint.yml)
[![Publish](https://img.shields.io/github/actions/workflow/status/IchBinLeoon/netviz/publish.yml?style=flat-square&label=publish)](https://github.com/IchBinLeoon/netviz/actions/workflows/publish.yml)
[![License](https://img.shields.io/github/license/IchBinLeoon/netviz?style=flat-square)](https://github.com/IchBinLeoon/netviz/blob/main/LICENSE)

A simple network traffic monitor and visualizer.

<img src=".github/preview.gif" alt="Preview">

## Installation
### Cargo
```
cargo install netviz
```

### Build it from source
```
git clone https://github.com/IchBinLeoon/netviz
cd netviz
cargo build --release
./target/release/netviz
```

## Usage
- Use `←` or `→` to switch between interfaces
- Press `p` to pause
- Press `q` to quit

```
netviz
```

## Contribute
Contributions are welcome! Feel free to open issues or submit pull requests!

## License
This project is licensed under the MIT License. See the [LICENSE](https://github.com/IchBinLeoon/netviz/blob/main/LICENSE) file for more details.
