use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    health_amount: u8,
    lost: bool,
}

const PLAYER_SPEED: f32 = 250.0;
const ROAD_SPEED: f32 = 400.0;

fn main() {
    let mut game = Game::new();

    // game setup goes here
    // Create the player sprite
    let player1 = game.add_sprite("player1", SpritePreset::RacingCarBlue);
    player1.translation.x = -500.0;
    player1.layer = 10.0;
    player1.collision = true;

    // Start the background music
    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.3);

    // Create the road lines
    for i in 0..10 {
        let roadline = game.add_sprite(format!("roadline_{}", i), SpritePreset::RacingBarrierWhite);
        roadline.scale = 0.1;
        roadline.translation.x = -600.0 + 150.0 * i as f32;
    }
    game.add_logic(game_logic);
    game.run(GameState { health_amount: 5, lost: false });
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // Collect keyboard input
    let mut direction = 0.0;
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W])
    {
        direction += 1.0;
    };
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::S])
    {
        direction -= 1.0;
    };

    // Moved the player sprite
    let player1 = engine.sprites.get_mut("player1").unwrap();
    player1.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    player1.rotation = direction * 0.15;
    if player1.translation.y < -360.0 || player1.translation.y > 360.0 {
        game_state.health_amount = 0;
    }

    // Move road objects
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("roadline") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -675.0 {
                sprite.translation.x += 1500.0;
            }
        }
    }
}
