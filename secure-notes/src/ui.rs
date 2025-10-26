use crate::map::{MapView, Router};
use crate::note::{GeoLocation, Note};
use crate::storage::SecureStorage;
use crate::tile_loader::TileCoord;
use eframe::egui;
use pulldown_cmark::{html, Parser};
use std::collections::HashMap;

pub struct NotesApp {
    storage: SecureStorage,
    password_input: String,
    unlock_error: Option<String>,
    
    // UI state
    selected_note_id: Option<String>,
    search_query: String,
    new_note_title: String,
    edit_content: String,
    edit_title: String,
    
    // View mode
    view_mode: ViewMode,
    show_markdown_preview: bool,
    
    // Map state
    map_view: MapView,
    show_map: bool,
    show_route: bool,
    selected_locations: Vec<GeoLocation>,
    route_start: Option<GeoLocation>,
    route_end: Option<GeoLocation>,
    selecting_mode: SelectingMode,
    tile_textures: HashMap<TileCoord, egui::TextureHandle>,
}

#[derive(PartialEq)]
enum ViewMode {
    List,
    Edit,
    View,
}

#[derive(PartialEq, Clone, Copy)]
enum SelectingMode {
    None,
    Start,
    End,
}

impl NotesApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            storage: SecureStorage::new(),
            password_input: String::new(),
            unlock_error: None,
            selected_note_id: None,
            search_query: String::new(),
            new_note_title: String::new(),
            edit_content: String::new(),
            edit_title: String::new(),
            view_mode: ViewMode::List,
            show_markdown_preview: false,
            map_view: MapView::new(),
            show_map: false,
            show_route: false,
            selected_locations: Vec::new(),
            route_start: None,
            route_end: None,
            selecting_mode: SelectingMode::None,
            tile_textures: HashMap::new(),
        }
    }

    fn render_unlock_screen(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                
                ui.heading("🔒 Secure Notes");
                ui.add_space(20.0);
                
                ui.label("End-to-End Encrypted Local Notes");
                ui.label("with OpenStreetMap Integration");
                ui.add_space(40.0);
                
                ui.horizontal(|ui| {
                    ui.label("Password:");
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut self.password_input)
                            .password(true)
                            .desired_width(250.0)
                    );
                    
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        self.attempt_unlock();
                    }
                });
                
                ui.add_space(15.0);
                
                // Show different buttons based on whether data exists
                ui.horizontal(|ui| {
                    if self.storage.has_existing_data() {
                        // Existing data - show unlock button
                        if ui.add_sized([200.0, 40.0], egui::Button::new("🔓 Unlock Existing Notes")).clicked() {
                            self.attempt_unlock();
                        }
                    } else {
                        // No existing data - show create button
                        if ui.add_sized([200.0, 40.0], egui::Button::new("✨ Create New Storage")).clicked() {
                            self.attempt_unlock();
                        }
                    }
                });
                
                if let Some(error) = &self.unlock_error {
                    ui.add_space(10.0);
                    ui.colored_label(egui::Color32::RED, error);
                }
                
                ui.add_space(20.0);
                
                if self.storage.has_existing_data() {
                    ui.label("📁 Existing encrypted notes found");
                    ui.label("Enter your password to unlock");
                } else {
                    ui.label("🆕 No existing notes found");
                    ui.label("Enter a password to create new encrypted storage");
                }
            });
        });
    }

    fn attempt_unlock(&mut self) {
        match self.storage.unlock(&self.password_input) {
            Ok(_) => {
                self.unlock_error = None;
                self.password_input.clear();
            }
            Err(e) => {
                self.unlock_error = Some(format!("Failed to unlock: {}", e));
            }
        }
    }

    fn render_main_ui(&mut self, ctx: &egui::Context) {
        // Top panel with toolbar
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("📝 Secure Notes");
                
                ui.separator();
                
                if ui.button("➕ New Note").clicked() {
                    self.create_new_note();
                }
                
                ui.separator();
                
                ui.label("🔍");
                ui.add(
                    egui::TextEdit::singleline(&mut self.search_query)
                        .hint_text("Search notes...")
                        .desired_width(200.0)
                );
                
                ui.separator();
                
                if ui.button(if self.show_map { "📝 Notes" } else { "🗺️ Map" }).clicked() {
                    self.show_map = !self.show_map;
                }
                
                if self.show_map {
                    ui.separator();
                    
                    if self.route_start.is_some() || self.route_end.is_some() || self.show_route {
                        if ui.button("🗑️ Clear / Restart").clicked() {
                            self.route_start = None;
                            self.route_end = None;
                            self.show_route = false;
                            self.selected_locations.clear();
                            self.selecting_mode = SelectingMode::None;
                        }
                    }
                    
                    if ui.button("🌍 Sample: DK→DE").clicked() {
                        self.show_route = true;
                        self.selected_locations = Router::get_sample_route();
                        self.route_start = None;
                        self.route_end = None;
                    }
                }
            });
        });

        // Main content
        if self.show_map {
            self.render_map_view(ctx);
        } else {
            // Sidebar with notes list
            egui::SidePanel::left("notes_list")
                .default_width(250.0)
                .show(ctx, |ui| {
                    self.render_notes_list(ui);
                });

            // Main panel with note content
            egui::CentralPanel::default().show(ctx, |ui| {
                self.render_note_content(ui);
            });
        }
    }

    fn render_notes_list(&mut self, ui: &mut egui::Ui) {
        ui.heading("Notes");
        ui.separator();

        let notes = if self.search_query.is_empty() {
            self.storage.get_all_notes()
        } else {
            self.storage.search_notes(&self.search_query)
        };

        egui::ScrollArea::vertical().show(ui, |ui| {
            for note in notes {
                let is_selected = self.selected_note_id.as_ref() == Some(&note.id);
                
                let response = ui.selectable_label(is_selected, &note.title);
                
                if response.clicked() {
                    self.selected_note_id = Some(note.id.clone());
                    self.edit_title = note.title.clone();
                    self.edit_content = note.content.clone();
                    self.view_mode = ViewMode::View;
                }
                
                ui.label(format!("📅 {}", note.modified_at.format("%Y-%m-%d %H:%M")));
                
                if let Some(loc) = &note.location {
                    ui.label(format!("📍 {}", loc.name));
                }
                
                if !note.tags.is_empty() {
                    ui.label(format!("🏷️ {}", note.tags.join(", ")));
                }
                
                ui.separator();
            }
        });
    }

    fn render_note_content(&mut self, ui: &mut egui::Ui) {
        if let Some(note_id) = &self.selected_note_id.clone() {
            if let Some(note) = self.storage.get_note(note_id).cloned() {
                ui.horizontal(|ui| {
                    ui.heading(&note.title);
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("🗑️ Delete").clicked() {
                            if let Err(e) = self.storage.delete_note(note_id) {
                                eprintln!("Failed to delete note: {}", e);
                            }
                            self.selected_note_id = None;
                            return;
                        }
                        
                        if self.view_mode == ViewMode::View {
                            if ui.button("✏️ Edit").clicked() {
                                self.view_mode = ViewMode::Edit;
                            }
                        } else if self.view_mode == ViewMode::Edit {
                            if ui.button("💾 Save").clicked() {
                                self.save_current_note();
                                self.view_mode = ViewMode::View;
                            }
                            if ui.button("❌ Cancel").clicked() {
                                self.edit_title = note.title.clone();
                                self.edit_content = note.content.clone();
                                self.view_mode = ViewMode::View;
                            }
                        }
                    });
                });

                ui.separator();

                if self.view_mode == ViewMode::Edit {
                    ui.horizontal(|ui| {
                        ui.label("Title:");
                        ui.text_edit_singleline(&mut self.edit_title);
                    });
                    
                    ui.separator();
                    
                    ui.horizontal(|ui| {
                        if ui.button(if self.show_markdown_preview { "Edit" } else { "Preview" }).clicked() {
                            self.show_markdown_preview = !self.show_markdown_preview;
                        }
                    });
                    
                    ui.separator();
                    
                    if self.show_markdown_preview {
                        self.render_markdown_preview(ui);
                    } else {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::multiline(&mut self.edit_content)
                                    .desired_width(f32::INFINITY)
                                    .desired_rows(20)
                                    .font(egui::TextStyle::Monospace)
                            );
                        });
                    }
                } else {
                    // View mode - render markdown
                    self.render_markdown_preview(ui);
                }

                ui.separator();
                
                // Location section
                if let Some(loc) = &note.location {
                    ui.label(format!("📍 Location: {}", loc.name));
                    ui.label(format!("   Coordinates: {:.4}, {:.4}", loc.latitude, loc.longitude));
                    
                    if ui.button("View on Map").clicked() {
                        self.map_view.set_center(loc.latitude, loc.longitude);
                        self.map_view.zoom = 12;
                        self.show_map = true;
                    }
                }

                // Quick location buttons
                ui.horizontal(|ui| {
                    if ui.button("Add Copenhagen").clicked() {
                        self.add_location_to_current_note(GeoLocation::copenhagen());
                    }
                    if ui.button("Add Berlin").clicked() {
                        self.add_location_to_current_note(GeoLocation::berlin());
                    }
                    if ui.button("Add Hamburg").clicked() {
                        self.add_location_to_current_note(GeoLocation::hamburg());
                    }
                });
            }
        } else {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                ui.heading("Select a note or create a new one");
            });
        }
    }

    fn render_markdown_preview(&self, ui: &mut egui::Ui) {
        let parser = Parser::new(&self.edit_content);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        
        egui::ScrollArea::vertical().show(ui, |ui| {
            // Simple markdown rendering
            for line in self.edit_content.lines() {
                if line.starts_with("# ") {
                    ui.heading(&line[2..]);
                } else if line.starts_with("## ") {
                    ui.label(egui::RichText::new(&line[3..]).heading().size(20.0));
                } else if line.starts_with("### ") {
                    ui.label(egui::RichText::new(&line[4..]).heading().size(16.0));
                } else if line.starts_with("- ") || line.starts_with("* ") {
                    ui.label(format!("  • {}", &line[2..]));
                } else if line.starts_with("**") && line.ends_with("**") && line.len() > 4 {
                    ui.label(egui::RichText::new(&line[2..line.len()-2]).strong());
                } else if line.starts_with("*") && line.ends_with("*") && line.len() > 2 {
                    ui.label(egui::RichText::new(&line[1..line.len()-1]).italics());
                } else if !line.is_empty() {
                    ui.label(line);
                } else {
                    ui.add_space(8.0);
                }
            }
        });
    }

    fn render_map_view(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🗺️ Interactive Map - Click to Route");
            
            ui.horizontal(|ui| {
                if ui.button("➕ Zoom In").clicked() {
                    self.map_view.zoom_in();
                }
                if ui.button("➖ Zoom Out").clicked() {
                    self.map_view.zoom_out();
                }
                ui.label(format!("Zoom: {}", self.map_view.zoom));
                
                ui.separator();
                
                // Quick location presets
                if ui.button("🇩🇰 Copenhagen").clicked() {
                    self.map_view.set_center(55.6761, 12.5683);
                    self.map_view.zoom = 11;
                }
                if ui.button("🇩🇪 Berlin").clicked() {
                    self.map_view.set_center(52.5200, 13.4050);
                    self.map_view.zoom = 11;
                }
                if ui.button("🇬🇧 London").clicked() {
                    self.map_view.set_center(51.5074, -0.1278);
                    self.map_view.zoom = 11;
                }
                if ui.button("🇫🇷 Paris").clicked() {
                    self.map_view.set_center(48.8566, 2.3522);
                    self.map_view.zoom = 11;
                }
                if ui.button("🇪🇸 Madrid").clicked() {
                    self.map_view.set_center(40.4168, -3.7038);
                    self.map_view.zoom = 11;
                }
            });
            
            // Status bar
            ui.horizontal(|ui| {
                match self.selecting_mode {
                    SelectingMode::Start => {
                        ui.colored_label(egui::Color32::BLUE, "🖱️ Click on map to set START point");
                    }
                    SelectingMode::End => {
                        ui.colored_label(egui::Color32::GREEN, "🖱️ Click on map to set END point");
                    }
                    SelectingMode::None => {
                        if self.route_start.is_none() {
                            ui.colored_label(egui::Color32::LIGHT_BLUE, "👆 Click anywhere on map to set your START point");
                        } else if self.route_end.is_none() {
                            ui.colored_label(egui::Color32::LIGHT_GREEN, "👆 Click on map to set your END point (route will auto-calculate)");
                        } else {
                            ui.label("✅ Route set! Click 'Clear Route' to start over or drag to explore");
                        }
                    }
                }
                
                ui.separator();
                
                if let Some(start) = &self.route_start {
                    ui.label(format!("📍 Start: {:.4}, {:.4}", start.latitude, start.longitude));
                }
                if let Some(end) = &self.route_end {
                    ui.label(format!("🎯 End: {:.4}, {:.4}", end.latitude, end.longitude));
                }
            });
            
            ui.separator();
            
            // Map canvas
            let available_size = ui.available_size();
            self.map_view.width = available_size.x;
            self.map_view.height = available_size.y;
            
            let (response, painter) = ui.allocate_painter(available_size, egui::Sense::click_and_drag());
            
            // Handle clicking to set points
            if response.clicked() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let screen_pos = pos - response.rect.left_top();
                    let (lat, lon) = self.map_view.screen_to_geo(screen_pos.x, screen_pos.y);
                    
                    match self.selecting_mode {
                        SelectingMode::Start => {
                            self.route_start = Some(GeoLocation::new(
                                lat,
                                lon,
                                format!("Start: {:.4}, {:.4}", lat, lon),
                            ));
                            self.selecting_mode = SelectingMode::None;
                            self.show_route = false;
                        }
                        SelectingMode::End => {
                            self.route_end = Some(GeoLocation::new(
                                lat,
                                lon,
                                format!("End: {:.4}, {:.4}", lat, lon),
                            ));
                            self.selecting_mode = SelectingMode::None;
                            self.show_route = false;
                        }
                        SelectingMode::None => {
                            // Auto-assign to start or end based on what's missing
                            if self.route_start.is_none() {
                                self.route_start = Some(GeoLocation::new(
                                    lat,
                                    lon,
                                    format!("Start: {:.4}, {:.4}", lat, lon),
                                ));
                            } else if self.route_end.is_none() {
                                self.route_end = Some(GeoLocation::new(
                                    lat,
                                    lon,
                                    format!("End: {:.4}, {:.4}", lat, lon),
                                ));
                                // Auto-calculate route when both points are set
                                self.calculate_route();
                            }
                        }
                    }
                }
            }
            
            // Handle dragging
            if response.dragged() {
                let delta = response.drag_delta();
                self.map_view.pan(delta.x, delta.y);
            }
            
            // Handle mouse scroll wheel for zooming
            let scroll_delta = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll_delta != 0.0 {
                if scroll_delta > 0.0 {
                    // Scroll up = zoom in
                    self.map_view.zoom_in();
                } else {
                    // Scroll down = zoom out
                    self.map_view.zoom_out();
                }
            }
            
            // Draw map background
            painter.rect_filled(
                response.rect,
                0.0,
                egui::Color32::from_rgb(170, 211, 223),
            );
            
            // Render OpenStreetMap tiles
            self.render_map_tiles(ui, &painter, response.rect, ctx);
            
            // Draw start point
            if let Some(start) = &self.route_start {
                let (screen_x, screen_y) = self.map_view.geo_to_screen(start.latitude, start.longitude);
                let pos = response.rect.left_top() + egui::vec2(screen_x, screen_y);
                
                // Draw pulsing start marker
                painter.circle_filled(pos, 10.0, egui::Color32::from_rgb(0, 150, 255));
                painter.circle_stroke(pos, 14.0, egui::Stroke::new(2.0, egui::Color32::BLUE));
                painter.text(
                    pos + egui::vec2(0.0, -20.0),
                    egui::Align2::CENTER_CENTER,
                    "📍 START",
                    egui::FontId::proportional(14.0),
                    egui::Color32::WHITE,
                );
            }
            
            // Draw end point
            if let Some(end) = &self.route_end {
                let (screen_x, screen_y) = self.map_view.geo_to_screen(end.latitude, end.longitude);
                let pos = response.rect.left_top() + egui::vec2(screen_x, screen_y);
                
                // Draw pulsing end marker
                painter.circle_filled(pos, 10.0, egui::Color32::from_rgb(255, 100, 0));
                painter.circle_stroke(pos, 14.0, egui::Stroke::new(2.0, egui::Color32::RED));
                painter.text(
                    pos + egui::vec2(0.0, -20.0),
                    egui::Align2::CENTER_CENTER,
                    "🎯 END",
                    egui::FontId::proportional(14.0),
                    egui::Color32::WHITE,
                );
            }
            
            // Draw route if enabled
            if self.show_route && !self.selected_locations.is_empty() {
                let mut points = Vec::new();
                
                for loc in &self.selected_locations {
                    let (screen_x, screen_y) = self.map_view.geo_to_screen(loc.latitude, loc.longitude);
                    let pos = response.rect.left_top() + egui::vec2(screen_x, screen_y);
                    points.push(pos);
                    
                    // Draw location marker
                    painter.circle_filled(pos, 5.0, egui::Color32::from_rgb(255, 200, 0));
                    painter.circle_stroke(pos, 7.0, egui::Stroke::new(1.0, egui::Color32::YELLOW));
                }
                
                // Draw route line
                for i in 0..points.len().saturating_sub(1) {
                    painter.line_segment(
                        [points[i], points[i + 1]],
                        egui::Stroke::new(4.0, egui::Color32::from_rgba_unmultiplied(0, 100, 200, 200)),
                    );
                }
                
                // Calculate and display total distance
                if self.selected_locations.len() >= 2 {
                    let mut total_distance = 0.0;
                    for i in 0..self.selected_locations.len() - 1 {
                        total_distance += Router::distance_km(
                            &self.selected_locations[i],
                            &self.selected_locations[i + 1]
                        );
                    }
                    
                    let info_pos = response.rect.left_bottom() + egui::vec2(10.0, -50.0);
                    
                    // Background box
                    painter.rect_filled(
                        egui::Rect::from_min_size(info_pos - egui::vec2(5.0, 35.0), egui::vec2(350.0, 40.0)),
                        4.0,
                        egui::Color32::from_rgba_unmultiplied(0, 0, 0, 180),
                    );
                    
                    painter.text(
                        info_pos - egui::vec2(0.0, 25.0),
                        egui::Align2::LEFT_BOTTOM,
                        format!("📏 Route: {} waypoints, {:.1} km total distance", 
                            self.selected_locations.len(), total_distance),
                        egui::FontId::proportional(16.0),
                        egui::Color32::WHITE,
                    );
                }
            }
            
            // Draw info overlay
            let info_pos = response.rect.left_top() + egui::vec2(10.0, 10.0);
            painter.rect_filled(
                egui::Rect::from_min_size(info_pos, egui::vec2(450.0, 50.0)),
                4.0,
                egui::Color32::from_rgba_unmultiplied(255, 255, 255, 230),
            );
            
            let instruction = if self.route_start.is_none() {
                "👆 CLICK anywhere on map to set START point"
            } else if self.route_end.is_none() {
                "👆 CLICK again to set END point (route auto-calculates!)"
            } else {
                "✅ Route ready! Drag to explore or click 'Clear' to restart"
            };
            
            painter.text(
                info_pos + egui::vec2(5.0, 5.0),
                egui::Align2::LEFT_TOP,
                format!("Zoom: {} | Center: {:.4}°, {:.4}°\n{}\nDrag to pan | Scroll to zoom | Use city buttons to jump",
                    self.map_view.zoom,
                    self.map_view.center_lat,
                    self.map_view.center_lon,
                    instruction
                ),
                egui::FontId::proportional(12.0),
                egui::Color32::BLACK,
            );
        });
    }

    fn create_new_note(&mut self) {
        let note = Note::new("New Note".to_string(), String::new());
        let note_id = note.id.clone();
        
        if let Err(e) = self.storage.add_note(note) {
            eprintln!("Failed to create note: {}", e);
            return;
        }
        
        self.selected_note_id = Some(note_id);
        self.edit_title = "New Note".to_string();
        self.edit_content = String::new();
        self.view_mode = ViewMode::Edit;
    }

    fn save_current_note(&mut self) {
        if let Some(note_id) = &self.selected_note_id {
            if let Some(note) = self.storage.get_note(note_id).cloned() {
                let mut updated_note = note;
                updated_note.update_title(self.edit_title.clone());
                updated_note.update_content(self.edit_content.clone());
                
                if let Err(e) = self.storage.update_note(updated_note) {
                    eprintln!("Failed to save note: {}", e);
                }
            }
        }
    }

    fn add_location_to_current_note(&mut self, location: GeoLocation) {
        if let Some(note_id) = &self.selected_note_id {
            if let Some(note) = self.storage.get_note(note_id).cloned() {
                let mut updated_note = note;
                updated_note.set_location(location);
                
                if let Err(e) = self.storage.update_note(updated_note) {
                    eprintln!("Failed to update note location: {}", e);
                }
            }
        }
    }
    
    fn render_map_tiles(&mut self, ui: &mut egui::Ui, painter: &egui::Painter, rect: egui::Rect, ctx: &egui::Context) {
        // Get visible tiles
        let tiles = self.map_view.get_visible_tiles();
        
        if let Some(tile_loader) = &self.map_view.tile_loader {
            for tile_coord in tiles {
                // Check if we already have this tile as a texture
                if !self.tile_textures.contains_key(&tile_coord) {
                    // Try to load the tile
                    if let Some(img) = tile_loader.get_tile(tile_coord) {
                        // Convert image to ColorImage
                        let size = [img.width() as usize, img.height() as usize];
                        let img_rgb = img.to_rgb8();
                        let pixels = img_rgb.as_flat_samples();
                        
                        let color_image = egui::ColorImage::from_rgb(
                            size,
                            pixels.as_slice(),
                        );
                        
                        // Create texture
                        let texture = ctx.load_texture(
                            format!("tile_{}_{}_{}",tile_coord.z, tile_coord.x, tile_coord.y),
                            color_image,
                            egui::TextureOptions::LINEAR,
                        );
                        
                        self.tile_textures.insert(tile_coord, texture);
                    }
                }
                
                // Draw the tile if we have it
                if let Some(texture) = self.tile_textures.get(&tile_coord) {
                    // Calculate tile position on screen
                    let tile_size = 256.0;
                    
                    // Get tile's top-left corner in world coordinates
                    let n = 2_f64.powi(tile_coord.z as i32);
                    let tile_lon = tile_coord.x as f64 / n * 360.0 - 180.0;
                    let tile_y_rad = (1.0 - 2.0 * tile_coord.y as f64 / n) * std::f64::consts::PI;
                    let tile_lat = tile_y_rad.sinh().atan().to_degrees();
                    
                    // Convert to screen coordinates
                    let (screen_x, screen_y) = self.map_view.geo_to_screen(tile_lat, tile_lon);
                    let tile_pos = rect.left_top() + egui::vec2(screen_x, screen_y);
                    
                    // Draw tile
                    let tile_rect = egui::Rect::from_min_size(
                        tile_pos,
                        egui::vec2(tile_size, tile_size),
                    );
                    
                    painter.image(
                        texture.id(),
                        tile_rect,
                        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                        egui::Color32::WHITE,
                    );
                }
            }
        }
        
        // Request repaint for tile loading
        ctx.request_repaint();
    }
    
    fn calculate_route(&mut self) {
        if let (Some(start), Some(end)) = (&self.route_start, &self.route_end) {
            // Use Router to calculate real road route via OSRM
            println!("🧭 Calculating route from ({}, {}) to ({}, {})", 
                start.latitude, start.longitude, end.latitude, end.longitude);
            
            let route_points = Router::calculate_route(start, end);
            
            // Convert RoutePoint to GeoLocation for rendering
            let mut route = Vec::new();
            for (i, point) in route_points.iter().enumerate() {
                route.push(GeoLocation::new(
                    point.lat,
                    point.lon,
                    if i == 0 {
                        "Start".to_string()
                    } else if i == route_points.len() - 1 {
                        "End".to_string()
                    } else {
                        format!("Waypoint {}", i)
                    },
                ));
            }
            
            self.selected_locations = route;
            self.show_route = true;
            
            // Center map between start and end
            let center_lat = (start.latitude + end.latitude) / 2.0;
            let center_lon = (start.longitude + end.longitude) / 2.0;
            self.map_view.set_center(center_lat, center_lon);
        }
    }
}

impl eframe::App for NotesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.storage.is_unlocked() {
            self.render_unlock_screen(ctx);
        } else {
            self.render_main_ui(ctx);
        }
    }
}
