use rstar::RTree;

use crate::raw_loader::EntityTemplate;

use super::components::*;

#[derive(Debug)]
pub enum ActionType {
    Wait {
        entity_id: EntityId,
        cost: u32,
    },
    MoveBy {
        entity_id: EntityId,
        dx: i64,
        dy: i64,
        cost: u32,
    },
    CreateEntity {
        entity_id: EntityId,
        position: Position,
        is_player: bool,
        is_solid: bool,
        template: EntityTemplate,
        cost: u32,
    },
    // TODO: Uniformiser/merger avec CreateEntity (système plus générique)?
    CreateItem {
        entity_id: EntityId,
        position: Position,
        glyph: Glyph,
        name: Name,
        cost: u32,
    },
    DamageEntity {
        attacker_id: EntityId,
        target_id: EntityId,
        cost: u32,
    },
    Die {
        entity_id: EntityId,
        cost: u32,
    },
    GrabItem {
        grabber_id: EntityId,
        cost: u32,
    },
    DecreaseEnergy {
        entity_id: EntityId,
        value: u32,
    },
}

pub fn populate_action(
    action_type: ActionType,
    state: &GameState,
    action: &mut Action,
    spatial_position: &RTree<PositionTreeObject>,
) {
    match action_type {
        ActionType::MoveBy {
            entity_id,
            dx,
            dy,
            cost,
            ..
        } => {
            move_by(action, state, entity_id, dx, dy, cost);
        }
        ActionType::CreateEntity {
            entity_id,
            position,
            is_player,
            is_solid,
            template,
            ..
        } => {
            create_entity(action, entity_id, position, is_player, is_solid, template);
        }
        ActionType::DamageEntity {
            attacker_id,
            target_id,
            ..
        } => {
            damage_entity(action, state, attacker_id, target_id);
        }
        ActionType::Die { entity_id, .. } => {
            die(action, entity_id);
        }
        ActionType::CreateItem {
            entity_id,
            position,
            glyph,
            name,
            ..
        } => create_item(action, entity_id, position, glyph, name),
        ActionType::GrabItem { grabber_id, .. } => {
            grab_item(action, state, spatial_position, grabber_id)
        }
        ActionType::DecreaseEnergy { entity_id, value } => {
            decrease_energy(action, state, entity_id, value)
        }
        ActionType::Wait { entity_id, cost } => wait(action, entity_id, cost),
    }
}

fn create_entity(
    action: &mut Action,
    entity_id: EntityId,
    position: Position,
    is_player: bool,
    is_solid: bool,
    template: EntityTemplate,
) {
    action.insert_position(entity_id, position);
    action.insert_attack(entity_id, template.attack);
    action.insert_health(entity_id, template.health);
    action.insert_initiative(entity_id, template.initiative);
    action.insert_glyph(entity_id, template.glyph);
    action.insert_name(entity_id, template.name);
    action.insert_energy(entity_id, 0.into());
    action.insert_actioncost(entity_id, 0.into());
    if is_player {
        action.insert_player(entity_id, Player);
    }
    if is_solid {
        action.insert_solid(entity_id, Solid);
    }
}

fn create_item(
    action: &mut Action,
    entity_id: EntityId,
    position: Position,
    glyph: Glyph,
    name: Name,
) {
    action.insert_position(entity_id, position);
    action.insert_glyph(entity_id, glyph);
    action.insert_name(entity_id, name);
    action.insert_item(entity_id, Item);
}

fn damage_entity(
    action: &mut Action,
    state: &GameState,
    attacker_id: EntityId,
    target_id: EntityId,
) {
    if let Some(attack) = state.get_attack(attacker_id) {
        if let Some(health) = state.get_health(target_id) {
            let new_health = health.0 - attack.0;
            println!(
                "Action: {} is hitting {} for {} damage! {} is now {} HP",
                attacker_id.0, target_id.0, attack.0, target_id.0, new_health
            );
            action.insert_health(target_id, Health(new_health));
        }
    }
}

fn wait(
    action: &mut Action,
    entity_id: EntityId,
    cost: u32,
) {
    println!("Action: wait at cost {}", cost);
    action.insert_actioncost(entity_id, cost.into());
}

fn move_by(
    action: &mut Action,
    state: &GameState,
    entity_id: EntityId,
    dx: i64,
    dy: i64,
    cost: u32,
) {
    if let Some(position) = state.get_position(entity_id) {
        action.insert_position(
            entity_id,
            Position {
                x: position.x + dx,
                y: position.y + dy,
            },
        );
        action.insert_actioncost(entity_id, cost.into());
    }
}

fn die(action: &mut Action, entity_id: EntityId) {
    println!("Action: {:?} died", entity_id);
    action.remove_all(entity_id);
}

fn grab_item(
    action: &mut Action,
    state: &GameState,
    spatial_position: &RTree<PositionTreeObject>,
    grabber_id: EntityId,
) {
    if let Some(&position) = state.get_position(grabber_id) {
        for &PositionTreeObject { entity_at, .. } in spatial_position.locate_all_at_point(&position)
        {
            if state.get_item(entity_at).is_some() {
                action.remove_position(entity_at);
                action.insert_carriedby(entity_at, CarriedBy(grabber_id));
            }
        }
    }
}

fn decrease_energy(action: &mut Action, state: &GameState, entity_id: EntityId, value: u32) {
    println!("Decreasing energy of {:?} : {:?}", entity_id, value);
    if let Some(energy) = state.get_energy(entity_id) {
        action.insert_actioncost(entity_id, 0.into());
        let new_energy = energy.0 - i64::from(value);
        action.insert_energy(entity_id, new_energy.into());
    }
}
