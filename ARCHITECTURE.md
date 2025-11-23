# NanoCalc - Arquitectura del Proyecto

## Visión General

NanoCalc es una aplicación científica para cálculos de propiedades ópticas, térmicas y electrónicas de materiales a nanoescala, construida con arquitectura modular y extensible.

## Capas de la Arquitectura

### 1. Presentation Layer (GUI)
- **Responsabilidad**: Interfaz de usuario, renderizado, eventos
- **Tecnología**: egui/eframe
- **Ubicación**: `src/gui/`

### 2. Application Layer
- **Responsabilidad**: Lógica de aplicación, coordinación, validación
- **Componentes**: Controllers, ViewModels, State management
- **Ubicación**: `src/app/`

### 3. Domain Layer
- **Responsabilidad**: Lógica de negocio, modelos físicos, cálculos
- **Componentes**: Physics models, traits, compute engine
- **Ubicación**: `src/physics/`, `src/compute/`, `src/core/`

### 4. Infrastructure Layer
- **Responsabilidad**: I/O, persistencia, exportación, plotting
- **Ubicación**: `src/export/`, `src/project/`, `src/plotting/`

## Principios de Diseño

1. **Separación de responsabilidades**: Cada módulo tiene una única responsabilidad
2. **Inversión de dependencias**: Los módulos de alto nivel no dependen de detalles de implementación
3. **Extensibilidad mediante traits**: Nuevos modelos físicos se agregan sin modificar código existente
4. **Pureza funcional en cálculos**: Los modelos físicos son funciones puras sin efectos secundarios
5. **Type safety**: Uso de newtype pattern para unidades físicas

## Módulos Principales

### core/
Tipos fundamentales, unidades físicas, constantes, errors.

### physics/
- `optical/`: Modelos de dispersión, absorción, índice efectivo
- `thermal/`: Modelos de conductividad térmica
- `electronic/`: Modelos de bandgap, densidad de estados
- `materials/`: Base de datos de propiedades de materiales

### compute/
Motor de cálculo que coordina la ejecución de modelos físicos, maneja paralelismo con rayon.

### gui/
- `views/`: Componentes de UI (panels, widgets)
- `state/`: Estado global de la aplicación
- `themes/`: Temas visuales

### export/
Generación de reportes en CSV, JSON, PNG.

### project/
Persistencia de sesiones de trabajo, serialización/deserialización.

### utils/
Helpers, validación, interpolación, integración numérica.

## Flujo de Datos

```
User Input → GUI → AppController → ComputeEngine → PhysicsModel → Results → GUI/Export
```

## Patrones Aplicados

- **Strategy Pattern**: Diferentes algoritmos de cálculo intercambiables
- **Factory Pattern**: Creación de modelos físicos
- **Observer Pattern**: Actualización de UI cuando cambian resultados
- **Command Pattern**: Para sistema de undo/redo (futuro)
- **Builder Pattern**: Construcción de parámetros complejos

## Dependencias Externas

- `egui`: GUI framework
- `eframe`: egui + native windowing
- `nalgebra`: Álgebra lineal
- `rayon`: Paralelismo
- `serde`: Serialización
- `plotters` o `egui_plot`: Gráficas
- `num-complex`: Números complejos para óptica

## Escalabilidad

El diseño permite:
- Agregar nuevos modelos físicos implementando traits
- Cambiar backend de GUI (egui → Tauri)
- Agregar backends de cálculo (GPU, distributed)
- Extender formatos de exportación
- Agregar plugins externos
