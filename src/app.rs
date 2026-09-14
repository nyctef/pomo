use rodio::mixer::Mixer;
use rodio::{Decoder, Player};
use std::time::{Duration, SystemTime};

pub struct MyApp<'m> {
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

impl<'m> MyApp<'m> {
    pub fn new(mixer: &'m Mixer, alarm_wav: &'static [u8], ahead: u64) -> Self {
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

    pub fn seconds_remaining(countdown: Countdown) -> u64 {
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

    pub fn pause(&mut self) {
        if let AppState::Counting(countdown) = self.state {
            let remaining_s = Self::seconds_remaining(countdown);
            self.state = AppState::Paused(Pause {
                remaining_s,
                kind: countdown.kind,
            });
        }
    }

    pub fn update(&mut self) {
        if let AppState::Counting(countdown) = self.state {
            let seconds = Self::seconds_remaining(countdown);
            if seconds == 0 {
                self.play_alarm();
            }
        }
    }

    pub fn get_state(&self) -> AppState {
        self.state.clone()
    }
}
