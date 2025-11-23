//! Core traits defining the physics model interfaces
//!
//! These traits provide extensibility: new physical models can be added
//! by implementing these traits without modifying existing code.

use crate::core::types::{CalcResult, ValidationResult};
use serde::{Deserialize, Serialize};

/// Base trait for all physics models
///
/// All physical models must implement this trait to be usable by the compute engine.
pub trait PhysicsModel: Send + Sync {
    /// Human-readable name of the model
    fn name(&self) -> &str;

    /// Short description of what the model calculates
    fn description(&self) -> &str;

    /// Validate input parameters before calculation
    fn validate(&self) -> ValidationResult<()>;

    /// Check if the model is applicable for given conditions
    fn is_applicable(&self) -> bool {
        self.validate().is_ok()
    }

    /// Get warnings about parameter ranges (non-fatal)
    fn warnings(&self) -> Vec<String> {
        Vec::new()
    }
}

/// Optical model trait for calculating optical properties
///
/// Models implementing this trait can calculate scattering, absorption,
/// and extinction for nanostructures.
pub trait OpticalModel: PhysicsModel {
    /// Calculate optical properties at a single wavelength
    fn calculate(&self) -> CalcResult<OpticalResult>;

    /// Calculate properties across a wavelength range
    fn calculate_spectrum(
        &self,
        wavelengths: &[f64], // nm
    ) -> CalcResult<Vec<OpticalResult>>;
}

/// Result of optical calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpticalResult {
    /// Wavelength in nm
    pub wavelength: f64,

    /// Scattering efficiency (dimensionless)
    pub q_sca: f64,

    /// Absorption efficiency (dimensionless)
    pub q_abs: f64,

    /// Extinction efficiency (dimensionless)
    pub q_ext: f64,

    /// Scattering cross-section in nm²
    pub c_sca: f64,

    /// Absorption cross-section in nm²
    pub c_abs: f64,

    /// Extinction cross-section in nm²
    pub c_ext: f64,

    /// Additional metadata
    pub metadata: OpticalMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OpticalMetadata {
    /// Number of terms used in series expansion (for Mie)
    pub num_terms: Option<usize>,

    /// Convergence achieved
    pub converged: bool,

    /// Size parameter x = 2πr/λ
    pub size_parameter: f64,

    /// Model-specific notes
    pub notes: Vec<String>,
}

impl OpticalResult {
    /// Conservation check: Q_ext should equal Q_sca + Q_abs
    pub fn check_conservation(&self) -> f64 {
        (self.q_ext - (self.q_sca + self.q_abs)).abs()
    }
}

/// Thermal model trait for calculating thermal properties
pub trait ThermalModel: PhysicsModel {
    /// Calculate thermal properties
    fn calculate(&self) -> CalcResult<ThermalResult>;

    /// Calculate temperature-dependent thermal conductivity
    fn calculate_temperature_sweep(
        &self,
        temperatures: &[f64], // K
    ) -> CalcResult<Vec<ThermalResult>>;
}

/// Result of thermal calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalResult {
    /// Temperature in Kelvin
    pub temperature: f64,

    /// Effective thermal conductivity in W/(m·K)
    pub kappa_eff: f64,

    /// Bulk thermal conductivity for comparison
    pub kappa_bulk: f64,

    /// Reduction factor (kappa_eff / kappa_bulk)
    pub reduction_factor: f64,

    /// Phonon mean free path in nm
    pub mfp: Option<f64>,

    /// Additional metadata
    pub metadata: ThermalMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThermalMetadata {
    /// Ratio d/λ_mfp
    pub size_to_mfp_ratio: Option<f64>,

    /// Dominant scattering mechanism
    pub dominant_mechanism: Option<String>,

    /// Model-specific notes
    pub notes: Vec<String>,
}

/// Electronic model trait for calculating electronic properties
pub trait ElectronicModel: PhysicsModel {
    /// Calculate electronic properties
    fn calculate(&self) -> CalcResult<ElectronicResult>;

    /// Calculate size-dependent bandgap
    fn calculate_size_sweep(
        &self,
        sizes: &[f64], // nm (diameter)
    ) -> CalcResult<Vec<ElectronicResult>>;
}

/// Result of electronic calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectronicResult {
    /// Nanoparticle diameter in nm
    pub diameter: f64,

    /// Bandgap in eV
    pub bandgap: f64,

    /// Bulk bandgap for comparison
    pub bulk_bandgap: f64,

    /// Confinement energy contribution in eV
    pub confinement_energy: f64,

    /// Coulombic correction in eV
    pub coulomb_correction: f64,

    /// Exciton Bohr radius in nm
    pub bohr_radius: Option<f64>,

    /// Confinement regime
    pub regime: ConfinementRegime,

    /// Additional metadata
    pub metadata: ElectronicMetadata,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConfinementRegime {
    Weak,        // r >> a_B
    Intermediate, // r ≈ a_B
    Strong,      // r << a_B
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ElectronicMetadata {
    /// Effective mass used (in units of m_e)
    pub effective_mass: Option<f64>,

    /// Dielectric constant
    pub dielectric_constant: Option<f64>,

    /// Model type (Brus, EMA, etc.)
    pub model_type: String,

    /// Model-specific notes
    pub notes: Vec<String>,
}

/// Trait for models that support caching
pub trait Cacheable {
    /// Generate a cache key from model parameters
    fn cache_key(&self) -> String;
}

/// Trait for models that can be parallelized
pub trait Parallelizable {
    /// Can this calculation be split across multiple threads?
    fn can_parallelize(&self) -> bool {
        true
    }

    /// Recommended chunk size for parallel processing
    fn recommended_chunk_size(&self) -> usize {
        100
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optical_result_conservation() {
        let result = OpticalResult {
            wavelength: 500.0,
            q_sca: 1.5,
            q_abs: 0.5,
            q_ext: 2.0,
            c_sca: 100.0,
            c_abs: 33.33,
            c_ext: 133.33,
            metadata: OpticalMetadata::default(),
        };

        assert!(result.check_conservation() < 1e-10);
    }
}
