//! # render
//! this 
use egui::{Ui, Color32, Pos2, Sense, Painter, Key, TextureHandle, ColorImage};
use epaint::Rect;
use anyhow::Result;
use crate::app::DinoGame;
use epaint::pos2;
use std::io::Cursor;
use image::ImageReader;

/// Renders part of the asset map to the painter
pub fn render(
    game: &mut DinoGame,
    x: f64,
    y: f64,
    painter: Painter,
    _ui: &mut Ui,
    ctx: &eframe::egui::Context,
    rx: f32, ry: f32,
    _size: f32,
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

            let size = [image.width() as _, image.height() as _];
            let image_buffer = image.to_rgba8();
            let pixels = image_buffer.as_flat_samples();
            egui::ColorImage::from_rgba_unmultiplied(
                size,
                pixels.as_slice(),
            ) },
            egui::TextureOptions::default()))
    } 
    let texture_id = egui::TextureId::from(&(game.asset_map.clone().expect("no texture id")));

    let x: f32 = x as f32;
    let y: f32 = y as f32;
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
    let scale = 0.8;
    let uv1 = pos2(0.03, 0.0);
    let uv2 = pos2(0.068, 0.5);
    render(game, x, y, painter.clone(), ui, ctx, rx, ry, scale, uv1, uv2)?;
    Ok(())
}

pub fn draw_dino_left(game: &mut DinoGame, x: f64, y: f64, painter: Painter, ui: &mut Ui, ctx: &eframe::egui::Context) -> Result<()> {
    let rx: f32 = 80.0;
    let ry = 90.0;
    let scale = 0.8;
    let uv1 = pos2(0.76, 0.0);
    let uv2 = pos2(0.795, 0.5);
    render(game, x, y, painter.clone(), ui, ctx, rx, ry, scale, uv1, uv2)?;
    Ok(())
}

pub fn draw_dino_right(game: &mut DinoGame, x: f64, y: f64, painter: Painter, ui: &mut Ui, ctx: &eframe::egui::Context) -> Result<()> {
    let rx: f32 = 80.0;
    let ry = 90.0;
    let scale = 0.8;
    let uv1 = pos2(0.79330065359, 0.0);
    let uv2 = pos2(0.82883986928, 0.5);
    render(game, x, y, painter.clone(), ui, ctx, rx, ry, scale, uv1, uv2)?;
    Ok(())
}

pub fn draw_dino_still(game: &mut DinoGame, x: f64, y: f64, painter: Painter, ui: &mut Ui, ctx: &eframe::egui::Context) -> Result<()> {
    let rx: f32 = 80.0;
    let ry = 90.0;
    let scale = 0.8;
    let uv1 = pos2(0.6862745098039216, 0.0);
    let uv2 = pos2(0.7209967320261438, 0.5);
    render(game, x, y, painter.clone(), ui, ctx, rx, ry, scale, uv1, uv2)?;
    Ok(())
}

pub fn draw_cacti_small(game: &mut DinoGame, x: f64, y: f64, painter: &Painter, ui: &mut Ui, ctx: &eframe::egui::Context) -> Result<()> {
    let rx: f32 = 40.0;
    let ry = 90.0;
    let scale = 0.8;
    let uv1 = pos2(0.185, 0.0);
    let uv2 = pos2(0.1985, 0.5);
    render(game, x, y, painter.clone(), ui, ctx, rx, ry, scale, uv1, uv2)?;
    Ok(())
}