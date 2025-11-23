# Modelos Electrónicos - NanoCalc

## 1. Bandgap en Nanoescala

### 🎯 Propósito
Estimar el ensanchamiento del bandgap debido al confinamiento cuántico en nanomateriales semiconductores.

### 📐 Modelo Base: Bandgap de Bulk

```
E_g(bulk) = E_g0 + α T² / (T + β)
```

Donde:
- `E_g0`: bandgap a 0 K
- `α, β`: parámetros de Varshni (dependientes del material)
- `T`: temperatura

---

## 2. Confinamiento Cuántico - Modelo Brus

### 📐 Ecuación de Brus (Effective Mass Approximation)

Para nanopartículas esféricas:

```
E_g(r) = E_g(bulk) + (ℏ²π²)/(2r²) × (1/m_e* + 1/m_h*) - 1.8e²/(4πε₀εᵣr)
```

Términos:
1. **E_g(bulk)**: Bandgap del material bulk
2. **Término cinético**: Energía de confinamiento cuántico
3. **Término coulombiano**: Interacción electrón-hueco

Donde:
- `r`: radio de la nanopartícula
- `m_e*`: masa efectiva del electrón
- `m_h*`: masa efectiva del hueco
- `ε_r`: constante dieléctrica relativa
- `ε₀`: permitividad del vacío

### 🔬 Supuestos

1. **Effective Mass Approximation (EMA)**: Válida para r > 2 nm
2. **Potencial infinito**: Paredes de potencial infinitas
3. **Interacción débil**: Aproximación de primer orden
4. **Banda parabólica**: Válida cerca de k = 0

### ⚠️ Rango de Validez

| Parámetro | Rango válido | Observaciones |
|-----------|--------------|---------------|
| **Radio** | 2 nm - 50 nm | r < 2 nm: tight-binding, r > 50 nm: bulk |
| **Material** | Semiconductores III-V, II-VI | No válido para metales |
| **Temperatura** | 4 K - 400 K | EMA válida |

---

## 3. Modelo Simplificado - Regla de Escalamiento

### 📐 Aproximación empírica

```
E_g(d) = E_g(bulk) + C / d^n
```

Donde:
- `d`: diámetro de la nanopartícula
- `C`: constante de ajuste [eV·nm^n]
- `n`: exponente (típicamente 1.0 - 2.0)

Para modelo simple (partícula en caja 3D):
```
n = 2
C = ℏ²π²/(2m*) con m* = masa efectiva reducida
```

### 📊 Parámetros Típicos

| Material | E_g(bulk) [eV] | C [eV·nm²] | n |
|----------|----------------|------------|---|
| CdSe | 1.74 | 2.5 | 1.8 |
| CdS | 2.42 | 3.0 | 1.9 |
| Si | 1.12 | 1.2 | 2.0 |
| GaAs | 1.43 | 1.8 | 2.0 |
| ZnO | 3.37 | 4.0 | 1.7 |

---

## 4. Exciton Binding Energy

### 📐 Energía de enlace excitónica

Para exciton en 3D:

```
E_b = (μ/m₀) × (1/ε_r²) × 13.6 eV
```

Donde:
- `μ = (m_e* m_h*)/(m_e* + m_h*)`: masa reducida
- `m₀`: masa del electrón libre
- `ε_r`: constante dieléctrica

### 📐 Radio de Bohr excitónico

```
a_B = ε_r × (m₀/μ) × 0.053 nm
```

### 🔬 Regímenes de Confinamiento

| Régimen | Condición | Características |
|---------|-----------|----------------|
| **Débil** | r >> a_B | Exciton confinado completo |
| **Intermedio** | r ≈ a_B | Transición |
| **Fuerte** | r << a_B | e⁻ y h⁺ confinados independientemente |

---

## 5. Densidad de Estados (DOS)

### 📐 DOS en Bulk (3D)

```
D₃D(E) = (1/2π²) × (2m*/ℏ²)^(3/2) × √E
```

### 📐 DOS en Pozo Cuántico (2D)

