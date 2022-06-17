use rusty_engine::prelude::*;
use std::{f32::consts::{FRAC_PI_2, TAU, PI}, time::Duration};

const SHOT_SPEED: f32 = 200.0;
const RELOAD_TIME: u64 = 150;

struct GameState {
    shot_counter: u32,
    shot_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            shot_counter: 0,
            shot_timer: Timer::new(Duration::from_millis(RELOAD_TIME), false),
        }
    }
}

fn main() {
    let mut game = Game::new();

    // game setup goes here
    setup_walls(&mut game);

    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(-300.0, 0.0);
    player.rotation = LEFT;
    player.scale = 0.5;
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
        player.rotation = (player.rotation + 0.05) % TAU;
    } else if engine.keyboard_state.pressed(KeyCode::Right) {
        player.rotation -= 0.05;
        // Deal with negative rotation overflow
        if player.rotation < 0.0 {
            player.rotation = TAU - player.rotation
        };
    }

    // Generate a new shot
    if shoot {
        game_state.shot_counter += 1;
        let sprite = engine.add_sprite(
            format!("shot{}", game_state.shot_counter),
            SpritePreset::RollingBallRed,
        );
        sprite.scale = 0.1;
        sprite.rotation = player_rotation;
        sprite.translation.x = player_x;
        sprite.translation.y = player_y;
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
        if event.pair.one_starts_with("shot") && event.pair.one_starts_with("wall"){
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
            }
        }
    }


}

fn setup_walls(game: &mut Game<GameState>) {
    let x_left: f32 = -740.0;
    let x_right: f32 = 740.0;
    let y: f32 = 0.0;
    let scale = 3.68471003;
    // Initially generated by the 'level_creator'.
     let a = game.add_sprite("wall_left", SpritePreset::RacingBarrierWhite); a.translation = Vec2::new(x_left, y); a.rotation = FRAC_PI_2; a.scale = scale; a.collision = true;
     let a = game.add_sprite("wall_right", SpritePreset::RacingBarrierWhite); a.translation = Vec2::new(x_right, y); a.rotation = FRAC_PI_2; a.scale = scale; a.collision = true;
}
