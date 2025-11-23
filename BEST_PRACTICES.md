# 🎯 Best Practices & Design Patterns for Scientific Software in Rust

## Principios Fundamentales

### 1. **Type Safety para Unidades Físicas** ⭐⭐⭐

**Problema**: Mezclar unidades causa errores catastróficos (ej: Mars Climate Orbiter).

**Solución**: Newtype pattern para unidades.

```rust
#[derive(Debug, Clone, Copy)]
pub struct Nanometer(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct Meter(pub f64);

impl Nanometer {
    pub fn to_meters(self) -> Meter {
        Meter(self.0 * 1e-9)
    }
}

// Compile-time safety: esto NO compila
fn calculate_something(length: Nanometer) { }
let x = Meter(1.0);
// calculate_something(x); // ERROR: expected Nanometer, found Meter
```

**Beneficios**:
- Errores de unidades detectados en compilación
- Documentación implícita
- Conversiones explícitas

---

### 2. **Pureza Funcional en Cálculos Físicos** ⭐⭐⭐

**Principio**: Los cálculos deben ser funciones puras (sin efectos secundarios).

```rust
// ✅ CORRECTO: Función pura
fn mie_scattering(
    radius: f64,
    wavelength: f64,
    n_particle: Complex64,
    n_medium: f64,
) -> MieResult {
    // Solo depende de inputs, sin estado mutable
    // ...
}

// ❌ INCORRECTO: Función impura
fn mie_scattering_bad(&mut self) -> MieResult {
    self.cache.insert(...); // Mutación de estado
    self.log_file.write(...); // I/O
    // ...
}
```

**Ventajas**:
- Fácil de testear
- Paralelizable automáticamente
- Reproducible
- Sin race conditions

**Patrón recomendado**:
```rust
// Separar cálculo de I/O
pub struct MieCalculator;

impl MieCalculator {
    // Cálculo puro
    pub fn calculate(&self, params: MieParams) -> CalcResult<MieResult> {
        // Pure calculation
    }
}

pub struct MieEngine {
    calculator: MieCalculator,
    cache: HashMap<String, MieResult>, // Estado separado
}

impl MieEngine {
    pub fn calculate_cached(&mut self, params: MieParams) -> CalcResult<MieResult> {
        // Check cache, call pure calculator
    }
}
```

---

### 3. **Validación Exhaustiva de Parámetros** ⭐⭐⭐

**Problema**: Parámetros no físicos producen resultados sin sentido.

```rust
pub trait PhysicsModel {
    fn validate(&self) -> ValidationResult<()>;
    
    fn warnings(&self) -> Vec<String> {
        Vec::new()
    }
}

impl MieModel {
    fn validate(&self) -> ValidationResult<()> {
        // Validaciones fuertes (errores)
        if self.radius <= 0.0 {
            return Err(ValidationError::OutOfRange {
                value: self.radius,
                min: 0.0,
                max: f64::INFINITY,
            });
        }
        
        Ok(())
    }
    
    fn warnings(&self) -> Vec<String> {
        let mut warnings = Vec::new();
        
        // Advertencias (no fatales)
        let x = self.size_parameter();
        if x > 10.0 {
            warnings.push(format!(
                "Size parameter x={:.1} is large. Many Mie terms required.",
                x
            ));
        }
        
        warnings
    }
}
```

**Jerarquía de validación**:
1. **Errores**: Parámetros no físicos → `Result::Err`
2. **Advertencias**: Fuera de rango óptimo → `Vec<String>`
3. **Información**: Contexto adicional → Metadata

---

### 4. **Error Handling Científico** ⭐⭐⭐

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CalculationError {
    #[error("Convergence failed after {iterations} iterations")]
    ConvergenceFailed { iterations: usize },
    
    #[error("Numerical instability: {0}")]
    NumericalInstability(String),
    
    #[error("Model not applicable: {0}")]
    ModelNotApplicable(String),
    
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),
}

pub type CalcResult<T> = Result<T, CalculationError>;
```

**Convención**:
- Usa `thiserror` para errores de dominio
- Usa `anyhow` solo en binarios (main.rs)
- Nunca `unwrap()` en código de producción
- Siempre propaga contexto

---

### 5. **Metadata Rica en Resultados** ⭐⭐

**Patrón**: Incluir información diagnóstica en resultados.

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct OpticalResult {
    // Resultados principales
    pub q_sca: f64,
    pub q_abs: f64,
    pub q_ext: f64,
    
    // Metadata para debugging y validación
    pub metadata: OpticalMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpticalMetadata {
    pub num_terms: Option<usize>,        // Términos usados en serie
    pub converged: bool,                  // ¿Convergió?
    pub size_parameter: f64,              // x = 2πr/λ
    pub computation_time_ms: Option<f64>, // Performance
    pub warnings: Vec<String>,            // Advertencias
}
```

**Beneficios**:
- Facilita debugging
- Permite validación posterior
- Transparencia para usuarios
- Trazabilidad

---

### 6. **Trait-Based Extensibility** ⭐⭐⭐

