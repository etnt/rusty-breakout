use rand::{thread_rng, Rng};
use rusty_engine::prelude::*;
use std::{f32::consts::{FRAC_PI_2, FRAC_2_PI, TAU, PI, FRAC_1_PI}, time::Duration, };

const SHOT_SPEED: f32 = 200.0;
const RELOAD_TIME: u64 = 150;

struct GameState {
    shot_counter: u32,
    shot_timer: Timer,
    sprites_to_delete: Vec<String>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            shot_counter: 0,
            shot_timer: Timer::new(Duration::from_millis(RELOAD_TIME), false),
            sprites_to_delete: Vec::new(),
        }
    }
}

fn main() {
    let mut game = Game::new();

    // game setup goes here
    setup_walls(&mut game);
    setup_bricks(&mut game);

    let player = game.add_sprite("player", SpritePreset::RacingBarrierWhite);
    player.translation = Vec2::new(-300.0, -300.0);
    //player.rotation = 0.0;
    player.scale = 0.3;
    player.collision = true;

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {

    // Update the timers.
    game_state.shot_timer.tick(engine.delta);

    // Get hold of the Player info.
    let player = engine.sprites.get_mut("player").unwrap();
    let player_x = player.translation.x;
    let player_y = player.translation.y;
    let player_rotation = player.rotation;
    let mut shoot = false;

    // Keyboard handling
    if engine.keyboard_state.pressed(KeyCode::Space) && game_state.shot_timer.finished() {
        shoot = true;
        game_state.shot_timer.reset();
    }  else if engine.keyboard_state.pressed(KeyCode::Left) {
        // Deal with positive rotation overflow
        player.translation.x -= 5.0;
    } else if engine.keyboard_state.pressed(KeyCode::Right) {
        player.translation.x += 5.0;
    }

    // Generate a new shot
    if shoot {
        game_state.shot_counter += 1;
        let sprite = engine.add_sprite(
            format!("shot{}", game_state.shot_counter),
            SpritePreset::RollingBallRed,
        );
        sprite.scale = 0.2;
        let (left, right): (f32, f32) = (FRAC_PI_2 + FRAC_2_PI, FRAC_PI_2 - FRAC_2_PI);
        sprite.rotation = thread_rng().gen_range(FRAC_1_PI..FRAC_PI_2);
        sprite.translation.x = player_x;
        sprite.translation.y = player_y + 15.0;
        sprite.collision = true;
        engine.audio_manager.play_sfx(SfxPreset::Impact1, 0.4);
    }

    // Move the shots
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("shot") {
            sprite.translation.x +=
                SHOT_SPEED * engine.delta_f32 * (sprite.rotation as f64).cos() as f32;
            sprite.translation.y +=
                SHOT_SPEED * engine.delta_f32 * (sprite.rotation as f64).sin() as f32;
        }
    }

    // deal with collisions
    for event in engine.collision_events.drain(..) {
        // We only care about the start of collisions, not the ending of them.
        if event.state.is_end() {
            continue;
        }
        if event.pair.one_starts_with("shot") {
            engine.audio_manager.play_sfx(SfxPreset::Impact2, 0.4);
            if event.pair.0.starts_with("shot") {
                let mut wall_rotation = 0.0;
                {
                    let wall = engine.sprites.get_mut(&event.pair.1).unwrap();
                    wall_rotation = wall.rotation;
                }
                let shot = engine.sprites.get_mut(&event.pair.0).unwrap();
                if shot.rotation < PI {
                    shot.rotation = 2.0 * wall_rotation - shot.rotation;
                } else if shot.rotation > PI {
                    shot.rotation = 2.0 * (PI + wall_rotation) - shot.rotation;
                } else {
                    shot.rotation += PI;
                }
                // Push the Sprite to be removed later.
                game_state.sprites_to_delete.push(event.pair.1.clone());
            } else if event.pair.1.starts_with("shot") {
                let mut wall_rotation = 0.0;
                {
                    let wall = engine.sprites.get_mut(&event.pair.0).unwrap();
                    wall_rotation = wall.rotation;
                }
                let shot = engine.sprites.get_mut(&event.pair.1).unwrap();
                if shot.rotation < PI {
                    shot.rotation = 2.0 * wall_rotation - shot.rotation;
                } else if shot.rotation > PI {
                    shot.rotation = 2.0 * (PI + wall_rotation) - shot.rotation;
                } else {
                    shot.rotation += PI;
                }
                // Push the Sprite to be removed later.
                game_state.sprites_to_delete.push(event.pair.0.clone());
            }
        }
    }

    // Remove the sprites.
    for sprite_to_delete in &game_state.sprites_to_delete {
        if sprite_to_delete.starts_with("brick") {
            engine.sprites.remove(sprite_to_delete);
        }
    }
    game_state.sprites_to_delete.drain(..);


}

fn setup_walls(game: &mut Game<GameState>) {
    let x_left: f32 = -750.0;
    let x_right: f32 = 750.0;
    let y_side: f32 = 0.0;
    let x_top: f32 = 0.0;
    let y_top: f32 = 547.0;
    let scale = 3.68471003;
    // Initially generated by the 'level_creator'.
    let a = game.add_sprite("wall_left", SpritePreset::RacingBarrierWhite); a.translation = Vec2::new(x_left, y_side); a.rotation = FRAC_PI_2; a.scale = scale; a.collision = true;
    let a = game.add_sprite("wall_right", SpritePreset::RacingBarrierWhite); a.translation = Vec2::new(x_right, y_side); a.rotation = FRAC_PI_2; a.scale = scale; a.collision = true;
    let a = game.add_sprite("wall_top", SpritePreset::RacingBarrierWhite); a.translation = Vec2::new(x_top, y_top); a.rotation = 0.0; a.scale = 6.2; a.collision = true;
}


fn setup_bricks(game: &mut Game<GameState>) {
    setup_pyramid_1(game);
}

fn setup_pyramid_1(game: &mut Game<GameState>) {
    let scale: f32 = 0.27680913;
    let zero: f32 = 0.0;
    let mut count: u32 = 0;
    for x in (0..=1000).step_by(200) {
        let a = game.add_sprite(format!("brick_{}",count), SpritePreset::RacingBarrierWhite); a.translation = Vec2::new(-427.0 + x as f32, 297.0); a.rotation = zero; a.scale = scale; a.collision = true;count += 1;
        let a = game.add_sprite(format!("brick_{}",count), SpritePreset::RacingBarrierWhite); a.translation = Vec2::new(-488.0 + x as f32, 297.0); a.rotation = zero; a.scale = scale; a.collision = true; count += 1;
        let a = game.add_sprite(format!("brick_{}",count), SpritePreset::RacingBarrierWhite); a.translation = Vec2::new(-549.0 + x as f32, 297.0); a.rotation = zero; a.scale = scale; a.collision = true; count += 1;
        let a = game.add_sprite(format!("brick_{}",count), SpritePreset::RacingBarrierWhite); a.translation = Vec2::new(-517.0 + x as f32, 278.0); a.rotation = zero; a.scale = scale; a.collision = true; count += 1;
        let a = game.add_sprite(format!("brick_{}",count), SpritePreset::RacingBarrierWhite); a.translation = Vec2::new(-467.0 + x as f32, 278.0); a.rotation = zero; a.scale = scale; a.collision = true; count += 1;
        let a = game.add_sprite(format!("brick_{}",count), SpritePreset::RacingBarrierWhite); a.translation = Vec2::new(-488.0 + x as f32, 258.0); a.rotation = zero; a.scale = scale; a.collision = true; count += 1;
    }
}
