# 🎯 NanoCalc - Comparación de Soluciones Técnicas

Este documento compara diferentes opciones de tecnología evaluadas durante el diseño de NanoCalc y justifica las decisiones tomadas.

---

## 1. GUI Framework

### Opciones Evaluadas

| Framework | Pros | Contras | Veredicto |
|-----------|------|---------|-----------|
| **egui/eframe** ✅ | • Rust nativo<br>• Immediate mode (simple)<br>• Cross-platform<br>• Ligero (~2 MB)<br>• Buena para científicos | • UI menos "nativa"<br>• Limitado para UI compleja | **ELEGIDO** |
| **Tauri** | • UI web (HTML/CSS/JS)<br>• Look nativo<br>• Flexibilidad total | • Requiere frontend separado<br>• Más complejo<br>• Bundle más grande<br>• Dos lenguajes | Segunda opción |
| **iced** | • Elm-inspired<br>• Type-safe UI<br>• Rust puro | • Menos maduro<br>• Ecosistema pequeño<br>• Documentación limitada | Futuro |
| **GTK-rs** | • Look nativo<br>• Maduro | • FFI complejo<br>• Platform-specific bugs<br>• Dependencias grandes | Descartado |
| **Qt (qmetaobject)** | • Look profesional<br>• Feature-rich | • C++ dependency<br>• Licencia compleja<br>• Setup complicado | Descartado |

### Decisión: **egui/eframe**

**Justificación**:
- Rust 100% (sin JS/C++)
- Código simple y directo
- Perfecto para aplicaciones científicas (no necesita look "nativo")
- Excelente performance para gráficas
- Fácil de aprender para contribuyentes

**Trade-off aceptado**: UI menos "nativa", pero suficiente para el dominio científico.

---

## 2. Math Libraries

### Opciones Evaluadas

| Librería | Pros | Contras | Veredicto |
|----------|------|---------|-----------|
| **nalgebra** ✅ | • Rust nativo<br>• Type-safe<br>• Álgebra lineal completa<br>• SIMD support | • Sintaxis verbosa | **ELEGIDO** |
| **ndarray** | • NumPy-like API<br>• Broadcasting<br>• Buen para ML | • Menos type-safe<br>• Runtime checks | Para análisis futuro |
| **faer** | • Muy rápido<br>• Moderno | • Nuevo, menos maduro<br>• API inestable | Observar |
| **GSL-rs** | • Completo (FFT, etc.)<br>• Muy maduro | • FFI a C<br>• Setup complejo<br>• No idiomatic | Descartado |

### Decisión: **nalgebra**

**Justificación**:
- API type-safe (previene errores en tiempo de compilación)
- Performance excelente con SIMD
- Pure Rust (sin dependencias C)
- Documentación excelente

---

## 3. Parallelism

### Opciones Evaluadas

| Approach | Pros | Contras | Veredicto |
|----------|------|---------|-----------|
| **rayon** ✅ | • Data parallelism trivial<br>• Work stealing<br>• Seguro (no data races) | • No control fino<br>• CPU-only | **ELEGIDO** |
| **tokio** | • Async/await<br>• Good para I/O | • Overhead para CPU-bound<br>• Complejo | No aplicable |
| **crossbeam** | • Control fino<br>• Channels<br>• Lock-free | • Más complejo<br>• Manual management | Si se necesita más adelante |
| **wgpu** (GPU) | • Muy rápido<br>• Parallelism masivo | • Complejo<br>• Platform-specific<br>• Overkill para MVP | Futuro (v2.0) |

### Decisión: **rayon**

**Justificación**:
```rust
// Paralelizar es trivial
wavelengths.par_iter().map(|&wl| calculate(wl)).collect()
```

- Parallelism automático con 1 línea
- Seguridad garantizada por Rust
- Performance excelente para CPU-bound
- No requiere pensar en threads

**Futuro**: Agregar GPU con wgpu para cálculos muy grandes (v2.0).

---

## 4. Plotting

### Opciones Evaluadas

