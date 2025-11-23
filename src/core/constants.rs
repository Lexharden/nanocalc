//! Physical constants with CODATA 2018 values
//!
//! All values are in SI units unless otherwise specified.

use std::f64::consts::PI;

/// Speed of light in vacuum [m/s]
pub const C: f64 = 2.99792458e8;

/// Speed of light in nm/s
pub const C_NM_S: f64 = 2.99792458e17;

/// Planck constant [J·s]
pub const H: f64 = 6.62607015e-34;

/// Reduced Planck constant ℏ [J·s]
pub const HBAR: f64 = 1.054571817e-34;

/// Boltzmann constant [J/K]
pub const K_B: f64 = 1.380649e-23;

/// Elementary charge [C]
pub const E: f64 = 1.602176634e-19;

/// Electron mass [kg]
pub const M_E: f64 = 9.1093837015e-31;

/// Proton mass [kg]
pub const M_P: f64 = 1.67262192369e-27;

/// Avogadro constant [mol⁻¹]
pub const N_A: f64 = 6.02214076e23;

/// Vacuum permittivity ε₀ [F/m]
pub const EPSILON_0: f64 = 8.8541878128e-12;

/// Vacuum permeability μ₀ [H/m]
pub const MU_0: f64 = 1.25663706212e-6;

/// Fine structure constant α (dimensionless)
pub const ALPHA: f64 = 7.2973525693e-3;

/// Rydberg constant [eV]
pub const RY: f64 = 13.605693122994;

/// Bohr radius [m]
pub const BOHR_RADIUS: f64 = 5.29177210903e-11;

/// Bohr radius [nm]
pub const BOHR_RADIUS_NM: f64 = 0.05291772109;

/// Conversion factors
pub mod conversions {
    /// Electron volt to Joule
    pub const EV_TO_J: f64 = 1.602176634e-19;

    /// Joule to electron volt
    pub const J_TO_EV: f64 = 6.241509074e18;

    /// Nanometer to meter
    pub const NM_TO_M: f64 = 1e-9;

    /// Meter to nanometer
    pub const M_TO_NM: f64 = 1e9;

    /// h*c product in eV·nm (useful for photon energy)
    pub const HC_EV_NM: f64 = 1239.84193;

    /// Atomic mass unit to kg
    pub const AMU_TO_KG: f64 = 1.66053906660e-27;
}

/// Useful compound constants
pub mod compound {
    use super::*;

    /// Thermal energy at 300 K in eV
    pub const K_B_T_300K_EV: f64 = 0.02585;

    /// Thermal energy at 300 K in Joules
    pub const K_B_T_300K_J: f64 = 4.14e-21;

    /// Characteristic length in nm for thermal de Broglie wavelength at 300K
    pub fn thermal_de_broglie_nm(mass_kg: f64) -> f64 {
        let lambda = H / (2.0 * PI * mass_kg * K_B * 300.0).sqrt();
        lambda * 1e9 // convert to nm
    }

    /// Plasma frequency to wavelength conversion
    pub fn plasma_wavelength_nm(omega_p_ev: f64) -> f64 {
        conversions::HC_EV_NM / omega_p_ev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fine_structure() {
        // α ≈ e²/(4πε₀ℏc)
        let alpha_calc = E * E / (4.0 * PI * EPSILON_0 * HBAR * C);
        assert!((alpha_calc - ALPHA).abs() / ALPHA < 1e-6);
    }

    #[test]
    fn test_hc_product() {
        // h*c in eV·nm
        let hc = H * C * conversions::M_TO_NM / conversions::EV_TO_J;
        assert!((hc - conversions::HC_EV_NM).abs() / conversions::HC_EV_NM < 1e-6);
    }

    #[test]
    fn test_rydberg() {
        // Ry = m_e e⁴ / (8ε₀²h²)
        let ry_calc = M_E * E.powi(4) / (8.0 * EPSILON_0.powi(2) * H.powi(2));
        let ry_j = ry_calc;
        let ry_ev = ry_j / E;
        assert!((ry_ev - RY).abs() / RY < 1e-6);
    }

    #[test]
    fn test_bohr_radius() {
        // a₀ = 4πε₀ℏ² / (m_e e²)
        let a0_calc = 4.0 * PI * EPSILON_0 * HBAR.powi(2) / (M_E * E.powi(2));
        assert!((a0_calc - BOHR_RADIUS).abs() / BOHR_RADIUS < 1e-6);
    }
}
