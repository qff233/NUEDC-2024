use embassy_stm32::{
    adc::{self, AdcChannel},
    timer::GeneralInstance4Channel,
};

use crate::pid::Pid;

use super::{coil::Coil, hall::Hall};

pub struct Control<
    'a,
    Instance: adc::Instance,
    XChannel: AdcChannel<Instance>,
    YChannel: AdcChannel<Instance>,
    Timer: GeneralInstance4Channel,
> {
    hall: Hall<'a, Instance, XChannel, YChannel>,
    coil: Coil<'a, Timer>,
    target_height: f32,
    balance_pid: (Pid, Pid),
    height_pid: Pid,
}

impl<
        'a,
        Instance: adc::Instance,
        XChannel: AdcChannel<Instance>,
        YChannel: AdcChannel<Instance>,
        Timer: GeneralInstance4Channel,
    > Control<'a, Instance, XChannel, YChannel, Timer>
{
    pub fn new(
        hall: Hall<'a, Instance, XChannel, YChannel>,
        coil: Coil<'a, Timer>,
        balance_x_pid: Pid,
        balance_y_pid: Pid,
        height_pid: Pid,
    ) -> Self {
        let balance_pid = (balance_x_pid, balance_y_pid);
        Self {
            hall,
            coil,
            target_height: 2.0,
            balance_pid,
            height_pid,
        }
    }

    pub fn set_height(&mut self, target_height: f32) {
        self.target_height = target_height
    }

    fn balance_tick(&mut self) {
        let (x_error, y_error) = self.hall.get_value();
        let x_offset = self.balance_pid.0.update(x_error);
        let y_offset = self.balance_pid.1.update(y_error);

        self.coil.update_diff(x_offset, y_offset);
    }

    fn height_tick(&mut self) {
        let current_height = 0.0;
        let error = self.target_height - current_height;
        let result = self.height_pid.update(error);
        self.coil.update_common(result);
    }

    pub fn tick(&mut self) {
        self.balance_tick();
        self.height_tick();
        self.coil.flush();
    }
}
