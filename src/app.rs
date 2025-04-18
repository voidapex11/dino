//! # Dino
//! This is my interpretation of the classic dinosaur game.
//! 
//! ## Structure
//! There will be a main menu, a screen befor the user starts the game, a screen for when the user
//! dies and a screen for after the player dies.

use egui::{Ui, Color32, Pos2, Sense, Painter, Key};
use epaint::{Mesh, Vertex};
use egui_demo_lib::easy_mark;
use rand::prelude::*;
use std::error;
use eframe::egui;
use crate::render;
use anyhow::Result;

#[derive(PartialEq)]
enum AppStatus {
    Menu,
    Credits,
    Settings,
    GameReadyToStart,
    PlayingGame,
    Died
}

#[derive(Copy, Clone, PartialEq)]
pub struct Enemy {
    pub start_x: f64,
    end_x: f64,
    image: usize,
    
    pub height: f64,
    can_duck: bool,
    ignore: bool,
}

impl Default for Enemy {
    fn default() -> Self {
        let mut rng = rand::rng();
        let e_type = rng.random_range(1..=2) as f64;
        Self {
            start_x: 1800.0,
            end_x: 1730.0+10.0*e_type,
            image: e_type as usize,
            height: e_type,
            can_duck: false,
            ignore: false,
        }
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct DinoGame {
    // Example stuff:
    label: String,

    dino_speed_y: f64,
    dino_y: f64,
    dino_distance: f64,

    dino_speed: f64,
    
    #[serde(skip)]
    enemys: Vec<Enemy>,

    #[serde(skip)]
    state: AppStatus,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for DinoGame {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            state: AppStatus::Menu,
            dino_y: 0.0,
            dino_speed_y: 0.0,
            dino_speed: 50.0,
            dino_distance: 0.0,
            enemys: Vec::new(),
        }
    }
}

impl DinoGame {

    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
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
            
            let play_button = ui.button("Play!");

            if play_button.clicked() {
                self.state = AppStatus::GameReadyToStart;
            };

            let credits_button = ui.button("Credits");

            if credits_button.clicked() {
                self.state = AppStatus::Credits;
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
    

    fn jump(&mut self) -> Result<()> {
        self.dino_speed_y -= self.dino_speed / 2.0;
        Ok(())
    }

    fn draw_dino(x: f64, y: f64, painter: Painter, ui: &mut Ui, ctx: &eframe::egui::Context) -> Result<()> {
        render::draw_dino(x, y, painter, ui, ctx);
        Ok(())
    }

    fn draw_enemy(enemy: Enemy, painter: &Painter) -> Result<()> {
        render::draw_enemy(enemy, painter);
        Ok(())
    }

    fn tick_game(&mut self, ui: &mut Ui) -> Result<()> {
        if self.dino_y < 100.0 {
            self.dino_speed_y += 1.0;
        } else {
            self.dino_y = 100.0;
            self.dino_speed_y = 0.0_f64.min(self.dino_speed_y);
        };
        self.dino_y = (100.0_f64).min(self.dino_y+self.dino_speed_y);
        
        self.dino_speed+=0.001;
        let mut kill = Vec::new();
        for enemy in self.enemys.iter_mut() {
            (*enemy).start_x -= self.dino_speed*0.5;
            (*enemy).end_x -= self.dino_speed*0.5;
            
            // if the enemy is off screen, remove it to save resources
            if (*enemy).end_x < -200.0 {
                (*enemy).ignore = true;
                kill.push(*enemy);
            }

            if ((self.dino_y+150.0+22.0) >= (250.0-6.5*((*enemy).height-1.0))) & (((*enemy).end_x) < 50.0) {
                self.state = AppStatus::Died;
            }
        }
        
        for to_rem in kill {
            if let Some(index) = self.enemys.iter().position(|value| *value == to_rem) {
                self.enemys.swap_remove(index);
            } 
        }
        
        let events = ui.input(|i| { i.clone() }).events.clone();
        for event in &events {
            match event {
                egui::Event::Key{key, ..} => {
                    if *key==Key::W || *key==Key::ArrowDown {
                        Self::jump(self)?;
                    }
                },
                egui::Event::Text(t) => {
                    if t=="W" || t==" " {
                        Self::jump(self)?;
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn update_game(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame, ui: &mut Ui) -> Result<()> {
        if ui.button("jump").clicked() {
            self.dino_speed_y = -20.0;
        };

        if ui.button("spawn").clicked() {
            self.enemys.push(Enemy::default())
        };
       
        if ui.button("clear").clicked() {
            self.enemys = Vec::new()
        };

        if ui.button("tick").clicked() {
            Self::tick_game(self, ui);
        };

        ui.add(egui::Slider::new(&mut self.dino_y, 0.0..=100.0).text("y"));
        ui.add(egui::Slider::new(&mut self.enemys.len(), 0..=10).text("enemys"));
        ui.heading("Dino Game");

                
        let (_, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

        for enemy in self.enemys.iter_mut() {
            if !(*enemy).ignore {
                Self::draw_enemy(*enemy, &painter)?;
            }
        }

        Self::draw_dino(30.0,self.dino_y+150.0, painter, ui, ctx)?;

        Ok(())
    }

    fn ready(&mut self, ui: &mut Ui) {
        ui.heading("ready?");
        ui.heading("skjdhaf");
        let events = ui.input(|i| { i.clone() }).events.clone();
        for event in &events {
            match event {
                egui::Event::Key{key, ..} => {
                    if *key==Key::W || *key==Key::ArrowDown {
                        self.state = AppStatus::PlayingGame;
                        let _ = Self::jump(self);
                    }
                },
                egui::Event::Text(t) => {
                    if t=="W" || t==" " {
                        self.state = AppStatus::PlayingGame;
                        let _ =Self::jump(self);
                    }
                }
                _ => {}
            }
        }
    }

    fn update_death(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame,ui: &mut Ui) -> Result<()> {
        ui.heading("You died, play again?");

        let events = ui.input(|i| { i.clone() }).events.clone();
        for event in &events {
            match event {
                egui::Event::Key{key, ..} => {
                    if *key==Key::W || *key==Key::ArrowDown {
                        self.enemys = Vec::new();
                        self.state = AppStatus::PlayingGame;
                        let _ = Self::jump(self);
                    }
                },
                egui::Event::Text(t) => {
                    if t=="W" || t==" " {
                        self.enemys = Vec::new();
                        self.state = AppStatus::PlayingGame;
                        let _ =Self::jump(self);
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}

impl eframe::App for DinoGame {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
 
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        
        
        match self.state { 
            AppStatus::PlayingGame => { 
                // Tell the backend to repaint as soon as possible 
                ctx.request_repaint(); 
            }
            _ => {}
        } 

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
                self.ready(ui);
                self.update_game(ctx, _frame, ui);
            } else if (self.state) == AppStatus::PlayingGame {
                self.tick_game(ui).unwrap();
                self.update_game(ctx, _frame, ui);
            } else if (self.state) == AppStatus::Died {
                self.update_death(ctx, _frame, ui).unwrap();
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
