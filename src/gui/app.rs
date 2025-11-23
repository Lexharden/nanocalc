//! Main GUI application with modern, intuitive interface

use crate::app::AppState;
use crate::core::{OpticalResult, RefractiveIndex};
use crate::physics::optical::mie::MieModel;
use crate::core::OpticalModel;
use egui::{CentralPanel, Context, SidePanel, TopBottomPanel, Rounding, Color32};
use egui_plot::{Line, Plot, PlotPoints, Legend, Corner};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    English,
    Spanish,
}

#[derive(Debug, Clone)]
pub struct ElementProperties {
    symbol: String,
    name: String,
    atomic_number: u32,
    n_real: f64,
    n_imag: f64,
}

pub struct NanoCalcApp {
    state: AppState,
    result: Option<OpticalResult>,
    spectrum_results: Vec<OpticalResult>,
    calculating: bool,
    error_message: Option<String>,
    show_about: bool,
    show_periodic_table: bool,
    show_element_properties: bool,
    selected_element: Option<ElementProperties>,
    language: Language,
    plot_reset_counter: u32,  // Para forzar reset del plot
    show_export_dialog: bool,
    export_filename: String,
    export_type: ExportType,
    log_messages: Vec<String>,  // Log de mensajes
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ExportType {
    CSV,
    JSON,
    PNG,
}

// Material presets for quick access
struct MaterialPreset {
    name: &'static str,
    n_real: f64,
    n_imag: f64,
    description: &'static str,
}

const MATERIAL_PRESETS: &[MaterialPreset] = &[
    MaterialPreset {
        name: "Gold (Au)",
        n_real: 0.47,
        n_imag: 2.40,
        description: "Gold nanoparticles at 520 nm",
    },
    MaterialPreset {
        name: "Silver (Ag)",
        n_real: 0.05,
        n_imag: 3.00,
        description: "Silver nanoparticles at 400 nm",
    },
    MaterialPreset {
        name: "Silicon (Si)",
        n_real: 4.15,
        n_imag: 0.04,
        description: "Silicon at 500 nm",
    },
    MaterialPreset {
        name: "TiO₂",
        n_real: 2.50,
        n_imag: 0.00,
        description: "Titanium dioxide (rutile)",
    },
];

impl Default for NanoCalcApp {
    fn default() -> Self {
        Self {
            state: AppState::default(),
            result: None,
            spectrum_results: Vec::new(),
            calculating: false,
            error_message: None,
            show_about: false,
            show_periodic_table: false,
            show_element_properties: false,
            selected_element: None,
            language: Language::English,
            plot_reset_counter: 0,
            show_export_dialog: false,
            export_filename: String::from("nanocalc_spectrum"),
            export_type: ExportType::CSV,
            log_messages: vec![String::from("✅ NanoCalc initialized")],
        }
    }
}

impl NanoCalcApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Configure fonts and style
        Self::configure_style(&cc.egui_ctx);
        Self::default()
    }

    fn configure_style(ctx: &Context) {
        use egui::{FontFamily, FontId, TextStyle, Visuals, Rounding};

        // Set modern fonts
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (TextStyle::Heading, FontId::new(26.0, FontFamily::Proportional)),
            (TextStyle::Body, FontId::new(15.0, FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new(14.0, FontFamily::Monospace)),
            (TextStyle::Button, FontId::new(15.0, FontFamily::Proportional)),
            (TextStyle::Small, FontId::new(13.0, FontFamily::Proportional)),
        ]
        .into();

        // Modern rounded corners and spacing
        style.spacing.item_spacing = egui::vec2(8.0, 10.0);
        style.spacing.button_padding = egui::vec2(12.0, 8.0);
        style.spacing.window_margin = egui::Margin::same(12.0);
        style.spacing.menu_margin = egui::Margin::same(8.0);
        style.visuals.window_rounding = Rounding::same(10.0);
        style.visuals.menu_rounding = Rounding::same(8.0);
        style.visuals.widgets.noninteractive.rounding = Rounding::same(6.0);
        style.visuals.widgets.inactive.rounding = Rounding::same(6.0);
        style.visuals.widgets.hovered.rounding = Rounding::same(6.0);
        style.visuals.widgets.active.rounding = Rounding::same(6.0);

        // Color scheme - modern blue tones
        let mut visuals = Visuals::dark();
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(45, 50, 65);
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(60, 65, 85);
        visuals.widgets.active.bg_fill = Color32::from_rgb(70, 130, 180);
        visuals.selection.bg_fill = Color32::from_rgba_premultiplied(70, 130, 180, 100);
        visuals.window_fill = Color32::from_rgb(30, 33, 42);
        visuals.panel_fill = Color32::from_rgb(35, 38, 48);
        
        style.visuals = visuals;
        ctx.set_style(style);
    }

    fn t(&self, en: &str, es: &str) -> String {
        match self.language {
            Language::English => en.to_string(),
            Language::Spanish => es.to_string(),
        }
    }

    fn get_element_properties(symbol: &str, name: &str, atomic_number: u32) -> ElementProperties {
        // Propiedades ópticas aproximadas para elementos comunes (550 nm)
        let (n_real, n_imag) = match symbol {
            "Au" => (0.47, 2.40),  // Oro
            "Ag" => (0.05, 3.00),  // Plata
            "Cu" => (0.94, 2.43),  // Cobre
            "Al" => (0.82, 6.50),  // Aluminio
            "Si" => (4.15, 0.04),  // Silicio
            "Ti" => (2.90, 3.10),  // Titanio
            "Fe" => (2.95, 3.50),  // Hierro
            "Ni" => (2.40, 4.30),  // Níquel
            "Pt" => (2.37, 4.26),  // Platino
            "Pd" => (1.80, 4.40),  // Paladio
            "Cr" => (3.10, 3.30),  // Cromo
            "Zn" => (1.70, 5.00),  // Zinc
            "C" => (2.40, 1.40),   // Carbono (grafito)
            _ => (1.50, 0.00),     // Valor por defecto
        };
        
        ElementProperties {
            symbol: symbol.to_string(),
            name: name.to_string(),
            atomic_number,
            n_real,
            n_imag,
        }
    }

    fn apply_material_preset(&mut self, preset: &MaterialPreset) {
        self.state.n_particle_real = preset.n_real;
        self.state.n_particle_imag = preset.n_imag;
    }

    fn calculate_single(&mut self) {
        self.calculating = true;
        self.error_message = None;
        
        let msg = self.t(
            &format!("🔬 Calculating at {} nm...", self.state.wavelength),
            &format!("🔬 Calculando en {} nm...", self.state.wavelength)
        );
        self.add_log(&msg);

        let model = MieModel::new(
            self.state.particle_radius,
            self.state.wavelength,
            RefractiveIndex::new(self.state.n_particle_real, self.state.n_particle_imag),
            self.state.n_medium,
        );

        match model.calculate() {
            Ok(result) => {
                self.result = Some(result);
                self.add_log(&self.t("✅ Single point calculated", "✅ Punto único calculado"));
            }
            Err(e) => {
                let error_msg = format!("Calculation error: {}", e);
                self.error_message = Some(error_msg.clone());
                self.add_log(&format!("❌ {}", error_msg));
            }
        }

        self.calculating = false;
    }

    fn calculate_spectrum(&mut self) {
        self.calculating = true;
        self.error_message = None;
        
        self.add_log(&self.t("📊 Calculating full spectrum (300-800 nm)...", "📊 Calculando espectro completo (300-800 nm)..."));

        let wavelengths: Vec<f64> = (300..=800).step_by(5).map(|w| w as f64).collect();

        let model = MieModel::new(
            self.state.particle_radius,
            self.state.wavelength,
            RefractiveIndex::new(self.state.n_particle_real, self.state.n_particle_imag),
            self.state.n_medium,
        );

        match model.calculate_spectrum(&wavelengths) {
            Ok(results) => {
                self.spectrum_results = results;
                self.plot_reset_counter += 1;  // Forzar reset del plot
                let msg = self.t(
                    &format!("✅ Spectrum calculated ({} points)", self.spectrum_results.len()),
                    &format!("✅ Espectro calculado ({} puntos)", self.spectrum_results.len())
                );
                self.add_log(&msg);
            }
            Err(e) => {
                let error_msg = format!("Spectrum calculation error: {}", e);
                self.error_message = Some(error_msg.clone());
                self.add_log(&format!("❌ {}", error_msg));
            }
        }

        self.calculating = false;
    }

    fn draw_input_panel(&mut self, ui: &mut egui::Ui) {
        ui.add_space(5.0);
        ui.heading(&self.t("Input Parameters", "Parámetros de Entrada"))
            .on_hover_text(&self.t(
                "Configure the nanoparticle and environment properties for optical calculations",
                "Configura las propiedades de la nanopartícula y el entorno para cálculos ópticos"
            ));
        ui.add_space(15.0);

        // Material Presets Section
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.strong(&self.t("Quick Presets", "Preajustes Rápidos"));
            });
            ui.add_space(5.0);

            egui::Grid::new("preset_grid")
                .num_columns(2)
                .spacing([8.0, 8.0])
                .show(ui, |ui| {
                    for preset in MATERIAL_PRESETS {
                        if ui.button(format!("{}", preset.name))
                            .on_hover_text(preset.description)
                            .clicked() 
                        {
                            self.apply_material_preset(preset);
                        }
                        if ui.available_width() < 50.0 {
                            ui.end_row();
                        }
                    }
                });
        });

        ui.add_space(12.0);

        // Periodic Table Button
        if ui.button("Select from Periodic Table").on_hover_text("Choose element optical properties").clicked() {
            self.show_periodic_table = true;
        }

        ui.add_space(12.0);

        // Particle Properties Card
        egui::Frame::none()
            .fill(Color32::from_rgb(40, 43, 53))
            .rounding(Rounding::same(8.0))
            .inner_margin(egui::Margin::same(12.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.strong("Particle Properties");
                    ui.label("ℹ️")
                        .on_hover_text(&self.t(
                            "Physical size and optical properties of the nanoparticle",
                            "Tamaño físico y propiedades ópticas de la nanopartícula"
                        ));
                });
                ui.add_space(8.0);

                // Radius input
                ui.horizontal(|ui| {
                    ui.label("Radius (r):");
                    ui.label("ℹ️")
                        .on_hover_text(&self.t(
                            "Particle radius in nanometers (1-1000 nm). Typical: 10-100 nm",
                            "Radio de la partícula en nanómetros (1-1000 nm). Típico: 10-100 nm"
                        ));
                    ui.add(egui::DragValue::new(&mut self.state.particle_radius)
                        .speed(1.0)
                        .range(1.0..=1000.0)
                        .suffix(" nm"));
                });

                ui.add_space(5.0);

                // Refractive index inputs
                ui.horizontal(|ui| {
                    ui.label("n (real):");
                    ui.label("ℹ️")
                        .on_hover_text(&self.t(
                            "Real part of refractive index. Controls light velocity in material",
                            "Parte real del índice de refracción. Controla la velocidad de la luz en el material"
                        ));
                    ui.add(egui::DragValue::new(&mut self.state.n_particle_real)
                        .speed(0.01)
                        .range(-10.0..=10.0)
                        .fixed_decimals(2));
                });

                ui.horizontal(|ui| {
                    ui.label("k (imag):");
                    ui.label("ℹ️")
                        .on_hover_text(&self.t(
                            "Imaginary part (extinction coefficient). Controls light absorption",
                            "Parte imaginaria (coeficiente de extinción). Controla la absorción de luz"
                        ));
                    ui.add(egui::DragValue::new(&mut self.state.n_particle_imag)
                        .speed(0.01)
                        .range(0.0..=10.0)
                        .fixed_decimals(2));
                });

                // Show complex index
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.colored_label(
                        Color32::from_rgb(100, 180, 255),
                        format!("n = {:.2} + {:.2}i", 
                            self.state.n_particle_real, 
                            self.state.n_particle_imag)
                    );
                });
            });

        ui.add_space(12.0);

        // Environment Card
        egui::Frame::none()
            .fill(Color32::from_rgb(40, 43, 53))
            .rounding(Rounding::same(8.0))
            .inner_margin(egui::Margin::same(12.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.strong("Environment");
                    ui.label("ℹ️")
                        .on_hover_text(&self.t(
                            "Surrounding medium and incident light properties",
                            "Propiedades del medio circundante y luz incidente"
                        ));
                });
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label("Wavelength (λ):");
                    ui.label("ℹ️")
                        .on_hover_text(&self.t(
                            "Wavelength of incident light (200-2000 nm). Visible: 400-700 nm",
                            "Longitud de onda de la luz incidente (200-2000 nm). Visible: 400-700 nm"
                        ));
                    ui.add(egui::DragValue::new(&mut self.state.wavelength)
                        .speed(1.0)
                        .range(200.0..=2000.0)
                        .suffix(" nm"));
                });

                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.label("n (medium):");
                    ui.label("ℹ️")
                        .on_hover_text(&self.t(
                            "Refractive index of surrounding medium (air=1.0, water=1.33, glass≈1.5)",
                            "Índice de refracción del medio circundante (aire=1.0, agua=1.33, vidrio≈1.5)"
                        ));
                    ui.add(egui::DragValue::new(&mut self.state.n_medium)
                        .speed(0.01)
                        .range(1.0..=3.0)
                        .fixed_decimals(2));
                });

                // Show photon energy
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.label("⚡");
                    let energy_ev = 1239.84193 / self.state.wavelength;
                    ui.colored_label(
                        Color32::from_rgb(100, 255, 180),
                        format!("E = {:.2} eV", energy_ev)
                    );
                });
            });

        ui.add_space(20.0);

        // Action Buttons
        ui.vertical_centered(|ui| {
            let btn_size = egui::vec2(ui.available_width() - 20.0, 40.0);
            
            if ui.add_sized(btn_size, egui::Button::new("🔬 Calculate Single Point"))
                .on_hover_text("Calculate optical properties at current wavelength")
                .clicked() 
            {
                self.calculate_single();
            }

            ui.add_space(8.0);

            if ui.add_sized(btn_size, egui::Button::new("📊 Calculate Full Spectrum"))
                .on_hover_text("Calculate properties across wavelength range (300-800 nm)")
                .clicked() 
            {
                self.calculate_spectrum();
            }
        });

        // Error/Warning Messages
        if let Some(ref error) = self.error_message {
            ui.add_space(12.0);
            egui::Frame::none()
                .fill(Color32::from_rgb(120, 40, 40))
                .rounding(Rounding::same(6.0))
                .inner_margin(egui::Margin::same(10.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("⚠️");
                        ui.colored_label(Color32::from_rgb(255, 150, 150), error);
                    });
                });
        }

        ui.add_space(10.0);
    }

    fn draw_results_panel(&mut self, ui: &mut egui::Ui) {
        ui.add_space(5.0);
        ui.heading(&self.t("Results", "Resultados"));
        ui.add_space(15.0);

        if let Some(ref result) = self.result {
            // Main info card
            egui::Frame::none()
                .fill(Color32::from_rgb(45, 48, 58))
                .rounding(Rounding::same(8.0))
                .inner_margin(egui::Margin::same(12.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("📏");
                        ui.strong("Calculation Parameters");
                    });
                    ui.add_space(5.0);
                    ui.label(format!("Wavelength: {:.1} nm", result.wavelength));
                    ui.label(format!("Size parameter: x = {:.4}", result.metadata.size_parameter));
                    
                    if result.metadata.size_parameter > 1.0 {
                        ui.colored_label(
                            Color32::from_rgb(255, 200, 100),
                            "⚠ Note: Rayleigh approximation may be inaccurate (x > 1)"
                        );
                    }
                });

            ui.add_space(12.0);

            // Efficiencies Card
            egui::Frame::none()
                .fill(Color32::from_rgb(40, 60, 80))
                .rounding(Rounding::same(8.0))
                .inner_margin(egui::Margin::same(12.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("📈");
                        ui.strong("Efficiency Factors (Dimensionless)");
                        ui.label("ℹ️")
                            .on_hover_text(&self.t(
                                "Efficiency = Cross-section / Geometric area. Values >1 indicate resonance effects",
                                "Eficiencia = Sección transversal / Área geométrica. Valores >1 indican efectos de resonancia"
                            ));
                    });
                    ui.add_space(8.0);

                    egui::Grid::new("efficiencies")
                        .num_columns(2)
                        .spacing([15.0, 8.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label("Q_sca:");
                                ui.label("ℹ️")
                                    .on_hover_text(&self.t(
                                        "Scattering efficiency: ratio of scattered power to incident power on geometric area",
                                        "Eficiencia de dispersión: razón de potencia dispersada a potencia incidente en área geométrica"
                                    ));
                            });
                            ui.colored_label(
                                Color32::from_rgb(100, 180, 255),
                                format!("{:.5}", result.q_sca)
                            ).on_hover_text(&self.t(
                                "Light scattered in all directions",
                                "Luz dispersada en todas direcciones"
                            ));
                            ui.end_row();

                            ui.horizontal(|ui| {
                                ui.label("Q_abs:");
                                ui.label("ℹ️")
                                    .on_hover_text(&self.t(
                                        "Absorption efficiency: ratio of absorbed power to incident power on geometric area",
                                        "Eficiencia de absorción: razón de potencia absorbida a potencia incidente en área geométrica"
                                    ));
                            });
                            ui.colored_label(
                                Color32::from_rgb(255, 140, 100),
                                format!("{:.5}", result.q_abs)
                            ).on_hover_text(&self.t(
                                "Light absorbed and converted to heat",
                                "Luz absorbida y convertida en calor"
                            ));
                            ui.end_row();

                            ui.horizontal(|ui| {
                                ui.label("Q_ext:");
                                ui.label("ℹ️")
                                    .on_hover_text(&self.t(
                                        "Extinction efficiency: total light removed from beam (Q_ext = Q_sca + Q_abs)",
                                        "Eficiencia de extinción: luz total removida del haz (Q_ext = Q_sca + Q_abs)"
                                    ));
                            });
                            ui.colored_label(
                                Color32::from_rgb(100, 255, 150),
                                format!("{:.5}", result.q_ext)
                            ).on_hover_text(&self.t(
                                "Total light removed = scattering + absorption",
                                "Luz total removida = dispersión + absorción"
                            ));
                            ui.end_row();
                        });
                });

            ui.add_space(12.0);

            // Cross Sections Card
            egui::Frame::none()
                .fill(Color32::from_rgb(60, 45, 70))
                .rounding(Rounding::same(8.0))
                .inner_margin(egui::Margin::same(12.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("🎯");
                        ui.strong("Cross Sections (nm²)");
                        ui.label("ℹ️")
                            .on_hover_text(&self.t(
                                "Effective areas for light-particle interactions in nm²",
                                "Áreas efectivas para interacciones luz-partícula en nm²"
                            ));
                    });
                    ui.add_space(8.0);

                    egui::Grid::new("cross_sections")
                        .num_columns(2)
                        .spacing([15.0, 8.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label("C_sca:");
                                ui.label("ℹ️")
                                    .on_hover_text(&self.t(
                                        "Scattering cross-section: effective area for scattering",
                                        "Sección transversal de dispersión: área efectiva para dispersión"
                                    ));
                            });
                            ui.colored_label(
                                Color32::from_rgb(100, 180, 255),
                                format!("{:.2}", result.c_sca)
                            ).on_hover_text(&self.t(
                                "C_sca = Q_sca × πr². Measure of scattering strength",
                                "C_sca = Q_sca × πr². Medida de la fuerza de dispersión"
                            ));
                            ui.end_row();

                            ui.horizontal(|ui| {
                                ui.label("C_abs:");
                                ui.label("ℹ️")
                                    .on_hover_text(&self.t(
                                        "Absorption cross-section: effective area for absorption",
                                        "Sección transversal de absorción: área efectiva para absorción"
                                    ));
                            });
                            ui.colored_label(
                                Color32::from_rgb(255, 140, 100),
                                format!("{:.2}", result.c_abs)
                            ).on_hover_text(&self.t(
                                "C_abs = Q_abs × πr². Measure of absorption strength",
                                "C_abs = Q_abs × πr². Medida de la fuerza de absorción"
                            ));
                            ui.end_row();

                            ui.horizontal(|ui| {
                                ui.label("C_ext:");
                                ui.label("ℹ️")
                                    .on_hover_text(&self.t(
                                        "Extinction cross-section: total effective area (C_sca + C_abs)",
                                        "Sección transversal de extinción: área total efectiva (C_sca + C_abs)"
                                    ));
                            });
                            ui.colored_label(
                                Color32::from_rgb(100, 255, 150),
                                format!("{:.2}", result.c_ext)
                            ).on_hover_text(&self.t(
                                "C_ext = C_sca + C_abs = Q_ext × πr²",
                                "C_ext = C_sca + C_abs = Q_ext × πr²"
                            ));
                            ui.end_row();

                            let geometric = std::f64::consts::PI * self.state.particle_radius.powi(2);
                            ui.horizontal(|ui| {
                                ui.label("Geometric (πr²):");
                                ui.label("ℹ️")
                                    .on_hover_text(&self.t(
                                        "Physical cross-sectional area of the particle. Compare with C_sca, C_abs, C_ext",
                                        "Área transversal física de la partícula. Comparar con C_sca, C_abs, C_ext"
                                    ));
                            });
                            ui.colored_label(
                                Color32::LIGHT_GRAY,
                                format!("{:.2}", geometric)
                            ).on_hover_text(&self.t(
                                "Reference area. If C > πr², particle interacts more than its physical size",
                                "Área de referencia. Si C > πr², la partícula interactúa más que su tamaño físico"
                            ));
                            ui.end_row();
                        });
                });

            ui.add_space(12.0);

            // Validation Card
            let conservation_error = result.check_conservation();
            let error_msg = format!("Conservation error: {:.2e}", conservation_error);
            let (bg_color, text_color, icon, message) = if conservation_error < 1e-6 {
                (
                    Color32::from_rgb(40, 80, 50),
                    Color32::from_rgb(150, 255, 180),
                    "⚡",
                    "Energy conservation satisfied".to_string()
                )
            } else {
                (
                    Color32::from_rgb(100, 80, 40),
                    Color32::from_rgb(255, 220, 150),
                    "⚠",
                    error_msg
                )
            };

            egui::Frame::none()
                .fill(bg_color)
                .rounding(Rounding::same(6.0))
                .inner_margin(egui::Margin::same(10.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(icon);
                        ui.colored_label(text_color, message);
                    });
                });

        } else {
            // Empty state
            egui::Frame::none()
                .fill(Color32::from_rgb(40, 43, 53))
                .rounding(Rounding::same(8.0))
                .inner_margin(egui::Margin::same(20.0))
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(30.0);
                        ui.label(egui::RichText::new("🔬").size(48.0));
                        ui.add_space(10.0);
                        ui.label("No results yet");
                        ui.add_space(5.0);
                        ui.colored_label(
                            Color32::GRAY,
                            "Click 'Calculate' to compute optical properties"
                        );
                        ui.add_space(30.0);
                    });
                });
        }
    }

    fn draw_plot_panel(&mut self, ui: &mut egui::Ui) {
        ui.add_space(5.0);
        ui.horizontal(|ui| {
            ui.heading("📈 Optical Spectrum");
            ui.label("ℹ️")
                .on_hover_text(&self.t(
                    "Optical properties across wavelength range. Shows how particle interacts with different colors of light",
                    "Propiedades ópticas a lo largo del rango de longitud de onda. Muestra cómo la partícula interactúa con diferentes colores de luz"
                ));
        });
        ui.add_space(15.0);

        if self.spectrum_results.is_empty() {
            // Empty state for plot
            egui::Frame::none()
                .fill(Color32::from_rgb(40, 43, 53))
                .rounding(Rounding::same(8.0))
                .inner_margin(egui::Margin::same(20.0))
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(60.0);
                        ui.label(egui::RichText::new("📊").size(64.0));
                        ui.add_space(15.0);
                        ui.label(egui::RichText::new("No spectrum data").size(18.0));
                        ui.add_space(8.0);
                        ui.colored_label(
                            Color32::GRAY,
                            "Click 'Calculate Full Spectrum' to generate wavelength scan"
                        );
                        ui.add_space(60.0);
                    });
                });
            return;
        }

        // Statistics card
        let max_q_sca = self.spectrum_results.iter()
            .map(|r| r.q_sca)
            .fold(f64::NEG_INFINITY, f64::max);
        let max_q_abs = self.spectrum_results.iter()
            .map(|r| r.q_abs)
            .fold(f64::NEG_INFINITY, f64::max);
        
        egui::Frame::none()
            .fill(Color32::from_rgb(45, 48, 58))
            .rounding(Rounding::same(6.0))
            .inner_margin(egui::Margin::same(10.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("📊");
                    ui.strong("Spectrum Statistics:");
                    ui.separator();
                    ui.label(format!("Max Q_sca: {:.4}", max_q_sca));
                    ui.separator();
                    ui.label(format!("Max Q_abs: {:.4}", max_q_abs));
                    ui.separator();
                    ui.label(format!("{} points", self.spectrum_results.len()));
                });
            });

        ui.add_space(10.0);

        // Prepare plot data
        let q_sca_points: PlotPoints = self
            .spectrum_results
            .iter()
            .map(|r| [r.wavelength, r.q_sca])
            .collect();

        let q_abs_points: PlotPoints = self
            .spectrum_results
            .iter()
            .map(|r| [r.wavelength, r.q_abs])
            .collect();

        let q_ext_points: PlotPoints = self
            .spectrum_results
            .iter()
            .map(|r| [r.wavelength, r.q_ext])
            .collect();

        // Main plot
        // Contenedor con padding personalizado para el plot
        egui::Frame::none()
            .inner_margin(egui::Margin {
                left: 20.0,   // Más espacio a la izquierda para el eje Y
                right: 10.0,
                top: 10.0,
                bottom: 10.0,
            })
            .show(ui, |ui| {
                // Calcular los límites Y basados en los datos actuales
                let mut y_min = f64::INFINITY;
                let mut y_max = f64::NEG_INFINITY;
                
                for result in &self.spectrum_results {
                    let vals = [result.q_sca, result.q_abs, result.q_ext];
                    for &val in &vals {
                        if val.is_finite() {
                            y_min = y_min.min(val);
                            y_max = y_max.max(val);
                        }
                    }
                }
                
                // Agregar margen del 10% arriba y abajo
                if y_min.is_finite() && y_max.is_finite() && y_max > y_min {
                    let range = y_max - y_min;
                    let margin = range * 0.1;
                    y_min = (y_min - margin).max(0.0);
                    y_max = y_max + margin;
                } else {
                    // Valores por defecto si no hay datos válidos
                    y_min = 0.0;
                    y_max = 1.0;
                }
                
                // Main plot con ajuste automático robusto y límites
                let plot_id = format!("spectrum_plot_{}", self.plot_reset_counter);
                Plot::new(&plot_id)
                    .legend(Legend::default().position(Corner::RightTop))
                    .x_axis_label(&self.t("Wavelength (nm)", "Longitud de onda (nm)"))
                    .y_axis_label(&self.t("Efficiency Factor Q", "Factor de Eficiencia Q"))
                    .label_formatter(|name, value| {
                        format!("{}\nλ = {:.1} nm\nQ = {:.4}", name, value.x, value.y)
                    })
                    .y_axis_min_width(30.0)
                    .height(450.0)  // Altura fija para asegurar visibilidad
                    .include_x(300.0)  // Asegurar rango X completo
                    .include_x(800.0)
                    .include_y(y_min)  // Límites Y calculados
                    .include_y(y_max)
                    .set_margin_fraction([0.05, 0.1].into())  // Márgenes para no permitir zoom out excesivo
                    .allow_boxed_zoom(true)
                    .allow_drag(true)
                    .allow_zoom(true)
                    .show(ui, |plot_ui| {
                        plot_ui.line(
                            Line::new(q_sca_points)
                                .color(Color32::from_rgb(70, 160, 255))
                                .width(2.5)
                                .name(&self.t("Q_sca (Scattering)", "Q_sca (Dispersión)")),
                        );
                        plot_ui.line(
                            Line::new(q_abs_points)
                                .color(Color32::from_rgb(255, 120, 70))
                                .width(2.5)
                                .name(&self.t("Q_abs (Absorption)", "Q_abs (Absorción)")),
                        );
                        plot_ui.line(
                            Line::new(q_ext_points)
                                .color(Color32::from_rgb(100, 220, 140))
                                .width(2.5)
                                .name(&self.t("Q_ext (Extinction)", "Q_ext (Extinción)")),
                        );
                        
                        // Mark visible spectrum region
                        plot_ui.vline(egui_plot::VLine::new(380.0)
                            .color(Color32::from_rgba_premultiplied(150, 150, 255, 50))
                            .style(egui_plot::LineStyle::Dashed { length: 5.0 }));
                        plot_ui.vline(egui_plot::VLine::new(750.0)
                            .color(Color32::from_rgba_premultiplied(255, 150, 150, 50))
                            .style(egui_plot::LineStyle::Dashed { length: 5.0 }));
                    });
            });

        ui.add_space(5.0);
        ui.horizontal(|ui| {
            ui.colored_label(Color32::GRAY, "|");
            ui.colored_label(Color32::from_rgb(150, 150, 255), "380 nm");
            ui.label("-");
            ui.colored_label(Color32::from_rgb(255, 150, 150), "750 nm");
            ui.colored_label(Color32::GRAY, "| = Visible spectrum range");
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(&self.t("🔄 Reset View", "🔄 Restablecer Vista"))
                    .on_hover_text(&self.t(
                        "Reset zoom to show full spectrum",
                        "Restablecer zoom para mostrar el espectro completo"
                    ))
                    .clicked() 
                {
                    // Incrementar contador para forzar recreación del plot
                    self.plot_reset_counter += 1;
                }
                
                ui.separator();
                
                // Export buttons
                if ui.button(&self.t("💾 CSV", "💾 CSV"))
                    .on_hover_text(&self.t(
                        "Export spectrum data to CSV file",
                        "Exportar datos del espectro a archivo CSV"
                    ))
                    .clicked() 
                {
                    self.export_type = ExportType::CSV;
                    self.show_export_dialog = true;
                }
                
                if ui.button(&self.t("📄 JSON", "📄 JSON"))
                    .on_hover_text(&self.t(
                        "Export spectrum data to JSON file",
                        "Exportar datos del espectro a archivo JSON"
                    ))
                    .clicked() 
                {
                    self.export_type = ExportType::JSON;
                    self.show_export_dialog = true;
                }
                
                if ui.button(&self.t("🖼️ PNG", "🖼️ PNG"))
                    .on_hover_text(&self.t(
                        "Export plot as PNG image",
                        "Exportar gráfica como imagen PNG"
                    ))
                    .clicked() 
                {
                    self.export_type = ExportType::PNG;
                    self.show_export_dialog = true;
                }
            });
        });
    }
    
    fn export_csv(&mut self) {
        if self.spectrum_results.is_empty() {
            return;
        }
        
        self.add_log(&self.t("💾 Exporting CSV...", "💾 Exportando CSV..."));
        
        let mut csv_content = String::from("Wavelength (nm),Q_sca,Q_abs,Q_ext\n");
        for result in &self.spectrum_results {
            csv_content.push_str(&format!(
                "{},{},{},{}\n",
                result.wavelength, result.q_sca, result.q_abs, result.q_ext
            ));
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            use std::fs::File;
            use std::io::Write;
            use std::env;
            
            let filename = format!("{}.csv", self.export_filename);
            
            if let Ok(mut file) = File::create(&filename) {
                let _ = file.write_all(csv_content.as_bytes());
                if let Ok(current_dir) = env::current_dir() {
                    let full_path = current_dir.join(&filename);
                    let msg = format!("✅ CSV: {}", full_path.display());
                    self.add_log(&msg);
                } else {
                    self.add_log(&format!("✅ CSV: {}", filename));
                }
            } else {
                self.add_log(&self.t("❌ Error exporting CSV", "❌ Error exportando CSV"));
            }
        }
    }
    
    fn export_json(&mut self) {
        if self.spectrum_results.is_empty() {
            return;
        }
        
        self.add_log(&self.t("💾 Exporting JSON...", "💾 Exportando JSON..."));
        
        let json_data = serde_json::json!({
            "metadata": {
                "particle_radius_nm": self.state.particle_radius,
                "n_particle_real": self.state.n_particle_real,
                "n_particle_imag": self.state.n_particle_imag,
                "n_medium": self.state.n_medium,
                "wavelength_nm": self.state.wavelength
            },
            "spectrum_data": self.spectrum_results.iter().map(|r| {
                serde_json::json!({
                    "wavelength_nm": r.wavelength,
                    "q_sca": r.q_sca,
                    "q_abs": r.q_abs,
                    "q_ext": r.q_ext
                })
            }).collect::<Vec<_>>()
        });
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            use std::fs::File;
            use std::io::Write;
            use std::env;
            
            let filename = format!("{}.json", self.export_filename);
            
            if let Ok(mut file) = File::create(&filename) {
                if let Ok(json_string) = serde_json::to_string_pretty(&json_data) {
                    let _ = file.write_all(json_string.as_bytes());
                    if let Ok(current_dir) = env::current_dir() {
                        let full_path = current_dir.join(&filename);
                        let msg = format!("✅ JSON: {}", full_path.display());
                        self.add_log(&msg);
                    } else {
                        self.add_log(&format!("✅ JSON: {}", filename));
                    }
                } else {
                    self.add_log(&self.t("❌ Error serializing JSON", "❌ Error serializando JSON"));
                }
            } else {
                self.add_log(&self.t("❌ Error exporting JSON", "❌ Error exportando JSON"));
            }
        }
    }

    fn draw_about_dialog(&mut self, ctx: &Context) {
        egui::Window::new(&self.t("About NanoCalc", "Acerca de NanoCalc"))
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.set_min_width(400.0);
                
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    
                    // App logo/title
                    ui.label(egui::RichText::new("NanoCalc")
                        .size(32.0)
                        .color(Color32::from_rgb(100, 180, 255))
                        .strong());
                    
                    ui.label(egui::RichText::new(self.t(
                        "Nanoscale Optical Properties Calculator",
                        "Calculadora de Propiedades Ópticas Nanoscópicas"
                    ))
                        .size(14.0)
                        .color(Color32::GRAY));
                    
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(15.0);
                });
                
                // Version info
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(self.t("Version:", "Versión:")).strong());
                    ui.label("0.1.0");
                });
                
                ui.add_space(8.0);
                
                // Developer info
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(self.t("Developer:", "Desarrollador:")).strong());
                    ui.label(egui::RichText::new("Yafel G.H.")
                        .color(Color32::from_rgb(100, 180, 255)));
                });
                
                ui.add_space(8.0);
                
                // License
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(self.t("License:", "Licencia:")).strong());
                    ui.label("MIT License © 2025");
                });
                
                ui.add_space(15.0);
                
                // Description
                ui.label(egui::RichText::new(self.t("Description:", "Descripción:")).strong());
                ui.add_space(5.0);
                ui.label(self.t(
                    "Open-source application for calculating optical, thermal, and electronic properties of nanomaterials using Mie scattering theory and advanced physics models.",
                    "Aplicación de código abierto para calcular propiedades ópticas, térmicas y electrónicas de nanomateriales usando la teoría de dispersión de Mie y modelos físicos avanzados."
                ));
                
                ui.add_space(20.0);
                ui.separator();
                ui.add_space(10.0);
                
                // Close button
                ui.vertical_centered(|ui| {
                    if ui.button(egui::RichText::new(&self.t("Close", "Cerrar")).size(14.0))
                        .on_hover_text(&self.t("Close this dialog", "Cerrar este diálogo")).clicked() {
                        self.show_about = false;
                    }
                });
                
                ui.add_space(5.0);
            });
    }

    fn draw_periodic_table(&mut self, ctx: &Context) {
        egui::Window::new(self.t(
            "Periodic Table - Element Selector",
            "Tabla Periódica - Selector de Elementos"
        ))
            .collapsible(false)
            .resizable(true)
            .default_width(950.0)
            .default_height(600.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(self.t(
                        "Select an element to view its properties:",
                        "Seleccione un elemento para ver sus propiedades:"
                    ))
                        .size(14.0)
                        .color(Color32::GRAY));
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(&self.t("Close", "Cerrar")).clicked() {
                            self.show_periodic_table = false;
                        }
                    });
                });
                
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
                
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        // Periodic table layout (simplified version with most common elements)
                        let elements = [
                            // Row 1
                            vec![("H", 1, "Hydrogen"), ("", 0, ""), ("", 0, ""), ("", 0, ""), ("", 0, ""), ("", 0, ""), 
                                 ("", 0, ""), ("", 0, ""), ("", 0, ""), ("", 0, ""), ("", 0, ""), ("", 0, ""), 
                                 ("", 0, ""), ("", 0, ""), ("", 0, ""), ("", 0, ""), ("", 0, ""), ("He", 2, "Helium")],
                            // Row 2
                            vec![("Li", 3, "Lithium"), ("Be", 4, "Beryllium"), ("", 0, ""), ("", 0, ""), ("", 0, ""), ("", 0, ""),
                                 ("", 0, ""), ("", 0, ""), ("", 0, ""), ("", 0, ""), ("", 0, ""), ("", 0, ""),
                                 ("B", 5, "Boron"), ("C", 6, "Carbon"), ("N", 7, "Nitrogen"), ("O", 8, "Oxygen"), 
                                 ("F", 9, "Fluorine"), ("Ne", 10, "Neon")],
                            // Row 3
                            vec![("Na", 11, "Sodium"), ("Mg", 12, "Magnesium"), ("", 0, ""), ("", 0, ""), ("", 0, ""), ("", 0, ""),
                                 ("", 0, ""), ("", 0, ""), ("", 0, ""), ("", 0, ""), ("", 0, ""), ("", 0, ""),
                                 ("Al", 13, "Aluminum"), ("Si", 14, "Silicon"), ("P", 15, "Phosphorus"), ("S", 16, "Sulfur"), 
                                 ("Cl", 17, "Chlorine"), ("Ar", 18, "Argon")],
                            // Row 4
                            vec![("K", 19, "Potassium"), ("Ca", 20, "Calcium"), ("Sc", 21, "Scandium"), ("Ti", 22, "Titanium"),
                                 ("V", 23, "Vanadium"), ("Cr", 24, "Chromium"), ("Mn", 25, "Manganese"), ("Fe", 26, "Iron"),
                                 ("Co", 27, "Cobalt"), ("Ni", 28, "Nickel"), ("Cu", 29, "Copper"), ("Zn", 30, "Zinc"),
                                 ("Ga", 31, "Gallium"), ("Ge", 32, "Germanium"), ("As", 33, "Arsenic"), ("Se", 34, "Selenium"),
                                 ("Br", 35, "Bromine"), ("Kr", 36, "Krypton")],
                            // Row 5
                            vec![("Rb", 37, "Rubidium"), ("Sr", 38, "Strontium"), ("Y", 39, "Yttrium"), ("Zr", 40, "Zirconium"),
                                 ("Nb", 41, "Niobium"), ("Mo", 42, "Molybdenum"), ("Tc", 43, "Technetium"), ("Ru", 44, "Ruthenium"),
                                 ("Rh", 45, "Rhodium"), ("Pd", 46, "Palladium"), ("Ag", 47, "Silver"), ("Cd", 48, "Cadmium"),
                                 ("In", 49, "Indium"), ("Sn", 50, "Tin"), ("Sb", 51, "Antimony"), ("Te", 52, "Tellurium"),
                                 ("I", 53, "Iodine"), ("Xe", 54, "Xenon")],
                            // Row 6 (simplified)
                            vec![("Cs", 55, "Cesium"), ("Ba", 56, "Barium"), ("La", 57, "Lanthanum"), ("", 0, ""), ("", 0, ""), ("", 0, ""),
                                 ("", 0, ""), ("", 0, ""), ("", 0, ""), ("", 0, ""), ("", 0, ""), ("", 0, ""), ("", 0, ""), ("", 0, ""),
                                 ("Hf", 72, "Hafnium"), ("Ta", 73, "Tantalum"), ("W", 74, "Tungsten"), ("Re", 75, "Rhenium")],
                            // Row 7 (metals)
                            vec![("Os", 76, "Osmium"), ("Ir", 77, "Iridium"), ("Pt", 78, "Platinum"), ("Au", 79, "Gold"),
                                 ("Hg", 80, "Mercury"), ("Tl", 81, "Thallium"), ("Pb", 82, "Lead"), ("Bi", 83, "Bismuth"),
                                 ("", 0, ""), ("", 0, ""), ("", 0, ""), ("", 0, ""), ("", 0, ""), ("", 0, ""),
                                 ("", 0, ""), ("", 0, ""), ("", 0, ""), ("", 0, "")],
                        ];
                        
                        for row in &elements {
                            ui.horizontal(|ui| {
                                for (symbol, atomic_num, name) in row {
                                    if !symbol.is_empty() && *atomic_num > 0 {
                                        let button = egui::Button::new(
                                            egui::RichText::new(format!("{}\n{}", symbol, atomic_num))
                                                .size(11.0)
                                        )
                                        .min_size(egui::vec2(45.0, 45.0));
                                        
                                        if ui.add(button)
                                            .on_hover_text(format!("{} (Z={})", name, atomic_num))
                                            .clicked() {
                                            self.selected_element = Some(Self::get_element_properties(symbol, name, *atomic_num));
                                            self.show_element_properties = true;
                                            self.show_periodic_table = false;
                                        }
                                    } else {
                                        // Empty space
                                        ui.add_space(47.0);
                                    }
                                }
                            });
                            ui.add_space(2.0);
                        }
                        
                        ui.add_space(15.0);
                        ui.separator();
                        ui.add_space(10.0);
                        
                        // Instructions
                        ui.label(egui::RichText::new(self.t("Note:", "Nota:"))
                            .color(Color32::from_rgb(100, 180, 255))
                            .strong());
                        ui.label(self.t(
                            "Click on any element to view its properties. Future versions will allow direct material property assignment.",
                            "Haga clic en cualquier elemento para ver sus propiedades. Las versiones futuras permitirán asignar propiedades de materiales directamente."
                        ));
                    });
            });
    }

    fn draw_element_properties(&mut self, ctx: &Context) {
        if let Some(element) = self.selected_element.clone() {
            egui::Window::new(&self.t(
                &format!("Element Properties - {}", element.symbol),
                &format!("Propiedades del Elemento - {}", element.symbol)
            ))
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.set_min_width(450.0);
                    
                    ui.vertical_centered(|ui| {
                        ui.add_space(10.0);
                        
                        // Element header
                        ui.label(egui::RichText::new(&element.symbol)
                            .size(48.0)
                            .color(Color32::from_rgb(100, 180, 255))
                            .strong());
                        
                        ui.label(egui::RichText::new(&element.name)
                            .size(20.0)
                            .color(Color32::GRAY));
                        
                        ui.label(egui::RichText::new(&format!("{}: {}",
                            self.t("Atomic Number", "Número Atómico"),
                            element.atomic_number
                        ))
                            .size(14.0)
                            .color(Color32::GRAY));
                        
                        ui.add_space(20.0);
                        ui.separator();
                        ui.add_space(15.0);
                    });
                    
                    // Optical properties section
                    ui.label(egui::RichText::new(&self.t(
                        "Optical Properties (@ 550 nm):",
                        "Propiedades Ópticas (@ 550 nm):"
                    )).strong().size(16.0));
                    
                    ui.add_space(10.0);
                    
                    egui::Grid::new("element_props_grid")
                        .num_columns(2)
                        .spacing([20.0, 12.0])
                        .show(ui, |ui| {
                            ui.label(egui::RichText::new(&self.t(
                                "Refractive Index (Real):",
                                "Índice de Refracción (Real):"
                            )).size(14.0));
                            ui.label(egui::RichText::new(&format!("{:.3}", element.n_real))
                                .size(14.0)
                                .color(Color32::from_rgb(100, 255, 150)));
                            ui.end_row();
                            
                            ui.label(egui::RichText::new(&self.t(
                                "Refractive Index (Imaginary):",
                                "Índice de Refracción (Imaginaria):"
                            )).size(14.0));
                            ui.label(egui::RichText::new(&format!("{:.3}", element.n_imag))
                                .size(14.0)
                                .color(Color32::from_rgb(100, 255, 150)));
                            ui.end_row();
                        });
                    
                    ui.add_space(15.0);
                    
                    // Info box
                    egui::Frame::none()
                        .fill(Color32::from_rgb(40, 43, 53))
                        .rounding(Rounding::same(6.0))
                        .inner_margin(egui::Margin::same(12.0))
                        .show(ui, |ui| {
                            ui.label(egui::RichText::new(&self.t(
                                "Note: These are approximate optical properties at 550 nm wavelength. Actual values may vary with wavelength and material form.",
                                "Nota: Estas son propiedades ópticas aproximadas a 550 nm de longitud de onda. Los valores reales pueden variar con la longitud de onda y la forma del material."
                            ))
                                .size(12.0)
                                .color(Color32::GRAY));
                        });
                    
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(15.0);
                    
                    // Action buttons
                    ui.horizontal(|ui| {
                        let apply_text = self.t(
                            "Apply Properties",
                            "Aplicar Propiedades"
                        );
                        let apply_tooltip = self.t(
                            "Apply these optical properties to the particle",
                            "Aplicar estas propiedades ópticas a la partícula"
                        );
                        
                        if ui.add_sized(
                            [200.0, 35.0],
                            egui::Button::new(egui::RichText::new(&apply_text).size(15.0))
                        )
                        .on_hover_text(&apply_tooltip)
                        .clicked()
                        {
                            self.state.n_particle_real = element.n_real;
                            self.state.n_particle_imag = element.n_imag;
                            self.show_element_properties = false;
                        }
                        
                        ui.add_space(10.0);
                        
                        let cancel_text = self.t("Cancel", "Cancelar");
                        if ui.add_sized(
                            [100.0, 35.0],
                            egui::Button::new(egui::RichText::new(&cancel_text).size(15.0))
                        )
                        .clicked()
                        {
                            self.show_element_properties = false;
                        }
                    });
                    
                    ui.add_space(10.0);
                });
        }
    }
    
    fn draw_export_dialog(&mut self, ctx: &Context) {
        let mut open = true;
        
        egui::Window::new(&self.t("Export Data", "Exportar Datos"))
            .collapsible(false)
            .resizable(false)
            .open(&mut open)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.set_min_width(400.0);
                
                ui.add_space(10.0);
                
                let export_label = match self.export_type {
                    ExportType::CSV => self.t("Export to CSV", "Exportar a CSV"),
                    ExportType::JSON => self.t("Export to JSON", "Exportar a JSON"),
                    ExportType::PNG => self.t("Export to PNG", "Exportar a PNG"),
                };
                
                ui.heading(export_label);
                ui.add_space(15.0);
                
                // Filename input
                ui.horizontal(|ui| {
                    ui.label(&self.t("Filename:", "Nombre del archivo:"));
                    ui.text_edit_singleline(&mut self.export_filename);
                });
                
                ui.add_space(5.0);
                
                // Show extension
                let extension = match self.export_type {
                    ExportType::CSV => ".csv",
                    ExportType::JSON => ".json",
                    ExportType::PNG => ".png",
                };
                
                ui.colored_label(
                    Color32::GRAY,
                    format!("{}: {}{}", 
                        self.t("Will be saved as", "Se guardará como"),
                        self.export_filename,
                        extension
                    )
                );
                
                ui.add_space(15.0);
                ui.separator();
                ui.add_space(10.0);
                
                // Buttons
                ui.horizontal(|ui| {
                    if ui.button(&self.t("Cancel", "Cancelar")).clicked() {
                        self.show_export_dialog = false;
                    }
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(&self.t("💾 Export", "💾 Exportar")).clicked() {
                            self.perform_export();
                            self.show_export_dialog = false;
                        }
                    });
                });
            });
        
        if !open {
            self.show_export_dialog = false;
        }
    }
    
    fn add_log(&mut self, message: &str) {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Format timestamp as HH:MM:SS (UTC)
        let secs = timestamp % 86400;
        let hours = (secs / 3600) % 24;
        let mins = (secs / 60) % 60;
        let secs = secs % 60;
        
        let log_entry = format!("[{:02}:{:02}:{:02}] {}", hours, mins, secs, message);
        self.log_messages.push(log_entry);
        
        // Keep only last 100 messages
        if self.log_messages.len() > 100 {
            self.log_messages.remove(0);
        }
    }
    
    fn perform_export(&mut self) {
        match self.export_type {
            ExportType::CSV => self.export_csv(),
            ExportType::JSON => self.export_json(),
            ExportType::PNG => self.export_png(),
        }
    }
    
    fn export_png(&mut self) {
        if self.spectrum_results.is_empty() {
            return;
        }
        
        #[cfg(feature = "export_png")]
        {
            use plotters::prelude::*;
            use std::env;
            
            self.add_log(&self.t("📊 Generating PNG plot...", "📊 Generando gráfica PNG..."));
            
            let filename = format!("{}.png", self.export_filename);
            
            // Create drawing area
            let root = BitMapBackend::new(&filename, (1200, 800)).into_drawing_area();
            root.fill(&WHITE).ok();
            
            // Find min/max values for proper scaling
            let mut y_min = f64::INFINITY;
            let mut y_max = f64::NEG_INFINITY;
            
            for result in &self.spectrum_results {
                y_min = y_min.min(result.q_sca).min(result.q_abs).min(result.q_ext);
                y_max = y_max.max(result.q_sca).max(result.q_abs).max(result.q_ext);
            }
            
            // Add 10% margin
            let margin = (y_max - y_min) * 0.1;
            y_min -= margin;
            y_max += margin;
            
            let mut chart = ChartBuilder::on(&root)
                .caption("Mie Scattering Spectrum", ("sans-serif", 40))
                .margin(20)
                .x_label_area_size(50)
                .y_label_area_size(70)
                .build_cartesian_2d(300.0..800.0, y_min..y_max)
                .ok();
            
            if let Some(ref mut chart) = chart {
                chart.configure_mesh()
                    .x_desc("Wavelength (nm)")
                    .y_desc("Efficiency Factor")
                    .draw()
                    .ok();
                
                // Draw Q_sca (blue)
                chart.draw_series(LineSeries::new(
                    self.spectrum_results.iter().map(|r| (r.wavelength, r.q_sca)),
                    &BLUE,
                )).ok()
                    .and_then(|series| {
                        series.label("Q_sca")
                            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
                        Some(())
                    });
                
                // Draw Q_abs (red)
                chart.draw_series(LineSeries::new(
                    self.spectrum_results.iter().map(|r| (r.wavelength, r.q_abs)),
                    &RED,
                )).ok()
                    .and_then(|series| {
                        series.label("Q_abs")
                            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
                        Some(())
                    });
                
                // Draw Q_ext (green)
                chart.draw_series(LineSeries::new(
                    self.spectrum_results.iter().map(|r| (r.wavelength, r.q_ext)),
                    &GREEN,
                )).ok()
                    .and_then(|series| {
                        series.label("Q_ext")
                            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));
                        Some(())
                    });
                
                chart.configure_series_labels()
                    .background_style(&WHITE.mix(0.8))
                    .border_style(&BLACK)
                    .draw()
                    .ok();
                
                root.present().ok();
                
                if let Ok(current_dir) = env::current_dir() {
                    let full_path = current_dir.join(&filename);
                    let msg = format!("✅ PNG: {}", full_path.display());
                    self.add_log(&msg);
                } else {
                    self.add_log(&format!("✅ PNG: {}", filename));
                }
            } else {
                self.add_log(&self.t("❌ Error creating PNG chart", "❌ Error creando gráfica PNG"));
            }
        }
        
        #[cfg(not(feature = "export_png"))]
        {
            self.add_log(&self.t("📸 PNG export requires plotters crate", "📸 Exportar PNG requiere crate plotters"));
        }
    }
}

