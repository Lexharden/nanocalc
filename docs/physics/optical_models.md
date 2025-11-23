# Modelos Ópticos - NanoCalc

## 1. Teoría de Dispersión de Mie (Mie Scattering)

### 🎯 Propósito
Calcular dispersión y absorción de luz por nanopartículas esféricas.

### 📐 Ecuaciones Fundamentales

#### Coeficientes de Mie
```
Qsca = (2/x²) Σ(n=1→∞) (2n+1)(|aₙ|² + |bₙ|²)
Qext = (2/x²) Σ(n=1→∞) (2n+1)Re(aₙ + bₙ)
Qabs = Qext - Qsca
```

Donde:
- `x = 2πr/λ` (parámetro de tamaño)
- `r`: radio de la partícula
- `λ`: longitud de onda en el medio
- `aₙ, bₙ`: coeficientes de Mie (funciones de Bessel y Hankel)

#### Coeficientes aₙ y bₙ

```
aₙ = [mψₙ(mx)ψₙ'(x) - ψₙ(x)ψₙ'(mx)] / [mψₙ(mx)ξₙ'(x) - ξₙ(x)ψₙ'(mx)]

bₙ = [ψₙ(mx)ψₙ'(x) - mψₙ(x)ψₙ'(mx)] / [ψₙ(mx)ξₙ'(x) - mξₙ(x)ψₙ'(mx)]
```

Donde:
- `m = n_partícula / n_medio` (índice refractivo relativo)
- `ψₙ, ξₙ`: funciones de Ricatti-Bessel

#### Secciones Transversales

```
Csca = (π r²) × Qsca   [unidades: nm²]
Cabs = (π r²) × Qabs
Cext = (π r²) × Qext
```

### 🔬 Supuestos

1. **Partículas esféricas**: La geometría debe ser exactamente esférica
2. **Partículas homogéneas**: Composición uniforme (no core-shell en MVP)
3. **Medio homogéneo**: El medio circundante es uniforme
4. **Partículas independientes**: No hay interacción entre partículas
5. **Onda plana incidente**: Iluminación homogénea
6. **No acoplamiento magnético**: μᵣ ≈ 1 (aproximación válida para la mayoría de nanomateriales)

### ⚠️ Rango de Validez

| Parámetro | Rango válido | Observaciones |
|-----------|--------------|---------------|
| **Tamaño (x)** | 0.01 < x < 1000 | x << 1: Rayleigh, x >> 1: óptica geométrica |
| **Radio** | 1 nm - 10 μm | Fuera de este rango usar otras aproximaciones |
| **n_particle** | 0.1 < Re(n) < 10 | Índices muy extremos pueden causar problemas numéricos |
| **Absorción** | Im(n) < 5 | Absorción muy alta requiere más términos en serie |

### 📊 Límites Conocidos

#### Límite de Rayleigh (x << 1, r << λ)
```
Qsca ≈ (8/3)x⁴|m²-1 / m²+2|²
Qabs ≈ 4x Im[(m²-1) / (m²+2)]
```

**Validación**: Para r = 10 nm, λ = 500 nm, Au:
- Teoría de Mie debe converger a Rayleigh

#### Límite Geométrico (x >> 1)
```
Qext → 2 (teorema óptico)
Qsca → 1
```

### 🧮 Implementación Numérica

#### Criterio de convergencia
Serie se trunca cuando:
```
|aₙ|² + |bₙ|² < ε × max(|aₖ|² + |bₖ|²) para k < n
```
Típicamente ε = 10⁻⁸

#### Número de términos necesarios
```
nmax ≈ x + 4x^(1/3) + 2
```

### 🎨 Casos de Prueba

| Material | Radio (nm) | λ (nm) | Qsca esperado | Referencia |
|----------|-----------|--------|---------------|------------|
| Au | 50 | 520 | ~3.5 | Bohren & Huffman |
| Ag | 30 | 400 | ~2.8 | Mie theory tables |
| TiO₂ | 100 | 600 | ~2.1 | BHMIE code |

---

## 2. Modelos de Medio Efectivo

### 2.1 Maxwell-Garnett (MG)

#### 🎯 Propósito
Calcular índice refractivo efectivo de un compuesto con inclusiones esféricas dispersas.

#### 📐 Ecuación

```
εeff = εm [1 + 3f(εp - εm)/(εp + 2εm - f(εp - εm))]
```

O en términos del índice refractivo:
```
neff² = nm² [1 + 3f(np² - nm²)/(np² + 2nm² - f(np² - nm²))]
```

