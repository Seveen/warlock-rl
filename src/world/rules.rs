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
                    println!("Attacking {entity_at:?} at pos: {new_position:?}");
                    return (ActionStatus::Reject, RuleStatus::StopChecking, reactions);
                }
                println!("Bumped on {entity_at:?} at pos: {new_position:?}");
                return (ActionStatus::Reject, RuleStatus::StopChecking, reactions);
            }
        }
    }

    (ActionStatus::Accept, RuleStatus::KeepChecking, reactions)
}

pub fn death(
    action: &Action,
    _state: &GameState,
    _spatial_position: &RTree<PositionTreeObject>,
) -> (ActionStatus, RuleStatus, Vec<ActionType>) {
    let mut reactions = Vec::new();
    for (&id, &health) in action.get_updated_health() {
        if health.0 <= 0 {
            println!("Rule: Jean-Michel {:?} is canning!", id);
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
    _state: &GameState,
    _spatial_position: &RTree<PositionTreeObject>,
) -> (ActionStatus, RuleStatus, Vec<ActionType>) {
    let mut reactions = Vec::new();

    for (&id, &action_cost) in action.get_updated_actioncost() {
        if action_cost.0 != 0 {
            println!("{:?}'s action cost {}", id, action_cost.0);
            reactions.push(ActionType::DecreaseEnergy {
                entity_id: id,
                value: action_cost.0,
            });
        }
    }

    (ActionStatus::Accept, RuleStatus::KeepChecking, reactions)
}
