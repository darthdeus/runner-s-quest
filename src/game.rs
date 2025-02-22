use comfy::*;
use comfy::log::{Level, log};

use crate::{door, items, player, WINDOW_HIGHT, WINDOW_WIDTH};
use crate::assets::load_sprites;
use crate::state::{GameState, Scene};
use crate::tilemap::tilemap_helper;
use crate::tilemap::tilemap_helper::TILEMAP_ORIGIN;


pub struct ComfyGame {
    pub engine: EngineState,
    pub state: Option<GameState>,
}

impl ComfyGame {
    pub fn new(engine: EngineState) -> Self {
        Self {
            state: None,
            engine,
        }
    }
}

impl GameLoop for ComfyGame {
    fn engine(&mut self) -> &mut EngineState {
        &mut self.engine
    }

    fn update(&mut self) {
        let mut c = self.engine.make_context();

        if self.state.is_none() {
            // debug mode
            //game_config_mut().dev.show_fps = true;
            //c.renderer.window().set_fullscreen(Some(Fullscreen::Borderless(None)));
            c.renderer.window().set_resizable(false);
            main_camera_mut().zoom = WINDOW_WIDTH / 2.0;
            main_camera_mut().center = vec2(WINDOW_WIDTH / 2.0, -WINDOW_HIGHT / 2.0);
            load_sprites(&mut c);
            let state = GameState::new(tilemap_helper::load_levels());
            self.state = Some(state);
        }

        if let Some(state) = self.state.as_mut() {
            run_early_update_stages(&mut c);

            setup(state, &mut c);

            handle_input(state, &mut c);

            update(state, &mut c);

            draw(state);

            run_late_update_stages(&mut c, delta());
        }
    }
}

fn setup(state: &mut GameState, _c: &mut EngineContext) {
    match state.scene {
        Scene::LoadMenu => { setup_load_menu(state) }
        Scene::LoadLevel => { setup_load_level(state) }
        _ => {}
    }
}

fn setup_load_menu(state: &mut GameState) {
    println!("setup_load_menu");
    state.scene = Scene::Menu;
}

fn setup_load_level(state: &mut GameState) {
    println!("setup_load_level");
    //TODO dispawn everything
    items::spawn_ladders(state);
    items::spawn_pulleys(state);
    items::spawn_keys(state);
    player::spawns(state);
    door::spawns(state);
    state.scene = Scene::EnterLevel;
}


fn handle_input(state: &mut GameState, c: &mut EngineContext) {
    match state.scene {
        Scene::Menu => {
            if is_key_pressed(KeyCode::Escape) {
                std::process::exit(0);
            }
            if is_key_pressed(KeyCode::Return) {
                println!("switch to loadLevel!");
                state.scene = Scene::LoadLevel;
            }
        }
        Scene::Game => {
            if is_key_pressed(KeyCode::Escape) {
                println!("switch to loadMenu!");
                state.scene = Scene::LoadMenu;
            }
            player::handle_input(state, c);
        }
        _ => {}
    }
}


fn draw(state: &mut GameState) {
    match state.scene {
        Scene::Menu => { draw_menu(state) }
        Scene::Game => { draw_play(state) }
        Scene::EnterLevel => {
            draw_enter_transition(state);
            draw_play(state);
        }
        Scene::ExitLevel => {
            draw_exit_transition(state);
            draw_play(state);
        }
        _ => {}
    }
}

fn draw_exit_transition(state: &mut GameState) {
    println!("draw_exit_transition");
    // when finish call state.next_level()
    state.next_level();
}

fn draw_enter_transition(state: &mut GameState) {
    println!("draw_enter_transition");
    // when finish change state.scene = Scene::Game
    state.scene = Scene::Game;
}


fn draw_play(state: &GameState) {
    println!("draw_play");
    state.tilemap.draw(texture_id("tileset"), TILEMAP_ORIGIN, state.tilemap.get_layer_id("deco2"), 7, WHITE);
    state.tilemap.draw(texture_id("tileset"), TILEMAP_ORIGIN, state.tilemap.get_layer_id("deco"), 6, WHITE);
    state.tilemap.draw(texture_id("tileset"), TILEMAP_ORIGIN, state.tilemap.get_layer_id("level"), 3, WHITE);
    state.tilemap.draw(texture_id("tileset"), TILEMAP_ORIGIN, state.tilemap.get_layer_id("background"), 2, GRAY);
    state.tilemap.draw(texture_id("tileset"), TILEMAP_ORIGIN, state.tilemap.get_layer_id("background2"), 1, GRAY);
}

fn draw_menu(_state: &GameState) {
    println!("draw_menu");
    draw_sprite(texture_id("game_logo"), vec2(WINDOW_WIDTH / 2.0, WINDOW_HIGHT / 2.0 * -1.0), WHITE, 0, vec2(128.0 * 2.0, 48.0 * 2.0))
}


fn update(state: &mut GameState, c: &mut EngineContext) {
    match state.scene {
        Scene::Menu => { update_menu(state, c) }
        Scene::Game => { update_play(state, c) }
        _ => {}
    }
}

fn update_menu(_state: &mut GameState, _c: &mut EngineContext) {
    println!("update_menu");
}

fn update_play(state: &mut GameState, c: &mut EngineContext) {
    println!("update_play");
    door::update(state, c);
    items::update(state, c);
}



