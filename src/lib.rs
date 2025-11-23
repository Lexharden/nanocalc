//! NanoCalc Library
//!
//! Scientific calculator for optical, thermal, and electronic properties
//! of nanomaterials.

pub mod core;
pub mod physics;
pub mod compute;
pub mod gui;
pub mod app;
pub mod export;
pub mod project;
pub mod plotting;
pub mod utils;

// Re-export main types
pub use core::{
    CalcResult, CalculationError, ElectronicModel, ElectronicResult, OpticalModel,
    OpticalResult, PhysicsModel, ThermalModel, ThermalResult, ValidationError,
};
