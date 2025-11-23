# NanoCalc Deployment Guide

## GitHub Pages Deployment

### Prerequisites

1. **Install Trunk** (WASM build tool):
   ```bash
   cargo install --locked trunk
   ```

2. **Add WASM target**:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

### Local Testing

Test the WASM build locally before deploying:

```bash
trunk serve --release
```

Then open http://127.0.0.1:8080 in your browser.

### Deploy to GitHub Pages

1. **Enable GitHub Pages**:
   - Go to your repository on GitHub
   - Navigate to Settings → Pages
   - Under "Source", select "GitHub Actions"

2. **Push to GitHub**:
   ```bash
   git add .
   git commit -m "Add GitHub Pages deployment"
   git push origin main
   ```

3. **Automatic Deployment**:
   - The GitHub Action will automatically build and deploy
   - Check the "Actions" tab to monitor progress
   - Your app will be available at: `https://USERNAME.github.io/NanoCalc/`

### Manual Build

To build manually for deployment:

```bash
trunk build --release --public-url /NanoCalc
```

The output will be in the `dist/` directory.

## Multi-Platform Support

### Web (All Operating Systems)
- **Access via**: GitHub Pages URL
- **Compatibility**: Any modern browser (Chrome, Firefox, Safari, Edge)
- **Requirements**: WebAssembly support (available in all modern browsers)

### Native Builds

#### macOS
```bash
cargo build --release
# Binary: target/release/nanocalc
```

#### Linux
```bash
cargo build --release
# Binary: target/release/nanocalc
```

#### Windows
```bash
cargo build --release
# Binary: target/release/nanocalc.exe
```

### Cross-Platform Binary Releases

To create releases for multiple platforms:

1. **Install cross-compilation tools**:
   ```bash
   # For Linux target on macOS
   rustup target add x86_64-unknown-linux-gnu
   
   # For Windows target on macOS
   rustup target add x86_64-pc-windows-gnu
   ```

2. **Build for different targets**:
   ```bash
   # macOS
   cargo build --release --target x86_64-apple-darwin
   
   # Linux
   cargo build --release --target x86_64-unknown-linux-gnu
   
   # Windows
   cargo build --release --target x86_64-pc-windows-gnu
   ```

## Browser Compatibility

### Supported Browsers
- ✅ Chrome/Edge 91+
- ✅ Firefox 89+
- ✅ Safari 15+
- ✅ Opera 77+

### Features
- Full GUI functionality
- Real-time calculations
- Plot visualization
- Multilanguage support (English/Spanish)
- Element selector with optical properties

## Performance Notes

### WASM Performance
- First load: ~2-3 MB download (includes WASM binary)
- Subsequent loads: Cached by browser
- Calculation speed: Near-native performance

### Native Performance
- Faster startup time
- Full CPU utilization
- Better for heavy computational workloads

## Troubleshooting

### Build Issues

**Problem**: `trunk` command not found
```bash
cargo install --locked trunk
```

**Problem**: WASM target not installed
```bash
rustup target add wasm32-unknown-unknown
```

**Problem**: Build fails with linking errors
```bash
cargo clean
cargo update
trunk build --release
```

### Deployment Issues

**Problem**: GitHub Pages shows 404
- Check that GitHub Pages is enabled in repository settings
- Verify the GitHub Action completed successfully
- Wait 2-3 minutes for propagation

**Problem**: App doesn't load in browser
- Check browser console for errors
- Ensure browser supports WebAssembly
- Clear browser cache and reload

## Configuration

### Trunk.toml
Configuration file for Trunk build system:
- `target`: HTML entry point
- `dist`: Output directory
- `public_url`: Base URL for GitHub Pages

### Cargo.toml
WASM-specific dependencies:
- `wasm-bindgen`: Rust/JavaScript interop
- `web-sys`: Web API bindings
- `console_error_panic_hook`: Better error messages

## Development Workflow

1. **Local development**:
   ```bash
   trunk serve
   ```

2. **Test changes**:
   - Open http://127.0.0.1:8080
   - Hot reload enabled

3. **Commit and push**:
   ```bash
   git add .
   git commit -m "Your changes"
   git push
   ```

4. **Automatic deployment**:
   - GitHub Action builds and deploys
   - Check Actions tab for status

## Resources

- [Trunk Documentation](https://trunkrs.dev/)
- [egui WASM Documentation](https://github.com/emilk/egui#compiling-for-the-web)
- [GitHub Pages Documentation](https://docs.github.com/en/pages)