```
D₂D(E) = (m*/πℏ²) × Σₙ Θ(E - Eₙ)
```

Donde:
- `Θ(x)`: función escalón de Heaviside
- `Eₙ`: energías de subbanda

### 📐 DOS en Nanohilo (1D)

```
D₁D(E) = (1/π) × √(2m*/ℏ²) × Σₙₘ 1/√(E - Eₙₘ)
```

### 📐 DOS en Quantum Dot (0D)

```
D₀D(E) = 2 Σₙₗₘ δ(E - Eₙₗₘ)
```

Completamente discreto (delta functions).

---

## 6. Energías en Pozos Cuánticos

### 📐 Pozo Infinito 1D

```
Eₙ = (n²π²ℏ²)/(2m*L²)     n = 1, 2, 3, ...
```

Donde `L` es el ancho del pozo.

### 📐 Pozo Finito 1D

Solución transcendental:

Para estado par:
```
√(2m*V₀/ℏ² - k²) = k tan(kL/2)
```

Para estado impar:
```
√(2m*V₀/ℏ² - k²) = -k cot(kL/2)
```

Donde:
- `V₀`: profundidad del pozo
- `k = √(2m*E/ℏ²)`

### 📐 Pozo Cuántico 2D (disco)

```
Eₙₘ = (ℏ²/2m*) × (χₙₘ/r)²
```

Donde `χₙₘ` son los ceros de las funciones de Bessel.

---

## 7. Método de Tauc para Bandgap Experimental

### 📐 Plot de Tauc

Para materiales con transición directa:

```
(αhν)² = A(hν - E_g)
```

Para transición indirecta:

```
(αhν)^(1/2) = B(hν - E_g)
```

Donde:
- `α`: coeficiente de absorción
- `hν`: energía del fotón
- `A, B`: constantes

**Método**: Graficar `(αhν)²` vs `hν` y extrapolar linealmente a α = 0.

---

## 8. Modelo Práctico para NanoCalc MVP

### 🎯 Implementación Simplificada

```rust
fn bandgap_nano(
    material: Material,
    diameter: f64,  // nm
    temperature: f64, // K
) -> f64 {
    // Bandgap de bulk con Varshni
    let eg_bulk = material.eg0 
        + material.alpha * temperature.powi(2) 
        / (temperature + material.beta);
    
    // Confinamiento cuántico (término cinético simplificado)
    let hbar = 1.054571817e-34; // J·s
    let m_eff = material.reduced_effective_mass();
    let r = diameter / 2.0 * 1e-9; // convertir a metros
    
    let confinement = (hbar.powi(2) * PI.powi(2)) 
        / (2.0 * m_eff * r.powi(2))
        / 1.602176634e-19; // convertir a eV
    
    // Término coulombiano (aproximado)
    let e = 1.602176634e-19; // C
    let epsilon0 = 8.854187817e-12; // F/m
    let coulomb = -1.8 * e.powi(2) 
        / (4.0 * PI * epsilon0 * material.dielectric_constant * r)
        / 1.602176634e-19; // a eV
    
    eg_bulk + confinement + coulomb
}
```

---

## 9. Propiedades de Materiales Comunes

### Semiconductores III-V

| Material | E_g(300K) [eV] | m_e*/m₀ | m_h*/m₀ | ε_r | a_B [nm] |
|----------|----------------|---------|---------|-----|----------|
| GaAs | 1.424 | 0.067 | 0.45 | 12.9 | 12.0 |
| GaN | 3.39 | 0.20 | 0.80 | 8.9 | 3.0 |
| InP | 1.35 | 0.077 | 0.60 | 12.4 | 11.0 |
| InAs | 0.354 | 0.023 | 0.41 | 15.1 | 34.0 |

### Semiconductores II-VI

