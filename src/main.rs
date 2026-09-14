#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod ui;

use app::MyApp;
use eframe::egui;
use egui::{Context, ViewportBuilder};

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

    // Get an OS-Sink handle to the default physical sound device.
    // Note that the playback stops when the handle is dropped.//!
    let sink = rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    // Load a sound from a file, using a path relative to Cargo.toml
    let alarm_wav = include_bytes!("../audio/alarm.wav");

    ctx.all_styles_mut(|s| {
        s.visuals.widgets.inactive.weak_bg_fill =
            egui::Color32::from_rgba_unmultiplied(0, 0, 0, 30);
        s.visuals.widgets.hovered.weak_bg_fill = egui::Color32::from_rgba_unmultiplied(0, 0, 0, 50);
    });

    eframe::run_native_ext(
        "My egui App",
        options,
        Some(ctx),
        Box::new(|_cc| Ok(Box::<MyApp>::new(MyApp::new(sink.mixer(), alarm_wav, 3)))),
    )
}
