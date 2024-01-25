use serde::{Deserialize, Serialize};

use crate::table::Table;

#[derive(Serialize, Deserialize)]
pub struct LatexTableGui {
    table: Table,
}

impl Default for LatexTableGui {
    // initialize with default values
    fn default() -> Self {
        Self {
            table: Table::default(),
        }
    }
}

impl LatexTableGui {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}


impl eframe::App for LatexTableGui {
    fn update(
        &mut self, 
        ctx: &egui::Context, 
        frame: &mut eframe::Frame
    ) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
            if ui.button("Generate dummy").clicked() {
                println!("Generate dummy table");
                self.table = Table::dummy();
            }
        });
    }
}