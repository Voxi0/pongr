#![allow(non_snake_case)]

// Raylib
use sola_raylib::prelude::*;

// Ball and paddle duh
mod ball;
mod paddle;
use ball::*;
use paddle::*;

// Constants
// Window
const WIN_TITLE: &str = "Pongr";
const WIN_WIDTH: f32 = 1920.0;
const WIN_HEIGHT: f32 = 1080.0;

fn main() {
    // Initialize Raylib
    let (mut rl, thread) = sola_raylib::init()
        .title(WIN_TITLE)
        .size(WIN_WIDTH as i32, WIN_HEIGHT as i32)
        .msaa_4x()
        .vsync()
        .build();

    // Render texture
    let mut renderTexture: RenderTexture2D = rl.load_render_texture(&thread, WIN_WIDTH as u32, WIN_HEIGHT as u32)
        .expect("Failed to load render texture");
    let srcRect: Rectangle = Rectangle {
        x: 0.0,
        y: 0.0,
        width: WIN_WIDTH,
        height: -WIN_HEIGHT,
    };
    let mut dstRect: Rectangle = Rectangle {
        x: 0.0,
        y: 0.0,
        width: WIN_WIDTH,
        height: WIN_HEIGHT,
    };

    // Ball
    let mut ball: Ball = Ball::new(WIN_WIDTH, WIN_HEIGHT, 500.0, 16.0, Color::RED)
        .expect("Failed to create the ball");

    // Paddles/Players
    let mut player1: Paddle = Paddle::new(WIN_HEIGHT, 10.0, KeyboardKey::KEY_W, KeyboardKey::KEY_S, Color::RED)
        .expect("Failed to create player 1");
    let mut player2: Paddle = Paddle::new(WIN_HEIGHT, 0.0, KeyboardKey::KEY_UP, KeyboardKey::KEY_DOWN, Color::RED)
        .expect("Failed to create player 2");
    player2.pos.x = WIN_WIDTH - (10.0 + player2.size.x);

    // Main loop
    while !rl.window_should_close() {
        // Update everything
        {
            let frameTime: f32 = rl.get_frame_time();

            ball.update(WIN_WIDTH, WIN_HEIGHT, frameTime);
            player1.update(WIN_HEIGHT, &mut rl, frameTime);
            player2.update(WIN_HEIGHT, &mut rl, frameTime);

            ball.checkCollision(&player1.getCollisionRect());
            ball.checkCollision(&player2.getCollisionRect());

            if ball.getPos().x < 0.0 {
                println!("player 2 wins");
            }
            else if ball.getPos().x > WIN_WIDTH {
                println!("player 1 wins");
            }
        }

        // Draw/Render everything onto a texture
        {
            let mut mode = rl.begin_texture_mode(&thread, &mut renderTexture);
            mode.clear_background(Color::WHITE);
            ball.draw(&mut mode);
            player1.draw(&mut mode);
            player2.draw(&mut mode);
            mode.draw_fps(10, 10);
        }

        // Draw/Render the texture with everything on it
        {
            dstRect.width = rl.get_screen_width() as f32;
            dstRect.height = rl.get_screen_height() as f32;
            let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);
            d.draw_texture_pro(&renderTexture, srcRect, dstRect, Vector2::new(0.0, 0.0), 0.0, Color::WHITE);
        }
    }
}