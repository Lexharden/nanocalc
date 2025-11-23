```
NanoCalc/
├── Cargo.toml                    # Workspace principal
├── Cargo.lock
├── README.md                     # Documentación del proyecto
├── ARCHITECTURE.md               # Este documento
├── LICENSE
├── .gitignore
│
├── docs/                         # Documentación técnica
│   ├── physics/                  # Modelos matemáticos
│   │   ├── optical_models.md
│   │   ├── thermal_models.md
│   │   └── electronic_models.md
│   ├── api/                      # API documentation
│   └── tutorials/                # Tutoriales de uso
│
├── examples/                     # Ejemplos de uso
│   ├── simple_mie.rs
│   ├── thermal_conductivity.rs
│   └── bandgap_estimation.rs
│
├── tests/                        # Tests de integración
│   ├── optical_tests.rs
│   ├── thermal_tests.rs
│   └── export_tests.rs
│
├── benches/                      # Benchmarks de performance
│   └── computation_benches.rs
│
├── assets/                       # Recursos
│   ├── icons/
│   ├── presets/                  # Materiales predefinidos
│   │   ├── materials.json
│   │   └── common_configs.json
│   └── themes/
│
└── src/
    ├── main.rs                   # Entry point
    ├── lib.rs                    # Library root
    │
    ├── core/                     # 🔵 CORE (tipos fundamentales)
    │   ├── mod.rs
    │   ├── units.rs              # Newtype wrappers para unidades físicas
    │   ├── constants.rs          # Constantes físicas (h, c, k_B, etc.)
    │   ├── types.rs              # Complex, Wavelength, Temperature, etc.
    │   ├── errors.rs             # Error types y Result aliases
    │   └── traits.rs             # Traits base del sistema
    │
    ├── physics/                  # 🔬 DOMAIN (modelos físicos)
    │   ├── mod.rs
    │   │
    │   ├── optical/              # Módulo óptico
    │   │   ├── mod.rs
    │   │   ├── mie.rs            # Teoría de Mie (dispersión esférica)
    │   │   ├── effective_medium.rs  # Maxwell-Garnett, Bruggeman
    │   │   ├── drude_lorentz.rs  # Modelo de Drude-Lorentz
    │   │   ├── absorption.rs     # Coeficientes de absorción
    │   │   └── traits.rs         # OpticalModel trait
    │   │
    │   ├── thermal/              # Módulo térmico
    │   │   ├── mod.rs
    │   │   ├── conductivity.rs   # κ efectiva
    │   │   ├── phonon_models.rs  # Modelos de fonones
    │   │   ├── size_effects.rs   # Efectos de tamaño en κ
    │   │   └── traits.rs         # ThermalModel trait
    │   │
    │   ├── electronic/           # Módulo electrónico
    │   │   ├── mod.rs
    │   │   ├── bandgap.rs        # Modelos de Eg (Tauc, confinement)
    │   │   ├── quantum_well.rs   # Pozos cuánticos 1D/2D
    │   │   ├── density_of_states.rs  # DOS simplificado
    │   │   └── traits.rs         # ElectronicModel trait
    │   │
    │   └── materials/            # Base de datos de materiales
    │       ├── mod.rs
    │       ├── database.rs       # MaterialDatabase struct
    │       ├── material.rs       # Material struct con propiedades
    │       └── presets.rs        # Carga de materiales predefinidos
    │
    ├── compute/                  # ⚙️ COMPUTE ENGINE
    │   ├── mod.rs
    │   ├── engine.rs             # ComputeEngine principal
    │   ├── parallel.rs           # Ejecutor paralelo con rayon
    │   ├── cache.rs              # Cache de resultados
    │   └── scheduler.rs          # Scheduling de tareas pesadas
    │
    ├── app/                      # 🎛️ APPLICATION LAYER
    │   ├── mod.rs
    │   ├── controller.rs         # AppController (orquestador)
    │   ├── state.rs              # AppState (estado global)
    │   ├── validation.rs         # Validación de inputs
    │   └── commands.rs           # Command pattern (para undo/redo futuro)
    │
    ├── gui/                      # 🖼️ PRESENTATION
    │   ├── mod.rs
    │   ├── app.rs                # NanoCalcApp (egui app principal)
    │   │
    │   ├── views/                # Vistas y panels
    │   │   ├── mod.rs
    │   │   ├── main_window.rs    # Ventana principal
    │   │   ├── input_panel.rs    # Panel de parámetros de entrada
    │   │   ├── results_panel.rs  # Panel de resultados
    │   │   ├── plot_panel.rs     # Panel de gráficas
    │   │   ├── material_selector.rs  # Selector de materiales
    │   │   └── settings_window.rs    # Ventana de configuración
    │   │
    │   ├── widgets/              # Widgets reutilizables
    │   │   ├── mod.rs
    │   │   ├── numeric_input.rs  # Input con validación numérica
    │   │   ├── unit_selector.rs  # Selector de unidades
    │   │   └── result_card.rs    # Card para mostrar resultados
    │   │
    │   ├── state.rs              # GUI state management
    │   └── theme.rs              # Tema visual
    │
    ├── plotting/                 # 📊 PLOTTING
    │   ├── mod.rs
    │   ├── engine.rs             # PlotEngine
    │   ├── spectrum_plot.rs      # Gráficas de espectros
    │   ├── scatter_plot.rs       # Scatter plots
    │   └── export_plot.rs        # Export plots a PNG
    │
    ├── export/                   # 💾 EXPORT
    │   ├── mod.rs
    │   ├── csv.rs                # Exportar a CSV
    │   ├── json.rs               # Exportar a JSON
    │   ├── report.rs             # Generador de reportes
    │   └── formats.rs            # Definición de formatos
    │
    ├── project/                  # 📁 PROJECT MANAGEMENT
    │   ├── mod.rs
    │   ├── manager.rs            # ProjectManager
    │   ├── session.rs            # Session struct (estado serializable)
    │   └── io.rs                 # Load/Save projects
    │
    └── utils/                    # 🛠️ UTILITIES
        ├── mod.rs
        ├── validation.rs         # Validadores genéricos
        ├── interpolation.rs      # Interpolación de datos
        ├── integration.rs        # Integración numérica simple
        └── format.rs             # Formateo de números/unidades
```

