use crate::*;

struct UiState {
    levels: [u32; 3],
    frame_rate: u32,
}

impl UiState {
    fn show(&self) {
        let names = ["red", "green", "blue"];
        rprintln!();
        for (name, level) in names.iter().zip(self.levels.iter()) {
            rprintln!("{}: {}", name, level);
        }
        rprintln!("frame rate: {}", self.frame_rate);
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            levels: [LEVELS - 1, LEVELS - 1, LEVELS - 1],
            frame_rate: 100,
        }
    }
}

pub struct Ui {
    knob: Knob,
    button_a: Button,
    button_b: Button,
    state: UiState,
}

impl Ui {
    pub fn new(knob: Knob, button_a: Button, button_b: Button) -> Self {
        Self {
            knob,
            button_a,
            button_b,
            state: UiState::default(),
        }
    }

    pub async fn run(&mut self) -> ! {
        if self.button_a.is_low() {
            self.state.levels[2] = self.knob.measure().await;
        } else if self.button_b.is_low() {
            self.state.levels[1] = self.knob.measure().await;
        } else if self.button_a.is_low() && self.button_b.is_low() {
            self.state.levels[0] = self.knob.measure().await;
        } else {
            self.state.frame_rate = self.knob.measure().await;
        }
        set_rgb_levels(|rgb| {
            *rgb = self.state.levels;
        })
        .await;
        self.state.show();
        loop {
            let level = self.knob.measure().await;
            let rgb_level;
            if self.button_a.is_low() || self.button_b.is_low() {
                if self.button_a.is_low() && self.button_b.is_low() {
                    rgb_level = 0;
                } else if self.button_b.is_low() {
                    rgb_level = 1;
                } else {
                    rgb_level = 2;
                }
                if level != self.state.levels[rgb_level] {
                    self.state.levels[rgb_level] = level;
                    self.state.show();
                    set_rgb_levels(|rgb| {
                        *rgb = self.state.levels;
                    })
                    .await;
                }
            } else if level != self.state.frame_rate {
                self.state.frame_rate = (level * 10) + 10;
                self.state.show();
                set_rgb_levels(|rgb| {
                    *rgb = self.state.levels;
                })
                .await;
            }
            Timer::after_millis(50).await;
        }
    }
}
