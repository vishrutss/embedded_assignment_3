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

pub static RGB_LEVELS: Mutex<ThreadModeRawMutex, [u32; 3]> = Mutex::new([0; 3]);
pub static FRAME_RATE: Mutex<ThreadModeRawMutex, u32> = Mutex::new(10);
pub const LEVELS: u32 = 16;

/// Get the current frame rate.
async fn get_frame_rate() -> u32 {
    let frame_rate = FRAME_RATE.lock().await;
    *frame_rate
}

/// Set the frame rate using a closure.
/// ### Arguments
/// * `setter` - A closure that takes a mutable reference to the frame rate and modifies it.
async fn set_frame_rate<F>(setter: F)
where
    F: FnOnce(&mut u32),
{
    let mut frame_rate = FRAME_RATE.lock().await;
    setter(&mut frame_rate)
}

/// Get the current RGB levels.
async fn get_rgb_levels() -> [u32; 3] {
    let rgb_levels = RGB_LEVELS.lock().await;
    *rgb_levels
}

/// Set the RGB levels using a closure.
/// ### Arguments
/// * `setter` - A closure that takes a mutable reference to the RGB levels and modifies them.
async fn set_rgb_levels<F>(setter: F)
where
    F: FnOnce(&mut [u32; 3]),
{
    let mut rgb_levels = RGB_LEVELS.lock().await;
    setter(&mut rgb_levels);
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    rtt_init_print!();
    let board = Microbit::default();

    bind_interrupts!(struct Irqs {
        SAADC => saadc::InterruptHandler;
    });

    let led_pin = |p| Output::new(p, Level::Low, OutputDrive::Standard);
    let red = led_pin(AnyPin::from(board.p9));
    let green = led_pin(AnyPin::from(board.p8));
    let blue = led_pin(AnyPin::from(board.p16));
    let rgb: Rgb = Rgb::new([red, green, blue], 100);

    let mut saadc_config = saadc::Config::default();
    saadc_config.resolution = saadc::Resolution::_14BIT;
    let saadc = saadc::Saadc::new(
        board.saadc,
        Irqs,
        saadc_config,
        [saadc::ChannelConfig::single_ended(board.p2)],
    );
    let knob = Knob::new(saadc).await;
    let mut ui = Ui::new(knob, board.btn_a, board.btn_b);

    join::join(rgb.run(), ui.run()).await;

    panic!("fell off end of main loop");
}
