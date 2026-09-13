#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::fmt::Alignment::Center;

use eframe::egui;
use egui::{CentralPanel, Context, FontId, RichText, Style, ViewportBuilder};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([320.0, 240.0])
            .with_decorations(false)
            .with_always_on_top(),
        ..Default::default()
    };
    let ctx = Context::default();

    let red = egui::Color32::from_rgb(175, 73, 73);

    // let mut light_style = Style::default();
    // light_style.visuals.window_fill = red;
    // ctx.set_style_of(Theme::Light, light_style);

    let mut red_style = Style::default();
    // red_style.visuals.window_fill = red;
    red_style.visuals.panel_fill = red;
    // ctx.set_style_of(Theme::Dark, dark_style);
    ctx.all_styles_mut(|style| {
        *style = red_style.clone();
    });

    eframe::run_native_ext(
        "My egui App",
        options,
        Some(ctx),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    time: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { time: 42 }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ui, |ui| {
            ui.horizontal_centered(|ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.label(
                        RichText::new(format!("{}", self.time))
                            .font(FontId::proportional(150.0))
                            .strong(),
                    );
                });
            });
        });
    }
}
