use crate::*;

type RgbPins = [Output<'static, AnyPin>; 3];

pub struct Rgb {
    rgb: RgbPins,
    // Shadow variables to minimize lock contention.
    levels: [u32; 3],
    tick_time: u64,
}

impl Rgb {
    /// Calculates the tick time for each frame based on the frame rate and the number of levels.
    /// ### Arguments
    /// * `frame_rate` - The desired frame rate
    /// ### Returns
    /// The tick time
    fn frame_tick_time(frame_rate: u64) -> u64 {
        1_000_000 / (3 * frame_rate * LEVELS as u64)
    }

    /// Creates a new `Rgb` instance.
    /// ### Arguments
    /// * `rgb` - The RGB pins.
    /// * `frame_rate` - The desired frame rate
    /// ### Returns
    /// A new `Rgb` instance.
    pub fn new(rgb: RgbPins, frame_rate: u64) -> Self {
        let tick_time = Self::frame_tick_time(frame_rate);
        Self {
            rgb,
            levels: [0; 3],
            tick_time,
        }
    }

    /// Function to scan the RGB levels and set the RGB pins accordingly
    /// ### Arguments
    /// * `led` - The index of the RGB to set
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

    /// Runs the RGB lights indefinitely.
    /// ### Arguments
    /// * `self` - The `Rgb` instance.
    pub async fn run(mut self) -> ! {
        loop {
            self.levels = get_rgb_levels().await;
            let frame_rate = get_frame_rate().await as u64;
            self.tick_time = Self::frame_tick_time(frame_rate);

            for led in 0..3 {
                self.step(led).await;
            }
        }
    }
}
