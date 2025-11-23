# Modelos Térmicos - NanoCalc

## 1. Conductividad Térmica Efectiva

### 🎯 Propósito
Calcular la conductividad térmica de nanomateriales considerando efectos de tamaño y fonones.

### 📐 Modelo Base: Conductividad de Bulk

```
κ = (1/3) C v l
```

Donde:
- `C`: capacidad calorífica volumétrica [J/(m³·K)]
- `v`: velocidad promedio de fonones [m/s]
- `l`: camino libre medio de fonones [m]

### 📐 Modelo de Debye (baja temperatura)

```
C(T) = (12π⁴/5) n k_B (T/Θ_D)³
```

Donde:
- `n`: densidad de átomos
- `k_B`: constante de Boltzmann
- `Θ_D`: temperatura de Debye

---

## 2. Efectos de Confinamiento en Nanoescala

### 📐 Modelo de Dispersión en Superficie (Fuchs-Sondheimer)

Para nanohilos/películas delgadas:

```
κ_eff/κ_bulk = 1 - (3/8)K
```

Donde:
```
K = (λ/d) × [(1-p)/(1+p)]
```

- `d`: dimensión característica (diámetro, espesor)
- `λ`: camino libre medio de fonones en bulk
- `p`: parámetro de especularidad (0 ≤ p ≤ 1)
  - p = 0: dispersión completamente difusa
  - p = 1: reflexión especular perfecta

### 🔬 Supuestos

1. **Dispersión en frontera dominante**: d < λ_bulk
2. **Temperatura uniforme**: No gradientes significativos
3. **Límite difuso**: p ≈ 0 para superficies rugosas

### ⚠️ Rango de Validez

| Parámetro | Rango válido | Observaciones |
|-----------|--------------|---------------|
| **d/λ** | 0.1 - 10 | d >> λ: bulk, d << λ: balístico |
| **Temperatura** | 10 K - 500 K | Modelo de Debye válido |
| **Tipo de material** | Cristalinos | No amorfo |

---

## 3. Modelo de Callaway

### 🎯 Propósito
Incluir múltiples mecanismos de dispersión de fonones.

### 📐 Ecuación

```
κ = (k_B/2π²v) (k_B T/ℏ)³ ∫₀^(Θ_D/T) [τ_C(x) x⁴ e^x / (e^x - 1)²] dx
```

Donde:
- `x = ℏω/(k_B T)` (frecuencia reducida)
- `τ_C`: tiempo de relajación combinado

#### Tiempo de relajación combinado

```
1/τ_C = 1/τ_U + 1/τ_N + 1/τ_B
```

Mecanismos:
- `τ_U`: dispersión Umklapp
- `τ_N`: dispersión normal (no afecta κ directamente)
- `τ_B`: dispersión en frontera (boundary scattering)

### 📐 Expresiones para τ

#### Umklapp (fonón-fonón)
```
1/τ_U = A ω² T e^(-Θ_D/3T)
```

#### Boundary scattering
```
1/τ_B = v/d_eff
```

Donde `d_eff` es la dimensión efectiva de confinamiento.

### 🔬 Supuestos

1. **Fonones acústicos dominantes**: Válido para T < Θ_D
2. **Dispersión isotrópica**: Sin direccionalidad preferencial
3. **Equilibrio local**: Distribución de Bose-Einstein

---

## 4. Conductividad Térmica de Nanocompuestos

### 📐 Modelo de Maxwell (análogo térmico de Maxwell-Garnett)

Para inclusiones esféricas:

```
κ_eff = κ_m [1 + 3f(κ_p - κ_m)/(κ_p + 2κ_m - f(κ_p - κ_m))]
```

Donde:
- `f`: fracción volumétrica de inclusiones
- `κ_p`: conductividad de partículas
- `κ_m`: conductividad de matriz

#### Con resistencia interfacial de Kapitza

```
κ_eff = κ_m [1 + 3fβ/(3 + fβ(1-β))]
```

Donde:
```
β = (κ_p/κ_m) / [1 + (R_K κ_p)/r]
```

- `R_K`: resistencia de Kapitza [m²·K/W]
- `r`: radio de partículas

### 🔬 Resistencia de Kapitza

Valores típicos:

| Interfaz | R_K (10⁻⁸ m²·K/W) |
|----------|-------------------|
| Si/SiO₂ | 1 - 10 |
| Au/agua | 50 - 200 |
| CNT/polímero | 10 - 100 |

---

## 5. Modelo Fenomenológico para Nanopartículas

### 📐 Ecuación simplificada

```
κ_np = κ_bulk × f(d/λ, T)
```

Donde:
```
f(d/λ, T) = 1 / [1 + (λ/d) × g(T)]
```

