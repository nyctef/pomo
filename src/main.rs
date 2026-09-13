#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{CentralPanel, Context, FontId, RichText, Style, ViewportBuilder};
use rodio::mixer::Mixer;
use rodio::{Decoder, Player};
use std::fs::File;
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

    // Get an OS-Sink handle to the default physical sound device.
    // Note that the playback stops when the handle is dropped.//!
    let sink = rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    // Load a sound from a file, using a path relative to Cargo.toml
    let alarm_wav = File::open("audio/alarm.wav").unwrap();

    eframe::run_native_ext(
        "My egui App",
        options,
        Some(ctx),
        Box::new(|_cc| Ok(Box::<MyApp>::new(MyApp::new(sink.mixer(), alarm_wav, 3)))),
    )
}

struct MyApp<'m> {
    time: SystemTime,
    mixer: &'m Mixer,
    alarm_wav: File,
    player: Player,
}

impl<'m> MyApp<'m> {
    fn new(mixer: &'m Mixer, alarm_wav: File, ahead: u64) -> Self {
        let now = std::time::SystemTime::now();
        let in_42_seconds = now + Duration::from_secs(ahead);
        let player = Player::connect_new(mixer);
        let decoded = Decoder::new(alarm_wav.try_clone().unwrap()).unwrap();
        player.append(decoded);
        player.pause();
        Self {
            time: in_42_seconds,
            mixer,
            alarm_wav,
            player,
        }
    }
}

impl<'m> eframe::App for MyApp<'m> {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let seconds = self
            .time
            .duration_since(SystemTime::now())
            .unwrap_or(Duration::from_secs(0))
            .as_secs();

        if seconds == 0 {
            print!("playing");
            // Play the alarm sound again when the timer reaches zero
            self.player.play();
            return;
        }

        CentralPanel::default().show(ui, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(
                    RichText::new(format!("{}", seconds))
                        .font(FontId::proportional(150.0))
                        .strong(),
                );
            });
        });

        // egui won't automatically redraw unless there's some kind of event
        ui.ctx().request_repaint_after(Duration::from_secs(1));
    }
}