**Patrón**: Definir traits para permitir extensiones sin modificar código existente.

```rust
pub trait OpticalModel: PhysicsModel {
    fn calculate(&self) -> CalcResult<OpticalResult>;
    fn calculate_spectrum(&self, wavelengths: &[f64]) -> CalcResult<Vec<OpticalResult>>;
}

// Usuarios pueden agregar nuevos modelos
pub struct CustomModel { /* ... */ }

impl PhysicsModel for CustomModel { /* ... */ }
impl OpticalModel for CustomModel { /* ... */ }

// El compute engine funciona con cualquier OpticalModel
pub fn compute<T: OpticalModel>(model: T) -> CalcResult<OpticalResult> {
    model.validate()?;
    model.calculate()
}
```

**Ventajas**:
- Open-closed principle
- Testeable independientemente
- Sin acoplamiento

---

### 7. **Constantes Físicas con Precisión** ⭐⭐

```rust
// ✅ CORRECTO: CODATA 2018
pub const HBAR: f64 = 1.054571817e-34; // J·s

// ❌ EVITAR: Precisión arbitraria
pub const HBAR: f64 = 1.054e-34;

// Documentar fuente
/// Planck constant (CODATA 2018)
/// https://physics.nist.gov/cgi-bin/cuu/Value?h
pub const H: f64 = 6.62607015e-34; // J·s
```

**Principios**:
- Usar valores CODATA oficiales
- Documentar fuente y año
- Incluir unidades en comentarios
- Crear módulo `constants` centralizado

---

### 8. **Testing Científico Robusto** ⭐⭐⭐

#### Test contra casos conocidos

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_mie_gold_nanoparticle() {
        // Caso de literatura (Bohren & Huffman, p. 477)
        let model = MieModel {
            radius: 50.0,
            wavelength: 520.0,
            n_particle: RefractiveIndex::new(0.47, 2.40), // Au
            n_medium: 1.33, // water
        };
        
        let result = model.calculate().unwrap();
        
        // Comparar con valor publicado
        assert_relative_eq!(result.q_ext, 3.52, epsilon = 0.01);
    }

    #[test]
    fn test_rayleigh_limit() {
        // Límite x → 0 debe converger a Rayleigh
        let model = MieModel::new(/* x << 1 */);
        let result = model.calculate().unwrap();
        
        let rayleigh_expected = calculate_rayleigh(/* ... */);
        assert_relative_eq!(result.q_sca, rayleigh_expected, epsilon = 0.001);
    }

    #[test]
    fn test_conservation() {
        // Leyes de conservación
        let model = MieModel::new(/* ... */);
        let result = model.calculate().unwrap();
        
        // Q_ext debe ser Q_sca + Q_abs
        assert_relative_eq!(
            result.q_ext,
            result.q_sca + result.q_abs,
            epsilon = 1e-10
        );
    }
}
```

**Categorías de tests**:
1. **Límites**: x→0, x→∞, T→0, etc.
2. **Simetría**: Reciprocidad, invariancia
3. **Conservación**: Energía, carga, etc.
4. **Literatura**: Casos publicados
5. **Regresión**: Prevenir cambios inesperados

---

### 9. **Documentation as Code** ⭐⭐

```rust
/// Calculate Mie scattering for a spherical nanoparticle.
///
/// This function implements the Mie solution to Maxwell's equations
/// for a plane wave incident on a homogeneous sphere.
///
/// # Physics
///
/// The scattering efficiency is given by:
/// ```text
/// Qsca = (2/x²) Σ(n=1→∞) (2n+1)(|aₙ|² + |bₙ|²)
/// ```
///
/// # Parameters
///
/// * `radius` - Particle radius in nanometers. Must be positive.
/// * `wavelength` - Incident wavelength in nanometers. Must be positive.
/// * `n_particle` - Complex refractive index of particle (n + ik).
/// * `n_medium` - Real refractive index of surrounding medium.
///
/// # Returns
///
/// `OpticalResult` containing scattering/absorption efficiencies and cross-sections.
///
/// # Errors
///
/// Returns `CalculationError` if:
/// - Parameters fail validation
/// - Series does not converge
/// - Numerical instability detected
///
/// # Validity Range
///
/// - Best for size parameter x < 10
/// - Accuracy decreases for highly absorbing particles (k > 5)
///
/// # Examples
///
/// ```
/// use nanocalc::physics::optical::mie::MieModel;
/// use nanocalc::core::RefractiveIndex;
///
/// let model = MieModel::new(
///     50.0,  // 50 nm radius
///     500.0, // 500 nm wavelength
///     RefractiveIndex::new(0.5, 2.5), // Au-like
///     1.33,  // water
/// );
///
/// let result = model.calculate().unwrap();
/// println!("Scattering efficiency: {:.4}", result.q_sca);
/// ```
///
/// # References
///
/// - Bohren, C. F., & Huffman, D. R. (1983). *Absorption and Scattering
///   of Light by Small Particles*. Wiley.
/// - Mätzler, C. (2002). *MATLAB functions for Mie scattering and absorption*.
pub fn calculate_mie(/* ... */) -> CalcResult<OpticalResult> {
    // ...
}
```

**Elementos esenciales**:
- Física: ecuaciones, modelo
- Parámetros: significado, unidades, restricciones
- Retorno: qué contiene
- Errores: cuándo falla
- Validez: rango de aplicabilidad
- Ejemplos: ejecutables con `cargo test`
- Referencias: papers originales

---

### 10. **Separation of Concerns** ⭐⭐⭐

```
┌─────────────────────────────────────┐
│  GUI Layer                          │  ← Solo UI, sin cálculos
│  (egui, eventos, renderizado)      │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│  Application Layer                  │  ← Orquestación, validación
│  (Controller, State Management)    │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│  Domain Layer                       │  ← Lógica pura, sin I/O
│  (Physics Models - Pure Functions)  │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│  Infrastructure Layer               │  ← I/O, persistencia
│  (Export, Database, Plotting)      │
└─────────────────────────────────────┘
```

**Reglas**:
- GUI nunca llama directamente a physics
- Physics models son puros (sin I/O)
- I/O solo en infrastructure layer
- Application layer coordina todo

---

### 11. **Performance sin Sacrificar Claridad** ⭐⭐

```rust
// ✅ Claro y rápido
pub fn calculate_spectrum_parallel(
    model: &MieModel,
    wavelengths: &[f64],
) -> CalcResult<Vec<OpticalResult>> {
    use rayon::prelude::*;
    
    wavelengths
        .par_iter()  // Paralelización trivial
        .map(|&wl| {
            let mut m = model.clone();
            m.wavelength = wl;
            m.calculate()
        })
        .collect()
}

