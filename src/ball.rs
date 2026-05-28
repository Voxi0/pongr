// Raylib
use sola_raylib::prelude::*;

// Timer
mod timer;
use timer::*;

pub struct Ball {
    pos: Vector2,
    vel: Vector2,
    pub moveSpeed: f32,
    pub size: f32,
    pub color: Color,
}
impl Ball {
    pub fn new(winWidth: f32, winHeight: f32, moveSpeed: f32, size: f32, color: Color) -> Result<Self, String> {
        // Ensure movement speed is greater than 0
        if moveSpeed <= 0.0 {
            return Err("Movement speed has to be a non-zero and positive value".to_string())
        }

        // Ensure ball isn't too small
        if size <= 0.0 {
            return Err("Ball size has to be greater than 0".to_string());
        }

        // Everything checks out, return an instance of the ball struct
        return Ok(Self {
            pos: Vector2::new(winWidth / 2.0, winHeight / 2.0),
            vel: Vector2::new(1.0, 1.0),
            moveSpeed: moveSpeed,
            size: size,
            color: color,
        });
    }

    pub fn update(&mut self, winHeight: f32, frameTime: f32) {
        // Ensure ball doesn't go out of the screen
        if self.pos.y < 0.0 {
            self.pos.y = 0.0;
            self.vel.y *= -1.0;
        }
        else if (self.pos.y + self.size) > winHeight {
            self.pos.y = winHeight - self.size;
            self.vel.y *= -1.0;
        }

        // Move the ball
        self.pos += self.vel * self.moveSpeed * frameTime;
    }

    pub fn draw(&self, d: &mut impl RaylibDraw) {
        d.draw_circle_v(self.pos, self.size, self.color);
    }

    pub fn checkCollision(&mut self, rect: &Rectangle) {
        let collisionRect: Rectangle = Rectangle::new(self.pos.x, self.pos.y, self.size, self.size);

        if collisionRect.check_collision_recs(rect) {
            let overlap: Rectangle = collisionRect.get_collision_rec(rect)
                .expect("Failed to get collision rec");

            if overlap.width < overlap.height {
                self.vel.x *= -1.0;

                // The ball hit the right paddle
                if self.vel.x > 0.0 {
                    self.pos.x = rect.x + rect.width;
                }

                // The ball hit the left paddle
                else {
                    self.pos.x = rect.x - self.size;
                }
            }

            else {
                self.vel.y *= -1.0;

                if self.pos.y < rect.y {
                    self.pos.y = rect.y - self.size;
                }

                else {
                    self.pos.y = rect.y + rect.height;
                }
            }
        }
    }

    pub fn reset(&mut self, rl: &mut RaylibHandle, winWidth: f32, winHeight: f32, timer: &mut Timer) {
        // Put the ball back in the center and remove its velocity
        self.pos = Vector2::new(winWidth / 2.0, winHeight / 2.0);
        self.vel = Vector2::new(0.0, 0.0);

        while !timer.done() {
            timer.update(&mut rl);
        }
        if timer.done {
            self.vel = Vector2::new(-1.0, 1.0);
        }
    }

    pub fn getPos(&self) -> Vector2 {return self.pos;}
}