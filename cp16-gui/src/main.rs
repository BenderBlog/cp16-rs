mod cp16_generator;

use eframe::egui;

use crate::cp16_generator::CP16Generator;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "CP-16 Generator",
        options,
        Box::new(|_cc| Ok(Box::<CP16Generator>::default())),
    )
}
