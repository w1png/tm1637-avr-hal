#![no_std]
#![no_main]

use tm1637_avr_hal::TM1637;

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let dio = pins.d4.downgrade().into_output();
    let clk = pins.d5.downgrade().into_output();

    let mut tm1637 = TM1637::new(
        dio,
        clk,
        |ms| arduino_hal::delay_ms(ms)
    );

    tm1637.setup();

    loop {
        tm1637.write_numbers(&[1, 2, 3, 4]);
        arduino_hal::delay_ms(1000);
        tm1637.write_numbers(&[5, 6, 7, 8]);
        arduino_hal::delay_ms(1000);
    }
}
