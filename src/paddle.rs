// Raylib
use sola_raylib::prelude::*;

pub struct PaddleBuilder {
    pos: Vector2,
    size: Vector2,
    moveSpeed: f32,
    color: Color,
    upKey: Option<KeyboardKey>,
    downKey: Option<KeyboardKey>,
}
pub struct Paddle {
    pub pos: Vector2,
    pub size: Vector2,
    pub moveSpeed: f32,
    pub color: Color,
    upKey: KeyboardKey,
    downKey: KeyboardKey,
}

// Paddle builder
impl PaddleBuilder {
    pub fn new() -> Self {
        let defaultPaddleSize: Vector2 = Vector2::new(14.0, 120.0);

        return Self {
            pos: Vector2::new(defaultPaddleSize.x + 10.0, defaultPaddleSize.y + 10.0),
            size: defaultPaddleSize,
            moveSpeed: 600.0,
            color: Color::RED,
            upKey: None,
            downKey: None,
        };
    }

    pub fn build(&self) -> Result<Paddle, String> {
        return Ok(Paddle {
            pos: self.pos,
            size: self.size,
            moveSpeed: self.moveSpeed,
            upKey: self.upKey.unwrap_or(KeyboardKey::KEY_UP),
            downKey: self.downKey.unwrap_or(KeyboardKey::KEY_DOWN),
            color: self.color,
        });
    }

    // Setters
    pub fn pos(mut self, pos: Vector2) -> Self {
        self.pos = pos;
        return self;
    }
    pub fn size(mut self, size: Vector2) -> Self {
        self.size = size;
        return self;
    }
    pub fn moveSpeed(mut self, moveSpeed: f32) -> Self {
        self.moveSpeed = moveSpeed;
        return self;
    }
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        return self;
    }
    pub fn upKey(mut self, upKey: KeyboardKey) -> Self {
        self.upKey = Some(upKey);
        return self;
    }
    pub fn downKey(mut self, downKey: KeyboardKey) -> Self {
        self.downKey = Some(downKey);
        return self;
    }
}

impl Paddle {
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