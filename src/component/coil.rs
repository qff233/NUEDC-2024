use defmt::debug;
use embassy_stm32::timer::{
    simple_pwm::SimplePwm, AdvancedInstance4Channel, Channel, GeneralInstance4Channel,
};
use micromath::F32Ext;

pub struct Coil<'a, Timer1: AdvancedInstance4Channel, Timer2: GeneralInstance4Channel> {
    balance_pwm: SimplePwm<'a, Timer1>,
    height_pwm: SimplePwm<'a, Timer2>,
    height_duty: u32,
}

impl<'a, Timer1: AdvancedInstance4Channel, Timer2: GeneralInstance4Channel>
    Coil<'a, Timer1, Timer2>
{
    pub fn new(
        mut balance_pwm: SimplePwm<'a, Timer1>,
        mut height_pwm: SimplePwm<'a, Timer2>,
    ) -> Self {
        balance_pwm.enable(Channel::Ch1);
        balance_pwm.enable(Channel::Ch2);
        balance_pwm.enable(Channel::Ch3);
        balance_pwm.enable(Channel::Ch4);

        height_pwm.set_duty(Channel::Ch2, height_pwm.get_max_duty() / 5 * 4);
        // height_pwm.set_duty(Channel::Ch2, 3200);
        height_pwm.enable(Channel::Ch2);
        // height_pwm.disable(Channel::Ch2);

        let height_duty = height_pwm.get_max_duty();
        Self {
            balance_pwm,
            height_pwm,
            height_duty,
        }
    }

    pub fn update_balance(&mut self, x: f32, y: f32) {
        let limit = self.balance_pwm.get_max_duty();

        // 1 y+吸 y-斥
        // 3 y+斥 y-吸
        // 2 x+吸 x-斥
        // 4 x+斥 x-吸

        // debug!("limit: {}", limit);
        if x.is_sign_positive() {
            // debug!("4");
            let mut duty = (x.round() as u32).min(self.balance_pwm.get_max_duty());
            duty = duty.min(limit);
            self.balance_pwm.set_duty(Channel::Ch2, 0);
            self.balance_pwm.set_duty(Channel::Ch4, duty);
            // debug!("x_pos_duty: {}", duty);
        } else {
            // debug!("2");
            let mut duty = (-x.round() as u32).min(self.balance_pwm.get_max_duty());
            duty = duty.min(limit);
            self.balance_pwm.set_duty(Channel::Ch2, duty);
            self.balance_pwm.set_duty(Channel::Ch4, 0);
            // debug!("x_neg_duty: {}", duty);
        }

        if y.is_sign_positive() {
            // debug!("3");
            let mut duty = (y.round() as u32).min(self.balance_pwm.get_max_duty());
            duty = duty.min(limit);
            self.balance_pwm.set_duty(Channel::Ch1, 0);
            self.balance_pwm.set_duty(Channel::Ch3, duty);
            // debug!("        y_pos_duty: {}", duty);
        } else {
            // debug!("1");
            let mut duty = (-y.round() as u32).min(self.balance_pwm.get_max_duty());
            duty = duty.min(limit);
            self.balance_pwm.set_duty(Channel::Ch1, duty);
            self.balance_pwm.set_duty(Channel::Ch3, 0);
            // debug!("y_neg_duty: {}", duty);
        }
    }

    pub fn update_height(&mut self, _value: f32) {
        // let new_duty = self.height_duty as f32 + value;
        // let limit = self.height_pwm.get_max_duty() as f32 / 3.0;
        // if new_duty < limit {
        //     self.height_duty = limit as u32;
        // }
        // self.height_pwm.set_duty(Channel::Ch2, self.height_duty);
    }
}
