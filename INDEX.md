# 📚 NanoCalc - Índice General de Documentación

## 🎯 Inicio Rápido

**¿Primera vez aquí?** Comienza con estos archivos en orden:

1. 📖 [README.md](README.md) - Qué es NanoCalc y features principales
2. 🚀 [QUICKSTART.md](QUICKSTART.md) - Compilar y ejecutar en 5 minutos
3. 📋 [DELIVERY_SUMMARY.md](DELIVERY_SUMMARY.md) - Resumen ejecutivo de lo entregado

---

## 📁 Documentación por Categoría

### 🏗️ Arquitectura y Diseño

| Documento | Descripción | Audiencia |
|-----------|-------------|-----------|
| [ARCHITECTURE.md](ARCHITECTURE.md) | Arquitectura completa del sistema (4 capas) | Desarrolladores |
| [PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md) | Estructura de carpetas y archivos explicada | Desarrolladores |
| [TECHNICAL_DECISIONS.md](TECHNICAL_DECISIONS.md) | Comparación de tecnologías y decisiones | Arquitectos |
| [BEST_PRACTICES.md](BEST_PRACTICES.md) | 12 patrones para software científico en Rust | Todos |

### 🔬 Modelos Físicos

| Documento | Contenido | Ecuaciones |
|-----------|-----------|------------|
| [docs/physics/optical_models.md](docs/physics/optical_models.md) | Mie, Maxwell-Garnett, Drude-Lorentz | ✅ Completas |
| [docs/physics/thermal_models.md](docs/physics/thermal_models.md) | Conductividad térmica, fonones, Callaway | ✅ Completas |
| [docs/physics/electronic_models.md](docs/physics/electronic_models.md) | Bandgap, confinamiento cuántico, DOS | ✅ Completas |

**Cada modelo incluye**:
- Ecuaciones fundamentales con notación clara
- Supuestos y aproximaciones
- Rango de validez
- Casos de prueba con referencias
- Implementación en Rust

### 📅 Planificación

| Documento | Contenido | Estado |
|-----------|-----------|--------|
| [ROADMAP.md](ROADMAP.md) | Plan de desarrollo por versiones (v0.1 → v2.0) | 📍 Detallado |
| [DELIVERY_SUMMARY.md](DELIVERY_SUMMARY.md) | Qué se ha entregado hasta ahora | ✅ Completo |

### 💾 Sistemas y Features

| Documento | Descripción | Implementación |
|-----------|-------------|----------------|
| [EXPORT_SYSTEM.md](EXPORT_SYSTEM.md) | CSV, JSON, PNG, Markdown exports | 📐 Diseñado |
| [QUICKSTART.md](QUICKSTART.md) | Guía rápida de uso | ✅ Práctico |

### 📜 Legal y Contribuciones

| Documento | Contenido |
|-----------|-----------|
| [LICENSE](LICENSE) | MIT License |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Guía para contribuir *(próximamente)* |

---

## 🎓 Rutas de Aprendizaje

### Para Nuevos Usuarios

```
1. README.md          → Entender qué hace NanoCalc
2. QUICKSTART.md      → Ejecutar primera simulación
3. optical_models.md  → Entender la física de Mie
4. EXPORT_SYSTEM.md   → Cómo guardar resultados
```

### Para Nuevos Desarrolladores

```
1. DELIVERY_SUMMARY.md     → Visión general del código
2. ARCHITECTURE.md         → Entender la estructura
3. PROJECT_STRUCTURE.md    → Ubicar cada módulo
4. BEST_PRACTICES.md       → Convenciones y patrones
5. src/core/traits.rs      → Sistema de extensibilidad
6. src/physics/optical/mie.rs → Ejemplo de implementación
```

### Para Contribuyentes de Física

```
1. optical_models.md       → Modelos actuales
2. thermal_models.md       → Modelos térmicos
3. electronic_models.md    → Modelos electrónicos
4. BEST_PRACTICES.md       → Cómo implementar modelos
5. src/physics/            → Ver código existente
```

### Para Arquitectos

```
1. ARCHITECTURE.md         → Capas y módulos
2. TECHNICAL_DECISIONS.md  → Por qué cada tecnología
3. BEST_PRACTICES.md       → Patrones aplicados
4. ROADMAP.md              → Visión a futuro
```

---

## 🔍 Búsqueda Rápida

### ¿Quieres saber...?

