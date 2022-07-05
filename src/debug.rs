use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, _app: &mut App) {
        if cfg!(debug_assertions) {
            // app.add_plugin(WorldInspectorPlugin::new()).register_inspectable::<EntityId>();
        }
    }
}
