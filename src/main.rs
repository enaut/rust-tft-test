#![no_std]
#![no_main]

use arduino_hal::Delay;
use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::pixelcolor::{Rgb565, RgbColor};
use panic_halt as _;
use st7735_lcd::Orientation;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").unwrap();

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/next/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let clk = pins.d13.into_output(); // CLK -> Pin 13
    let miso = pins.d12.into_pull_up_input(); // Not connected
    let mosi = pins.d11.into_output(); // DIN -> Pin 11
    let mut cs = pins.d10.into_output(); // chip select (CS) -> Pin 10

    cs.set_low();

    // Data and Reset for the TFT
    let dc = pins.d9.into_output();
    let mut rst = pins.d8.into_output();
    rst.set_high();
    arduino_hal::delay_ms(100);
    rst.set_low();

    // vcc = 5V
    // bl = 5V
    // gnd = GND

    let (spi, _other) = arduino_hal::Spi::new(
        dp.SPI,
        clk,
        mosi,
        miso,
        cs,
        arduino_hal::spi::Settings::default(),
    );

    let mut disp = st7735_lcd::ST7735::new(spi, dc, rst, true, false, 160, 128);

    let mut delay = Delay::new();
    disp.init(&mut delay).unwrap();

    ufmt::uwriteln!(&mut serial, "Initialized!\r").unwrap();
    disp.set_orientation(&Orientation::Landscape).unwrap();

    ufmt::uwriteln!(&mut serial, "Orientation set!\r").unwrap();
    disp.clear(Rgb565::GREEN).unwrap();
    disp.set_pixel(50, 50, 0).unwrap();

    ufmt::uwriteln!(&mut serial, "Screen cleared?\r").unwrap();

    loop {
        arduino_hal::delay_ms(1000);
        disp.clear(Rgb565::BLACK).unwrap();

        ufmt::uwriteln!(&mut serial, "Loop ticked!\r").unwrap();
    }
}
