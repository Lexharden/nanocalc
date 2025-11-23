# 🚀 NanoCalc - Quick Start Guide

## 🌐 Web Version (Easiest Way)

**Try NanoCalc directly in your browser - no installation required!**

👉 **[Launch NanoCalc](https://lexharden.github.io/nanocalc/app/)**

Works on Windows, macOS, Linux - any modern browser!

---

## 💻 Native Desktop Installation

### 1️⃣ Verificar Requisitos

```bash
# Rust instalado (1.75+)
rustc --version

# Si no está instalado: https://rustup.rs/
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2️⃣ Compilar el Proyecto

```bash
cd /Volumes/SSD/Desarrollo/rust/NanoCalc

# Debug (rápido de compilar, más lento de ejecutar)
cargo build

# Release (optimizado, recomendado)
cargo build --release
```

### 3️⃣ Ejecutar la Aplicación

```bash
# Modo debug
cargo run

# Modo release (más rápido)
cargo run --release
```

---

## 🚀 GitHub Pages Deployment

### Quick Deployment (3 Steps)

**Step 1**: Install Trunk
```bash
cargo install --locked trunk
rustup target add wasm32-unknown-unknown
```

**Step 2**: Test Locally
```bash
trunk serve --release
# Open http://127.0.0.1:8080
```

**Step 3**: Push to GitHub
```bash
git add .
git commit -m "Add GitHub Pages support"
git push origin main
```

Then enable GitHub Pages in repository **Settings → Pages → Source: GitHub Actions**

Your app will be at: `https://YOUR_USERNAME.github.io/NanoCalc/`

📚 See [DEPLOYMENT.md](DEPLOYMENT.md) for detailed deployment guide.

### 4️⃣ Primera Simulación

Cuando se abra la ventana:

1. **Panel Izquierdo - Parámetros**:
   - Radius: `50.0 nm` (nanopartícula de oro típica)
   - n (real): `0.5` (parte real del índice de Au)
   - k (imag): `2.5` (parte imaginaria de Au)
   - Wavelength: `500.0 nm` (luz verde)
   - n (medium): `1.33` (agua)

2. **Calcular**:
   - Click en `🔬 Calculate` → Ver resultados en panel central
   - Click en `📊 Calculate Spectrum` → Ver gráfica de 300-800 nm

3. **Interpretar Resultados**:
   - `Q_sca`: Eficiencia de scattering (adimensional)
   - `Q_abs`: Eficiencia de absorción
   - `Q_ext`: Eficiencia de extinción (Q_sca + Q_abs)
   - `C_sca`, `C_abs`, `C_ext`: Secciones transversales en nm²

---

## Comandos Útiles

### Desarrollo

```bash
# Verificar que compila (sin generar binario)
cargo check

# Ejecutar tests
cargo test

# Formatear código
cargo fmt

# Linter (recomendaciones)
cargo clippy

# Generar documentación
cargo doc --open
```

### Limpiar

```bash
# Eliminar archivos de compilación
cargo clean

# Esto libera ~500 MB en target/
```

---

## Ejemplos de Uso

### Nanopartícula de Oro en Agua (Plasmón)

```
Radius: 50 nm
Wavelength: 520 nm (pico plasmónico)
n_particle: 0.47 + 2.40i (Au a 520 nm)
n_medium: 1.33 (agua)

Resultado esperado: Q_sca > Q_abs (dispersión dominante)
```

### Nanopartícula de Plata

```
Radius: 30 nm
Wavelength: 400 nm
n_particle: 0.05 + 2.87i (Ag a 400 nm)
n_medium: 1.0 (aire)

Resultado: Pico de scattering agudo
```

### Quantum Dot de Semiconductor

```
Radius: 5 nm (régimen de confinamiento fuerte)
Wavelength: 450 nm
n_particle: 2.5 + 0.01i (CdSe)
n_medium: 1.5 (polímero)

Resultado: Absorción dominante
```

---

## Atajos de Teclado (Futuro)

*Próximamente en v0.2*

- `Ctrl/Cmd + S`: Guardar proyecto
- `Ctrl/Cmd + O`: Abrir proyecto
- `Ctrl/Cmd + E`: Exportar resultados
- `F11`: Pantalla completa

---

## Troubleshooting

### No compila

```bash
# Actualizar Rust
rustup update

# Limpiar y recompilar
cargo clean
cargo build --release
```

### GUI no se abre

```bash
# Verificar permisos (macOS)
xattr -d com.apple.quarantine target/release/nanocalc

# Verificar librerías gráficas (Linux)
sudo apt install libgtk-3-dev
```

### Cálculos muy lentos

```bash
# Asegúrate de usar release mode
cargo run --release

# NOT: cargo run (debug mode es ~10x más lento)
```

---

## Estructura de Archivos Importante

```
NanoCalc/
├── README.md           ← Documentación general
├── DELIVERY_SUMMARY.md ← Resumen de entrega
├── ARCHITECTURE.md     ← Arquitectura técnica
├── BEST_PRACTICES.md   ← Patrones de desarrollo
├── docs/physics/       ← Modelos matemáticos
├── src/
│   ├── main.rs         ← Entry point
│   ├── lib.rs          ← Library root
│   ├── core/           ← Tipos, traits, constantes
│   ├── physics/        ← Modelos físicos
│   └── gui/            ← Interfaz gráfica
└── Cargo.toml          ← Dependencias
```

---

## Próximos Pasos

### Para Desarrolladores

1. Leer `ARCHITECTURE.md` para entender el diseño
2. Leer `BEST_PRACTICES.md` para convenciones
3. Ver `docs/physics/optical_models.md` para entender Mie
4. Implementar tests adicionales
5. Agregar más modelos siguiendo los traits

### Para Usuarios

1. Probar con diferentes materiales
2. Generar espectros completos
3. Reportar bugs o sugerencias
4. Compartir casos de uso

---

## Recursos

### Documentación Interna
- [Arquitectura](ARCHITECTURE.md)
- [Roadmap](ROADMAP.md)
- [Modelos Ópticos](docs/physics/optical_models.md)
- [Modelos Térmicos](docs/physics/thermal_models.md)
- [Modelos Electrónicos](docs/physics/electronic_models.md)

### Referencias Externas
- [Rust Book](https://doc.rust-lang.org/book/)
- [egui Tutorial](https://github.com/emilk/egui)
- [Mie Theory (Bohren & Huffman)](https://www.amazon.com/Absorption-Scattering-Light-Small-Particles/dp/0471293407)

---

## Contacto y Soporte

- **Issues**: [GitHub Issues](https://github.com/yourusername/nanocalc/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/nanocalc/discussions)
- **Email**: your.email@example.com

---

## Notas de Versión Actual

**v0.1.0 - MVP** *(22 Nov 2025)*

✅ Implementado:
- Mie scattering (aproximación de Rayleigh)
- GUI con egui
- Cálculo de espectros
- Visualización de resultados

🚧 Próximamente (v0.2):
- Mie completo (todos los tamaños)
- Modelos térmicos y electrónicos
- Base de datos de materiales
- Exportación CSV/JSON

---

**¡Disfruta explorando las propiedades ópticas de nanomateriales con NanoCalc!** 🔬✨