| Library | Pros | Contras | Veredicto |
|---------|------|---------|-----------|
| **egui_plot** ✅ | • Integrado con egui<br>• Interactivo<br>• Ligero | • Features limitadas | **ELEGIDO** (GUI) |
| **plotters** ✅ | • Export PNG/SVG<br>• Muy customizable<br>• No GUI required | • No interactivo | **ELEGIDO** (Export) |
| **plotly** | • Muy feature-rich<br>• Web-based | • Pesado<br>• Requiere JS runtime | Descartado |
| **matplotlib (PyO3)** | • Python ecosystem | • Python dependency<br>• FFI overhead | Descartado |

### Decisión: **egui_plot + plotters**

**Justificación**:
- `egui_plot`: Interactividad en GUI (zoom, pan)
- `plotters`: Export de alta calidad (PNG, SVG)
- Ambos ligeros y Rust-nativos
- Combinación óptima para uso científico

---

## 5. Serialization

### Opciones Evaluadas

| Format | Pros | Contras | Veredicto |
|--------|------|---------|-----------|
| **serde_json** ✅ | • Human-readable<br>• Universal<br>• Fácil debug | • Tamaño grande<br>• Parsing lento | **ELEGIDO** (Projects) |
| **bincode** | • Muy rápido<br>• Compacto | • Binary (no legible)<br>• Rust-specific | Caché interno |
| **MessagePack** | • Compacto<br>• Cross-language | • Menos común | Futuro |
| **CSV** ✅ | • Excel compatible<br>• Simple | • Solo tabular | **ELEGIDO** (Export) |
| **HDF5** | • Estándar científico<br>• Muy eficiente | • C dependency<br>• Complejo | Futuro (v2.0) |

### Decisión: **JSON + CSV**

**Justificación**:
- JSON: Proyectos humanos-legibles y debuggeables
- CSV: Compatible con todas las herramientas de análisis
- Balance entre simplicidad y usabilidad

**Futuro**: HDF5 para datasets masivos.

---

## 6. Architecture Pattern

### Opciones Evaluadas

| Pattern | Pros | Contras | Veredicto |
|---------|------|---------|-----------|
| **Layered (Clean)** ✅ | • Separación clara<br>• Testeable<br>• Mantenible | • Más boilerplate | **ELEGIDO** |
| **MVC** | • Conocido<br>• Simple | • Acoplamiento<br>• No ideal para Rust | Descartado |
| **ECS (Specs)** | • Performance<br>• Data-oriented | • Overkill<br>• Difícil de entender | No aplicable |
| **Monolith** | • Simple<br>• Rápido al inicio | • No escalable<br>• No testeable | Descartado |

### Decisión: **Layered Architecture**

```
GUI → Application → Domain → Infrastructure
```

**Justificación**:
- Separación de responsabilidades
- Lógica de negocio (physics) totalmente independiente
- Fácil testear cada capa
- Fácil cambiar GUI sin afectar cálculos
- Patrón estándar en software profesional

---

## 7. Error Handling

### Opciones Evaluadas

| Approach | Pros | Contras | Veredicto |
|----------|------|---------|-----------|
| **thiserror** ✅ | • Type-safe<br>• Ergonómico<br>• Para libraries | • Requires enum por tipo | **ELEGIDO** (Library) |
| **anyhow** ✅ | • Flexible<br>• Context chains | • Menos type-safe | **ELEGIDO** (Binary) |
| **eyre** | • Mejor contexto<br>• Hooks | • Overhead | Si se necesita |
| **Panic** | • Simple | • No recuperable<br>• Mala práctica | Descartado |

### Decisión: **thiserror (lib) + anyhow (bin)**

**Justificación**:
```rust
// Library code: tipo-safe
#[derive(Error, Debug)]
pub enum CalculationError {
    #[error("Convergence failed")]
    ConvergenceFailed,
}

// Binary code: flexible
fn main() -> anyhow::Result<()> {
    calculate().context("Failed to calculate")?;
    Ok(())
}
```

---

## 8. Testing Strategy

### Opciones Evaluadas

