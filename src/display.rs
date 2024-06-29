use core::convert::Infallible;

use embedded_graphics_core::geometry::Point;
use embedded_graphics_core::pixelcolor::{Rgb888, RgbColor};
use esp_hal::clock::Clocks;
use esp_hal::Delay;
use esp_hal::gpio::{GpioPin, Output, PushPull, Unknown};
use esp_hal::peripherals::SPI2;
use esp_hal::prelude::{_embedded_hal_blocking_delay_DelayMs, _embedded_hal_blocking_spi_Write, _embedded_hal_digital_v2_OutputPin};
use esp_hal::spi::{FullDuplexMode, SpiMode};
use esp_hal::spi::master::Spi;
use fugit::RateExtU32;

pub struct ILI9488<'a> {
    spi: Spi<'static, SPI2, FullDuplexMode>,
    dc: GpioPin<Output<PushPull>, 15>,
    rst: GpioPin<Output<PushPull>, 16>,
    clocks: &'a Clocks<'a>
}

pub enum Orientation {
    Portrait,
    PortraitInverted,
    Landscape,
    LandscapeInverted
}

pub trait OrientationMode {
    fn orientation_mode(&self) -> u8;
}

#[allow(unused)]
pub enum Command {
    /* Level 2 Commands */
    Nop = 0x00,
    SoftwareReset = 0x01,
    ReadDispId = 0x04,
    ReadErrorDsi = 0x05,
    ReadDispStatus = 0x09,
    ReadDispPowerMode = 0x0A,
    ReadDispMadctrl = 0x0B,
    ReadDispPixelFormat = 0x0C,
    ReadDispImageMode = 0x0D,
    ReadDispSignalMode = 0x0E,
    ReadDispSelfDiagnostic = 0x0F,
    EnterSleepMode = 0x10,
    SleepOut = 0x11,
    PartialModeOn = 0x12,
    NormalDispModeOn = 0x13,
    DispInversionOff = 0x20,
    DispInversionOn = 0x21,
    PixelOff = 0x22,
    PixelOn = 0x23,
    DisplayOff = 0x28,
    DisplayOn = 0x29,
    ColumnAddressSet = 0x2A,
    PageAddressSet = 0x2B,
    MemoryWrite = 0x2C,
    MemoryRead = 0x2E,
    PartialArea = 0x30,
    VertScrollDefinition = 0x33,
    TearingEffectLineOff = 0x34,
    TearingEffectLineOn = 0x35,
    MemoryAccessControl = 0x36,
    VertScrollStartAddress = 0x37,
    IdleModeOff = 0x38,
    IdleModeOn = 0x39,
    ColmodPixelFormatSet = 0x3A,
    WriteMemoryContinue = 0x3C,
    ReadMemoryContinue = 0x3E,
    SetTearScanline = 0x44,
    GetScanline = 0x45,
    WriteDisplayBrightness = 0x51,
    ReadDisplayBrightness = 0x52,
    WriteCtrlDisplay = 0x53,
    ReadCtrlDisplay = 0x54,
    WriteContentAdaptBrightness = 0x55,
    ReadContentAdaptBrightness = 0x56,
    WriteMinCabLevel = 0x5E,
    ReadMinCabLevel = 0x5F,
    ReadAbcSelfDiagRes = 0x68,
    ReadId1 = 0xDA,
    ReadId2 = 0xDB,
    ReadId3 = 0xDC,

    /* Level 2 Commands */
    InterfaceModeControl = 0xB0,
    FrameRateControlNormal = 0xB1,
    FrameRateControlIdle8Color = 0xB2,
    FrameRateControlPartial = 0xB3,
    DisplayInversionControl = 0xB4,
    BlankingPorchControl = 0xB5,
    DisplayFunctionControl = 0xB6,
    EntryModeSet = 0xB7,
    BacklightControl1 = 0xB9,
    BacklightControl2 = 0xBA,
    HsLanesControl = 0xBE,
    PowerControl1 = 0xC0,
    PowerControl2 = 0xC1,
    PowerControlNormal3 = 0xC2,
    PowerControlIdel4 = 0xC3,
    PowerControlPartial5 = 0xC4,
    VcomControl1 = 0xC5,
    CabcControl1 = 0xC6,
    CabcControl2 = 0xC8,
    CabcControl3 = 0xC9,
    CabcControl4 = 0xCA,
    CabcControl5 = 0xCB,
    CabcControl6 = 0xCC,
    CabcControl7 = 0xCD,
    CabcControl8 = 0xCE,
    CabcControl9 = 0xCF,
    NvmemWrite = 0xD0,
    NvmemProtectionKey = 0xD1,
    NvmemStatusRead = 0xD2,
    ReadId4 = 0xD3,
    AdjustControl1 = 0xD7,
    ReadIdVersion = 0xD8,
    PositiveGammaCorrection = 0xE0,
    NegativeGammaCorrection = 0xE1,
    DigitalGammaControl1 = 0xE2,
    DigitalGammaControl2 = 0xE3,
    SetImageFunction = 0xE9,
    AdjustControl2 = 0xF2,
    AdjustControl3 = 0xF7,
    AdjustControl4 = 0xF8,
    AdjustControl5 = 0xF9,
    SpiReadSettings = 0xFB,
    AdjustControl6 = 0xFC,
    AdjustControl7 = 0xFF

}

