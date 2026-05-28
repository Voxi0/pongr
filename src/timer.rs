use sola_raylib::prelude::*;

pub struct Timer {
    lifeTime: f64,
}

impl Timer {
    pub fn new(timeInSeconds: f64) -> Result<Self, String> {
        // Ensure timer lifetime is more than zero seconds and is a non-zero value
        if timeInSeconds <= 0.0 {
            return Err("Timer should run for more than zero seconds and cannot be a negative number".to_string());
        }

        // Return an instance of the timer
        return Ok(Self {
            lifeTime: timeInSeconds,
        });
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        if self.lifeTime > 0.0 {
            self.lifeTime -= rl.get_frame_time() as f64;
        }
    }

    pub fn timerDone(&self) -> bool {
        return self.lifeTime <= 0.0;
    }
}