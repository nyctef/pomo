use crate::app::{AppState, Countdown, CountdownType, MyApp, Pause};
use egui::{CentralPanel, FontId, Frame, RichText};
use std::time::Duration;

impl<'m> eframe::App for MyApp<'m> {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.update();

        let state = self.get_state();
        let frame = Frame::default().fill(get_bg_color(&state));
        CentralPanel::default().frame(frame).show(ui, |ui| {
            if ui.is_pointer_over_egui() {
                show_controls(self, ui);
            } else {
                if let AppState::Counting(countdown) = state {
                    show_countdown(self, countdown, ui);
                }

                if let AppState::Paused(pause) = state {
                    show_pause(self, pause, ui);
                }

                if let AppState::Completed = state {
                    show_completed(self, ui);
                }
            }
        });

        // egui won't automatically redraw unless there's some kind of event
        ui.ctx().request_repaint_after(Duration::from_secs(1));
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

fn show_controls<'m>(app: &mut MyApp<'m>, ui: &mut egui::Ui) {
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

fn show_pause<'m>(_app: &mut MyApp<'m>, pause: Pause, ui: &mut egui::Ui) {
    ui.centered_and_justified(|ui| {
        ui.label(
            RichText::new(format!("{}", pause.remaining_s))
                .font(FontId::proportional(150.0))
                .strong(),
        );
    });
}

fn show_countdown<'m>(_app: &mut MyApp<'m>, countdown: Countdown, ui: &mut egui::Ui) {
    let seconds = MyApp::seconds_remaining(countdown);
    ui.centered_and_justified(|ui| {
        ui.label(
            RichText::new(format!("{}", seconds))
                .font(FontId::proportional(150.0))
                .strong(),
        );
    });
}

fn show_completed<'m>(_app: &mut MyApp<'m>, ui: &mut egui::Ui) {
    ui.centered_and_justified(|ui| {
        ui.label(
            RichText::new("✅")
                .font(FontId::proportional(150.0))
                .strong(),
        );
    });
}
