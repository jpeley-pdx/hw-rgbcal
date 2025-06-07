use crate::*;

type RgbPins = [Output<'static, AnyPin>; 3];

pub const LEVELS: u8 = 16;

pub struct Rgb {
    rgb: RgbPins,
    // Shadow variables to minimize lock contention.
    levels: [u8; 3],
    tick_time: u64,
}

async fn get_rgb_levels() -> [u8; 3] {
    let rgb_levels = RGB_LEVELS.lock().await;
    *rgb_levels
}

async fn set_rgb_levels<F>(setter: F)
where
    F: FnOnce(&mut [u8; 3]),
{
    let mut rgb_levels = RGB_LEVELS.lock().await;
    setter(&mut rgb_levels);
}

impl Rgb {
    fn frame_tick_time(frame_rate: u64) -> u64 {
        1_000_000 / (3 * frame_rate * LEVELS as u64)
    }

    pub fn new(rgb: RgbPins, frame_rate: u64) -> Self {
        let tick_time = Self::frame_tick_time(frame_rate);
        Self {
            rgb,
            levels: [0; 3],
            tick_time,
        }
    }

    async fn step(&mut self, led: usize) {
        let level = self.levels[led];
        if level > 0 {
            self.rgb[led].set_high();
            let on_time = level as u64 * self.tick_time;
            Timer::after_micros(on_time).await;
            self.rgb[led].set_low();
        }
        let level = LEVELS - level;
        if level > 0 {
            let off_time = level as u64 * self.tick_time;
            Timer::after_micros(off_time).await;
        }
    }

    pub async fn run(mut self) -> ! {
        loop {
            self.levels = get_rgb_levels().await;

            for led in 0..3 {
                self.step(led).await;
            }
        }
    }
}
