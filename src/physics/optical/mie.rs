//! Mie scattering theory implementation (simplified for MVP)
//!
//! This is a placeholder implementation. Full Mie theory requires Bessel functions
//! and series convergence. For MVP, we implement a Rayleigh approximation.

use crate::core::*;
use num_complex::Complex64;
use std::f64::consts::PI;

/// Mie scattering model (Rayleigh approximation for MVP)
pub struct MieModel {
    /// Particle radius in nm
    pub radius: f64,
    /// Wavelength in nm
    pub wavelength: f64,
    /// Particle refractive index
    pub n_particle: RefractiveIndex,
    /// Medium refractive index (real only for MVP)
    pub n_medium: f64,
}

impl MieModel {
    pub fn new(
        radius: f64,
        wavelength: f64,
        n_particle: RefractiveIndex,
        n_medium: f64,
    ) -> Self {
        Self {
            radius,
            wavelength,
            n_particle,
            n_medium,
        }
    }

    /// Calculate size parameter x = 2πr/λ
    fn size_parameter(&self) -> f64 {
        2.0 * PI * self.radius / self.wavelength
    }

    /// Rayleigh approximation (x << 1)
    fn rayleigh_approximation(&self) -> OpticalResult {
        let x = self.size_parameter();
        let m = self.n_particle.to_complex() / self.n_medium;
        
        // Scattering efficiency (Rayleigh)
        let m2_minus_1 = m * m - Complex64::new(1.0, 0.0);
        let m2_plus_2 = m * m + Complex64::new(2.0, 0.0);
        let factor = m2_minus_1 / m2_plus_2;
        
        let q_sca = (8.0 / 3.0) * x.powi(4) * factor.norm_sqr();
        
        // Absorption efficiency
        let q_abs = 4.0 * x * (m2_minus_1 / m2_plus_2).im;
        
        // Extinction
        let q_ext = q_sca + q_abs;
        
        // Cross sections
        let geometric_area = PI * self.radius.powi(2);
        let c_sca = q_sca * geometric_area;
        let c_abs = q_abs * geometric_area;
        let c_ext = q_ext * geometric_area;
        
        OpticalResult {
            wavelength: self.wavelength,
            q_sca,
            q_abs,
            q_ext,
            c_sca,
            c_abs,
            c_ext,
            metadata: OpticalMetadata {
                num_terms: Some(1),
                converged: true,
                size_parameter: x,
                notes: vec!["Rayleigh approximation".to_string()],
            },
        }
    }
}

impl PhysicsModel for MieModel {
    fn name(&self) -> &str {
        "Mie Scattering (Rayleigh Approximation)"
    }

    fn description(&self) -> &str {
        "Calculate scattering and absorption for spherical nanoparticles (x < 1)"
    }

    fn validate(&self) -> ValidationResult<()> {
        if self.radius <= 0.0 {
            return Err(ValidationError::InvalidParameter(
                "Radius must be positive".to_string(),
            ));
        }
        if self.wavelength <= 0.0 {
            return Err(ValidationError::InvalidParameter(
                "Wavelength must be positive".to_string(),
            ));
        }
        if self.n_medium <= 0.0 {
            return Err(ValidationError::InvalidParameter(
                "Medium refractive index must be positive".to_string(),
            ));
        }
        Ok(())
    }

    fn warnings(&self) -> Vec<String> {
        let mut warnings = Vec::new();
        let x = self.size_parameter();
        
        if x > 1.0 {
            warnings.push(format!(
                "Size parameter x={:.2} > 1. Rayleigh approximation may be inaccurate. \
                 Full Mie theory recommended.",
                x
            ));
        }
        
        warnings
    }
}

impl OpticalModel for MieModel {
    fn calculate(&self) -> CalcResult<OpticalResult> {
        self.validate()?;
        Ok(self.rayleigh_approximation())
    }

    fn calculate_spectrum(&self, wavelengths: &[f64]) -> CalcResult<Vec<OpticalResult>> {
        wavelengths
            .iter()
            .map(|&wl| {
                let mut model = self.clone();
                model.wavelength = wl;
                model.calculate()
            })
            .collect()
    }
}

impl Clone for MieModel {
    fn clone(&self) -> Self {
        Self {
            radius: self.radius,
            wavelength: self.wavelength,
            n_particle: self.n_particle,
            n_medium: self.n_medium,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mie_basic() {
        let model = MieModel::new(
            10.0,                            // 10 nm radius
            500.0,                           // 500 nm wavelength
            RefractiveIndex::new(0.5, 2.5),  // Au-like
            1.33,                            // water
        );

        let result = model.calculate().unwrap();
        
        // Basic sanity checks
        assert!(result.q_sca >= 0.0);
        assert!(result.q_abs >= 0.0);
        assert!(result.q_ext >= result.q_sca + result.q_abs - 1e-10);
        
        // Conservation
        assert!(result.check_conservation() < 1e-6);
    }

    #[test]
    fn test_size_parameter() {
        let model = MieModel::new(
            50.0,
            500.0,
            RefractiveIndex::new(1.5, 0.0),
            1.0,
        );
        
        let x = model.size_parameter();
        let expected = 2.0 * PI * 50.0 / 500.0;
        assert!((x - expected).abs() < 1e-10);
    }
}
