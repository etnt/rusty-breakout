use rand::{thread_rng, Rng};
use rusty_engine::prelude::*;
use std::{
    f32::consts::{FRAC_1_PI, FRAC_PI_2, PI, TAU},
    time::Duration, collections::HashSet,
};

const SHOT_SPEED: f32 = 200.0;
const RELOAD_TIME: u64 = 150;
const UP_TIME: u64 = 50;

struct GameState {
    stop: bool,
    shot_counter: u32,
    shots_active: u32,
    shot_timer: Timer,
    ammo: u32,
    sprites_to_delete: HashSet<String>,
    up_timer: Timer,
    num_of_bricks: u32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            stop: false,
            shot_counter: 0,
            shots_active: 0,
            shot_timer: Timer::new(Duration::from_millis(RELOAD_TIME), false),
            ammo: 1,
            sprites_to_delete: HashSet::new(),
            up_timer: Timer::new(Duration::from_millis(UP_TIME), false),
            num_of_bricks: 0,
        }
    }
}

fn main() {
    let mut game = Game::new();

    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.1);

    let mut game_state = GameState::default();

    // game setup goes here
    setup_walls(&mut game);
    let num_of_bricks = setup_bricks(&mut game);
    game_state.num_of_bricks = num_of_bricks;

    let ammo_display = game.add_text("ammo_display", format!("Ammo: {}", game_state.ammo));
    ammo_display.translation = Vec2::new(550.0, 330.0);

    let player = game.add_sprite("player", SpritePreset::RacingBarrierRed);
    player.translation = Vec2::new(-300.0, -300.0);
    player.scale = 0.35;
    player.collision = true;

    game.add_logic(game_logic);
    game.run(game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {

    if game_state.stop {
        return;
    }

    // Update the timers.
    game_state.shot_timer.tick(engine.delta);
    game_state.up_timer.tick(engine.delta);

    // Get hold of the Player info.
    let player = engine.sprites.get_mut("player").unwrap();
    let player_x = player.translation.x;
    let player_y = player.translation.y;
    if game_state.up_timer.finished() {
        // Reset any slant
        player.rotation = 0.0;
    }
    let mut shoot = false;

    // Keyboard handling
    if engine.keyboard_state.pressed(KeyCode::Space)
        && game_state.shot_timer.finished()
        && (game_state.ammo > 0)
    {
        shoot = true;
        game_state.ammo -= 1;
        game_state.shot_timer.reset();
    } else if engine.keyboard_state.pressed(KeyCode::Left) {
        player.translation.x -= 7.0;
    } else if engine.keyboard_state.pressed(KeyCode::Right) {
        player.translation.x += 7.0;
    } else if engine.keyboard_state.pressed(KeyCode::A) && game_state.up_timer.finished() {
        player.rotation = TAU - 0.15;
        game_state.up_timer.reset();
    } else if engine.keyboard_state.pressed(KeyCode::D) && game_state.up_timer.finished() {
        player.rotation = 0.15;
        game_state.up_timer.reset();
    }

    // Generate a new shot
    if shoot {
        game_state.shot_counter += 1;
        game_state.shots_active += 1;
        let sprite = engine.add_sprite(
            format!("shot{}", game_state.shot_counter),
            SpritePreset::RollingBallRed,
        );
        sprite.scale = 0.3;
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
            // Bounds check, shot is leaving scene.
            if sprite.translation.y < -400.0 {
                game_state.sprites_to_delete.insert(sprite.label.clone());
            }
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
            if event.pair.0.contains("red") || event.pair.1.contains("red") {
                game_state.ammo += 1;
            }
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
                game_state.sprites_to_delete.insert(event.pair.1.clone());
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
                game_state.sprites_to_delete.insert(event.pair.0.clone());
            }
        }
    }

    // Remove the sprites.
    for sprite_to_delete in &game_state.sprites_to_delete {
        if sprite_to_delete.starts_with("brick") {
            engine.sprites.remove(sprite_to_delete);
            game_state.num_of_bricks -= 1;
        } else if sprite_to_delete.starts_with("shot") {
            engine.sprites.remove(sprite_to_delete);
            game_state.shots_active -= 1;
        }
    }
    let _ = game_state.sprites_to_delete.drain();

    // Update the Ammo display
    let ammo_display = engine.texts.get_mut("ammo_display").unwrap();
    ammo_display.value = format!("Ammo: {}", game_state.ammo);

    // check for lost game
    if (game_state.shots_active == 0) && (game_state.ammo == 0) {
        game_state.stop = true;
        let game_over_text = engine.add_text("game_over", "You Lost!");
        game_over_text.font_size = 128.0;
        engine.audio_manager.stop_music();
        engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.5);
    }

    // Check for won game
    if game_state.num_of_bricks == 0 {
        game_state.stop = true;
        let game_over_text = engine.add_text("game_over", "You Won!");
        game_over_text.font_size = 128.0;
        engine.audio_manager.stop_music();
        engine.audio_manager.play_sfx(SfxPreset::Jingle1, 0.5);
    }


}