| Strategy | Pros | Contras | Veredicto |
|----------|------|---------|-----------|
| **Unit + Property** ✅ | • Cobertura completa<br>• Casos edge | • Más trabajo | **ELEGIDO** |
| **Solo Unit** | • Simple | • No cubre integración | Insuficiente |
| **Solo Integration** | • Realista | • Lento<br>• Difícil debug | Insuficiente |
| **Snapshot Testing** | • Fácil mantener | • No explica por qué | Complementario |

### Decisión: **Unit + Property-based + Integration**

**Justificación**:
```rust
// Unit: Casos específicos
#[test]
fn test_gold_nanoparticle() {
    assert_eq!(result.q_sca, 3.52, epsilon=0.01);
}

// Property: Leyes físicas
#[quickcheck]
fn prop_conservation(radius: f64, wavelength: f64) {
    let result = calculate(radius, wavelength);
    assert!(result.q_ext == result.q_sca + result.q_abs);
}

// Integration: End-to-end
#[test]
fn test_full_workflow() {
    let app = NanoCalcApp::new();
    app.calculate_spectrum();
    assert!(app.results.len() > 0);
}
```

---

## 9. Deployment

### Opciones Evaluadas

| Method | Pros | Contras | Veredicto |
|--------|------|---------|-----------|
| **GitHub Releases** ✅ | • Simple<br>• Gratis<br>• CI/CD fácil | • Manual download | **ELEGIDO** |
| **Cargo install** | • Easy para Rustaceans | • Requiere compilar<br>• Lento | Complementario |
| **Flatpak/Snap** | • Linux standard | • Setup complejo | Futuro |
| **Homebrew** | • macOS standard | • Mantenimiento | Futuro |
| **Microsoft Store** | • Discoverability | • Proceso largo<br>• Costo | Futuro |

### Decisión: **GitHub Releases + Cargo**

**Justificación**:
- Releases: Binarios pre-compilados para mayoría
- Cargo: Usuarios avanzados pueden compilar
- Simple, gratis, suficiente para open-source

---

## 10. Continuous Integration

### Opciones Evaluadas

| Platform | Pros | Contras | Veredicto |
|----------|------|---------|-----------|
| **GitHub Actions** ✅ | • Integrado<br>• Gratis<br>• Matrix builds | • YAML verboso | **ELEGIDO** |
| **GitLab CI** | • Powerful<br>• Auto DevOps | • Requiere GitLab | No aplicable |
| **Travis CI** | • Maduro | • Ya no gratis | Descartado |

### Decisión: **GitHub Actions**

**Justificación**:
```yaml
# .github/workflows/ci.yml
on: [push, pull_request]
jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
```

- Gratis para open-source
- Build matrix automática (Win/Mac/Linux)
- Cache de Cargo
- Integración perfecta con GitHub

---

## Resumen de Decisiones

| Aspecto | Solución Elegida | Alternativa | Razón |
|---------|------------------|-------------|-------|
| **GUI** | egui/eframe | Tauri | Simplicidad, Rust puro |
| **Math** | nalgebra | ndarray | Type safety |
| **Parallel** | rayon | crossbeam | Simplicidad |
| **Plot** | egui_plot + plotters | plotly | Ligero, nativo |
| **Serialization** | JSON + CSV | HDF5 | Legibilidad |
| **Architecture** | Layered | MVC | Separación clara |
| **Error** | thiserror + anyhow | eyre | Balance |
| **Testing** | Unit + Property | Solo unit | Cobertura |
| **Deploy** | GitHub Releases | Stores | Simplicidad |
| **CI** | GitHub Actions | GitLab | Integración |

---

## Principios de Decisión

Las decisiones siguieron estos principios en orden:

1. **Simplicidad** > Flexibilidad
2. **Rust puro** > FFI a C/C++
3. **Type safety** > Performance (en caso de conflicto)
4. **Mantenibilidad** > Features
5. **Open source** > Propietario

Excepciones se justifican caso por caso.

---

## Futuras Re-evaluaciones

Estas decisiones se revisarán en:

- **v0.3**: Considerar Tauri si UI nativa se vuelve crucial
- **v1.0**: Evaluar GPU (wgpu) para performance
- **v2.0**: HDF5 para datasets científicos grandes

---

**Este documento vive y evoluciona con el proyecto.**

*Última actualización: 22 de Noviembre de 2025*
