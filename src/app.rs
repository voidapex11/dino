//! # Dino
//! This is my interpretation of the classic dinosaur game.
//! 
//! ## Structure
//! There will be a main menu, a screen befor the user starts the game, a screen for when the user
//! dies and a screen for after the player dies.

use egui::{Ui, Sense, Painter, Key};
use egui_demo_lib::easy_mark;
use rand::prelude::*;
use std::path::Path;
use eframe::egui;
use crate::render;
use anyhow::Result;

pub fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage> {
    let image = image::ImageReader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

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
    
    #[serde(skip)]
    dino_speed_y: f64,
    
    #[serde(skip)]
    dino_y: f64,

    #[serde(skip)]
    dino_distance: f64,

    #[serde(skip)]
    dino_speed: f64,
    
    #[serde(skip)]
    enemys: Vec<Enemy>,

    #[serde(skip)]
    state: AppStatus,

    #[serde(skip)] // This how you opt-out of serialization of a field
    tick: i32,
    #[serde(skip)]
    cooldown: i32,

    #[serde(skip)]
    pub asset_map: Option<egui::TextureHandle>,
}

impl Default for DinoGame {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            tick: 0,
            state: AppStatus::GameReadyToStart,
            dino_y: 100.0,
            dino_speed_y: 0.0,
            dino_speed: 25.0,
            dino_distance: 0.0,
            cooldown: 20,
            enemys: Vec::new(),
            asset_map: None,
        }
    }
}

impl DinoGame {

    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        let mut temp: Self = Self { ..Default::default() };
        temp.asset_map=Some(cc.egui_ctx.load_texture(
            "asset_map",
            load_image_from_path(Path::new("asset-map.png")).unwrap(),
            egui::TextureOptions::default()));
        
        temp
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
        });
    }
    

    fn jump(&mut self) -> Result<()> {
        self.dino_speed_y -= 20.0;
        Ok(())
    }

    fn draw_dino_rest(&mut self, x: f64, y: f64, painter: Painter, ui: &mut Ui, ctx: &eframe::egui::Context) -> Result<()> {
        render::draw_dino_rest_state(self, x*(render::size as f64), y*(render::size as f64), painter, ui, ctx)?;
        Ok(())
    }

    fn draw_dino(&mut self, mut x: f64, mut y: f64, painter: &Painter, ui: &mut Ui, ctx: &eframe::egui::Context) -> Result<()> {
        x*=render::size as f64;
        y*=render::size as f64;
        if self.dino_y != 100.0 || self.state == AppStatus::Died {
            render::draw_dino_still(self, x, y, painter.clone(), ui, ctx)?;
            return Ok(())
        }

        if ((self.tick - (self.tick % 7)) %2)==0 {
            render::draw_dino_right(self, x, y, painter.clone(), ui, ctx)?;
        } else {
            render::draw_dino_left(self, x, y, painter.clone(), ui, ctx)?;
        }
        Ok(())
    }

    fn draw_enemy(&mut self, enemy: Enemy, painter: Painter, ui: &mut Ui, ctx: &eframe::egui::Context) -> Result<()> {
        //render::draw_enemy(enemy, &painter)?;
        render::draw_cacti_small(self, enemy.start_x*(render::size as f64), 271.0*(render::size as f64), &painter, ui, ctx)?;
        Ok(())
    }

    fn tick_game(&mut self, ui: &mut Ui) -> Result<()> {
        self.dino_speed+=0.02;
        
        //enemy spawning
        if self.tick > 0 || self.dino_y < 100.0 {
            self.tick += 1;
            self.dino_distance+=self.dino_speed*0.3;
            
            if self.cooldown==0 {
                let mut rng = rand::rng();
                let chance = rng.random_range(1..=1300);
                if chance <= 31 || self.enemys.is_empty() && chance <= 100 {
                    self.enemys.push(Enemy::default());
                    self.cooldown=32;
                }
            }
        }
        if self.cooldown!=0 {
            self.cooldown-=1;
        }

        // gravity
        if self.dino_y < 100.0 {
            self.dino_speed_y+= 1.2;
        } else {
            self.dino_y = 100.0;
            self.dino_speed_y = 0.0_f64.min(self.dino_speed_y);
        };
        self.dino_y = (100_f64).min(self.dino_y+self.dino_speed_y);
        
        self.dino_speed+=0.006;

        let mut kill = Vec::new();
        for enemy in self.enemys.iter_mut() {
            enemy.start_x -= self.dino_speed*0.3;
            enemy.end_x -= self.dino_speed*0.3;
            
            // if the enemy is off screen, remove it to save resources
            if enemy.end_x < -20.0 {
                enemy.ignore = true;
                kill.push(*enemy);
            }

            if (self.dino_y >= 64.0) & ((enemy.start_x < 100.0) & (enemy.end_x >= 0.0)) {
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

    fn update_game(
        &mut self,
        ctx: &eframe::egui::Context,
        _frame: &mut eframe::Frame,
        ui: &mut Ui
    ) -> Result<()> {
        ui.heading("Dino Game");

                
        let (_, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());
        
        // scoreboard
        render::draw_numbers(((self.dino_distance/85.0) as i32).to_string(), self, 800.0*(render::size as f64), 210.0* render::size as f64, &painter.clone(), ui, &mut ctx.clone())?;

        for enemy in (self.enemys).clone().iter_mut() {
            if !enemy.ignore {
                Self::draw_enemy(self,*enemy, painter.clone(), ui, ctx)?;
            }
        }

        if self.state == AppStatus::PlayingGame || self.state == AppStatus::Died {
            Self::draw_dino(self, 30.0, self.dino_y+ 150.0, &painter.clone(), ui, ctx)?;
        } else {
            Self::draw_dino_rest(self, 30.0, self.dino_y+150.0, painter.clone(), ui, ctx)?;
        }

        if self.tick > 0 {
            render::draw_floor(self, ((30.0+2400.0-self.dino_distance%2400.0-20.0)*(render::size as f64)).into(), (324.0*render::size).into(), &painter.clone(), ui, ctx)?;
            render::draw_floor(self, ((30.0-self.dino_distance%2400.0)*(render::size as f64)).into(), (324.0*render::size).into(), &painter.clone(), ui, ctx)?;
        }

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

    fn update_death(
        &mut self,
        _ctx: &eframe::egui::Context,
        _frame: &mut eframe::Frame, 
        ui: &mut Ui
    ) -> Result<()> {
        ui.heading("You died, play again?");

        let events = ui.input(|i| { i.clone() }).events.clone();
        for event in &events {
            match event {
                egui::Event::Key{key, ..} => {
                    if *key==Key::W || *key==Key::ArrowDown {
                        *self = DinoGame::default();
                        self.state= AppStatus::PlayingGame;
                        let _ = Self::jump(self);
                    }
                },
                egui::Event::Text(t) => {
                    if t=="W" || t==" " {
                        *self = DinoGame::default();
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
        
        
        if self.state == AppStatus::PlayingGame { 
            // Tell the backend to repaint as soon as possible 
            ctx.request_repaint(); 
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
                self.update_game(ctx, _frame, ui).unwrap();
            } else if (self.state) == AppStatus::PlayingGame {
                self.tick_game(ui).unwrap();
                self.update_game(ctx, _frame, ui).unwrap();
            } else if (self.state) == AppStatus::Died {
                self.update_death(ctx, _frame, ui).unwrap();
                self.update_game(ctx, _frame, ui).unwrap();
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
