mod game;
mod main_menu;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::AppState;

use self::{game::*, main_menu::*};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::Loading, setup)
            .add_system(button_highlight)
            .add_enter_system(AppState::MainMenu, main_menu_setup)
            .add_exit_system(AppState::MainMenu, main_menu_cleanup)
            .add_system(main_menu_buttons_interaction.run_in_state(AppState::MainMenu))
            .add_enter_system(AppState::InGame, game_setup)
            .add_exit_system(AppState::InGame, game_cleanup);
    }
}

pub struct FontHandle(Handle<Font>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = FontHandle(asset_server.load("fonts/square.ttf"));
    commands.insert_resource(font_handle);

    commands.spawn_bundle(UiCameraBundle::default());
}

fn button_highlight(mut query: Query<(&mut UiColor, &Interaction), Changed<Interaction>>) {
    for (mut color, interaction) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => *color = Color::rgb(0.3, 0.3, 0.3).into(),
            Interaction::Hovered => *color = Color::rgb(0.2, 0.2, 0.2).into(),
            Interaction::None => *color = Color::NONE.into(),
        }
    }
}