| Pregunta | Documento |
|----------|-----------|
| ¿Cómo compilar? | [QUICKSTART.md](QUICKSTART.md) |
| ¿Qué arquitectura usa? | [ARCHITECTURE.md](ARCHITECTURE.md) |
| ¿Cómo funciona Mie? | [optical_models.md](docs/physics/optical_models.md) |
| ¿Cómo agregar un modelo? | [BEST_PRACTICES.md](BEST_PRACTICES.md) |
| ¿Por qué egui y no Tauri? | [TECHNICAL_DECISIONS.md](TECHNICAL_DECISIONS.md) |
| ¿Cuál es el plan futuro? | [ROADMAP.md](ROADMAP.md) |
| ¿Cómo exportar datos? | [EXPORT_SYSTEM.md](EXPORT_SYSTEM.md) |
| ¿Qué se ha entregado? | [DELIVERY_SUMMARY.md](DELIVERY_SUMMARY.md) |

### Por Tema

**Física y Matemática**:
- Óptica: `docs/physics/optical_models.md`
- Térmica: `docs/physics/thermal_models.md`
- Electrónica: `docs/physics/electronic_models.md`

**Código**:
- Traits: `src/core/traits.rs`
- Constantes: `src/core/constants.rs`
- Mie implementation: `src/physics/optical/mie.rs`
- GUI: `src/gui/app.rs`

**Diseño**:
- Arquitectura: `ARCHITECTURE.md`
- Estructura: `PROJECT_STRUCTURE.md`
- Decisiones: `TECHNICAL_DECISIONS.md`
- Patrones: `BEST_PRACTICES.md`

**Proceso**:
- Inicio: `QUICKSTART.md`
- Roadmap: `ROADMAP.md`
- Resumen: `DELIVERY_SUMMARY.md`

---

## 📊 Estado de Documentación

| Categoría | Completitud | Notas |
|-----------|-------------|-------|
| **Arquitectura** | 🟢 100% | Completa y detallada |
| **Física** | 🟢 100% | Todos los modelos documentados |
| **Código** | 🟡 80% | Core completo, ejemplos pendientes |
| **Tutoriales** | 🟡 60% | Quickstart listo, más tutoriales próximamente |
| **API Docs** | 🟡 50% | Código documentado, falta generar |
| **Contribución** | 🔴 0% | CONTRIBUTING.md pendiente |

---

## 📐 Diagramas Importantes

### Arquitectura de Capas
Ver: [ARCHITECTURE.md - Sección 1.1](ARCHITECTURE.md)

### Flujo de Datos
Ver: [ARCHITECTURE.md - Sección 2.1](ARCHITECTURE.md)

### Estructura de Módulos
Ver: [PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md)

---

## 🔗 Enlaces Externos Relevantes

### Rust
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

### egui
- [egui GitHub](https://github.com/emilk/egui)
- [egui Demo](https://www.egui.rs/)
- [egui Docs](https://docs.rs/egui/)

### Física
- Bohren & Huffman: *Absorption and Scattering of Light by Small Particles*
- Cahill et al.: *Nanoscale thermal transport*
- Brus: *Electron-electron and electron-hole interactions in small semiconductor crystallites*

---

## 📝 Convenciones de Documentación

### Iconos Usados

- ✅ Completo y funcionando
- 🟢 100% documentado
- 🟡 Parcialmente completo
- 🔴 Pendiente
- 🚧 En progreso
- 📍 Planificado
- 📐 Diseñado (no implementado)
- ⭐ Muy importante
- 🔬 Relacionado con física
- 💻 Relacionado con código
- 📊 Contiene diagramas

### Estructura de Documentos

Todos los documentos técnicos siguen esta estructura:
1. Título y descripción breve
2. Tabla de contenidos (si es largo)
3. Secciones numeradas
4. Ejemplos de código cuando aplica
5. Diagramas/tablas cuando ayuda
6. Referencias
7. Fecha de última actualización

---

## 🆘 Ayuda y Soporte

### Si tienes una pregunta sobre...

**Compilación o ejecución**:
→ [QUICKSTART.md](QUICKSTART.md)

**Cómo funciona algo**:
→ [ARCHITECTURE.md](ARCHITECTURE.md) o [PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md)

**Física o matemática**:
→ `docs/physics/` + Referencias citadas

**Cómo contribuir**:
→ [BEST_PRACTICES.md](BEST_PRACTICES.md) + `CONTRIBUTING.md` (próximamente)

**Bugs o problemas**:
→ GitHub Issues (cuando esté público)

---

## 🔄 Mantenimiento de Documentación

Esta documentación se actualiza cuando:
- Se agrega una nueva feature
- Se cambia la arquitectura
- Se implementa un nuevo modelo
- Se toma una decisión técnica importante
- Se alcanza un milestone del roadmap

**Última revisión general**: 22 de Noviembre de 2025

---

## 📬 Sugerencias

¿Falta documentación sobre algo? ¿Algo no está claro?

- Abre un issue: `docs: [título del tema]`
- O contacta: your.email@example.com

---

**Toda la documentación está en formato Markdown para fácil lectura en GitHub o editores como VSCode, Obsidian, Typora, etc.**

---

*Happy coding! 🚀🔬*
