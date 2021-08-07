#![no_std]
#![no_main]

use arduino_hal::spi::{DataOrder, SerialClockRate};
use arduino_hal::Delay;
use embedded_graphics::prelude::{Point, Primitive, Size, Transform};
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::Drawable;
use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::pixelcolor::{Rgb565, RgbColor};
use embedded_hal::spi::{Mode, Polarity};
use panic_halt as _;
use st7735_lcd::Orientation;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let clk = pins.d13.into_output(); // CLK -> Pin D13
    let miso = pins.d12.into_pull_up_input(); // Not connected
    let mosi = pins.d11.into_output(); // DIN -> Pin D11
    let mut cs = pins.d10.into_output(); // chip select (CS) -> Pin D10

    cs.set_low();

    // vcc = 5V
    // bl = 5V
    // gnd = GND

    let (spi, _other) = arduino_hal::Spi::new(
        dp.SPI,
        clk,
        mosi,
        miso,
        cs,
        arduino_hal::spi::Settings {
            data_order: DataOrder::MostSignificantFirst,
            clock: SerialClockRate::OscfOver2,
            mode: Mode {
                polarity: Polarity::IdleLow,
                phase: embedded_hal::spi::Phase::CaptureOnFirstTransition,
            },
        },
    );

    // Data and Reset for the TFT
    let dc = pins.d9.into_output(); // D/C -> Pin D9
    let mut rst = pins.d8.into_output(); // Rst -> Pin D8

    // Additional connections VCC -> BL -> 5V
    // GND -> GND

    rst.set_high();
    arduino_hal::delay_ms(100);
    rst.set_low();

    let mut disp = st7735_lcd::ST7735::new(spi, dc, rst, true, false, 160, 128);

    let mut delay = Delay::new();
    disp.init(&mut delay).unwrap();

    disp.set_orientation(&Orientation::Landscape).unwrap();

    disp.clear(Rgb565::GREEN).unwrap();

    loop {
        arduino_hal::delay_ms(1000);
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb565::RED)
            .stroke_width(3)
            .fill_color(Rgb565::GREEN)
            .build();

        Rectangle::new(Point::new(30, 20), Size::new(10, 15))
            .into_styled(style)
            .draw(&mut disp)
            .unwrap();

        // Rectangle with translation applied
        Rectangle::new(Point::new(30, 20), Size::new(10, 15))
            .translate(Point::new(-20, -10))
            .into_styled(style)
            .draw(&mut disp)
            .unwrap();

        ufmt::uwriteln!(&mut serial, "Loop ticked!\r").unwrap();
    }
}
