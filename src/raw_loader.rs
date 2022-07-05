use bevy::{
    asset::{AssetLoader, BoxedFuture, LoadContext, LoadState, LoadedAsset},
    prelude::{App, AssetServer, Color, Commands, Handle, HandleUntyped, Plugin, Res, AddAsset},
    reflect::TypeUuid,
    utils::HashMap,
};

use anyhow::Result;
use iyes_loopless::{
    prelude::{AppLooplessStateExt, IntoConditionalSystem},
    state::NextState,
};

use crate::{world::components::*, AppState};

pub struct RawLoaderPlugin;

impl Plugin for RawLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<GameData>()
            .init_asset_loader::<RawAssetLoader>()
            .add_enter_system(AppState::Loading, load_game_data)
            .add_system(check_assets_ready.run_in_state(AppState::Loading));
    }
}

struct AssetsLoading(Vec<HandleUntyped>);
pub struct GameDataHandle(pub Handle<GameData>);

fn load_game_data(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut loading = AssetsLoading(vec![]);
    let monsters: Handle<GameData> = asset_server.load("monsters.raw");

    loading.0.push(monsters.clone_untyped());

    let game_data_handle = GameDataHandle(monsters);
    commands.insert_resource(game_data_handle);
    commands.insert_resource(loading);
}

fn check_assets_ready(
    mut commands: Commands,
    server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
) {
    match server.get_group_load_state(loading.0.iter().map(|h| h.id)) {
        LoadState::Failed => {
            // one of our assets had an error
        }
        LoadState::Loaded => {
            commands.insert_resource(NextState(AppState::MainMenu));
            commands.remove_resource::<AssetsLoading>();
        }
        _ => {
            // NotLoaded/Loading: not fully ready yet
        }
    }
}

#[derive(Default)]
pub struct RawAssetLoader;

impl AssetLoader for RawAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<()>> {
        Box::pin(async move {
            let source = std::str::from_utf8(bytes)?;
            let entities = raw_loader::entities(source)?;

            load_context.set_default_asset(LoadedAsset::new(GameData { entities }));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["raw"]
    }
}

peg::parser!(
    grammar raw_loader() for str {
        pub rule entities() -> Entities = templates:(template() / comment())* {
            templates.into_iter().flatten().fold(Entities::new(), |mut entities: Entities, (id, template)| {
                entities.insert(id, template);
                entities
            })
        }

        rule template() -> Option<(String, EntityTemplate)>
        = id:(word()) _ name:(word()) _ glyph:(glyph()) _ attack:(attack()) _ health:(health()) _ initiative:(initiative()) end() {
            Some((id, EntityTemplate { name: Name(name), glyph, attack, health, initiative }))
        }
        rule comment() -> Option<(String, EntityTemplate)> = "#" skip_to_line_end() { None }

        rule attack() -> Attack = attack:(i64()) { Attack(attack) }
        rule health() -> Health = health:(i64()) { Health(health) }
        rule initiative() -> Initiative = initiative:(i64()) { Initiative(initiative) }

        rule glyph() -> Glyph = character:([_]) _ color:(color()) { Glyph { character, color } }
        rule color() -> Color = "#" color:$(hex()*<6>) {? Color::hex(color).or(Err("Color error")) }

        rule word() -> String = word:$(character()+) { word.to_owned() }
        rule character() -> char = character:(['a'..='z' | 'A'..='Z' | '_']) { character }

        rule i64() -> i64 = digits:$(digit()+) {? digits.parse::<i64>().or(Err("Digits error")) }

        rule hex() -> char = hex:(['0'..='9' | 'A'..='F']) { hex }
        rule digit() -> char = digit:(['0'..='9']) { digit }

        rule skip_to_line_end() = [^ '\n']* ['\n']

        rule end() = quiet!{"\n" / "\r\n" / eof()}
        rule eof() = quiet!{![_]}
        rule _() = quiet!{[' ']+}
    }
);

type Entities = HashMap<String, EntityTemplate>;

#[derive(Debug, TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct GameData {
    pub entities: Entities,
}

#[derive(Debug, Clone)]
pub struct EntityTemplate {
    pub name: Name,
    pub glyph: Glyph,
    pub attack: Attack,
    pub health: Health,
    pub initiative: Initiative,
}
