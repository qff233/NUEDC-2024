use embassy_stm32::mode::Blocking;
use embassy_stm32::peripherals;
use embassy_stm32::timer::qei::Qei;
use embassy_stm32::{gpio::Input, i2c::I2c};
use embassy_time::{block_for, Duration};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X13_BOLD, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use ssd1306_i2c::{interface::I2cInterface, mode::GraphicsMode};

enum State {
    EditHeight,
    None(bool),
}

enum ClickState {
    Release,
    Trig(bool),
}

struct ClickFilter {
    state: ClickState,
}
impl ClickFilter {
    pub fn new() -> Self {
        Self {
            state: ClickState::Release,
        }
    }

    pub fn update(&mut self, trig: bool) -> bool {
        match &mut self.state {
            ClickState::Release => {
                if trig {
                    self.state = ClickState::Trig(false);
                }
                false
            }
            ClickState::Trig(trigged) => {
                if *trigged {
                    if !trig {
                        self.state = ClickState::Release;
                        true
                    } else {
                        false
                    }
                } else {
                    if trig {
                        *trigged = true;
                    }
                    false
                }
            }
        }
    }
}

pub struct Interface<'a> {
    qei: Qei<'a, peripherals::TIM3>,
    button: Input<'a>,
    oled: GraphicsMode<I2cInterface<I2c<'a, Blocking>>>,
    target_height: f32,
    last_count: u16,
    click_filter: ClickFilter,
    state: State,
}

impl<'a> Interface<'a> {
    pub fn new(
        qei: Qei<'a, peripherals::TIM3>,
        click_pin: Input<'a>,
        i2c_dev: embassy_stm32::i2c::I2c<'a, Blocking>,
    ) -> Self {
        let mut oled: GraphicsMode<_> = ssd1306_i2c::Builder::new()
            .with_size(ssd1306_i2c::displaysize::DisplaySize::Display128x64NoOffset)
            .with_i2c_addr(0x3c)
            .with_rotation(ssd1306_i2c::displayrotation::DisplayRotation::Rotate0)
            .connect_i2c(i2c_dev)
            .into();
        block_for(Duration::from_millis(100));
        oled.init().unwrap();
        oled.flush().unwrap();

        let last_count = qei.count();
        Self {
            qei,
            button: click_pin,
            oled,
            target_height: 0.0,
            last_count,
            click_filter: ClickFilter::new(),
            state: State::None(true),
        }
    }

    fn draw_height(
        oled: &mut GraphicsMode<I2cInterface<I2c<'a, Blocking>>>,
        height: f32,
        edit: bool,
    ) {
        let text_style_bold = MonoTextStyleBuilder::new()
            .font(&FONT_6X13_BOLD)
            .text_color(BinaryColor::On)
            .build();
        if edit {
            Text::with_baseline("*", Point::new(100, 0), text_style_bold, Baseline::Top)
                .draw(oled)
                .unwrap();
        }
        Text::with_baseline("height:", Point::zero(), text_style_bold, Baseline::Top)
            .draw(oled)
            .unwrap();
        let mut buf = [0u8; 8];
        let s = format_no_std::show(&mut buf, format_args!("{:.2}", height)).unwrap();
        Text::with_baseline(s, Point::new(50, 0), text_style_bold, Baseline::Top)
            .draw(oled)
            .unwrap();
    }

    pub fn update(&mut self) {
        // 处理按压
        if self.click_filter.update(self.button.is_low()) {
            self.state = match self.state {
                State::EditHeight => State::None(true),
                State::None(_) => {
                    self.last_count = self.qei.count();
                    State::EditHeight
                }
            };
        }

        match &mut self.state {
            State::EditHeight => {
                // 处理旋转增量
                let mut delta_count = self.qei.count() as i32 - self.last_count as i32;
                self.last_count = self.qei.count();
                if delta_count.abs() > 65535 / 2 {
                    if delta_count.is_positive() {
                        delta_count = 65535 - delta_count
                    } else {
                        delta_count += 65535
                    }
                }

                let delta = delta_count as f32 / 4.0 * 0.01;
                self.target_height += delta;
                // debug!("delta_count:{}  height:{}", delta_count, self.target_height);

                // 编辑提示
                self.oled.clear();
                Self::draw_height(&mut self.oled, self.target_height, true);
                self.oled.flush().unwrap();
            }
            State::None(once) => {
                if *once {
                    // 编辑提示
                    // debug!("flush oled!");
                    self.oled.clear();
                    Self::draw_height(&mut self.oled, self.target_height, false);
                    self.oled.flush().unwrap();
                    *once = false;
                }
            }
        }
    }

    fn get_target_height(&self) -> f32 {
        self.target_height
    }
}
