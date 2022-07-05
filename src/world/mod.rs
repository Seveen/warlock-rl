pub mod actions;
pub mod components;
mod rules;

use std::collections::VecDeque;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use rstar::RTree;

use crate::{
    raw_loader::{GameData, GameDataHandle},
    save::*,
    AppState,
};

use self::{
    actions::*,
    components::{
        Action, EntityId, FutureState, GameState, GameWorld, Glyph, Name, Position,
        PositionTreeObject,
    },
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EntityIdGenerator::new())
            .add_enter_system(AppState::GenerateWorld, spawn_world)
            .add_enter_system(AppState::LoadWorld, load_world)
            .add_exit_system(AppState::GenerateWorld, spawn_potion)
            .add_exit_system(AppState::GenerateWorld, spawn_orcs)
            .add_exit_system(AppState::GenerateWorld, spawn_player);
    }
}

pub type Game = GameWorld<ActionType, GameEvent>;

pub enum GameEvent {
    Spawned {
        entity_id: EntityId,
        x: i64,
        y: i64,
        glyph: Glyph,
        name: components::Name,
        is_player: bool,
    },
    MovedTo {
        entity_id: EntityId,
        x: i64,
        y: i64,
    },
    Deleted {
        entity_id: EntityId,
    },
    EntityNearby {
        entity_id: EntityId,
        name: Name,
        position: Position,
    },
}

fn spawn_world(mut commands: Commands) {
    let game_world = Game::new(
        vec![rules::collision, rules::death, rules::compute_energy_cost],
        populate_action,
        vec![on_created, on_moved, on_deleted],
        vec![],
        vec![],
    );

    create_save_path(&mut commands, "world1").expect("Failed to build save path");
    commands.insert_resource(game_world);
    commands.insert_resource(NextState(AppState::InGame));
}

fn load_world(mut commands: Commands) {
    let save_path = create_save_path(&mut commands, "world1").expect("Failed to build save path");
    let game_state = load_saved_game(save_path).expect("Failed to load saved game");

    let game_world = Game::new_with_initial_state(
        vec![rules::collision, rules::death],
        populate_action,
        vec![on_created, on_moved, on_deleted],
        vec![],
        vec![update_nearby_list],
        game_state,
    );

    commands.insert_resource(game_world);
    commands.insert_resource(NextState(AppState::InGame));
}

fn on_created(
    events_queue: &mut VecDeque<GameEvent>,
    action: &Action,
    state: &GameState,
    _spatial_position: &RTree<PositionTreeObject>,
) {
    let future_state = FutureState { action, state };

    for (&id, &position) in action.get_updated_position() {
        if state.get_position(id).is_none() {
            if let Some(&glyph) = future_state.get_glyph(id) {
                if let Some(name) = future_state.get_name(id) {
                    let is_player = future_state.get_player(id).is_some();
                    events_queue.push_back(GameEvent::Spawned {
                        entity_id: id,
                        x: position.x,
                        y: position.y,
                        glyph,
                        name: name.clone(),
                        is_player,
                    });
                }
            }
        }
    }
}

fn on_deleted(
    events_queue: &mut VecDeque<GameEvent>,
    action: &Action,
    state: &GameState,
    _spatial_position: &RTree<PositionTreeObject>,
) {
    let future_state = FutureState { action, state };

    for &id in action.get_removed_position() {
        if state.get_position(id).is_some() && future_state.get_position(id).is_none() {
            events_queue.push_back(GameEvent::Deleted { entity_id: id })
        }
    }
}

fn on_moved(
    events_queue: &mut VecDeque<GameEvent>,
    action: &Action,
    _state: &GameState,
    _spatial_position: &RTree<PositionTreeObject>,
) {
    // Update sprite positions
    for (&id, &position) in action.get_updated_position() {
        events_queue.push_back(GameEvent::MovedTo {
            entity_id: id,
            x: position.x,
            y: position.y,
        });
    }
}

fn update_nearby_list(
    events_queue: &mut VecDeque<GameEvent>,
    state: &GameState,
    spatial_position: &RTree<PositionTreeObject>,
) {
    for &player_id in state.player.keys() {
        if let Some(&player_position) = state.get_position(player_id) {
            for PositionTreeObject { index, entity_at } in
                spatial_position.locate_within_distance(player_position, 15)
            {
                if let Some(name) = state.get_name(*entity_at) {
                    events_queue.push_back(GameEvent::EntityNearby {
                        entity_id: *entity_at,
                        name: name.clone(),
                        position: *index,
                    });
                }
            }
        }
    }
}

struct EntityIdGenerator {
    next_id: u64,
}

impl EntityIdGenerator {
    fn new() -> Self {
        EntityIdGenerator { next_id: 0 }
    }

    fn next(&mut self) -> EntityId {
        let next = self.next_id;
        self.next_id += 1;
        EntityId(next)
    }
}

// DEBUG //////////////////////////////////////////////////////////////////////////////////////////
fn spawn_player(
    mut world: ResMut<Game>,
    mut id_generator: ResMut<EntityIdGenerator>,
    data_asset: Res<Assets<GameData>>,
    game_data_handle: Res<GameDataHandle>,
) {
    let game_data = data_asset
        .get(&game_data_handle.0)
        .expect("Failed to get game data");
    if let Some(player_template) = game_data.entities.get("player") {
        let template = player_template.clone();
        let player_id = id_generator.next();
        println!("Player is {:?}", player_id);

        world.enqueue_action(ActionType::CreateEntity {
            entity_id: player_id,
            position: Position { x: 0, y: 0 },
            is_player: true,
            is_solid: true,
            template,
            cost: 0,
        });
        world.process_actions();
    }
}

fn spawn_orcs(
    mut world: ResMut<Game>,
    mut id_generator: ResMut<EntityIdGenerator>,
    data_asset: Res<Assets<GameData>>,
    game_data_handle: Res<GameDataHandle>,
) {
    let game_data = data_asset
        .get(&game_data_handle.0)
        .expect("Failed to get game data");
    if let Some(orc_template) = game_data.entities.get("orc") {
        for i in 0..=1 {
            let template = orc_template.clone();
            let entity_id = id_generator.next();
            world.enqueue_action(ActionType::CreateEntity {
                entity_id,
                position: Position { x: 5, y: i },
                is_player: false,
                is_solid: true,
                template,
                cost: 0,
            });
        }
        world.process_actions();
    }
}

fn spawn_potion(
    mut world: ResMut<Game>,
    mut id_generator: ResMut<EntityIdGenerator>,
    _data_asset: Res<Assets<GameData>>,
    _game_data_handle: Res<GameDataHandle>,
) {
    world.enqueue_action(ActionType::CreateItem {
        entity_id: id_generator.next(),
        position: Position { x: 2, y: 2 },
        glyph: Glyph {
            character: '!',
            color: Color::GREEN,
        },
        name: Name("Health Potion".to_owned()),
        cost: 0,
    });
    world.process_actions();
}
