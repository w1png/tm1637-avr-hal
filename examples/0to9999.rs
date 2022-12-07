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
        for i in 0..10 {
            for j in 0..10 {
                for k in 0..10 {
                    for l in 0..10 {
                        tm1637.write_numbers(&[i, j, k, l]);
                        arduino_hal::delay_ms(100);
                    }
                }
            }
        }
    }
}
