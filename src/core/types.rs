//! Core traits and types for NanoCalc
//!
//! This module defines the fundamental abstractions used throughout the system.

use num_complex::Complex64;
use std::fmt;

/// Physical units using newtype pattern for type safety
pub mod units {
    use serde::{Deserialize, Serialize};

    /// Nanometers
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
    pub struct Nanometer(pub f64);

    /// Micrometers
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
    pub struct Micrometer(pub f64);

    /// Wavelength in nanometers
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
    pub struct Wavelength(pub f64);

    /// Temperature in Kelvin
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
    pub struct Kelvin(pub f64);

    /// Energy in electron volts
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
    pub struct ElectronVolt(pub f64);

    /// Thermal conductivity in W/(m·K)
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
    pub struct ThermalConductivity(pub f64);

    impl Nanometer {
        pub fn to_meters(self) -> f64 {
            self.0 * 1e-9
        }
    }

    impl Wavelength {
        pub fn to_energy_ev(self) -> ElectronVolt {
            const HC: f64 = 1239.84193; // h*c in eV·nm
            ElectronVolt(HC / self.0)
        }

        pub fn to_frequency_hz(self) -> f64 {
            const C: f64 = 2.99792458e17; // speed of light in nm/s
            C / self.0
        }
    }

    impl Kelvin {
        pub fn to_celsius(self) -> f64 {
            self.0 - 273.15
        }
    }
}

/// Complex refractive index n + ik
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RefractiveIndex {
    pub real: f64,      // n (real part)
    pub imaginary: f64, // k (extinction coefficient)
}

impl RefractiveIndex {
    pub fn new(n: f64, k: f64) -> Self {
        Self {
            real: n,
            imaginary: k,
        }
    }

    pub fn to_complex(self) -> Complex64 {
        Complex64::new(self.real, self.imaginary)
    }

    pub fn to_permittivity(self) -> Complex64 {
        let n = self.to_complex();
        n * n
    }
}

impl fmt::Display for RefractiveIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.4} + {:.4}i", self.real, self.imaginary)
    }
}

/// Parameter validation result
pub type ValidationResult<T> = Result<T, ValidationError>;

/// Validation errors
#[derive(Debug, Clone, thiserror::Error)]
pub enum ValidationError {
    #[error("Value {value} is out of valid range [{min}, {max}]")]
    OutOfRange { value: f64, min: f64, max: f64 },

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Physical constraint violated: {0}")]
    PhysicsViolation(String),
}

/// Calculation errors
#[derive(Debug, thiserror::Error)]
pub enum CalculationError {
    #[error("Convergence failed after {iterations} iterations")]
    ConvergenceFailed { iterations: usize },

    #[error("Numerical instability detected: {0}")]
    NumericalInstability(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Model not applicable: {0}")]
    ModelNotApplicable(String),

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),
}

pub type CalcResult<T> = Result<T, CalculationError>;