| Material | E_g(300K) [eV] | m_e*/m₀ | m_h*/m₀ | ε_r | a_B [nm] |
|----------|----------------|---------|---------|-----|----------|
| CdSe | 1.74 | 0.13 | 0.45 | 10.0 | 5.6 |
| CdS | 2.42 | 0.21 | 0.80 | 8.9 | 2.8 |
| ZnS | 3.68 | 0.34 | 1.76 | 8.3 | 2.5 |
| ZnO | 3.37 | 0.24 | 0.45 | 8.5 | 2.34 |

### Grupo IV

| Material | E_g(300K) [eV] | m_e*/m₀ | m_h*/m₀ | ε_r | Nota |
|----------|----------------|---------|---------|-----|------|
| Si | 1.12 | 0.26 | 0.36 | 11.7 | Indirecto |
| Ge | 0.66 | 0.082 | 0.044 | 16.0 | Indirecto |
| C (diamond) | 5.47 | 0.36 | 0.70 | 5.7 | Indirecto |

---

## 10. Casos de Prueba

### Test 1: Límite bulk
```
d → ∞ ⟹ E_g → E_g(bulk)
```

### Test 2: Escalamiento correcto
```
Para CdSe, d = 3 nm:
E_g ≈ 1.74 + 2.5/3² ≈ 2.02 eV
```

### Test 3: Datos experimentales

| Material | d [nm] | E_g(exp) [eV] | Referencia |
|----------|--------|---------------|------------|
| CdSe | 3.0 | 2.00 | Murray et al. 1993 |
| CdSe | 5.0 | 1.95 | Murray et al. 1993 |
| Si | 4.0 | 2.15 | Wilson et al. 1993 |
| GaAs | 10.0 | 1.60 | Micic et al. 1995 |

---

## 11. Efectos de Temperatura

### 📐 Varshni Parameters

| Material | α [meV/K] | β [K] |
|----------|-----------|-------|
| GaAs | 0.5405 | 204 |
| Si | 0.473 | 636 |
| Ge | 0.4774 | 235 |
| CdSe | 0.4 | 250 |
| ZnO | 0.5 | 900 |

---

## 12. Absorción Óptica y Onset

### 📐 Coeficiente de absorción cerca del bandgap

Transición directa:
```
α(hν) = A × √(hν - E_g) / hν
```

Transición indirecta:
```
α(hν) = B × (hν - E_g ± E_phonon)² / hν
```

### 🔬 Regla de selección

Para quantum dots esféricos:
- Transiciones permitidas: Δl = ±1
- Estado fundamental: 1S_e → 1S_h

---

## 📚 Referencias

1. **Brus, L. E.** (1984). Electron–electron and electron‐hole interactions in small semiconductor crystallites. *J. Chem. Phys.*, 80, 4403.
2. **Efros, A. L., & Rosen, M.** (2000). The electronic structure of semiconductor nanocrystals. *Annu. Rev. Mater. Sci.*, 30, 475.
3. **Ekimov, A. I., & Onushchenko, A. A.** (1981). Quantum size effect in three-dimensional microscopic semiconductor crystals. *JETP Lett.*, 34, 345.
4. **Yu, P. Y., & Cardona, M.** (2010). *Fundamentals of Semiconductors*. Springer.
5. **Harrison, P., & Valavanis, A.** (2016). *Quantum Wells, Wires and Dots*. Wiley.

---

## 🧮 Constantes Físicas

```rust
const HBAR: f64 = 1.054571817e-34; // J·s
const M_E: f64 = 9.1093837015e-31; // kg (masa electrón)
const E_CHARGE: f64 = 1.602176634e-19; // C
const EPSILON_0: f64 = 8.854187817e-12; // F/m
const EV_TO_JOULE: f64 = 1.602176634e-19;
const RY: f64 = 13.605693122994; // eV (Rydberg)
const BOHR_RADIUS: f64 = 0.05291772109; // nm
```

---

## 🎨 Extensiones Futuras

1. **Tight-binding**: Para d < 2 nm
2. **Multi-exciton**: Estados excitados
3. **Forma no esférica**: Elipsoides, rods, platelets
4. **Core-shell**: Heteroestructuras
5. **Defectos**: Estados en el gap
6. **Carga**: Quantum dots cargados (triones)
