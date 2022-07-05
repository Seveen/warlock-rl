use bevy::prelude::*;

use super::FontHandle;

#[derive(Component)]
pub struct GameItem;

pub fn game_setup(mut commands: Commands, _font_handle: Res<FontHandle>) {
    commands
        // Root
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(GameItem)
        .with_children(|parent| {
            // // Bottom panel
            // parent
            //     .spawn_bundle(NodeBundle {
            //         style: Style {
            //             size: Size::new(Val::Percent(80.0), Val::Percent(20.0)),
            //             flex_direction: FlexDirection::ColumnReverse,
            //             flex_wrap: FlexWrap::Wrap,
            //             align_items: AlignItems::FlexStart,
            //             ..default()
            //         },
            //         color: Color::rgb(0.15, 0.15, 0.15).into(),
            //         ..default()
            //     })
            //     .insert(GameItem)
            //     .with_children(|parent| {
            //         // Log Text
            //         parent.spawn_bundle(TextBundle {
            //             text: Text::with_section(
            //                 "Coucou ce texte est long très long très long très long très long très long très long très long très long très long très long très long très long très long très long très long aaaaa ".to_string(),
            //                 TextStyle {
            //                     font: font_handle.0.clone(),
            //                     ..default()
            //                 },
            //                 TextAlignment { ..default() },
            //             ),
            //             style: Style {
            //                 flex_wrap: FlexWrap::Wrap,
            //                 ..default()
            //             },
            //             ..default()
            //         });

            //         parent.spawn_bundle(TextBundle {
            //             text: Text::with_section(
            //                 "Oui".to_string(),
            //                 TextStyle {
            //                     font: font_handle.0.clone(),
            //                     ..default()
            //                 },
            //                 TextAlignment { ..default() },
            //             ),
            //             ..default()
            //         });
            //     });

            // Right Panel
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(20.0), Val::Percent(100.0)),
                        ..default()
                    },
                    color: Color::GRAY.into(),
                    ..default()
                })
                .insert(GameItem);
        });
}

pub fn game_cleanup(mut commands: Commands, query: Query<Entity, With<GameItem>>) {
    for item in query.iter() {
        commands.entity(item).despawn();
    }
}