## Explicación por Área

### 📦 Core (`src/core/`)
**Propósito**: Fundamentos del sistema sin dependencias externas (salvo std).

- `units.rs`: Define tipos como `Nanometer(f64)`, `Kelvin(f64)` para type safety
- `constants.rs`: Constantes físicas con precisión CODATA
- `types.rs`: Tipos compuestos (ComplexRefractiveIndex, Wavelength, etc.)
- `errors.rs`: Jerarquía de errores del sistema
- `traits.rs`: Traits base como `PhysicsModel`, `Calculable`, `Validatable`

### 🔬 Physics (`src/physics/`)
**Propósito**: Implementaciones de modelos físicos puros (sin I/O, sin estado mutable).

Cada submódulo (`optical/`, `thermal/`, `electronic/`) tiene:
- Implementaciones de algoritmos específicos
- `traits.rs` que define la interfaz del modelo
- Tests exhaustivos con casos conocidos

**Principio**: Los cálculos son **funciones puras**. Ejemplo:
```rust
fn mie_scattering(radius: Nanometer, wavelength: Nanometer, n_particle: Complex, n_medium: f64) -> MieResult
```

### ⚙️ Compute (`src/compute/`)
**Propósito**: Orquestar cálculos, paralelizar con rayon, cachear resultados.

- `engine.rs`: API principal para ejecutar cálculos
- `parallel.rs`: Ejecuta múltiples wavelengths/tamaños en paralelo
- `cache.rs`: Evita recalcular con mismos parámetros
- `scheduler.rs`: Para cálculos pesados (ej: Mie con 1000 wavelengths)

### 🎛️ App (`src/app/`)
**Propósito**: Coordinar GUI ↔ Compute, validar inputs, manejar estado.

- `controller.rs`: Mediador entre GUI y motor de cálculo
- `state.rs`: `AppState` con parámetros actuales, resultados, historial
- `validation.rs`: Validación semántica (ej: "radio debe ser < wavelength/10 para Rayleigh")

### 🖼️ GUI (`src/gui/`)
**Propósito**: Interfaz de usuario reactiva con egui.

Organización:
- `app.rs`: Struct principal que implementa `eframe::App`
- `views/`: Componentes grandes (panels, windows)
- `widgets/`: Componentes reutilizables pequeños
- `state.rs`: Estado específico de UI (qué panel está abierto, etc.)

**Principio**: La GUI solo **presenta** y **recolecta datos**. No hace cálculos.

### 📊 Plotting (`src/plotting/`)
**Propósito**: Generar gráficas a partir de resultados.

Usa `egui_plot` para interactividad en la GUI y `plotters` para export a PNG.

### 💾 Export (`src/export/`)
**Propósito**: Serializar resultados a diferentes formatos.

- CSV: Para análisis en Excel/Python
- JSON: Para intercambio con otras apps
- Report: Documento estructurado con metadata, parámetros y resultados

### 📁 Project (`src/project/`)
**Propósito**: Persistir sesiones de trabajo completas.

Permite guardar/cargar un proyecto `.nanocalc` (JSON) con:
- Parámetros de entrada
- Material seleccionado
- Resultados calculados
- Configuración de gráficas

### 🛠️ Utils (`src/utils/`)
**Propósito**: Funciones auxiliares reutilizables.

- Interpolación lineal/spline para datos experimentales
- Validación genérica de rangos
- Formateo de números científicos

## Flujo de Archivos

1. **Usuario abre app** → `main.rs` → `gui/app.rs`
2. **Usuario ingresa parámetros** → `views/input_panel.rs` → `app/controller.rs` (validación)
3. **Usuario presiona "Calculate"** → `controller.rs` → `compute/engine.rs` → `physics/optical/mie.rs`
4. **Resultados listos** → `compute/engine.rs` → `controller.rs` → `views/results_panel.rs` + `views/plot_panel.rs`
5. **Usuario exporta** → `views/results_panel.rs` → `export/csv.rs` / `export/json.rs`

## Convenciones

- Archivos `mod.rs`: Re-exportan public API del módulo
- Tests en mismo archivo con `#[cfg(test)]`
- Benchmarks en directorio `benches/`
- Documentación con `///` y ejemplos ejecutables
- Cada módulo físico tiene un ejemplo en `examples/`
