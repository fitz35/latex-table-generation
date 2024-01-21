use std::f32::INFINITY;

use wasm_bindgen::prelude::*;

use crate::gui::LatexTableGui;

mod gui;

/// Your handle to the web app from JavaScript.
#[derive(Clone)]
#[wasm_bindgen]
pub struct WebHandle {
    runner: eframe::WebRunner,
}

#[wasm_bindgen]
impl WebHandle {
    /// Installs a panic hook, then returns.
    #[allow(clippy::new_without_default)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Redirect [`log`] message to `console.log` and friends:
        eframe::WebLogger::init(log::LevelFilter::Debug).ok();

        Self {
            runner: eframe::WebRunner::new(),
        }
    }

    /// Call this once from JavaScript to start your app.
    #[wasm_bindgen]
    pub async fn start(&self, canvas_id: &str) -> Result<(), wasm_bindgen::JsValue> {
        let mut web_options = eframe::WebOptions::default();
        web_options.max_size_points  = egui::vec2(200.0, INFINITY);

        self.runner
            .start(
                canvas_id,
                web_options,
                Box::new(|cc| Box::new(LatexTableGui::new(cc))),
            )
            .await
    }

    // ----------------------- debug -----------------------

    /// The JavaScript can check whether or not your app has crashed:
    #[wasm_bindgen]
    pub fn has_panicked(&self) -> bool {
        self.runner.has_panicked()
    }

    #[wasm_bindgen]
    pub fn panic_message(&self) -> Option<String> {
        self.runner.panic_summary().map(|s| s.message())
    }

    #[wasm_bindgen]
    pub fn panic_callstack(&self) -> Option<String> {
        self.runner.panic_summary().map(|s| s.callstack())
    }

    /// Shut down eframe and clean up resources.
    #[wasm_bindgen]
    pub fn destroy(&self) {
        self.runner.destroy();
    }

    // ----------------------- link to the rust from js -----------------------

    /// Example on how to call into your app from JavaScript.
    #[wasm_bindgen]
    pub fn example(&self) {
        if let Some(app) = self.runner.app_mut::<LatexTableGui>() {
            
        }
    }

    
}