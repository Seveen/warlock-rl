use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    save::SaveEvent,
    turn::NextAction,
    world::{actions::ActionType, components::*},
    AppState,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(movement_input.run_in_state(AppState::InGame))
            .add_system(grab_input.run_in_state(AppState::InGame))
            .add_system(toggle_camera_lock.run_in_state(AppState::InGame))
            .add_system(debug_save.run_in_state(AppState::InGame));
    }
}

#[derive(Component)]
pub struct CameraLock(pub bool);

fn toggle_camera_lock(
    keyboard: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut camera: Query<(Entity, &mut CameraLock), With<Camera>>,
    players: Query<Entity, (Without<Camera>, With<Player>)>,
) {
    if keyboard.just_pressed(KeyCode::Q) {
        if let Ok((camera, mut lock)) = camera.get_single_mut() {
            for entity in players.iter() {
                if lock.0 {
                    commands.entity(entity).remove_children(&[camera]);
                } else {
                    commands.entity(entity).add_child(camera);
                }
                lock.0 = !lock.0;
            }
        }
    }
}

fn movement_input(
    keyboard: Res<Input<KeyCode>>,
    mut next_action: ResMut<NextAction>,
    players: Query<&EntityId, With<Player>>,
) {
    for &entity_id in players.iter() {
        if keyboard.just_pressed(KeyCode::W) {
            next_action.push(ActionType::MoveBy {
                entity_id,
                dx: 0,
                dy: 1,
                cost: 100,
            });
        }
        if keyboard.just_pressed(KeyCode::S) {
            next_action.push(ActionType::MoveBy {
                entity_id,
                dx: 0,
                dy: -1,
                cost: 100,
            });
        }
        if keyboard.just_pressed(KeyCode::A) {
            next_action.push(ActionType::MoveBy {
                entity_id,
                dx: -1,
                dy: 0,
                cost: 100,
            });
        }
        if keyboard.just_pressed(KeyCode::D) {
            next_action.push(ActionType::MoveBy {
                entity_id,
                dx: 1,
                dy: 0,
                cost: 100,
            });
        }
        if keyboard.just_pressed(KeyCode::Space) {
            next_action.push(ActionType::Wait {
                entity_id,
                cost: 100,
            });
        }
    }
}

fn grab_input(
    keyboard: Res<Input<KeyCode>>,
    mut next_action: ResMut<NextAction>,
    players: Query<&EntityId, With<Player>>,
) {
    for &entity_id in players.iter() {
        if keyboard.just_pressed(KeyCode::G) {
            next_action.push(ActionType::GrabItem {
                grabber_id: entity_id,
                cost: 100,
            });
        }
    }
}

// DEBUG ////////////////////////////////////////////////////////////////
fn debug_save(keyboard: Res<Input<KeyCode>>, mut save_event: EventWriter<SaveEvent>) {
    if keyboard.just_pressed(KeyCode::R) {
        save_event.send(SaveEvent);
    }
}
