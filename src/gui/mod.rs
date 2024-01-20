use serde::{Deserialize, Serialize};



#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct LatexTableGui {


}

impl Default for LatexTableGui {
    fn default() -> Self {
        Self {
            // ...
        }
    }
}


impl eframe::App for LatexTableGui {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        todo!()
    }
}