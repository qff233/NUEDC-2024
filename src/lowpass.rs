pub struct LowPass {
    dt: f32,
    tf: f32,
    y_prev: f32,
}

impl LowPass {
    pub fn new(dt: f32, tf: f32) -> Self {
        Self {
            dt,
            tf,
            y_prev: 0.0,
        }
    }

    pub fn update(&mut self, y: f32) -> f32 {
        let alpha = self.tf / (self.tf + self.dt);
        let result = alpha * self.y_prev + (1.0 - alpha) * y;
        self.y_prev = result;
        result
    }
}
