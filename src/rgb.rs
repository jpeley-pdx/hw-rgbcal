use crate::*;

type RgbPins = [Output<'static, AnyPin>; 3];

pub const LEVELS: u8 = 16;

pub struct Rgb {
    rgb: RgbPins,
    // Shadow variables to minimize lock contention.
    levels: [u8; 3],
    tick_time: u64,
    rate: u64, // added a variable for framerate storage
}

impl Rgb {
    // function to calculate the tick time
    fn frame_tick_time(frame_rate: u64) -> u64 {
        1_000_000 / (3 * frame_rate * LEVELS as u64)
    }

    // Constructor for RGB loop
    pub fn new(rgb: RgbPins, frame_rate: u64) -> Self {
        
        // store the framerate and tick time
        let rate = frame_rate;      
        let tick_time = Self::frame_tick_time(rate);

        Self {
            rgb,
            levels: [0; 3],
            tick_time,
            rate,
        }
    }

    // Function to blink the LED
    async fn step(&mut self, led: usize) {
        
        // get the level for the current LED
        let level = self.levels[led];

        // if the lED is not off
        if level > 0 {

            // turn on the LED
            self.rgb[led].set_high();

            // calculate the on time
            let on_time = level as u64 * self.tick_time;

            // delay for the speficied on time
            Timer::after_micros(on_time).await;

            // turn off the LED
            self.rgb[led].set_low();
        }

        let level = LEVELS - level;
        if level > 0 {
            let off_time = level as u64 * self.tick_time;
            Timer::after_micros(off_time).await;
        }
    }

    // Function to show RGB Loop values
    // this was copied from the UI Loop and modified
    // it enables verification that the
    // values are transitioning between loops
    fn show(&mut self) {
        rprintln!("--- RGB Loop ---");
        let names = ["red", "green", "blue"];
        rprintln!("Framerate: {}", self.rate);
        for (name, level) in names.iter().zip(self.levels.iter()) {
            rprintln!("{}: {}", name, level);
        }
        rprintln!("----------------");
    }

    // Run RGB Loop
    pub async fn run(mut self) -> ! {

        loop {
            // get the current RGB Values
            self.levels = get_rgb_levels().await;

            // get the current framerate
            self.rate = get_rgb_rate().await as u64;

            // calc the tick time
            self.tick_time = Rgb::frame_tick_time(self.rate);

            // update LED outputs
            for led in 0..3 {
                self.step(led).await;
            }

            // send the RGB loop values to the debug interface
            self.show();
        }
    }
}
