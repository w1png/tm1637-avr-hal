#![no_std]

use arduino_hal::port::{self, mode::Output};

const DIGITS: [u8; 10] = [
    0x3f, 0x06, 0x5b, 0x4f, 0x66,
    0x6d, 0x7d, 0x07, 0x7f, 0x6f
];

pub struct TM1637 {
    dio: port::Pin<Output>,
    clk: port::Pin<Output>,
    delay_ms: fn(u16),
}

impl TM1637 {
    pub fn new(dio: port::Pin<Output>, clk: port::Pin<Output>, delay_ms: fn(u16)) -> Self {
        TM1637 {
            dio,
            clk,
            delay_ms,
        }
    }

    pub fn start(&mut self) {
        self.dio.set_high();
        self.clk.set_high();
        (self.delay_ms)(5);

        self.dio.set_low();
        self.clk.set_low();
        (self.delay_ms)(5);
    }

    pub fn stop(&mut self) {
        self.clk.set_low();
        self.dio.set_low();
        (self.delay_ms)(5);

        self.clk.set_high();
        self.dio.set_high();
        (self.delay_ms)(5);
    }

    pub fn write_bit(&mut self, bit: bool) {
        self.clk.set_low();
        if bit {
            self.dio.set_high();
        } else {
            self.dio.set_low();
        }
        (self.delay_ms)(5);

        self.clk.set_high();
    }

    pub fn write_value(&mut self, value: u8) {
        for i in 0..8 {
            self.clk.set_low();
            (self.delay_ms)(5);
            self.write_bit(value & (1 << i) != 0);
            (self.delay_ms)(5);
            self.clk.set_high();
            (self.delay_ms)(5);
        }

        self.clk.set_low();
        (self.delay_ms)(5);
        self.clk.set_high();
        (self.delay_ms)(5);
    }

    pub fn write_numbers(&mut self, numbers: &[u8]) {
        self.start();
        self.write_value(0x40);
        self.stop();

        self.start();
        self.write_value(0xc0);
        for number in numbers {
            self.write_value(DIGITS[*number as usize]);
        }
        self.stop();
    }

    pub fn setup(&mut self) {
        self.start();
        self.write_value(0x8f);
        self.stop();

        self.write_numbers(&[0, 0, 0, 0]);
    }
}

