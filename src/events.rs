use bevy::{
    math::Vec3,
    prelude::{
        App, Commands, Entity, EventReader, EventWriter, Plugin, Query, Res, ResMut, Transform,
    },
};
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::{
    graphics::{spawn_ascii_sprite, AsciiSheet, TILE_SIZE},
    world::{
        components::{self, EntityId, Glyph, Name, Position},
        Game, GameEvent,
    },
    AppState,
};

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveSprite>()
            .add_event::<SpawnSprite>()
            .add_event::<DeleteSprite>()
            .add_event::<EntityNearby>()
            .add_system(map_events.run_in_state(AppState::InGame))
            .add_system(move_listener.run_in_state(AppState::InGame))
            .add_system(delete_listener.run_in_state(AppState::InGame))
            .add_system(spawn_listener.run_in_state(AppState::InGame));
    }
}

fn map_events(
    mut world: ResMut<Game>,
    mut movement_events: EventWriter<MoveSprite>,
    mut spawn_events: EventWriter<SpawnSprite>,
    mut delete_events: EventWriter<DeleteSprite>,
    mut entity_nearby_events: EventWriter<EntityNearby>,
) {
    for event in world.events_queue.drain(..) {
        match event {
            GameEvent::MovedTo { entity_id, x, y } => {
                movement_events.send(MoveSprite { entity_id, x, y })
            }
            GameEvent::Spawned {
                entity_id,
                x,
                y,
                glyph,
                name,
                is_player,
            } => spawn_events.send(SpawnSprite {
                entity_id,
                x,
                y,
                name,
                glyph,
                is_player,
            }),
            GameEvent::Deleted { entity_id } => delete_events.send(DeleteSprite { entity_id }),
            GameEvent::EntityNearby {
                entity_id,
                name,
                position,
            } => entity_nearby_events.send(EntityNearby {
                entity_id,
                name,
                position,
            }),
        }
    }
}

pub struct MoveSprite {
    pub entity_id: EntityId,
    pub x: i64,
    pub y: i64,
}

fn move_listener(
    mut events: EventReader<MoveSprite>,
    mut sprites: Query<(&mut Transform, &EntityId)>,
) {
    for event in events.iter() {
        for (mut transform, &id) in sprites.iter_mut() {
            if id == event.entity_id {
                transform.translation.x = event.x as f32 * TILE_SIZE;
                transform.translation.y = event.y as f32 * TILE_SIZE;
            }
        }
    }
}

pub struct SpawnSprite {
    pub entity_id: EntityId,
    pub x: i64,
    pub y: i64,
    pub name: components::Name,
    pub glyph: Glyph,
    pub is_player: bool,
}

fn spawn_listener(
    mut commands: Commands,
    mut events: EventReader<SpawnSprite>,
    ascii_sheet: Res<AsciiSheet>,
) {
    for event in events.iter() {
        let translation = Vec3::new(
            event.x as f32 * TILE_SIZE,
            event.y as f32 * TILE_SIZE,
            100.0,
        );
        spawn_ascii_sprite(
            &mut commands,
            &ascii_sheet,
            event.glyph,
            translation,
            event.name.0.clone(),
            event.entity_id,
            event.is_player,
        );
    }
}

pub struct DeleteSprite {
    pub entity_id: EntityId,
}

fn delete_listener(
    mut commands: Commands,
    mut events: EventReader<DeleteSprite>,
    sprites: Query<(Entity, &EntityId)>,
) {
    for event in events.iter() {
        for (entity, &id) in sprites.iter() {
            if id == event.entity_id {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub struct EntityNearby {
    pub entity_id: EntityId,
    pub name: Name,
    pub position: Position,
}
