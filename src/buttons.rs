use alloc::boxed::Box;
use alloc::vec::Vec;
use core::convert::Infallible;
use esp_hal::clock::Clocks;
use esp_hal::Delay;
use esp_hal::prelude::{_embedded_hal_blocking_delay_DelayMs, _embedded_hal_digital_v2_InputPin, _embedded_hal_digital_v2_OutputPin};
use esp_println::println;
use crate::display::ILI9488;

use crate::gui::Menu;

pub struct ButtonMatrix<'a, R, C>
    where
        R: _embedded_hal_digital_v2_InputPin<Error = Infallible> + ?Sized,
        C: _embedded_hal_digital_v2_OutputPin<Error = Infallible> + ?Sized
{
    rows: Vec<Box<R>>,
    columns: Vec<Box<C>>,
    clocks: &'a Clocks<'a>
}

impl<'a, R: ?Sized, C: ?Sized> ButtonMatrix<'a, R, C>
    where
        R: _embedded_hal_digital_v2_InputPin<Error = Infallible>,
        C: _embedded_hal_digital_v2_OutputPin<Error = Infallible>
{
    pub fn new(rows: Vec<Box<R>>, columns: Vec<Box<C>>, clocks: &'a Clocks<'a>) -> ButtonMatrix<'a, R, C> {
        ButtonMatrix {
            rows,
            columns,
            clocks
        }
    }

    pub fn poll<>(&mut self, menu: &mut Menu<fn(&mut ILI9488)>)
        where <R as _embedded_hal_digital_v2_InputPin>::Error: core::fmt::Debug,
              <C as _embedded_hal_digital_v2_OutputPin>::Error: core::fmt::Debug
    {
        let mut delay = Delay::new(self.clocks);
        for (col_idx, col) in self.columns.iter_mut().enumerate() {
            col.set_high().unwrap();
            for (row_idx, row) in self.rows.iter_mut().enumerate() {
                if row.is_high().unwrap() {
                    match (row_idx, col_idx) {
                        (0, 0) => {
                            println!("Handled Right");
                            delay.delay_ms(400u32);
                        },
                        (0, 1) => {
                            println!("Handled Down");
                            menu.move_down();
                            delay.delay_ms(400u32);
                        },
                        (1, 0) => {
                            println!("Handled Up");
                            menu.move_up();
                            delay.delay_ms(400u32);
                        },
                        (1, 1) => {
                            println!("Handled OK");
                            menu.move_ok();
                            delay.delay_ms(400u32);
                        },
                        (2, 0) => {
                            println!("Handled Left");
                            delay.delay_ms(400u32);
                        },
                        (2, 1) => {
                            println!("Handled Return");
                            menu.move_return();
                            delay.delay_ms(400u32);
                        }
                        _ => {println!("no match");}
                    }
                }
            }
            col.set_low().unwrap();
        }
    }
}

#[derive(Debug)]
pub enum ButtonError {
    GpioError
}