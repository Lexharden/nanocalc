# 💾 Sistema de Exportación - NanoCalc

## Visión General

El sistema de exportación permite guardar resultados de cálculos en múltiples formatos para análisis posterior, compartir con colaboradores, o generar reportes.

---

## Formatos Soportados

### 1. CSV (Comma-Separated Values)

**Uso**: Análisis en Excel, Python, R, MATLAB

**Estructura**:
```csv
# NanoCalc Export v0.1
# Date: 2025-11-22T10:30:00Z
# Model: Mie Scattering (Rayleigh Approximation)
# Parameters:
#   Particle Radius: 50.0 nm
#   Medium: Water (n=1.33)
#   Particle n: 0.5 + 2.5i
wavelength_nm,q_sca,q_abs,q_ext,c_sca_nm2,c_abs_nm2,c_ext_nm2
300.0,0.1234,0.0567,0.1801,97.12,44.65,141.77
305.0,0.1256,0.0578,0.1834,98.85,45.50,144.35
...
```

**Características**:
- Header con metadata
- Columnas con unidades explícitas
- Compatible con Excel/Google Sheets

**Implementación**:
```rust
pub fn export_csv(results: &[OpticalResult], path: &Path) -> Result<()> {
    let mut writer = csv::Writer::from_path(path)?;
    
    // Header con metadata
    writer.write_record(&["# NanoCalc Export"])?;
    writer.write_record(&[format!("# Date: {}", chrono::Utc::now())])?;
    
    // Column headers
    writer.write_record(&[
        "wavelength_nm",
        "q_sca",
        "q_abs",
        "q_ext",
        "c_sca_nm2",
        "c_abs_nm2",
        "c_ext_nm2",
    ])?;
    
    // Data
    for result in results {
        writer.write_record(&[
            result.wavelength.to_string(),
            result.q_sca.to_string(),
            result.q_abs.to_string(),
            result.q_ext.to_string(),
            result.c_sca.to_string(),
            result.c_abs.to_string(),
            result.c_ext.to_string(),
        ])?;
    }
    
    writer.flush()?;
    Ok(())
}
```

---

### 2. JSON (JavaScript Object Notation)

**Uso**: Intercambio con aplicaciones web, APIs, procesamiento automático

**Estructura**:
```json
{
  "metadata": {
    "version": "0.1.0",
    "timestamp": "2025-11-22T10:30:00Z",
    "model": {
      "type": "MieScattering",
      "approximation": "Rayleigh"
    },
    "parameters": {
      "particle_radius_nm": 50.0,
      "n_particle": {
        "real": 0.5,
        "imaginary": 2.5
      },
      "n_medium": 1.33
    }
  },
  "results": [
    {
      "wavelength": 300.0,
      "q_sca": 0.1234,
      "q_abs": 0.0567,
      "q_ext": 0.1801,
      "c_sca": 97.12,
      "c_abs": 44.65,
      "c_ext": 141.77,
      "metadata": {
        "size_parameter": 0.628,
        "converged": true,
        "num_terms": 1
      }
    }
  ]
}
```

**Características**:
- Metadata completa
- Tipado implícito
- Jerárquico
- Fácil de parsear

**Implementación**:
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Export {
    metadata: ExportMetadata,
    results: Vec<OpticalResult>,
}

#[derive(Serialize, Deserialize)]
pub struct ExportMetadata {
    version: String,
    timestamp: String,
    model: ModelInfo,
    parameters: serde_json::Value,
}

pub fn export_json(export: &Export, path: &Path) -> Result<()> {
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, export)?;
    Ok(())
}
```

---

### 3. PNG (Portable Network Graphics)

**Uso**: Incluir gráficas en presentaciones, papers, reportes

**Características**:
- Alta resolución (configurable)
- Transparencia opcional
- Metadata embebida (autor, fecha, parámetros)

**Implementación**:
```rust
pub fn export_plot_png(
    results: &[OpticalResult],
    path: &Path,
    width: u32,
    height: u32,
) -> Result<()> {
    use plotters::prelude::*;
    
    let root = BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;
    
    let max_q = results.iter()
        .map(|r| r.q_ext)
        .fold(0.0f64, f64::max);
    
    let mut chart = ChartBuilder::on(&root)
        .caption("Optical Spectrum", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(300f64..800f64, 0f64..max_q * 1.1)?;
    
    chart.configure_mesh()
        .x_desc("Wavelength (nm)")
        .y_desc("Efficiency (dimensionless)")
        .draw()?;
    
    // Q_sca
    chart.draw_series(LineSeries::new(
        results.iter().map(|r| (r.wavelength, r.q_sca)),
        &BLUE,
    ))?
    .label("Q_sca")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    
    // Q_abs
    chart.draw_series(LineSeries::new(
        results.iter().map(|r| (r.wavelength, r.q_abs)),
        &RED,
    ))?
    .label("Q_abs")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    
    // Q_ext
    chart.draw_series(LineSeries::new(
        results.iter().map(|r| (r.wavelength, r.q_ext)),
        &GREEN,
    ))?
    .label("Q_ext")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));
    
    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    
    root.present()?;
    Ok(())
}
```

---

### 4. Markdown Report

**Uso**: Documentación, GitHub, obsidian, notion

**Estructura**:
```markdown
# NanoCalc Calculation Report

