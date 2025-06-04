//! # render
//! this 
use egui::{Ui, Color32, Pos2, Painter};
use epaint::Rect;
use anyhow::{Result, anyhow};
use crate::app::DinoGame;
use epaint::pos2;
use std::io::Cursor;
use image::ImageReader;
use log::debug;

pub const size: f32 = 0.8;

/// Renders part of the asset map to the painter
pub fn render(
    game: &mut DinoGame,
    x: f64,
    y: f64,
    painter: Painter,
    _ui: &mut Ui,
    ctx: &eframe::egui::Context,
    mut rx: f32, mut ry: f32,
    loc_size: f32,
    uv1: Pos2,
    uv2: Pos2
) -> Result<()> {
    if game.asset_map.is_none() {
        game.asset_map=Some(ctx.load_texture(
            "asset_map",
            { let raw_data = include_bytes!("asset-map.png");
            let image = (ImageReader::new(Cursor::new(raw_data))
                .with_guessed_format()
                .expect("Cursor io never fails")).decode()?;

            let img_size = [image.width() as _, image.height() as _];
            let image_buffer = image.to_rgba8();
            let pixels = image_buffer.as_flat_samples();
            egui::ColorImage::from_rgba_unmultiplied(
                img_size,
                pixels.as_slice(),
            ) },
            egui::TextureOptions::default()))
    } 
    let texture_id = egui::TextureId::from(&(game.asset_map.clone().expect("no texture id")));

    let x: f32 = x as f32;
    let y: f32 = y as f32;
    rx*=loc_size;
    ry*=loc_size;
    // 1517/120

    painter.image(//12.6666667
        texture_id,
        Rect::from_min_max(pos2(x, y), pos2(x+rx, y+ry)),
        egui::Rect::from_min_max(uv1, uv2),
        Color32::WHITE,
    );
    Ok(())
}

/// draws the dino at a given x and y
pub fn draw_dino_rest_state(game: &mut DinoGame, x: f64, y: f64, painter: Painter, ui: &mut Ui, ctx: &eframe::egui::Context) -> Result<()> {
    let rx: f32 = 80.0;
    let ry = 90.0;
    let scale = size*1.0;
    let uv1 = pos2(0.03, 0.0);
    let uv2 = pos2(0.068, 0.5);
    render(game, x, y, painter.clone(), ui, ctx, rx, ry, scale, uv1, uv2)?;
    Ok(())
}

pub fn draw_dino_left(game: &mut DinoGame, x: f64, y: f64, painter: Painter, ui: &mut Ui, ctx: &eframe::egui::Context) -> Result<()> {
    let rx: f32 = 88.0;
    let ry = 90.0;
    let scale = size*1.0;
    let uv1 = pos2(1854.0/2446.0, 0.0);
    let uv2 = pos2(1942.0/2446.0, 0.5);
    render(game, x, y, painter.clone(), ui, ctx, rx, ry, scale, uv1, uv2)?;
    Ok(())
}

pub fn draw_dino_right(game: &mut DinoGame, x: f64, y: f64, painter: Painter, ui: &mut Ui, ctx: &eframe::egui::Context) -> Result<()> {
    let rx: f32 = 88.0;
    let ry = 90.0;
    let scale = size*1.0;
    let uv1 = pos2(1942.0/2446.0, 0.0);
    let uv2 = pos2(2030.0/2446.0, 0.5);
    render(game, x, y, painter.clone(), ui, ctx, rx, ry, scale, uv1, uv2)?;
    Ok(())
}

pub fn draw_dino_still(game: &mut DinoGame, x: f64, y: f64, painter: Painter, ui: &mut Ui, ctx: &eframe::egui::Context) -> Result<()> {
    let rx: f32 = 88.0;
    let ry = 90.0;
    let scale = size*1.0;
    let uv1 = pos2(0.686_274_5, 0.0);
    let uv2 = pos2(0.720_996_74, 0.5);
    render(game, x, y, painter.clone(), ui, ctx, rx, ry, scale, uv1, uv2)?;
    Ok(())
}

pub fn draw_cacti_small(game: &mut DinoGame, x: f64, y: f64, painter: &Painter, ui: &mut Ui, ctx: &eframe::egui::Context) -> Result<()> {
    let rx: f32 = 40.0;
    let ry = 90.0;
    let scale = size*1.0;
    let uv1 = pos2(446.0/2446.0, 0.0);
    let uv2 = pos2(480.0/2446.0, 0.5);
    render(game, x, y, painter.clone(), ui, ctx, rx, ry, scale, uv1, uv2)?;
    Ok(())
}

pub fn draw_floor(game: &mut DinoGame, x: f64, y: f64, painter: &Painter, ui: &mut Ui, ctx: &eframe::egui::Context) -> Result<()> {
    let rx: f32 = 2400.0;
    let ry = 25.0;
    let scale = size*1.0;
    let uv1 = pos2(2.0/2446.0, 102.0/194.0);
    let uv2 = pos2(2402.0/2446.0, 128.0/194.0);
    render(game, x, y, painter.clone(), ui, ctx, rx, ry, scale, uv1, uv2)?;
    Ok(())
}

fn get_number_cords(number:f32) -> Result<[f32; 2]> {
    match number {
        0.0=>Ok([1292.0, 1314.0]),
        1.0=>Ok([1314.0, 1334.0]),
        2.0=>Ok([1332.0, 1354.0]),
        3.0=>Ok([1352.0, 1374.0]),
        4.0=>Ok([1372.0, 1394.0]),
        5.0=>Ok([1392.0, 1414.0]),
        6.0=>Ok([1412.0, 1434.0]),
        7.0=>Ok([1432.0, 1454.0]),
        8.0=>Ok([1452.0, 1474.0]),
        9.0=>Ok([1472.0, 1494.0]),
        _=> Err(anyhow!("Invalid number"))
    }
}
pub fn draw_numbers(mut numbers: String, game: &mut DinoGame, x: f64, y: f64, painter: &Painter, ui: &mut Ui, ctx: &mut eframe::egui::Context) -> Result<()> {
    let mut space: f32 = 0.0;
    for _ in 0..(4-numbers.chars().count()).max(0) {
        numbers = "0".to_owned()+&numbers;
    };
    debug!("{}", numbers);
    for c in numbers.chars() {
        let c_float = (c.to_digit(10).ok_or_else( || -1)).unwrap() as f32;
        debug!("number as float: {}", c_float);
        let gap = get_number_cords(c_float)?;
        draw_number(c_float, game, x+space as f64, y, painter, ui, &mut ctx.clone()).unwrap();
        space +=(gap[1]-gap[0]+3.0)*0.7;
    };
    Ok(())
}

pub fn draw_number(number: f32, game: &mut DinoGame, x: f64, y: f64, painter: &Painter, ui: &mut Ui, ctx: &mut eframe::egui::Context) -> Result<()> {
    let cords = get_number_cords(number)?;
    let rx: f32 = cords[1]- cords[0]+2.0;
    let ry = 25.0;
    let scale = size *5.0 /4.0 *0.7;
    let uv1 = pos2((cords[0]+1.0)/2446.0, 0.0/194.0);
    let uv2 = pos2((cords[1]-1.0)/2446.0, 25.0/194.0);
    render(game, x, y, painter.clone(), ui, ctx, rx, ry, scale, uv1, uv2)?;
    Ok(())
}
