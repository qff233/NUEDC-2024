use embassy_stm32::timer::{simple_pwm::SimplePwm, Channel, GeneralInstance4Channel};

pub struct Coil<'a, Timer: GeneralInstance4Channel> {
    pwm: SimplePwm<'a, Timer>,
    duty: [u32; 4],
}

impl<'a, Timer: GeneralInstance4Channel> Coil<'a, Timer> {
    pub fn new(mut pwm: SimplePwm<'a, Timer>) -> Self {
        let max = pwm.get_max_duty();
        pwm.enable(Channel::Ch1);
        pwm.enable(Channel::Ch2);
        pwm.enable(Channel::Ch3);
        pwm.enable(Channel::Ch4);
        Self {
            pwm,
            duty: [max / 2; 4],
        }
    }

    pub fn flush(&mut self) {
        let limit = self.pwm.get_max_duty() * 4 / 5;
        for duty in &mut self.duty {
            if *duty > limit {
                *duty = limit
            }
        }
        self.pwm.set_duty(Channel::Ch1, self.duty[0]);
        self.pwm.set_duty(Channel::Ch2, self.duty[1]);
        self.pwm.set_duty(Channel::Ch3, self.duty[2]);
        self.pwm.set_duty(Channel::Ch4, self.duty[3]);
    }

    pub fn update_diff(&mut self, x: f32, y: f32) {
        let delta_x = x / 2.0;
        let delta_y = y / 2.0;

        self.duty[0] += delta_x as u32;
        self.duty[1] += delta_y as u32;
        self.duty[2] -= delta_x as u32;
        self.duty[3] -= delta_y as u32;
    }

    pub fn update_common(&mut self, value: f32) {
        self.duty[0] += value as u32;
        self.duty[1] += value as u32;
        self.duty[2] += value as u32;
        self.duty[3] += value as u32;
    }
}
