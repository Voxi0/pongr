// Raylib
use sola_raylib::prelude::*;

pub struct Paddle {
    pub pos: Vector2,
    pub size: Vector2,
    pub moveSpeed: f32,
    pub color: Color,
    upKey: KeyboardKey,
    downKey: KeyboardKey,
}
impl Paddle {
    pub fn new(winHeight: f32, posX: f32, upKey: KeyboardKey, downKey: KeyboardKey, color: Color) -> Result<Self, String> {
        let size: Vector2 = Vector2::new(14.0, 120.0);

        // Everything checks out, return an instance of the paddle struct
        return Ok(Self {
            pos: Vector2::new(posX, (winHeight - size.y) / 2.0),
            size: size,
            moveSpeed: 400.0,
            upKey: upKey,
            downKey: downKey,
            color: color,
        });
    }

    pub fn update(&mut self, winHeight: f32, rl: &mut RaylibHandle, frameTime: f32) {
        if rl.is_key_down(self.upKey) && self.pos.y > 0.0 {
            self.pos.y -= self.moveSpeed * frameTime;
        }
        else if rl.is_key_down(self.downKey) && (self.pos.y + self.size.y) < winHeight {
            self.pos.y += self.moveSpeed * frameTime;
        }
    }

    pub fn draw(&self, d: &mut impl RaylibDraw) {
        d.draw_rectangle_v(self.pos, self.size, self.color);
    }

    pub fn getCollisionRect(&self) -> Rectangle {
        return Rectangle::new(self.pos.x, self.pos.y, self.size.x, self.size.y);
    }
}