impl eframe::App for NanoCalcApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Modern top panel with gradient-like effect
        TopBottomPanel::top("top_panel")
            .exact_height(70.0)
            .show(ctx, |ui| {
                egui::Frame::none()
                    .fill(Color32::from_rgb(25, 28, 35))
                    .inner_margin(egui::Margin::symmetric(15.0, 12.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // Logo/Icon
                            ui.label(egui::RichText::new("🔬").size(28.0)
                                .color(Color32::from_rgb(100, 180, 255))
                                .strong());
                            
                            ui.add_space(8.0);
                            
                            // Title
                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new("NanoCalc")
                                    .size(24.0)
                                    .color(Color32::from_rgb(100, 180, 255))
                                    .strong());
                                ui.label(egui::RichText::new(self.t(
                                    "Nanoscale Optical Properties Calculator",
                                    "Calculadora de Propiedades Ópticas Nanoscópicas"
                                ))
                                    .size(13.0)
                                    .color(Color32::GRAY));
                            });

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                // Version badge
                                egui::Frame::none()
                                    .fill(Color32::from_rgb(50, 80, 120))
                                    .rounding(Rounding::same(4.0))
                                    .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                                    .show(ui, |ui| {
                                        ui.label(egui::RichText::new("v0.1.0")
                                            .size(11.0)
                                            .color(Color32::from_rgb(180, 200, 255)));
                                    });
                                
                                ui.add_space(10.0);

                                // Language selector
                                egui::ComboBox::from_id_salt("language_selector")
                                    .selected_text(match self.language {
                                        Language::English => "EN",
                                        Language::Spanish => "ES",
                                    })
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.language, Language::English, "English");
                                        ui.selectable_value(&mut self.language, Language::Spanish, "Español");
                                    });
                                
                                ui.add_space(10.0);

                                // Periodic Table button
                                if ui.button(&self.t("Elements", "Elementos"))
                                    .on_hover_text(&self.t("Open Periodic Table", "Abrir Tabla Periódica"))
                                    .clicked() {
                                    self.show_periodic_table = true;
                                }
                                
                                ui.add_space(5.0);

                                // About button
                                if ui.button(&self.t("About", "Acerca de"))
                                    .on_hover_text(&self.t("About NanoCalc", "Acerca de NanoCalc"))
                                    .clicked() {
                                    self.show_about = true;
                                }
                            });
                        });
                    });
            });

        // Left sidebar with inputs
        SidePanel::left("input_panel")
            .exact_width(350.0)
            .resizable(false)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        self.draw_input_panel(ui);
                    });
            });

        // Main content area
        CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.horizontal_top(|ui| {
                        // Results panel
                        ui.vertical(|ui| {
                            ui.set_min_width(350.0);
                            ui.set_max_width(450.0);
                            self.draw_results_panel(ui);
                        });

                        ui.add_space(15.0);

                        // Plot panel (takes remaining space)
                        ui.vertical(|ui| {
                            self.draw_plot_panel(ui);
                        });
                    });
                });
        });

        // Bottom panel with activity log
        TopBottomPanel::bottom("bottom_panel")
            .exact_height(120.0)
            .show(ctx, |ui| {
                egui::Frame::none()
                    .fill(Color32::from_rgb(25, 28, 35))
                    .inner_margin(egui::Margin::symmetric(15.0, 8.0))
                    .show(ui, |ui| {
                        // Status line
                        ui.horizontal(|ui| {
                            ui.colored_label(Color32::GRAY, self.t(
                                "Model: Mie Scattering",
                                "Modelo: Dispersión de Mie"
                            ));
                            ui.separator();
                            if self.calculating {
                                ui.spinner();
                                ui.label(&self.t("Calculating...", "Calculando..."));
                            } else {
                                ui.colored_label(Color32::from_rgb(100, 255, 150), 
                                    &self.t("Ready", "Listo"));
                            }
                            
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button(&self.t("🗑 Clear Log", "🗑 Limpiar Log")).clicked() {
                                    self.log_messages.clear();
                                    self.log_messages.push(self.t("✅ Log cleared", "✅ Log limpiado").to_string());
                                }
                                ui.separator();
                                ui.hyperlink_to(&self.t("Documentation", "Documentación"), 
                                    "https://github.com/lexharden/nanocalc");
                                ui.separator();
                                ui.colored_label(Color32::GRAY, "MIT License © 2025");
                            });
                        });
                        
                        ui.add_space(4.0);
                        ui.separator();
                        ui.add_space(4.0);
                        
                        // Log panel
                        ui.label(egui::RichText::new(self.t("📋 Activity Log:", "📋 Registro de Actividad:"))
                            .color(Color32::from_rgb(150, 150, 150))
                            .size(11.0));
                        
                        egui::ScrollArea::vertical()
                            .max_height(60.0)
                            .auto_shrink([false, false])
                            .stick_to_bottom(true)
                            .show(ui, |ui| {
                                ui.style_mut().visuals.override_text_color = Some(Color32::from_rgb(200, 200, 200));
                                for msg in self.log_messages.iter().rev().take(50) {
                                    ui.label(egui::RichText::new(msg).size(11.0).font(egui::FontId::monospace(11.0)));
                                }
                            });
                    });
            });

        // Show About dialog if requested
        if self.show_about {
            self.draw_about_dialog(ctx);
        }

        // Show Periodic Table if requested
        if self.show_periodic_table {
            self.draw_periodic_table(ctx);
        }

        // Show Element Properties if requested
        if self.show_element_properties {
            self.draw_element_properties(ctx);
        }
        
        // Show Export dialog if requested
        if self.show_export_dialog {
            self.draw_export_dialog(ctx);
        }
    }
}