fn setup_walls(game: &mut Game<GameState>) {
    let x_left: f32 = -750.0;
    let x_right: f32 = 750.0;
    let y_side: f32 = 0.0;
    let x_top: f32 = 0.0;
    let y_top: f32 = 547.0;
    let scale = 3.684_71;
    // Initially generated by the 'level_creator'.
    let a = game.add_sprite("wall_left", SpritePreset::RacingBarrierWhite);
    a.translation = Vec2::new(x_left, y_side);
    a.rotation = FRAC_PI_2;
    a.scale = scale;
    a.collision = true;
    let a = game.add_sprite("wall_right", SpritePreset::RacingBarrierWhite);
    a.translation = Vec2::new(x_right, y_side);
    a.rotation = FRAC_PI_2;
    a.scale = scale;
    a.collision = true;
    let a = game.add_sprite("wall_top", SpritePreset::RacingBarrierWhite);
    a.translation = Vec2::new(x_top, y_top);
    a.rotation = 0.0;
    a.scale = 6.2;
    a.collision = true;
}

// Return the number of bricks setup.
fn setup_bricks(game: &mut Game<GameState>) -> u32 {
    setup_pyramid_1(game)
}

fn random_color() -> (String, SpritePreset) {
    if thread_rng().gen_range(0..10) < 1 {
        ("red".to_string(), SpritePreset::RacingBarrierRed)
    } else {
        ("white".to_string(), SpritePreset::RacingBarrierWhite)
    }
}

fn setup_pyramid_1(game: &mut Game<GameState>) -> u32 {
    let scale: f32 = 0.27680913;
    let zero: f32 = 0.0;
    let mut count: u32 = 0;
    for y in (0..=300).step_by(100) {
        for x in (0..=1000).step_by(200) {
            let (color_str, barrier_color) = random_color();
            let a = game.add_sprite(format!("brick_{}_{}", color_str, count), barrier_color);
            a.translation = Vec2::new(-427.0 + x as f32, 297.0 - y as f32);
            a.rotation = zero;
            a.scale = scale;
            a.collision = true;
            count += 1;
            let (color_str, barrier_color) = random_color();
            let a = game.add_sprite(format!("brick_{}_{}", color_str, count), barrier_color);
            a.translation = Vec2::new(-488.0 + x as f32, 297.0 - y as f32);
            a.rotation = zero;
            a.scale = scale;
            a.collision = true;
            count += 1;
            let (color_str, barrier_color) = random_color();
            let a = game.add_sprite(format!("brick_{}_{}", color_str, count), barrier_color);
            a.translation = Vec2::new(-549.0 + x as f32, 297.0 - y as f32);
            a.rotation = zero;
            a.scale = scale;
            a.collision = true;
            count += 1;
            let (color_str, barrier_color) = random_color();
            let a = game.add_sprite(format!("brick_{}_{}", color_str, count), barrier_color);
            a.translation = Vec2::new(-517.0 + x as f32, 278.0 - y as f32);
            a.rotation = zero;
            a.scale = scale;
            a.collision = true;
            count += 1;
            let (color_str, barrier_color) = random_color();
            let a = game.add_sprite(format!("brick_{}_{}", color_str, count), barrier_color);
            a.translation = Vec2::new(-467.0 + x as f32, 278.0 - y as f32);
            a.rotation = zero;
            a.scale = scale;
            a.collision = true;
            count += 1;
            let (color_str, barrier_color) = random_color();
            let a = game.add_sprite(format!("brick_{}_{}", color_str, count), barrier_color);
            a.translation = Vec2::new(-488.0 + x as f32, 258.0 - y as f32);
            a.rotation = zero;
            a.scale = scale;
            a.collision = true;
            count += 1;
        }
    }
    count
}
