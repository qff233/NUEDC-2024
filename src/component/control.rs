use defmt::debug;
use embassy_stm32::{
    adc::{self, AdcChannel},
    timer::{AdvancedInstance4Channel, GeneralInstance4Channel},
};
use embassy_time::Duration;

use crate::{lowpass::LowPass, pid::Pid};

use super::{coil::Coil, hall::Hall, tof050f::Tof050f};

pub struct Control<
    'a,
    Instance: adc::Instance,
    XChannel: AdcChannel<Instance>,
    YChannel: AdcChannel<Instance>,
    Timer1: AdvancedInstance4Channel,
    Timer2: GeneralInstance4Channel,
> {
    hall: Hall<'a, Instance, XChannel, YChannel>,
    tof050: Tof050f<'a>,
    coil: Coil<'a, Timer1, Timer2>,
    target_height: f32,
    balance_pid: (Pid, Pid),
    height_pid: Pid,
    hall_lowpass: (LowPass, LowPass),
}

impl<
        'a,
        Instance: adc::Instance,
        XChannel: AdcChannel<Instance>,
        YChannel: AdcChannel<Instance>,
        Timer1: AdvancedInstance4Channel,
        Timer2: GeneralInstance4Channel,
    > Control<'a, Instance, XChannel, YChannel, Timer1, Timer2>
{
    pub fn new(
        mut hall: Hall<'a, Instance, XChannel, YChannel>,
        tof050: Tof050f<'a>,
        coil: Coil<'a, Timer1, Timer2>,
        balance_x_pid: Pid,
        balance_y_pid: Pid,
        height_pid: Pid,
        hall_lowpass: (LowPass, LowPass),
    ) -> Self {
        embassy_time::block_for(Duration::from_millis(500));
        hall.calibi();
        let balance_pid = (balance_x_pid, balance_y_pid);
        Self {
            hall,
            tof050,
            coil,
            target_height: 1.0,
            balance_pid,
            height_pid,
            hall_lowpass,
        }
    }

    pub fn set_height(&mut self, target_height: f32) {
        self.target_height = target_height
    }

    fn balance_control_tick(&mut self) {
        let (mut x_error, mut y_error) = self.hall.get_value();
        x_error = self.hall_lowpass.0.update(x_error);
        y_error = self.hall_lowpass.1.update(y_error);
        // debug!("hall_value: {} {}", x_error, y_error);
        let x_offset = self.balance_pid.0.update(x_error);
        let y_offset = self.balance_pid.1.update(y_error);
        // debug!("offset: {} {}", x_offset, y_offset);

        self.coil.update_balance(x_offset, y_offset);
    }

    fn height_control_tick(&mut self) {
        // let current_height = self.tof050.get_value();
        // // debug!("tof050f: {}cm", current_height);
        // let result = if current_height > 100.0 {
        //     0.0
        // } else {
        //     let error = self.target_height - current_height;
        //     self.height_pid.update(error)
        // };
        // self.coil.update_height(result);
    }

    pub fn tick(&mut self) {
        self.balance_control_tick();
        self.height_control_tick();
    }
}
