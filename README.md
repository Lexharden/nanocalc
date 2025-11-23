# 🔬 NanoCalc

**Professional Nanoscale Optical Properties Calculator**

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Release](https://img.shields.io/github/v/release/Lexharden/nanocalc)](https://github.com/Lexharden/nanocalc/releases)

Advanced Mie scattering calculations, thermal properties, and electronic band structure for nanoparticles. Built with Rust for maximum performance.

**Native desktop application for Windows, macOS, and Linux.**

Visit the [project homepage](https://lexharden.github.io/nanocalc/) for more information.

## ✨ Features

- **🌊 Mie Scattering** - Accurate calculations for spherical nanoparticles
- **📊 Spectral Analysis** - Full wavelength scanning (300-800 nm)
- **⚗️ Periodic Table** - Built-in optical properties database
- **🎨 Material Presets** - Quick access to common materials (Au, Ag, Si, TiO2)
- **🌍 Multilingual** - English and Spanish support
- **⚡ High Performance** - Native Rust with parallel processing
- **💾 Export** - CSV, JSON, and PNG outputs
- **🖥️ Cross-Platform** - Windows, macOS, and Linux native apps

## 📥 Download

| Platform | Download | Size |
|----------|----------|------|
| 🪟 **Windows** | [nanocalc-windows-x64.exe.zip](https://github.com/Lexharden/nanocalc/releases/latest/download/nanocalc-windows-x64.exe.zip) | ~4 MB |
| 🍎 **macOS (Apple Silicon)** | [NanoCalc-macos-arm64.app.zip](https://github.com/Lexharden/nanocalc/releases/latest/download/NanoCalc-macos-arm64.app.zip) | ~2.8 MB |
| 🍎 **macOS (Intel)** | [NanoCalc-macos-x64.app.zip](https://github.com/Lexharden/nanocalc/releases/latest/download/NanoCalc-macos-x64.app.zip) | ~3 MB |
| 🐧 **Linux** | [nanocalc-linux-x64.tar.gz](https://github.com/Lexharden/nanocalc/releases/latest/download/nanocalc-linux-x64.tar.gz) | ~4.2 MB |

**[View all releases](https://github.com/Lexharden/nanocalc/releases)**

## 🚀 Quick Start

### Using Pre-built Binaries

1. Download the appropriate version for your platform from the [releases page](https://github.com/Lexharden/nanocalc/releases)
2. **Windows**: Run the `.exe` file directly
3. **macOS**: 
   - Open the `.dmg` file and drag NanoCalc to Applications
   - If you get "damaged" error: `xattr -cr /Applications/NanoCalc.app`
   - Right-click the app and select "Open" for first launch
4. **Linux**: Make it executable with `chmod +x nanocalc-linux-x64` and run
5. Start calculating!

> **macOS Note**: Since the app is not notarized by Apple, you may need to allow it in System Preferences > Privacy & Security.

### Build from Source

```bash
# Clone repository
git clone https://github.com/Lexharden/nanocalc.git
cd nanocalc

# Build desktop app
cargo build --release

# Run
cargo run --release
```

## 🤝 Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

This project is licensed under the MIT License - see [LICENSE](LICENSE) for details.

## 👨‍💻 Author

**Yafel G.H.**

- GitHub: [@Lexharden](https://github.com/Lexharden)
- Email: yafel@example.com

## 🙏 Acknowledgments

- Mie scattering theory implementation
- egui framework for the GUI
- Rust community for excellent tools

---

Made with ❤️ and Rust • [Report Issues](https://github.com/Lexharden/nanocalc/issues) • [Request Features](https://github.com/Lexharden/nanocalc/issues/new)
