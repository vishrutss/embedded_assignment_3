# rgbcal: RGB LED calibration tool
Bart Massey and Vishrut Sharma 2024

This tool is designed to find out a decent frame rate and
maximum RGB component values to produce a white-looking RGB
of reasonable brightness.

See below for UI.

**XXX This tool is *mostly* finished! Please wire your
hardware up (see below), finish it, comment it, and use it
to find good values. Then document those values in this
README.**

## Build and Run

Run with `cargo embed --release`. You'll need `cargo embed`, as
`cargo run` / `probe-rs run` does not reliably maintain a
connection for printing. See
https://github.com/probe-rs/probe-rs/issues/1235 for the
details.

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

I think the frame rate is probably set higher than it needs
to be right now: it can be tuned lower.

## Submitted by: Vishrut Sharma
## Results
I acheived a good white by setting the red, green, and blue levels
to 15, 8, and 9 respectively. And a frame rate of 60 was the lowest I could go without the LED blinking.

red: 15 <br>
green: 8 <br>
blue: 9 <br>
frame rate: 60

## Writeup
For this assignment I first started by wiring the RGB LED and the potentiometer to the MB2. The initial code worked as expected and
generated a blue light which could then be controlled by the potentiometer. By having a look at the code that was provided, I was able to add support for the green and red colors as well.

I then added code in `ui.rs` to control the frame rate and the color levels using the potentiometer and also the ability to switch
the function of the potentiometer using the buttons on the MB2.

The main challenge was to share the frame rate value between `ui.rs` and `rgb.rs`. First I had a look at how the RGB values were being shared between `ui.rs` and `rgb.rs` and then I used the same approach to share the frame rate value as well which was to use set and get functions in `main.rs` similar to the ones used for the RGB values.

I then called the get function in `rgb.rs` to get the frame rate value and passed it to `frame_tick_time` which calculated the `tick_time` and then used it to set the frame rate.

I then tested the code by changing the frame rate and the RGB values using the potentiometer and found the best values for the RGB LED to produce a white light.

Overall, I had a great time working on this assignment.