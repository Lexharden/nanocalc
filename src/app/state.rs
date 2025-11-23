//! Application state management

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub particle_radius: f64,
    pub wavelength: f64,
    pub n_particle_real: f64,
    pub n_particle_imag: f64,
    pub n_medium: f64,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            particle_radius: 50.0,  // nm
            wavelength: 500.0,       // nm
            n_particle_real: 0.5,    // Au at 500nm (approx)
            n_particle_imag: 2.5,
            n_medium: 1.33,          // water
        }
    }
}
