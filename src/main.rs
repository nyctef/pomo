#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{CentralPanel, Context, FontId, RichText, Style, ViewportBuilder};
use std::thread;
use std::time::{Duration, SystemTime};

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

    let bg_ctx = ctx.clone();
    thread::spawn(|| bg_timer(bg_ctx));

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
    time: SystemTime,
}

impl Default for MyApp {
    fn default() -> Self {
        let now = std::time::SystemTime::now();
        let in_42_seconds = now + Duration::from_secs(42);
        Self {
            time: in_42_seconds,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let seconds = self
            .time
            .duration_since(SystemTime::now())
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
        CentralPanel::default().show(ui, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(
                    RichText::new(format!("{}", seconds))
                        .font(FontId::proportional(150.0))
                        .strong(),
                );
            });
        });
    }
}

/// thread to update the gui regularly.
/// This could be improved to only do it while the timer is active and the window is visible
fn bg_timer(ctx: Context) {
    let one_second = Duration::from_secs(1);
    loop {
        thread::sleep(one_second);
        ctx.request_repaint();
    }
}
