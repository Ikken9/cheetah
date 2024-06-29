#![no_std]
#![no_main]

mod graphics;
mod display;
mod rf;
mod settings;
mod buzzer;
mod gui;
mod battery;
mod buttons;
mod wifi;

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::Infallible;
use core::mem::MaybeUninit;
use embedded_graphics_core::pixelcolor::Rgb888;
use esp_backtrace as _;
use esp_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, IO};
use esp_println::println;
use esp_hal::{timer::TimerGroup, Rng};
use esp_wifi::{EspWifiInitFor, initialize};
use tinybmp::Bmp;
use crate::buttons::ButtonMatrix;
use crate::display::{ILI9488, Orientation};
use crate::gui::{bluetooth_text, Level, Menu, settings_text, show_loadscreen_logo, show_main_menu, show_wifi_menu, wifi_text};

#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }
}

#[entry]
#[allow(unused)]
fn main() -> ! {
    init_heap();
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let mut io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let clocks = ClockControl::max(system.clock_control).freeze();
    let mcpwm0 = peripherals.MCPWM0;

    let spi2 = peripherals.SPI2;

    let sclk = io.pins.gpio12;
    let mosi = io.pins.gpio11;
    let miso = io.pins.gpio13;
    let cs = io.pins.gpio10;

    let dc = io.pins.gpio15.into_push_pull_output();
    let rst = io.pins.gpio16.into_push_pull_output();

    let rows: Vec<Box<dyn _embedded_hal_digital_v2_InputPin<Error = Infallible>>> = vec![
        Box::new(io.pins.gpio4.into_pull_down_input()),
        Box::new(io.pins.gpio5.into_pull_down_input()),
        Box::new(io.pins.gpio6.into_pull_down_input())
    ];

    let columns: Vec<Box<dyn _embedded_hal_digital_v2_OutputPin<Error = Infallible>>> = vec![
        Box::new(io.pins.gpio1.into_push_pull_output()),
        Box::new(io.pins.gpio2.into_push_pull_output())
    ];

    let mut matrix = ButtonMatrix::new(
        rows,
        columns,
        &clocks
    );

    let buzzer_pin = io.pins.gpio8;
    // setup logger
    // To change the log_level change the env section in .cargo/config.toml
    // or remove it and set ESP_LOGLEVEL manually before running cargo run
    // this requires a clean rebuild because of https://github.com/rust-lang/cargo/issues/10358
    esp_println::logger::init_logger_from_env();
    log::info!("Logger is setup");
    println!("Hello world!");
    let timer = TimerGroup::new(peripherals.TIMG1, &clocks).timer0;
    let _init = initialize(
        EspWifiInitFor::Wifi,
        timer,
        Rng::new(peripherals.RNG),
        system.radio_clock_control,
        &clocks,
    )
        .unwrap();

    let mut display = ILI9488::new(spi2, sclk, mosi, miso, cs, dc, rst, &clocks);

    display.init(Orientation::Portrait).expect("Expected to init display");

    show_loadscreen_logo(&mut display);

    let bmp: Bmp<Rgb888> = Bmp::from_slice(include_bytes!("../assets/cursor.bmp")).unwrap();

    let mut menu: Menu<fn(&mut ILI9488)> = Menu::new(&mut display, &bmp, 12, 36);

    let main_menu_level = Level::new(Some(show_main_menu), 12, 36);
    let wifi_menu_level = Level::new(Some(show_wifi_menu), 12, 36);

    let wifi_text = Level::new(None, 12, 36);
    let bluetooth_text = Level::new(None, 12, 61);
    let settings_text = Level::new(None, 12, 86);

    menu.append_horizontal(main_menu_level);
    menu.append_vertical(wifi_text);
    menu.append_horizontal(wifi_menu_level);
    menu.append_vertical(bluetooth_text);
    menu.append_vertical(settings_text);

    menu.set_focus(menu.head.clone());

    // let mut buzzer = Buzzer::new(buzzer_pin, mcpwm0, &clocks);
    // buzzer.init();

    //menu.set_focus(menu.head.clone());


    loop {
        matrix.poll(&mut menu);
        //buzzer.buzz(&clocks);
    }
}


