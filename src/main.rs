#![allow(unused)]

use macroquad::prelude::*;
mod ecs;
mod game;
use game::Game;

fn window_conf() -> Conf {
    Conf {
        window_title: "bh".to_string(),
        window_width: 1200,
        window_height: 800,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        clear_background(LIGHTGRAY);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 60.0, 30.0, WHITE);
        game.update();
        game.draw();
        next_frame().await;
    }
}

// // use macroquad::prelude::*;
// // use macroquad::rand::gen_range;

// // fn window_conf() -> Conf {
// //     Conf {
// //         window_title: "bh".to_string(),
// //         window_width: 1200,
// //         window_height: 800,
// //         fullscreen: false,
// //         ..Default::default()
// //     }
// // }

// // #[derive(Copy, Clone)]
// // struct Circle {
// //     x: f32,
// //     y: f32,
// // }

// // struct Bullet {
// //     x: f32,
// //     y: f32,
// // }

// // #[macroquad::main(window_conf)]
// // async fn main() {
// //     let mut player: Circle = Circle { x: 100.0, y: 100.0 };

// //     let n_enemies = 5;
// //     let mut enemies: Vec<Circle> = (0..n_enemies)
// //         .map(|_| Circle {
// //             x: gen_range(30.0, 1170.0),
// //             y: gen_range(30.0, 770.0),
// //         })
// //         .collect();

// //     let mut bullets: Vec<Bullet> = Vec::new();

// //     let radius = 10.0;
// //     let min_distance = radius * 2.0;

// //     let mut game_over: bool = false;

// //     loop {
// //         if !game_over {
// //             // Movement
// //             if is_key_down(KeyCode::Right) && player.x + radius < 1200.0 {
// //                 player.x += 2.0;
// //             }
// //             if is_key_down(KeyCode::Left) && player.x - radius > 0.0 {
// //                 player.x -= 2.0;
// //             }
// //             if is_key_down(KeyCode::Down) && player.y + radius < 800.0 {
// //                 player.y += 2.0;
// //             }
// //             if is_key_down(KeyCode::Up) && player.y - radius > 0.0 {
// //                 player.y -= 2.0;
// //             }

// //             if is_key_down(KeyCode::Space) {
// //                 bullets.push(Bullet {
// //                     x: player.x,
// //                     y: player.y,
// //                 });
// //             }

// //             for bullet in bullets.iter_mut() {
// //                 bullet.x += 4.0;
// //                 // draw_circle(bullet.x, bullet.y, 5.0, YELLOW);
// //             }

// //             bullets.retain(|b| b.x <= screen_width());

// //             for i in 0..enemies.len() {
// //                 // Move toward player
// //                 let (dx, dy) = {
// //                     let enemy = &enemies[i];
// //                     let dx = player.x - enemy.x;
// //                     let dy = player.y - enemy.y;
// //                     (dx, dy)
// //                 };

// //                 let distance = (dx * dx + dy * dy).sqrt();
// //                 if distance > 0.0 {
// //                     let speed = 0.5;
// //                     enemies[i].x += dx / distance * speed;
// //                     enemies[i].y += dy / distance * speed;
// //                 }

// //                 // Check collision with player
// //                 if check_collision(&player, &enemies[i], radius) {
// //                     // game_over = true;
// //                 }

// //                 // Prevent overlap with other enemies
// //                 for j in 0..enemies.len() {
// //                     if i == j {
// //                         continue;
// //                     }

// //                     let dx = enemies[i].x - enemies[j].x;
// //                     let dy = enemies[i].y - enemies[j].y;
// //                     let dist = (dx * dx + dy * dy).sqrt();

// //                     if dist > 0.0 && dist < min_distance {
// //                         let overlap = 0.5 * (min_distance - dist);
// //                         let push_x = dx / dist * overlap;
// //                         let push_y = dy / dist * overlap;

// //                         enemies[i].x += push_x;
// //                         enemies[i].y += push_y;
// //                         enemies[j].x -= push_x;
// //                         enemies[j].y -= push_y;
// //                     }
// //                 }
// //             }

// //             // Drawing
// //             clear_background(DARKGRAY);
// //             draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 30.0, WHITE);
// //             draw_circle(player.x, player.y, radius, PINK);
// //             for enemy in &enemies {
// //                 draw_circle(enemy.x, enemy.y, radius, RED);
// //             }
// //             for bullet in &bullets {
// //                 draw_circle(bullet.x, bullet.y, 5.0, BLUE);
// //             }
// //         } else {
// //             clear_background(WHITE);
// //             let text = "Game Over. Press [enter] to play again.";
// //             let font_size = 30.;
// //             let text_size = measure_text(text, None, font_size as _, 1.0);

// //             draw_text(
// //                 text,
// //                 screen_width() / 2. - text_size.width / 2.,
// //                 screen_height() / 2. + text_size.height / 2.,
// //                 font_size,
// //                 DARKGRAY,
// //             );

// //             if is_key_down(KeyCode::Enter) {
// //                 player.x = 100.0;
// //                 player.y = 100.0;

// //                 enemies = (0..n_enemies)
// //                     .map(|_| Circle {
// //                         x: gen_range(30.0, 1170.0),
// //                         y: gen_range(30.0, 770.0),
// //                     })
// //                     .collect();

// //                 game_over = false;
// //             }
// //         }
// //         next_frame().await;
// //     }
// // }

// // fn check_collision(circle1: &Circle, circle2: &Circle, radius: f32) -> bool {
// //     let dx: f32 = circle1.x - circle2.x;
// //     let dy: f32 = circle1.y - circle2.y;
// //     let distance: f32 = (dx * dx + dy * dy).sqrt();

// //     distance <= radius + radius
// // }