**Date**: 2025-11-22 10:30:00 UTC  
**Model**: Mie Scattering (Rayleigh Approximation)  
**Software Version**: NanoCalc v0.1.0

---

## Parameters

| Parameter | Value | Unit |
|-----------|-------|------|
| Particle Radius | 50.0 | nm |
| Wavelength | 500.0 | nm |
| n (particle, real) | 0.5 | - |
| k (particle, imag) | 2.5 | - |
| n (medium) | 1.33 | - |

## Results

| Property | Value | Unit |
|----------|-------|------|
| Q_sca | 0.1234 | - |
| Q_abs | 0.0567 | - |
| Q_ext | 0.1801 | - |
| C_sca | 97.12 | nm² |
| C_abs | 44.65 | nm² |
| C_ext | 141.77 | nm² |

## Diagnostics

- **Size Parameter (x)**: 0.628
- **Convergence**: Yes
- **Terms Used**: 1
- **Conservation Error**: 1.2e-10

## Spectrum

![Spectrum](spectrum.png)

## References

- Bohren, C. F., & Huffman, D. R. (1983). *Absorption and Scattering of Light by Small Particles*. Wiley.

---

*Generated by NanoCalc v0.1.0*
```

---

## API de Exportación

### Trait Principal

```rust
pub trait Exporter {
    type Data;
    type Options;
    
    fn export(
        &self,
        data: &Self::Data,
        path: &Path,
        options: Self::Options,
    ) -> Result<()>;
}
```

### Implementaciones

```rust
pub struct CsvExporter;

impl Exporter for CsvExporter {
    type Data = Vec<OpticalResult>;
    type Options = CsvOptions;
    
    fn export(
        &self,
        data: &Self::Data,
        path: &Path,
        options: Self::Options,
    ) -> Result<()> {
        // Implementation
    }
}

pub struct JsonExporter;
pub struct PngExporter;
pub struct MarkdownExporter;
```

---

## Opciones de Exportación

```rust
#[derive(Debug, Clone)]
pub struct CsvOptions {
    pub delimiter: char,
    pub include_metadata: bool,
    pub precision: usize,
}

#[derive(Debug, Clone)]
pub struct JsonOptions {
    pub pretty: bool,
    pub include_metadata: bool,
}

#[derive(Debug, Clone)]
pub struct PngOptions {
    pub width: u32,
    pub height: u32,
    pub dpi: u32,
    pub transparent_bg: bool,
}
```

---

## Uso en GUI

```rust
impl NanoCalcApp {
    fn export_results(&mut self, format: ExportFormat) {
        let path = rfd::FileDialog::new()
            .set_file_name(&format!("nanocalc_export.{}", format.extension()))
            .save_file();
        
        if let Some(path) = path {
            match format {
                ExportFormat::Csv => {
                    let exporter = CsvExporter;
                    if let Err(e) = exporter.export(
                        &self.spectrum_results,
                        &path,
                        CsvOptions::default(),
                    ) {
                        self.show_error(&format!("Export failed: {}", e));
                    }
                }
                ExportFormat::Json => { /* ... */ }
                ExportFormat::Png => { /* ... */ }
            }
        }
    }
}
```

---

## Validación de Exports

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_csv_roundtrip() {
        let results = vec![/* ... */];
        
        // Export
        export_csv(&results, "test.csv").unwrap();
        
        // Re-import
        let imported = import_csv("test.csv").unwrap();
        
        // Verify
        assert_eq!(results.len(), imported.len());
        for (orig, imp) in results.iter().zip(imported.iter()) {
            assert_relative_eq!(orig.q_sca, imp.q_sca, epsilon = 1e-6);
        }
    }
}
```

---

## Metadatos Recomendados

Todos los exports deben incluir:

1. **Software**: `nanocalc v0.1.0`
2. **Timestamp**: ISO 8601 format
3. **Model**: Tipo y aproximación
4. **Parameters**: Todos los inputs
5. **Units**: Explícitas para cada campo
6. **Validation**: Estado de convergencia
7. **Hash**: Checksum para verificación (opcional)

---

## Futuras Extensiones

### v0.2
- [ ] Excel (.xlsx) con múltiples hojas
- [ ] LaTeX tables
- [ ] PDF reports con matplotlib-style

### v0.3
- [ ] HDF5 para datasets grandes
- [ ] NetCDF para compatibilidad científica
- [ ] MATLAB .mat files

### v1.0
- [ ] Formatos específicos de instrumentos
- [ ] Cloud storage integration
- [ ] Database export (PostgreSQL, SQLite)

---

## Referencias

- [RFC 4180 - CSV Format](https://tools.ietf.org/html/rfc4180)
- [JSON Schema](https://json-schema.org/)
- [PNG Specification](http://www.libpng.org/pub/png/spec/)
