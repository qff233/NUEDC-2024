use embassy_stm32::peripherals;
use embassy_time::Timer;

use crate::component::coil::Coil;
use crate::component::control::Control;
use crate::component::hall::Hall;
use crate::pid::Pid;

#[embassy_executor::task]
pub async fn control_task(
    hall: Hall<'static, peripherals::ADC1, peripherals::PA0, peripherals::PA1>,
    coil: Coil<'static, peripherals::TIM1>,
) {
    let balance_p = 1.0;
    let balance_i = 0.0;
    let balance_d = 0.0;
    let balance_time_interval = 0.0001;
    let balance_output_ramp = 0.0;
    let balance_output_limit = 50.0;
    let balance_integral_limit = 50.0;
    let mut control = Control::new(
        hall,
        coil,
        Pid::new(
            balance_p,
            balance_i,
            balance_d,
            balance_time_interval,
            balance_output_ramp,
            balance_output_limit,
            balance_integral_limit,
        ),
        Pid::new(
            balance_p,
            balance_i,
            balance_d,
            balance_time_interval,
            balance_output_ramp,
            balance_output_limit,
            balance_integral_limit,
        ),
        Pid::new(1.0, 0.0, 0.0, 0.0001, 0.0, 50.0, 50.0),
    );

    loop {
        control.tick();
        Timer::after_micros(100).await;
    }
}