impl ILI9488<'_> {
    pub fn new<'a>(spi2: SPI2,
                   sclk: GpioPin<Unknown, 12>,
                   mosi: GpioPin<Unknown, 11>,
                   miso: GpioPin<Unknown, 13>,
                   cs: GpioPin<Unknown, 10>,
                   dc: GpioPin<Output<PushPull>, 15>,
                   rst: GpioPin<Output<PushPull>, 16>,
                   clocks: &'a Clocks<'a>) -> ILI9488<'a> {

        let spi = Spi::new(spi2, 40000.kHz(), SpiMode::Mode0, clocks).with_pins(
            Some(sclk),
            Some(mosi),
            Some(miso),
            Some(cs),
        );

        ILI9488 {
            spi,
            dc,
            rst,
            clocks
        }
    }

    pub fn init(&mut self, orientation: Orientation) -> Result<(), Infallible> {
        self.hard_reset().expect("Expected hardware reset");
        self.reset().expect("Expected software reset ");

        self.sleep_out().expect("Expected sleep out");
        self.memory_access_control().expect("Expected Memory Access Control mode");

        self.set_dbi_mode().expect("Expected to set DBI mode");
        self.enable_partial_mode().expect("Expected to set Partial Mode On");
        self.display_on().expect("Expected to turn On Display");
        self.set_cursor().expect("Expected to Set Cursor");

        self.set_brightness(0x0F).expect("Expected to Set Brightness");
        self.set_brightness_control().expect("Expected to Set Brightness Control");
        self.set_framerate().expect("Expected to set framerate");

        self.set_orientation(orientation).expect("Expected to set screen orientation");

        self.clear(Rgb888::BLACK);

        Ok(())
    }

    fn send_command(&mut self, cmd: u8) -> Result<(), DisplayError> {
        self.dc.set_low().map_err(|_| DisplayError::GpioError)?;
        self.spi.write(&[cmd]).map_err(|_| DisplayError::SpiCommunicationError)?;
        Ok(())
    }

    fn send_data(&mut self, data: &[u8]) -> Result<(), DisplayError> {
        self.dc.set_high().map_err(|_| DisplayError::GpioError)?;
        self.spi.write(data).map_err(|_| DisplayError::SpiCommunicationError)?;
        Ok(())
    }

    fn reset(&mut self) -> Result<(), DisplayError> {
        self.send_command(Command::SoftwareReset as u8)?;
        Delay::new(&self.clocks).delay_ms(100u32);
        self.send_data(&[0x00])?;
        Delay::new(&self.clocks).delay_ms(100u32);
        Ok(())
    }

    fn hard_reset(&mut self) -> Result<(), DisplayError> {
        self.rst.set_low().map_err(|_| DisplayError::GpioError)?;
        Delay::new(&self.clocks).delay_ms(100u32);
        self.rst.set_high().map_err(|_| DisplayError::GpioError)?;
        Delay::new(&self.clocks).delay_ms(100u32);
        Ok(())
    }

    fn sleep_out(&mut self) -> Result<(), DisplayError> {
        self.send_command(Command::SleepOut as u8)?;
        Delay::new(&self.clocks).delay_ms(100u32);
        Ok(())
    }

    fn memory_access_control(&mut self) -> Result<(), DisplayError> {
        self.send_command(Command::MemoryAccessControl as u8)?;
        self.send_data(&[0xE8])?;
        Delay::new(&self.clocks).delay_ms(100u32);
        Ok(())
    }

    fn set_cursor(&mut self) -> Result<(), DisplayError> {
        // Column Address Set
        self.send_command(Command::ColumnAddressSet as u8)?;
        // set start x
        self.send_data(&[0x00])?;
        self.send_data(&[0x00])?;
        // set end x
        self.send_data(&[0x01])?;
        self.send_data(&[0x3F])?;
        self.send_command(Command::Nop as u8)?;

        // Page Address Set
        self.send_command(Command::PageAddressSet as u8)?;
        // set start y
        self.send_data(&[0x00])?;
        self.send_data(&[0x00])?;
        // set end y
        self.send_data(&[0x01])?;
        self.send_data(&[0xDF])?;
        self.send_command(Command::Nop as u8)?;

        Delay::new(&self.clocks).delay_ms(100u32);
        Ok(())
    }

    fn set_dbi_mode(&mut self) -> Result<(), DisplayError> {
        self.send_command(Command::ColmodPixelFormatSet as u8)?;
        self.send_data(&[0x06])?;
        Delay::new(&self.clocks).delay_ms(100u32);
        Ok(())
    }

    fn enable_partial_mode(&mut self) -> Result<(), DisplayError> {
        self.send_command(Command::PartialModeOn as u8)?;
        Delay::new(&self.clocks).delay_ms(100u32);
        Ok(())
    }

    fn set_orientation(&mut self, orientation: Orientation) -> Result<(), DisplayError> {
        self.send_command(Command::MemoryAccessControl as u8)?;
        self.send_data(&[orientation as u8])?;
        Ok(())
    }

    fn set_brightness(&mut self, brightness: u8) -> Result<(), DisplayError> {
        self.send_command(Command::WriteDisplayBrightness as u8)?;
        self.send_data(&[brightness])?;
        Delay::new(&self.clocks).delay_ms(100u32);
        Ok(())
    }

    fn set_brightness_control(&mut self) -> Result<(), DisplayError> {
        self.send_command(Command::WriteCtrlDisplay as u8)?;
        self.send_data(&[0x2C])?;
        Delay::new(&self.clocks).delay_ms(100u32);
        Ok(())
    }

    fn display_on(&mut self) -> Result<(), DisplayError> {
        self.send_command(Command::DisplayOn as u8)?;
        Delay::new(&self.clocks).delay_ms(100u32);
        Ok(())
    }

    fn display_off(&mut self) -> Result<(), DisplayError> {
        self.send_command(Command::DisplayOff as u8)?;
        Ok(())
    }

    fn set_framerate(&mut self) -> Result<(), DisplayError> {
        self.send_command(Command::FrameRateControlNormal as u8)?;
        self.send_data(&[0xB0])?;
        self.send_data(&[0x11])?;
        Delay::new(&self.clocks).delay_ms(50u32);
        Ok(())
    }

    pub fn draw_pixel(&mut self, x: u16, y: u16, color: Rgb888) {
        self.set_pixel_destination_address(x, y).expect("TODO: panic message");
        self.send_command(Command::MemoryWrite as u8).expect("TODO: panic message");
        self.send_data(&[color.b(), color.g(), color.r()]).expect("TODO: panic message");
    }

    pub fn draw_contiguous<I: IntoIterator<Item = (Point, Rgb888)>>(&mut self, data: I) {
        for (point, color) in data {
            self.draw_pixel(point.x as u16, point.y as u16, color)
        }
    }

    pub fn fill<I: Iterator<Item = Point>>(&mut self, points: I, color: Rgb888) {
        for point in points {
            self.draw_pixel(point.x as u16, point.y as u16, color)
        }
    }

    pub fn clear(&mut self, color: Rgb888) {
        self.send_command(Command::MemoryWrite as u8).expect("TODO: panic message");
        for _ in 0..480 {
            for _ in 0..320 {
                self.send_data(&[color.b(), color.g(), color.r()]).expect("Unable to perform clear screen")
            }
        }
    }

    fn set_pixel_destination_address(&mut self, x: u16, y: u16) -> Result<(), DisplayError> {
        // let xt = (x >> 8) & 0xFF;
        // let xb = x & 0xFF;
        let xt = ((0x0140 - 0x01 - x) >> 8) & 0xFF;
        let xb = (0x0140 - 0x01 - x) & 0xFF;

        let yt = (y >> 8) & 0xFF;
        let yb = y & 0xFF;

        self.send_command(Command::ColumnAddressSet as u8)?;
        // send start x
        self.send_data(&[xt as u8])?;
        self.send_data(&[xb as u8])?;
        // send end x
        self.send_data(&[xt as u8])?;
        self.send_data(&[xb as u8])?;
        self.send_command(Command::Nop as u8)?;

        self.send_command(Command::PageAddressSet as u8)?;
        // set start y
        self.send_data(&[yt as u8])?;
        self.send_data(&[yb as u8])?;
        // set end y
        self.send_data(&[yt as u8])?;
        self.send_data(&[yb as u8])?;
        self.send_command(Command::Nop as u8)?;
        Ok(())
    }
}

impl OrientationMode for Orientation {
    fn orientation_mode(&self) -> u8 {
        match self {
            Orientation::Portrait => 0x40 | 0x08,
            Orientation::Landscape => 0x20 | 0x08,
            Orientation::PortraitInverted => 0x80 | 0x08,
            Orientation::LandscapeInverted => 0x40 | 0x80 | 0x20 | 0x08,
        }
    }
}

#[derive(Debug)]
pub enum DisplayError {
    SpiCommunicationError,
    GpioError,
}