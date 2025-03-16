//! # Dino
//! This is my interpretation of the classic dinosaur game.
//! 
//! ## Structure
//! There will be a main menu, a screen befor the user starts the game, a screen for when the user
//! dies and a screen for after the player dies.

use egui::{Ui, Color32, Pos2, Sense, Painter};
use epaint::{Mesh, Vertex};
use egui_demo_lib::easy_mark;

#[derive(PartialEq)]
enum AppStatus {
    Menu,
    Credits,
    Settings,
    GameReadyToStart,
    PlayingGame,
    Died
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)]
    state: AppStatus,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            state: AppStatus::GameReadyToStart
        }
    }
}

impl TemplateApp {
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

    /// Displays the main menu
    fn update_menu(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
        // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Dinosaur game");

                
            ui.label("Write somethingghasdfasdf: ");
            ui.text_edit_singleline(&mut self.label);
                

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
                
            let play_button = ui.button("Play!");

            if play_button.clicked() {
                self.state = AppStatus::GameReadyToStart;
            };

            let credits_button = ui.button("Credits");

            if credits_button.clicked() {
                self.state = AppStatus::Credits;
            };

            if ui.button("Increment").clicked() {
                self.value += 1.0;
            };
        });
    }
    
    /// Displays the credits
    fn update_credits(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame, ui: &mut Ui) {
         ui.vertical_centered(|ui| {
            ui.heading("Credits");
            easy_mark::easy_mark(ui, "# Made by voidapex11"); // using markup as well as
                                                              // programaticaly constructing is
                                                              // because why not
            //ui.label("");
         });
    }

    fn draw_dino(x: f64, y: f64, painter: Painter) {
        let mesh = Mesh::default();
        let color = Color32::from_rgb(200,100,0);

    }

    fn update_game(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame, ui: &mut Ui) {
        let (response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

        ui.heading("Now Playing!");
        /*let start = Pos2::new(50.0,50.0);
        let end = Pos2::new(150.0,150.0);
        painter.line_segment([start,end],Stroke::new(10.0, Color32::RED));

        
        // Draw a blue filled circle
        let center = Pos2::new(100.0, 100.0);
        painter.circle_filled(center, 50.0, Color32::BLUE);

        // Draw a green rectangle with rounded corners
        let rect = Rect::from_min_size(Pos2::new(50.0, 150.0), egui::vec2(150.0, 75.0));
        painter.rect_filled(rect, 10.0, Color32::GREEN);
        */
        let mut mesh = Mesh::default();
    
    // Define the color for the triangle
    let color = Color32::from_rgb(200, 100, 100);
    
    // Create vertices with positions, colors, and UV coordinates
    // (The UVs here are arbitrary since we’re not texturing the shape)
    let v0 = Vertex { pos: Pos2::new(50.0, 50.0), color, uv: Pos2::new(0.0, 0.0)};
    let v1 = Vertex { pos: Pos2::new(150.0, 50.0), color, uv: Pos2::new(0.0, 0.0)};
    let v2 = Vertex { pos: Pos2::new(100.0, 150.0), color, uv: Pos2::new(0.0, 0.0)};
    let v3 = Vertex { pos: Pos2::new(150.0, 150.0), color, uv: Pos2::new(0.0, 0.0)};
    
    // Push vertices into the mesh
    mesh.vertices.push(v0);
    mesh.vertices.push(v1);
    mesh.vertices.push(v2);
    mesh.vertices.push(v3);

    // Define the triangle by referencing vertex indices (order matters for the winding)
    mesh.indices.extend_from_slice(&[0, 1, 2, 1, 2, 3]);
    
    // Add the custom mesh to the painter
    painter.add(egui::epaint::Shape::mesh(mesh));
        
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
 
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if (self.state) == AppStatus::Menu {
                self.update_menu(ctx, _frame, ui);
            } else if (self.state) == AppStatus::Credits {
                self.update_credits(ctx, _frame, ui);
            } else if (self.state) == AppStatus::GameReadyToStart {
                self.update_game(ctx, _frame, ui);
            } else{
                ui.label("Invalid app state");
            }

            ui.separator();

            
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                ui.add(egui::github_link_file!(
                    "https://github.com/emilk/eframe_template/blob/main/",
                    "Source code."
                ));
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