Con:
```
g(T) = [1 + (T/T_ref)^α]
```

Parámetros típicos:
- `α ≈ 1.5` (ajustable)
- `T_ref`: temperatura de referencia (300 K)

### 📊 Casos de Prueba

| Material | d (nm) | T (K) | κ/κ_bulk | Referencia |
|----------|--------|-------|----------|------------|
| Si | 100 | 300 | ~0.5 | Li et al. 2003 |
| Si | 20 | 300 | ~0.1 | Chen et al. 2008 |
| Ge | 50 | 300 | ~0.3 | Wang et al. 2008 |

---

## 6. Propiedades Térmicas de Materiales Comunes

### Conductividad bulk (300 K)

| Material | κ [W/(m·K)] | λ [nm] | Θ_D [K] |
|----------|-------------|--------|---------|
| Si | 148 | 40 | 645 |
| Ge | 60 | 20 | 374 |
| Au | 318 | 30 | 165 |
| Ag | 429 | 55 | 225 |
| Al₂O₃ | 35 | 5 | 1000 |
| SiO₂ | 1.4 | 0.5 | 470 |

### Velocidad de sonido

| Material | v [m/s] |
|----------|---------|
| Si | 8433 |
| Ge | 5400 |
| Au | 3240 |
| Ag | 3650 |

---

## 7. Efectos de Temperatura

### 📐 Dependencia con temperatura (T < Θ_D)

```
κ(T) ∝ T³  (régimen fonónico puro)
```

### 📐 Régimen de alta temperatura (T > Θ_D)

```
κ(T) ∝ 1/T  (dispersión Umklapp dominante)
```

### 📊 Curva típica κ(T)

```
     κ
     │     
     │    /\
     │   /  \___
     │  /       \___
     │ /            \___
     │/________________\___
     └──────────────────────> T
       T³    pico   1/T
```

- **Pico**: típicamente entre Θ_D/10 y Θ_D/5
- **Nanoescala**: Pico se reduce y se ensancha

---

## 8. Modelo para NanoCalc MVP

### 🎯 Implementación Práctica

Para el MVP, usar modelo simplificado:

```rust
fn thermal_conductivity_nano(
    material: Material,
    diameter: f64,  // nm
    temperature: f64, // K
) -> f64 {
    let kappa_bulk = material.kappa_bulk(temperature);
    let lambda_mfp = material.lambda_phonon(temperature);
    let p = 0.0; // dispersión difusa
    
    let ratio = lambda_mfp / diameter;
    let correction = 1.0 - (3.0/8.0) * ratio * (1.0 - p) / (1.0 + p);
    
    kappa_bulk * correction.max(0.1) // límite inferior 10% del bulk
}
```

### 🔬 Supuestos del MVP

1. Dispersión difusa (p = 0)
2. Temperatura ambiente (300 K)
3. Geometría esférica equivalente
4. Sin resistencia interfacial en primera versión

---

## 9. Validación y Tests

### Test 1: Límite bulk
```
d → ∞ ⟹ κ_eff → κ_bulk
```

### Test 2: Límite balístico
```
d << λ ⟹ κ_eff ∝ d/λ
```

### Test 3: Conservación de energía
```
∇·(κ∇T) + Q = 0
```

### Test 4: Datos experimentales
Comparar con:
- Silicon nanowires (Li et al., APL 2003)
- Germanium nanoparticles (Wang et al., JAP 2008)

---

## 📚 Referencias

1. **Cahill, D. G., et al.** (2014). Nanoscale thermal transport. *J. Appl. Phys.*, 124, 071101.
2. **Li, D., et al.** (2003). Thermal conductivity of individual silicon nanowires. *Appl. Phys. Lett.*, 83, 2934.
3. **Callaway, J.** (1959). Model for lattice thermal conductivity at low temperatures. *Phys. Rev.*, 113, 1046.
4. **Chen, G.** (2005). *Nanoscale Energy Transport and Conversion*. Oxford University Press.
5. **Ziman, J. M.** (1960). *Electrons and Phonons*. Oxford University Press.

---

## 🧮 Constantes Físicas Necesarias

```rust
const K_B: f64 = 1.380649e-23; // J/K
const HBAR: f64 = 1.054571817e-34; // J·s
const N_A: f64 = 6.02214076e23; // mol⁻¹
```

---

## 🎨 Extensiones Futuras

1. **Anisotropía**: Conductividad tensorial κᵢⱼ
2. **Multicapa**: Películas delgadas multicapa
3. **Nanohilos**: Geometría 1D
4. **Interfaz rugosa**: Modelos de scattering modificados
5. **Efectos cuánticos**: T << 10 K