// Medir primero, optimizar después
#[cfg(feature = "profiling")]
fn measure_performance() {
    use std::time::Instant;
    
    let start = Instant::now();
    let result = expensive_calculation();
    let duration = start.elapsed();
    
    eprintln!("Calculation took: {:?}", duration);
}
```

**Principios**:
- Claridad primero, performance después
- Medir antes de optimizar
- Rayon para paralelización simple
- `#[inline]` solo si benchmarks lo justifican

---

### 12. **Semantic Versioning Estricto** ⭐⭐

```toml
[package]
name = "nanocalc"
version = "0.2.1"  # MAJOR.MINOR.PATCH
```

**Convención**:
- `0.x.y`: Pre-release, API inestable
- `PATCH`: Bug fixes (compatible)
- `MINOR`: New features (compatible)
- `MAJOR`: Breaking changes

**Changelog detallado**:
```markdown
## [0.2.1] - 2025-03-15

### Added
- Maxwell-Garnett effective medium model

### Fixed
- Convergence bug in Mie for x > 100

### Changed
- Improved performance of spectrum calculations (2x faster)

### Breaking (for v1.0.0)
- None
```

---

## 📋 Checklist para Nuevos Módulos

Antes de considerar un módulo "completo":

- [ ] Traits bien definidos
- [ ] Validación de inputs
- [ ] Manejo de errores
- [ ] Documentación con ejemplos
- [ ] Tests contra casos conocidos
- [ ] Tests de límites
- [ ] Tests de conservación
- [ ] Benchmarks de performance
- [ ] Constantes físicas documentadas
- [ ] Metadata en resultados
- [ ] Rango de validez especificado
- [ ] Referencias a literatura

---

## 🚫 Anti-Patterns a Evitar

### 1. **God Objects**
```rust
// ❌ EVITAR
pub struct Everything {
    mie: MieCalculator,
    thermal: ThermalCalculator,
    electronic: ElectronicCalculator,
    gui: GuiState,
    database: Database,
    // ... 50 campos más
}
```

### 2. **Unwrap en Producción**
```rust
// ❌ EVITAR
let result = calculate().unwrap(); // Panic!

// ✅ CORRECTO
let result = calculate()?; // Propagar error
```

### 3. **Números Mágicos**
```rust
// ❌ EVITAR
let x = 6.62607015e-34 * freq;

// ✅ CORRECTO
let x = constants::H * freq;
```

### 4. **Mutación Innecesaria**
```rust
// ❌ EVITAR
fn calculate(&mut self) -> Result { /* ... */ }

// ✅ CORRECTO
fn calculate(&self) -> Result { /* ... */ }
```

---

## 📚 Recursos Recomendados

### Rust
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/)

### Numerical Computing
- *Numerical Recipes* (Press et al.)
- *Computational Physics* (Landau & Páez)

### Scientific Software
- *Best Practices for Scientific Computing* (Wilson et al., PLOS Biology 2014)
- *Good enough practices in scientific computing* (Wilson et al., PLOS Comp Bio 2017)

---

**Principio fundamental**: **Claridad > Cleverness**

El código científico debe ser:
1. **Correcto** (validado)
2. **Comprensible** (documentado)
3. **Reproducible** (determinista)
4. **Mantenible** (modular)
5. **Eficiente** (optimizado cuando necesario)

En ese orden de prioridad.
