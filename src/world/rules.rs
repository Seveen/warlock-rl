use rstar::RTree;

use super::{actions::*, components::*};

pub fn collision(
    action: &Action,
    state: &GameState,
    spatial_position: &RTree<PositionTreeObject>,
) -> (ActionStatus, RuleStatus, Vec<ActionType>) {
    let mut reactions = Vec::new();

    let future_state = FutureState { action, state };

    for (&moved_id, &new_position) in action.get_updated_position() {
        for &PositionTreeObject { entity_at, .. } in
            spatial_position.locate_all_at_point(&new_position)
        {
            if moved_id == entity_at {
                continue;
            }

            if future_state.get_solid(entity_at).is_some() {
                if future_state.get_health(entity_at).is_some() {
                    reactions.push(ActionType::DamageEntity {
                        attacker_id: moved_id,
                        target_id: entity_at,
                        cost: 0,
                    });
                    if let Some(name) = state.get_name(entity_at) {
                        println!("Attacking {name} {entity_at} at pos: {new_position:?}");
                    }
                    return (ActionStatus::Reject, RuleStatus::StopChecking, reactions);
                }
                if let Some(name) = state.get_name(entity_at) {
                    println!("Bumped on {name} {entity_at} at pos: {new_position:?}");
                }
                return (ActionStatus::Reject, RuleStatus::StopChecking, reactions);
            }
        }
    }

    (ActionStatus::Accept, RuleStatus::KeepChecking, reactions)
}

pub fn death(
    action: &Action,
    state: &GameState,
    _spatial_position: &RTree<PositionTreeObject>,
) -> (ActionStatus, RuleStatus, Vec<ActionType>) {
    let mut reactions = Vec::new();
    for (&id, &health) in action.get_updated_health() {
        if health.0 <= 0 {
            if let Some(name) = state.get_name(id) {
                println!("{name} {id} is dying!");
            }
            reactions.push(ActionType::Die {
                entity_id: id,
                cost: 0,
            });
        }
    }

    (ActionStatus::Accept, RuleStatus::KeepChecking, reactions)
}

pub fn compute_energy_cost(
    action: &Action,
    state: &GameState,
    _spatial_position: &RTree<PositionTreeObject>,
) -> (ActionStatus, RuleStatus, Vec<ActionType>) {
    let mut reactions = Vec::new();

    for (&id, &action_cost) in action.get_updated_actioncost() {
        if action_cost.0 != 0 {
            if let Some(initiative) = state.get_initiative(id) {
                let ratio = 100 / initiative.0;
                let action_cost = action_cost.0 * ratio;
                reactions.push(ActionType::DecreaseEnergy {
                    entity_id: id,
                    value: action_cost,
                });
            }
        }
    }

    (ActionStatus::Accept, RuleStatus::KeepChecking, reactions)
}
