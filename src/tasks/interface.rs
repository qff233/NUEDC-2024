use embassy_stm32::{gpio, i2c::I2c, mode::Blocking, peripherals, timer::qei::Qei};
use embassy_time::Timer;

use crate::component::interface::Interface;

#[embassy_executor::task]
pub async fn interface_task(
    qei: Qei<'static, peripherals::TIM3>,
    button: gpio::Input<'static>,
    i2c: I2c<'static, Blocking>,
) {
    let mut interface = Interface::new(qei, button, i2c);
    loop {
        interface.update();
        Timer::after_millis(100).await
    }
}
