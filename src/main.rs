use bevy::{prelude::*, DefaultPlugins};
use debug::DebugPlugin;
use events::EventsPlugin;
use graphics::GraphicsPlugin;
use input::InputPlugin;
use iyes_loopless::prelude::*;
use raw_loader::RawLoaderPlugin;
use save::SavePlugin;
use turn::TurnPlugin;
use ui::GameUiPlugin;
use world::WorldPlugin;

mod debug;
mod events;
mod graphics;
mod input;
mod raw_loader;
mod save;
mod ui;
mod world;
mod turn;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Loading,
    MainMenu,
    GenerateWorld,
    LoadWorld,
    InGame,
}

fn main() {
    App::new()
        .add_loopless_state(AppState::Loading)
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(GraphicsPlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(EventsPlugin)
        .add_plugin(TurnPlugin)
        .add_plugin(RawLoaderPlugin)
        .add_plugin(SavePlugin)
        .add_plugin(GameUiPlugin)
        .run();
}
