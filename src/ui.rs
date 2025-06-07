use crate::*;

struct UiState {
    levels: [u8; 3], // changed to u8 to save memory
    frame_rate: u8,  // changed to u8 to save memory
}

impl UiState {
    // function to show UI Loop values
    fn show(&self) {
        rprintln!("--- UI Loop ---");
        let names = ["red", "green", "blue"];
        rprintln!("Framerate: {}", self.frame_rate);
        for (name, level) in names.iter().zip(self.levels.iter()) {
            rprintln!("{}: {}", name, level);
        }
        rprintln!("----------------");
    }
}

impl Default for UiState {
    // function to set defaults
    fn default() -> Self {
        Self {
            levels: [0, 0, 0],
            frame_rate: 60,
        }
    }
}

// UI Data Struct
pub struct Ui {
    knob: Knob,
    _button_a: Button,
    _button_b: Button,
    output: UiState,
    state: u8,
}

// State constants
pub const RED:       u8 = 0;
pub const GREEN:     u8 = 1;
pub const BLUE:      u8 = 2;
pub const FRAMERATE: u8 = 3;

impl Ui {

    // Constructor
    pub fn new(knob: Knob, _button_a: Button, _button_b: Button) -> Self {
        Self {
            knob,
            _button_a,
            _button_b,
            output: UiState::default(),
            state: 3,
        }
    }

    // Function to set the framerate based on the knob level
    fn set_frame_rate(&mut self, level: u8) {

        match level {
            0 => self.output.frame_rate = 60,
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

    // Run UI Loop
    pub async fn run(&mut self) -> ! {

        loop {

            // Get the knob value
            let level = self.knob.measure().await;

            // check the buttons and set the state accordingly
            if self._button_a.is_low() & self._button_b.is_low() {
                self.state = RED;
            } else if self._button_a.is_low() {
                self.state = BLUE;
            } else if self._button_b.is_low() {
                self.state = GREEN;
            } else {
                self.state = FRAMERATE;
            }

            // Update values depending on the current state
            // default dtate is FRAMERATE, so the knob adjusts the
            // framerate in the absence of any button press
            // for the next rev, have HW add another couple buttons
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
            
            // send the UI loop values to the debug interface
            self.output.show();

            // Send the LED values to the RGB loop
            set_rgb_levels(|rgb| {
                *rgb = self.output.levels;
            })
            .await;

            // Send the framerate value to the RGB loop
            set_rgb_rate(|rate| {
                *rate = self.output.frame_rate;
                rprintln!("set framerate: {}", self.output.frame_rate);
            })
            .await;

            // loop delay
            Timer::after_millis(50).await;
        }
    }
}
