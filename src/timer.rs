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

    pub fn update(&mut self, frameTime: f64) {
        if self.lifeTime > 0.0 {
            self.lifeTime -= frameTime;
        }
    }

    pub fn done(&self) -> bool {
        return self.lifeTime <= 0.0;
    }

    pub fn setLifeTime(&mut self, lifeTime: f64) {
        if self.lifeTime <= 0.0 {
            self.lifeTime = lifeTime;
        }
    }
}