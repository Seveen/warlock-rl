use bevy::{prelude::*, render::camera::ScalingMode};
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::{
    input::CameraLock,
    world::components::{EntityId, Glyph, Player},
    AppState,
};

pub struct GraphicsPlugin;
pub struct AsciiSheet(pub Handle<TextureAtlas>);

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.05;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(CLEAR))
            .insert_resource(WindowDescriptor {
                width: 1920.0,
                height: 1080.0,
                title: "RL".to_string(),
                resizable: false,
                ..Default::default()
            })
            .add_enter_system(AppState::Loading, load_ascii_tileset);
    }
}

pub fn spawn_ascii_sprite(
    commands: &mut Commands,
    ascii_sheet: &AsciiSheet,
    glyph: Glyph,
    translation: Vec3,
    name: String,
    id: EntityId,
    is_player: bool,
) -> Entity {
    let mut sprite = TextureAtlasSprite::new(glyph.character as usize);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
    sprite.color = glyph.color;

    let mut entity = commands.spawn_bundle(SpriteSheetBundle {
        sprite,
        texture_atlas: ascii_sheet.0.clone(),
        transform: Transform {
            translation,
            ..Default::default()
        },
        ..Default::default()
    });
    let entity_id = entity.id();

    entity.insert(Name::new(name));
    entity.insert(id);

    if is_player {
        entity.insert(Player);
        let camera = spawn_camera(commands);
        commands.entity(entity_id).add_child(camera);
    }

    entity_id
}

fn load_ascii_tileset(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("rexpaint_cp437_10x10.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::splat(10.0), 16, 16);

    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(AsciiSheet(atlas_handle));
}

fn spawn_camera(commands: &mut Commands) -> Entity {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    camera.transform.translation.x = 0.36;
    // camera.transform.translation.y = -0.2; // Remonte la caméra pour le bottom panel

    commands.spawn_bundle(camera).insert(CameraLock(true)).id()
}
