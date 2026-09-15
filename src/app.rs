use rodio::mixer::Mixer;
use rodio::{Decoder, Player};
use std::time::{Duration, SystemTime};
use log;

pub struct PomoApp<'m> {
    state: AppState,
    mixer: &'m Mixer,
    alarm_wav: &'static [u8],
    player: Player,
}

#[derive(Clone, Copy)]
pub enum CountdownType {
    Work,
    Break,
}

#[derive(Clone, Copy)]
pub struct Countdown {
    pub deadline: SystemTime,
    pub kind: CountdownType,
}

#[derive(Clone, Copy)]
pub struct Pause {
    pub remaining_s: u64,
    pub kind: CountdownType,
}

#[derive(Clone, Copy)]
pub struct Intermission {
    pub last_reminder: SystemTime,
    pub next_kind: CountdownType,
}

#[derive(Clone)]
pub enum AppState {
    Counting(Countdown),
    Paused(Pause),
    Completed(Intermission),
}

impl<'m> PomoApp<'m> {
    pub fn new(mixer: &'m Mixer, alarm_wav: &'static [u8], ahead: u64) -> Self {
        let now = std::time::SystemTime::now();
        let deadline = now + Duration::from_secs(ahead);
        let player = Player::connect_new(mixer);
        Self {
            state: AppState::Completed(Intermission {
                last_reminder: now,
                next_kind: CountdownType::Work,
            }),
            mixer,
            alarm_wav,
            player,
        }
    }

    pub fn seconds_remaining(countdown: Countdown) -> u64 {
        countdown
            .deadline
            .duration_since(SystemTime::now())
            .unwrap_or(Duration::from_secs(0))
            .as_secs()
    }

    fn play_alarm(&mut self, countdown: &Countdown) {
        let decoder = Decoder::new(std::io::Cursor::new(self.alarm_wav)).unwrap();
        self.player.append(decoder);
        self.state = AppState::Completed(Intermission {
            last_reminder: SystemTime::now(),
            next_kind: flip_countdown_type(countdown.kind),
        });
    }

    pub fn play_pause(&mut self) {
        log::debug!("play_pause() called");
        if let AppState::Counting(countdown) = self.state {
            let remaining_s = Self::seconds_remaining(countdown);
            self.state = AppState::Paused(Pause {
                remaining_s,
                kind: countdown.kind,
            });
            return;
        }

        if let AppState::Paused(pause) = self.state {
            let deadline = SystemTime::now() + Duration::from_secs(pause.remaining_s);
            self.state = AppState::Counting(Countdown {
                deadline,
                kind: pause.kind,
            });
            return;
        }

        if let AppState::Completed(intermission) = self.state {
            let deadline = SystemTime::now() + duration_for_type(intermission.next_kind);
            self.state = AppState::Counting(Countdown {
                deadline,
                kind: intermission.next_kind,
            });
        }
    }

    pub fn update(&mut self) {
        if let AppState::Counting(countdown) = self.state {
            let seconds = Self::seconds_remaining(countdown);
            if seconds == 0 {
                self.play_alarm(&countdown);
            }
        }
    }

    pub fn get_state(&self) -> AppState {
        self.state.clone()
    }
}

fn flip_countdown_type(kind: CountdownType) -> CountdownType {
    match kind {
        CountdownType::Work => CountdownType::Break,
        CountdownType::Break => CountdownType::Work,
    }
}

fn duration_for_type(kind: CountdownType) -> Duration {
    match kind {
        CountdownType::Work => Duration::from_mins(25),
        CountdownType::Break => Duration::from_mins(5),
    }
}
