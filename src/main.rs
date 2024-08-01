#![no_std]
#![no_main]

mod component;
mod lowpass;
mod pid;
mod tasks;

use component::coil::Coil;
use component::hall::Hall;
use component::tof050f::Tof050f;
use defmt::*;

use embassy_executor::Spawner;
use embassy_stm32::time::khz;
use embassy_stm32::timer::qei::{Qei, QeiPin};
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::usart::Uart;
use embassy_stm32::{adc, bind_interrupts, peripherals, usart, usb};
use embassy_stm32::{gpio, i2c};
use embassy_stm32::{time::mhz, Config};
use embassy_time::Timer;

use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    OTG_FS => usb::InterruptHandler<peripherals::USB_OTG_FS>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut config = Config::default();
    let p = {
        use embassy_stm32::rcc::*;
        config.rcc.sys = Sysclk::PLL1_P;
        config.rcc.hse = Some(Hse {
            freq: mhz(25),
            mode: HseMode::Oscillator,
        });
        config.rcc.pll_src = PllSource::HSE;
        config.rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV25,
            mul: PllMul::MUL168,
            divp: Some(PllPDiv::DIV2),
            divq: None,
            divr: None,
        });
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV2;
        config.rcc.apb2_pre = APBPrescaler::DIV1;
        // Main system clock at 170 MHz
        embassy_stm32::init(config)
    };

    info!("System starting...");
    info!("interface starting...");
    let mut i2c_config = i2c::Config::default();
    i2c_config.sda_pullup = true;
    i2c_config.scl_pullup = true;
    spawner
        .spawn(tasks::interface::interface_task(
            Qei::new(p.TIM3, QeiPin::new_ch1(p.PA6), QeiPin::new_ch2(p.PA7)),
            embassy_stm32::gpio::Input::new(p.PA5, embassy_stm32::gpio::Pull::Up),
            i2c::I2c::new_blocking(p.I2C2, p.PB10, p.PB3, khz(400), i2c_config),
        ))
        .unwrap();

    let usart_config = usart::Config::default();

    info!("control starting...");
    let balance_pwm = SimplePwm::new(
        p.TIM1,
        Some(PwmPin::new_ch1(p.PA8, gpio::OutputType::PushPull)),
        Some(PwmPin::new_ch2(p.PA9, gpio::OutputType::PushPull)),
        Some(PwmPin::new_ch3(p.PA10, gpio::OutputType::PushPull)),
        Some(PwmPin::new_ch4(p.PA11, gpio::OutputType::PushPull)),
        khz(10),
        embassy_stm32::timer::low_level::CountingMode::EdgeAlignedDown,
    );
    let height_pwm = SimplePwm::new(
        p.TIM4,
        Some(PwmPin::new_ch1(p.PB6, gpio::OutputType::PushPull)),
        Some(PwmPin::new_ch2(p.PB7, gpio::OutputType::PushPull)),
        None,
        None,
        khz(20),
        embassy_stm32::timer::low_level::CountingMode::EdgeAlignedDown,
    );
    spawner
        .spawn(tasks::control::control_task(
            Hall::new(adc::Adc::new(p.ADC1), p.PA0, p.PA1),
            Tof050f::new(Uart::new_blocking(p.USART2, p.PA3, p.PA2, usart_config).unwrap()),
            Coil::new(balance_pwm, height_pwm),
        ))
        .unwrap();

    // let mut pwm = SimplePwm::new(
    //     p.TIM4,
    //     Some(PwmPin::new_ch1(p.PB6, gpio::OutputType::PushPull)),
    //     Some(PwmPin::new_ch2(p.PB7, gpio::OutputType::PushPull)),
    //     None,
    //     None,
    //     khz(20),
    //     embassy_stm32::timer::low_level::CountingMode::EdgeAlignedDown,
    // );
    // pwm.set_duty(embassy_stm32::timer::Channel::Ch1, 0);
    // pwm.set_duty(embassy_stm32::timer::Channel::Ch2, pwm.get_max_duty() / 2);
    // pwm.enable(embassy_stm32::timer::Channel::Ch1);
    // pwm.enable(embassy_stm32::timer::Channel::Ch2);
    info!("start pwm");

    loop {
        info!("running");
        Timer::after_millis(1000).await
    }
}
