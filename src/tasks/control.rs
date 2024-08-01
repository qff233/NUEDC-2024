use defmt::debug;
use embassy_stm32::peripherals;
use embassy_time::Timer;

use crate::component::coil::Coil;
use crate::component::control::Control;
use crate::component::hall::Hall;
use crate::component::tof050f::Tof050f;
use crate::lowpass::LowPass;
use crate::pid::Pid;

#[embassy_executor::task]
pub async fn control_task(
    hall: Hall<'static, peripherals::ADC1, peripherals::PA0, peripherals::PA1>,
    tof050f: Tof050f<'static>,
    coil: Coil<'static, peripherals::TIM1, peripherals::TIM4>,
) {
    let mut control = Control::new(
        hall,
        tof050f,
        coil,
        // 10s  height_pwm.set_duty(Channel::Ch2, height_pwm.get_max_duty() / 5 * 4);
        // Pid::new(2525000.0, 0.0, 2200.0, 0.00005, 0.0, 8400.0, 8400.0, 4000.0),
        // Pid::new(1525000.0, 0.0, 1500.0, 0.00005, 0.0, 8400.0, 8400.0, 4000.0),
        // Pid::new(1.0, 0.0, 0.0, 0.0001, 0.0, 50.0, 50.0, 2000.0),
        // (LowPass::new(0.00005, 0.0002), LowPass::new(0.00005, 0.0002)),
        Pid::new(2525000.0, 0.0, 2250.0, 0.00005, 0.0, 8400.0, 8400.0, 8000.0),
        Pid::new(1525000.0, 0.0, 1800.0, 0.00005, 0.0, 8400.0, 8400.0, 8000.0),
        Pid::new(1.0, 0.0, 0.0, 0.0001, 0.0, 50.0, 50.0, 2000.0),
        (LowPass::new(0.00005, 0.0002), LowPass::new(0.00005, 0.0002)),
    );

    loop {
        // let tick1 = embassy_time::Instant::now();
        control.tick();
        // let tick2 = embassy_time::Instant::now();
        // debug!("time: {}", tick2 - tick1);
        Timer::after_micros(50).await;
    }
}
