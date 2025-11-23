//! NanoCalc - Nanoscale Properties Calculator
//!
//! A scientific application for calculating optical, thermal, and electronic
//! properties of nanomaterials.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use nanocalc::gui::NanoCalcApp;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if RUST_LOG is set)

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_icon(load_icon()),
        ..Default::default()
    };

    eframe::run_native(
        "NanoCalc - Nanoscale Properties Calculator",
        options,
        Box::new(|cc| Ok(Box::new(NanoCalcApp::new(cc)))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect panic messages to console.error
    console_error_panic_hook::set_once();

    // Make sure panics are logged using console.error
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    wasm_bindgen_futures::spawn_local(async {
        let web_options = eframe::WebOptions::default();
        
        eframe::WebRunner::new()
            .start(
                "canvas",
                web_options,
                Box::new(|cc| Ok(Box::new(NanoCalcApp::new(cc)))),
            )
            .await
            .expect("Failed to start eframe");
    });
}

fn load_icon() -> egui::IconData {
    // For now, use a simple colored square as icon
    // In future, load from assets/icons/app_icon.png
    let (width, height) = (32, 32);
    let mut pixels = vec![0u8; width * height * 4];

    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) * 4;
            // Create a simple gradient
            pixels[idx] = ((x as f32 / width as f32) * 255.0) as u8;     // R
            pixels[idx + 1] = ((y as f32 / height as f32) * 255.0) as u8; // G
            pixels[idx + 2] = 200;                                         // B
            pixels[idx + 3] = 255;                                         // A
        }
    }

    egui::IconData {
        rgba: pixels,
        width: width as u32,
        height: height as u32,
    }
}
