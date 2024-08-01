use defmt::debug;
use embassy_stm32::adc::{self, Adc, AdcChannel};
use embassy_time::Duration;

pub struct Hall<
    'a,
    Instance: adc::Instance,
    XChannel: AdcChannel<Instance>,
    YChannel: AdcChannel<Instance>,
> {
    adc: Adc<'a, Instance>,
    x_hall_pin: XChannel,
    y_hall_pin: YChannel,
    offset: (f32, f32),
}

impl<
        'a,
        Instance: adc::Instance,
        XChannel: AdcChannel<Instance>,
        YChannel: AdcChannel<Instance>,
    > Hall<'a, Instance, XChannel, YChannel>
{
    fn get_raw(
        adc: &mut Adc<'a, Instance>,
        x_hall_pin: &mut XChannel,
        y_hall_pin: &mut YChannel,
    ) -> (f32, f32) {
        (
            adc.blocking_read(x_hall_pin) as f32 / 32767.5 - 1.0,
            adc.blocking_read(y_hall_pin) as f32 / 32767.5 - 1.0,
        )
    }
    pub fn new(
        mut adc: Adc<'a, Instance>,
        mut x_hall_pin: XChannel,
        mut y_hall_pin: YChannel,
    ) -> Self {
        Self {
            adc,
            x_hall_pin,
            y_hall_pin,
            offset: (0.0, 0.0),
        }
    }

    pub fn calibi(&mut self) {
        let mut x_count: f32 = 0.0;
        let mut y_count: f32 = 0.0;

        for _ in 0..500 {
            let (x, y) = Self::get_raw(&mut self.adc, &mut self.x_hall_pin, &mut self.y_hall_pin);
            x_count += x;
            y_count += y;
            embassy_time::block_for(Duration::from_millis(1));
        }

        let x_offset = x_count / 500.0;
        let y_offset = y_count / 500.0;
        debug!("x,y offset: {},{}", x_offset, y_offset);
        self.offset = (x_offset, y_offset);
    }

    pub fn get_value(&mut self) -> (f32, f32) {
        let (x, y) = Self::get_raw(&mut self.adc, &mut self.x_hall_pin, &mut self.y_hall_pin);
        (x - self.offset.0, y - self.offset.1)
    }
}
