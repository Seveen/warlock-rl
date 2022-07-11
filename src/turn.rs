use std::collections::VecDeque;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use rand::prelude::*;

use crate::{
    world::{actions::ActionType, components::PositionTreeObject, Game},
    AppState,
};

pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_enter_system(AppState::InGame, setup)
            .add_system(turn_order.run_in_state(AppState::InGame));
    }
}

pub struct NextAction(VecDeque<ActionType>);

impl NextAction {
    fn new() -> Self {
        NextAction(VecDeque::new())
    }

    pub fn push(&mut self, action: ActionType) {
        self.0.push_back(action);
    }

    pub fn pop(&mut self) -> Option<ActionType> {
        self.0.pop_front()
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(NextAction::new());
}

fn turn_order(mut next_action: ResMut<NextAction>, mut world: ResMut<Game>) {
    let mut rng = rand::thread_rng();

    if let Some(&player_id) = world.state.player.keys().next() {
        for _ in 0..10 {
            let mut should_process_entities = false;

            if let Some(&energy) = world.state.get_energy(player_id) {
                if energy.0 >= 0 {
                    if let Some(action) = next_action.pop() {
                        world.enqueue_action(action);
                        world.process_actions();
                    }
                } else {
                    // Others turn
                    should_process_entities = true;
                }
            }

            if should_process_entities {
                for energy in world.state.energy.values_mut() {
                    energy.0 += 50;
                }

                let mut entities = vec![];

                if let Some(&player_position) = world.state.get_position(player_id) {
                    for PositionTreeObject { entity_at, .. } in world
                        .spatial_position
                        .locate_within_distance(player_position, 4000)
                    {
                        entities.push(*entity_at);
                    }
                }

                for entity_at in entities {
                    if let (Some(&energy), None) = (
                        world.state.get_energy(entity_at),
                        world.state.get_player(entity_at),
                    ) {
                        if energy.0 >= 0 {
                            // Debug //
                            // TODO: IA should compute next move for entity
                            let delta = rng.gen_range(-1..=1);
                            let x_or_y = rand::random();
                            let action = ActionType::MoveBy {
                                entity_id: entity_at,
                                dx: if x_or_y { delta } else { 0 },
                                dy: if !x_or_y { delta } else { 0 },
                                cost: 100,
                            };

                            //

                            world.enqueue_action(action);
                            world.process_actions();
                        }
                    }
                }
            }
        }
    }
}
