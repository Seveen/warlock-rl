use bevy::prelude::*;
use iyes_loopless::state::NextState;

use crate::AppState;

use super::FontHandle;

#[derive(Component)]
pub struct MainMenuItem;

#[derive(Component)]
pub struct ButtonUsage(ButtonTag);

#[derive(Copy, Clone)]
enum ButtonTag {
    NewGame,
    LoadGame,
}

pub fn main_menu_setup(mut commands: Commands, font_handle: Res<FontHandle>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(MainMenuItem)
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .insert(MainMenuItem)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                flex_direction: FlexDirection::ColumnReverse,
                                justify_content: JustifyContent::Center,
                                size: Size::new(Val::Percent(20.0), Val::Percent(20.0)),
                                ..default()
                            },
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .insert(MainMenuItem)
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(ButtonBundle {
                                    color: Color::NONE.into(),
                                    ..default()
                                })
                                .insert(MainMenuItem)
                                .insert(ButtonUsage(ButtonTag::NewGame))
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(TextBundle {
                                            text: Text::with_section(
                                                "New game".to_string(),
                                                TextStyle {
                                                    font: font_handle.0.clone(),
                                                    ..default()
                                                },
                                                TextAlignment { ..default() },
                                            ),
                                            ..default()
                                        })
                                        .insert(MainMenuItem);
                                });
                            parent
                                .spawn_bundle(ButtonBundle {
                                    color: Color::NONE.into(),
                                    ..default()
                                })
                                .insert(MainMenuItem)
                                .insert(ButtonUsage(ButtonTag::LoadGame))
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(TextBundle {
                                            text: Text::with_section(
                                                "Load game".to_string(),
                                                TextStyle {
                                                    font: font_handle.0.clone(),
                                                    ..default()
                                                },
                                                TextAlignment { ..default() },
                                            ),
                                            ..default()
                                        })
                                        .insert(MainMenuItem);
                                });
                        });
                });
        });
}

pub fn main_menu_cleanup(mut commands: Commands, query: Query<Entity, With<MainMenuItem>>) {
    for item in query.iter() {
        commands.entity(item).despawn();
    }
}

pub fn main_menu_buttons_interaction(
    mut commands: Commands,
    query: Query<(&ButtonUsage, &Interaction), Changed<Interaction>>,
) {
    for (&ButtonUsage(tag), &interaction) in query.iter() {
        if interaction == Interaction::Clicked {
            match tag {
                ButtonTag::NewGame => commands.insert_resource(NextState(AppState::GenerateWorld)),
                ButtonTag::LoadGame => commands.insert_resource(NextState(AppState::LoadWorld)),
            }
        }
    }
}
