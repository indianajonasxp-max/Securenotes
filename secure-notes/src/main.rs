mod crypto;
mod note;
mod storage;
mod ui;
mod map;
mod tile_loader;

use eframe::egui;
use ui::NotesApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Secure Notes - E2EE with Maps",
        options,
        Box::new(|cc| {
            // Setup custom fonts and style
            let mut style = (*cc.egui_ctx.style()).clone();
            style.spacing.item_spacing = egui::vec2(8.0, 8.0);
            cc.egui_ctx.set_style(style);
            
            Ok(Box::new(NotesApp::new(cc)))
        }),
    )
}
