use crate::*;

struct UiState {
    levels: [u8; 3],
    frame_rate: u8,
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
            levels: [0, 0, 0],
            frame_rate: 100,
        }
    }
}

pub struct Ui {
    knob: Knob,
    _button_a: Button,
    _button_b: Button,
    output: UiState,
    state: u8,
}


pub const RED:       u8 = 0;
pub const GREEN:     u8 = 1;
pub const BLUE:      u8 = 2;
pub const FRAMERATE: u8 = 3;

impl Ui {
    pub fn new(knob: Knob, _button_a: Button, _button_b: Button) -> Self {
        Self {
            knob,
            _button_a,
            _button_b,
            output: UiState::default(),
            state: 0,
        }
    }

    fn set_frame_rate(&mut self, level: u8) {

        match level {
            0 => self.output.frame_rate = 0,
            1 => self.output.frame_rate = 60,
            2 => self.output.frame_rate = 70,
            3 => self.output.frame_rate = 80,
            4 => self.output.frame_rate = 90,
            5 => self.output.frame_rate = 100,
            6 => self.output.frame_rate = 110,
            7 => self.output.frame_rate = 120,
            8 => self.output.frame_rate = 130,
            9 => self.output.frame_rate = 140,
            10 => self.output.frame_rate = 150,
            11 => self.output.frame_rate = 160,
            _ =>  self.output.frame_rate = 160,
        }

    }

    pub async fn run(&mut self) -> ! {

        loop {
            let level = self.knob.measure().await;

            if self._button_a.is_low() & self._button_b.is_low() {
                self.state = RED;
            } else if self._button_a.is_low() {
                self.state = BLUE;
            } else if self._button_b.is_low() {
                self.state = GREEN;
            } else {
                self.state = FRAMERATE;
            }

            match self.state {  
                FRAMERATE => {self.set_frame_rate(level); rprintln!("State: FRAMERATE"); },
                RED       => {  rprintln!("State: RED");
                                self.set_frame_rate(self.output.frame_rate);
                                self.output.levels[RED as usize]   = level; 
                                self.output.levels[GREEN as usize] = self.output.levels[GREEN as usize]; 
                                self.output.levels[BLUE as usize]  = self.output.levels[BLUE as usize]; 
                                },
                GREEN     => {  rprintln!("State: GREEN");
                                self.set_frame_rate(self.output.frame_rate);
                                self.output.levels[RED as usize]   = self.output.levels[RED as usize]; 
                                self.output.levels[GREEN as usize] = level; 
                                self.output.levels[BLUE as usize]  = self.output.levels[BLUE as usize]; 
                                },
                BLUE      => {  rprintln!("State: BLUE");
                                self.set_frame_rate(self.output.frame_rate);
                                self.output.levels[RED as usize]   = self.output.levels[RED as usize]; 
                                self.output.levels[GREEN as usize] = self.output.levels[GREEN as usize]; 
                                self.output.levels[BLUE as usize]  = level; 
                                },
                _ => self.set_frame_rate(level),
            }
            
            
            self.output.show();
            set_rgb_levels(|rgb| {
                *rgb = self.output.levels;
            })
            .await;

            Timer::after_millis(50).await;
        }
    }
}
