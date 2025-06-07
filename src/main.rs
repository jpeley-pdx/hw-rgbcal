//! Embedded Rust Programming - Prog 3
//! RGB Calibration
//!
//! https://canvas.pdx.edu/courses/101554/assignments/1064100
//!
//! Assignment
//! Fork https://github.com/pdx-cs-rust-embedded/hw-rgbcal-skeletonLinks to an external site. 
//! Follow the instructions in the README there to wire up your MB2.
//! Compile and run the code with cargo embed --release to verify that everything is working. 
//!     You should be able to turn the blue LED brighter and dimmer with the knob.
//! Comment the code with doc comments and internal comments to clarify its function. This is a good way to learn the code.
//! Add the rest of the code as specified in the README.  Comment that too, of course.
//! Do the calibration and put the values in the README.
//! Put one of the following in your submission.
//! (Preferred)* Make a 5-second video of your system in operation.  Call it VIDEO.mpg (or whatever extension is easy to make).
//! (Otherwise)* Take a photo of your wiring. Call it PHOTO.jpg.
//! Submit a ZIP as usual: see below.


#![no_std]
#![no_main]

mod knob;
mod rgb;
mod ui;
pub use knob::*;
pub use rgb::*;
pub use ui::*;

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use embassy_executor::Spawner;
use embassy_futures::join;
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};
use embassy_time::Timer;
use microbit_bsp::{
    embassy_nrf::{
        bind_interrupts,
        gpio::{AnyPin, Level, Output, OutputDrive},
        saadc,
    },
    Button, Microbit,
};
use num_traits::float::FloatCore;


// declare mutex controls for levels and framerate
pub static RGB_LEVELS: Mutex<ThreadModeRawMutex, [u8; 3]> = Mutex::new([0; 3]);
pub static RGB_RATE: Mutex<ThreadModeRawMutex, u8> = Mutex::new(60);

// Function to safely acquire the RGB values
async fn get_rgb_levels() -> [u8; 3] {
    let rgb_levels = RGB_LEVELS.lock().await;
    *rgb_levels
}

// Function to safely acquire the framerate value
async fn get_rgb_rate() -> u8 {
    let rgb_rate = RGB_RATE.lock().await;
    *rgb_rate
}

// Function to safely set the RGB values
async fn set_rgb_levels<F>(setter: F)
where
    F: FnOnce(&mut [u8; 3]),
{
    let mut rgb_levels = RGB_LEVELS.lock().await;
    setter(&mut rgb_levels);
}

// Function to safely acquire the framerte value
async fn set_rgb_rate<F>(setter: F)
where
    F: FnOnce(&mut u8),
{
    let mut rgb_rate = RGB_RATE.lock().await;
    setter(&mut rgb_rate);
}


// Main loop
#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    
    //  init debug output
    rtt_init_print!();

    // Acquire the board
    let board = Microbit::default();

    // setup interrupts
    bind_interrupts!(struct Irqs {
        SAADC => saadc::InterruptHandler;
    });

    // declare and setup the LEDs, Buttons, and RGB Loop
    let led_pin = |p| Output::new(p, Level::Low, OutputDrive::Standard);
    let red = led_pin(AnyPin::from(board.p9));
    let green = led_pin(AnyPin::from(board.p8));
    let blue = led_pin(AnyPin::from(board.p16));
    let rgb: Rgb = Rgb::new([red, green, blue], 60);

    // Configure the ADC
    let mut saadc_config = saadc::Config::default();
    saadc_config.resolution = saadc::Resolution::_14BIT;
    let saadc = saadc::Saadc::new(
        board.saadc,
        Irqs,
        saadc_config,
        [saadc::ChannelConfig::single_ended(board.p2)],
    );

    // declare and acquire the knob and ui loop
    let knob = Knob::new(saadc).await;
    let mut ui = Ui::new(knob, board.btn_a, board.btn_b);

    // Start the UI and RGB loops
    join::join(rgb.run(), ui.run()).await;

    panic!("fell off end of main loop");
}