Donde:
- `f`: fracción volumétrica de inclusiones (0 < f < 1)
- `εp, εm`: permitividades de partícula y medio
- `np, nm`: índices refractivos de partícula y medio

#### 🔬 Supuestos

1. **Baja concentración**: f << 0.3 (inclusiones no interactúan)
2. **Inclusiones pequeñas**: r << λ (régimen cuasiestático)
3. **Medio isotrópico**: Propiedades no direccionales
4. **Inclusiones esféricas**: Geometría específica

#### ⚠️ Rango de Validez

- **f < 0.3**: Para fracciones mayores, usar Bruggeman
- **r < λ/20**: Aproximación cuasiestática
- Válido para metales en dieléctricos y viceversa

### 2.2 Bruggeman (Effective Medium Approximation)

#### 🎯 Propósito
Medio efectivo para composites simétricos (sin fase dominante).

#### 📐 Ecuación

```
f(εp - εeff)/(εp + 2εeff) + (1-f)(εm - εeff)/(εm + 2εeff) = 0
```

Se resuelve numéricamente para `εeff`.

#### 🔬 Supuestos

1. **Simetría**: No hay matriz dominante
2. **Mezcla homogénea**: Distribución uniforme
3. **Tamaño pequeño**: r << λ

#### ⚠️ Rango de Validez

- **0 < f < 1**: Válido en todo el rango
- Mejor que MG para f > 0.3
- Converge a MG para f → 0

### 2.3 Comparación MG vs Bruggeman

| Característica | Maxwell-Garnett | Bruggeman |
|----------------|----------------|-----------|
| **Topología** | Inclusiones en matriz | Simétrico |
| **f óptimo** | f < 0.3 | Todo rango |
| **Ejemplo** | NPs de Au en vidrio | Cermet 50/50 |
| **Convergencia** | Explícita | Numérica |

---

## 3. Modelo de Drude-Lorentz

### 🎯 Propósito
Modelar la respuesta óptica de metales (plasmonics).

#### 📐 Ecuación de Drude (electrones libres)

```
ε(ω) = ε∞ - ωp²/(ω² + iγω)
```

Donde:
- `ε∞`: permitividad a alta frecuencia
- `ωp`: frecuencia de plasma
- `γ`: tasa de amortiguamiento

#### 📐 Modelo de Drude-Lorentz (incluye transiciones interbanda)

```
ε(ω) = ε∞ - ωp²/(ω² + iγω) + Σⱼ fⱼωp²/(ωⱼ² - ω² - iΓⱼω)
```

Términos adicionales:
- `fⱼ`: fuerza del oscilador j
- `ωⱼ`: frecuencia de resonancia
- `Γⱼ`: ancho de línea

### 🔬 Parámetros Típicos (Au)

| Parámetro | Valor |
|-----------|-------|
| ωp | 9.03 eV |
| γ | 0.072 eV |
| ε∞ | 9.84 |

### 📊 Casos de Prueba

- **Au a 520 nm**: ε ≈ -5.4 + 2.3i
- **Ag a 400 nm**: ε ≈ -3.8 + 0.5i

---

## 4. Coeficientes de Absorción

### 📐 Absorción de Beer-Lambert

```
A(λ) = 1 - exp(-α(λ) × d)
```

Donde:
- `α(λ)`: coeficiente de absorción [cm⁻¹]
- `d`: espesor de la muestra

#### Relación con índice complejo

```
α = 4π k / λ
```

Donde `k = Im(n)`.

### 🔬 Absorción en Nanopartículas (suspensión)

```
αeff = N × Cabs
```

Donde:
- `N`: concentración de partículas [partículas/cm³]
- `Cabs`: sección transversal de absorción

---

## 📚 Referencias

1. **Bohren, C. F., & Huffman, D. R.** (1983). *Absorption and Scattering of Light by Small Particles*. Wiley.
2. **Mätzler, C.** (2002). *MATLAB functions for Mie scattering and absorption*. IAP Research Report.
3. **Kreibig, U., & Vollmer, M.** (1995). *Optical Properties of Metal Clusters*. Springer.
4. **Johnson, P. B., & Christy, R. W.** (1972). Optical constants of noble metals. *Phys. Rev. B*, 6, 4370.

---

## 🧪 Tests de Validación

Para cada modelo, implementar:

1. **Test de límites**: Verificar convergencia a casos conocidos
2. **Test de simetría**: Verificar reciprocidad cuando aplique
3. **Test con datos experimentales**: Comparar con literatura
4. **Test de conservación**: Qext = Qsca + Qabs
5. **Test de convergencia numérica**: Verificar estabilidad con diferentes precisiones
