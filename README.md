# Rust Embedded CS510

## Homework 3: RGB Calibration

## Student John Eley

## Program Assignment

- Fork https://github.com/pdx-cs-rust-embedded/hw-rgbcal-skeletonLinks to an external site
- Follow the instructions in the README there to wire up your MB2.
- Compile and run the code with cargo embed --release to verify that everything is working. You should be able to turn the blue LED brighter and dimmer with the knob.
- Comment the code with doc comments and internal comments to clarify its function. This is a good way to learn the code.
- Add the rest of the code as specified in the README.  Comment that too, of course.
- Do the calibration and put the values in the README.
- Put one of the following in your submission:
*(Preferred)* Make a 5-second video of your system in operation.  Call it VIDEO.mpg (or whatever extension is easy to make).
*(Otherwise)* Take a photo of your wiring. Call it PHOTO.jpg.
- Submit a ZIP as usual: see below.

## Wiring

Connect the RGB LED to the MB2 as follows:

* Red to P9 (GPIO1)
* Green to P8 (GPIO2)
* Blue to P16 (GPIO3)
* Gnd to Gnd

Connect the potentiometer (knob) to the MB2 as follows:

* Pin 1 to Gnd
* Pin 2 to P2
* Pin 3 to +3.3V

## UI

The knob controls the individual settings: frame rate and
color levels. Which parameter the knob controls should be
determined by which buttons are held. (Right now, the knob
jus always controls Blue. You should see the color change
from green to teal-blue as you turn the knob clockwise.)

* No buttons held: Change the frame rate in steps of 10
  frames per second from 10..160.
* A button held: Change the blue level from off to on over
  16 steps.
* B button held: Change the green level from off to on over
  16 steps.
* A+B buttons held: Change the red level from off to on over
  16 steps.

The "frame rate" (also known as the "refresh rate") is the
time to scan out all three colors. (See the scanout code.)
At 30 frames per second, every 1/30th of a second the LED
should scan out all three colors. If the frame rate is too
low, the LED will appear to "blink". If it is too high, it
will eat CPU for no reason.


**LED Specifications**

[LED Wiring Diagram](https://docs.sunfounder.com/projects/sf-components/en/latest/component_rgb_led.html#:~:text=We%20use%20the%20common%20cathode%20one.&text=An%20RGB%20LED%20has%204,%2C%20GND%2C%20Green%20and%20Blue)
