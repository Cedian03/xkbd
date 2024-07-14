use core::convert::Infallible;

use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin},
};

const DELAY_US: u32 = 30;

pub struct COL2ROW;
pub struct ROW2COL;

pub struct KeyScan<T>(T);

impl KeyScan<COL2ROW> {
    pub fn scan<const W: usize, const H: usize>(
        rows: &mut [&mut dyn InputPin<Error = Infallible>; H],
        cols: &mut [&mut dyn OutputPin<Error = Infallible>; W],
        delay: &mut impl DelayNs,
    ) -> [[bool; W]; H] {
        let mut matrix = [[false; W]; H];

        for x in 0..W {
            let col_pin = &mut cols[x];
            col_pin.set_high().unwrap();
            delay.delay_us(DELAY_US);

            for y in 0..H {
                let row_pin = &mut rows[y];
                matrix[y][x] = row_pin.is_high().unwrap();
            }

            col_pin.set_low().unwrap();
            delay.delay_us(DELAY_US);
        }

        matrix
    }
}

impl KeyScan<ROW2COL> {
    pub fn scan<const W: usize, const H: usize>(
        rows: &mut [&mut dyn OutputPin<Error = Infallible>; H],
        cols: &mut [&mut dyn InputPin<Error = Infallible>; W],
        delay: &mut impl DelayNs,
    ) -> [[bool; W]; H] {
        let mut matrix = [[false; W]; H];

        for y in 0..H {
            let row_pin = &mut rows[y];
            row_pin.set_high().unwrap();
            delay.delay_us(DELAY_US);

            for x in 0..W {
                let col_pin = &mut cols[y];
                matrix[y][x] = col_pin.is_high().unwrap();
            }

            row_pin.set_low().unwrap();
            delay.delay_us(DELAY_US);
        }

        matrix
    }
}
