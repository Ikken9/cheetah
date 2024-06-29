use esp_hal::clock::Clocks;
use esp_hal::gpio::{GpioPin, Unknown};
use esp_hal::mcpwm::{MCPWM, PeripheralClockConfig};
use esp_hal::mcpwm::operator::PwmPinConfig;
use esp_hal::mcpwm::timer::PwmWorkingMode;
use esp_hal::peripherals::MCPWM0;
use fugit::RateExtU32;

pub struct Buzzer<'a> {
    pin: GpioPin<Unknown, 8>,
    mcpwm0: MCPWM0,
    clocks: &'a Clocks<'a>
}


impl Buzzer<'_> {
    pub fn new<'a>(pin: GpioPin<Unknown, 8>, mcpwm0: MCPWM0, clocks: &'a Clocks<'a>) -> Buzzer<'a> {
        Buzzer {
            pin,
            mcpwm0,
            clocks
        }
    }

    pub fn init(&mut self) {
        let clock_cfg = PeripheralClockConfig::with_frequency(self.clocks, 32.MHz()).unwrap();
        let mut mcpwm = MCPWM::new(&mut self.mcpwm0, clock_cfg);

        mcpwm.operator0.set_timer(&mcpwm.timer0);

        let mut pwm_pin = mcpwm
            .operator0
            .with_pin_a(&mut self.pin, PwmPinConfig::UP_ACTIVE_HIGH);

        let timer_clock_cfg = clock_cfg
            .timer_clock_with_frequency(99, PwmWorkingMode::Increase, 4.kHz())
            .unwrap();
        mcpwm.timer0.start(timer_clock_cfg);

        pwm_pin.set_timestamp(50);
    }

    // pub fn buzz(&mut self, clocks: &Clocks<'static>) {
    //     self.pin.set_high().unwrap();
    //     Delay::new(&clocks).delay_ms(500u32);
    //     self.pin.set_low().unwrap();
    //     Delay::new(&clocks).delay_ms(500u32);
    // }
}
