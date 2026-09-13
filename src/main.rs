#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{CentralPanel, Context, FontId, Frame, RichText, Style, ViewportBuilder, frame};
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

    // Get an OS-Sink handle to the default physical sound device.
    // Note that the playback stops when the handle is dropped.//!
    let sink = rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    // Load a sound from a file, using a path relative to Cargo.toml
    let alarm_wav = include_bytes!("../audio/alarm.wav");

    eframe::run_native_ext(
        "My egui App",
        options,
        Some(ctx),
        Box::new(|_cc| Ok(Box::<MyApp>::new(MyApp::new(sink.mixer(), alarm_wav, 3)))),
    )
}

struct MyApp<'m> {
    state: AppState,
    mixer: &'m Mixer,
    alarm_wav: &'static [u8],
    player: Player,
}

#[derive(Clone, Copy)]
enum CountdownType {
    Work,
    Break,
}

#[derive(Clone, Copy)]
struct Countdown {
    pub deadline: SystemTime,
    pub kind: CountdownType,
}

#[derive(Clone, Copy)]
struct Pause {
    pub remaining_s: u64,
    pub kind: CountdownType,
}

enum AppState {
    Counting(Countdown),
    Paused(Pause),
    Completed,
}

impl<'m> MyApp<'m> {
    fn new(mixer: &'m Mixer, alarm_wav: &'static [u8], ahead: u64) -> Self {
        let now = std::time::SystemTime::now();
        let deadline = now + Duration::from_secs(ahead);
        let player = Player::connect_new(mixer);
        Self {
            state: AppState::Counting(Countdown {
                deadline,
                kind: CountdownType::Work,
            }),
            mixer,
            alarm_wav,
            player,
        }
    }

    fn seconds_remaining(countdown: Countdown) -> u64 {
        countdown
            .deadline
            .duration_since(SystemTime::now())
            .unwrap_or(Duration::from_secs(0))
            .as_secs()
    }

    fn play_alarm(&mut self) {
        let decoder = Decoder::new(std::io::Cursor::new(self.alarm_wav)).unwrap();
        self.player.append(decoder);
        self.state = AppState::Completed;
    }

    fn pause(&mut self) {
        if let AppState::Counting(countdown) = self.state {
            let remaining_s = Self::seconds_remaining(countdown);
            self.state = AppState::Paused(Pause {
                remaining_s,
                kind: countdown.kind,
            });
        }
    }
}

impl<'m> eframe::App for MyApp<'m> {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let frame = Frame::default().fill(get_bg_color(&self.state));
        CentralPanel::default().frame(frame).show(ui, |ui| {
            if ui.is_pointer_over_egui() {
                show_controls(self, ui);
            } else {
                if let AppState::Counting(countdown) = self.state {
                    show_countdown(self, countdown, ui);
                }

                if let AppState::Completed = self.state {
                    ui.label(
                        RichText::new("✅")
                            .font(FontId::proportional(150.0))
                            .strong(),
                    );
                }
            }
        });
    }
}

fn get_bg_color(state: &AppState) -> egui::Color32 {
    let red = egui::Color32::from_rgb(175, 73, 73);
    let green = egui::Color32::from_rgb(73, 175, 73);
    let blue = egui::Color32::from_rgb(73, 73, 175);
    let black = egui::Color32::from_rgb(0, 0, 0);
    match state {
        AppState::Counting(countdown) => match countdown.kind {
            CountdownType::Work => red,
            CountdownType::Break => green,
        },
        AppState::Paused(pause) => match pause.kind {
            CountdownType::Work => red.lerp_to_gamma(black, 0.7),
            CountdownType::Break => green.lerp_to_gamma(black, 0.7),
        },
        AppState::Completed => blue,
    }
}

fn show_controls<'m>(app: &mut MyApp<'m>, ui: &mut egui::Ui) -> () {
    ui.centered_and_justified(|ui| {
        let pause = ui.button(
            RichText::new("⏸")
                .font(FontId::proportional(150.0))
                .strong(),
        );
        if pause.clicked() {
            app.pause();
        }
    });
}

fn show_countdown<'m>(app: &mut MyApp<'m>, countdown: Countdown, ui: &mut egui::Ui) -> () {
    let seconds = MyApp::seconds_remaining(countdown);
    if seconds == 0 {
        app.play_alarm();
    }
    ui.centered_and_justified(|ui| {
        ui.label(
            RichText::new(format!("{}", seconds))
                .font(FontId::proportional(150.0))
                .strong(),
        );
    });

    // egui won't automatically redraw unless there's some kind of event
    ui.ctx().request_repaint_after(Duration::from_secs(1));
}
