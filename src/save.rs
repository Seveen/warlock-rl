use anyhow::{anyhow, Result};
use bevy::prelude::*;
use directories::UserDirs;
use iyes_loopless::prelude::*;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use crate::world::{components::GameState, Game};

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SaveEvent>()
            .add_system(save_world.run_on_event::<SaveEvent>());
    }
}

#[derive(Clone)]
pub struct SavePath(pub PathBuf);

fn build_save_path(name: &str) -> Result<SavePath> {
    get_saved_games_folder().map(|saves| SavePath(saves.join(format!("{name}.sav"))))
}

pub fn create_save_path(commands: &mut Commands, name: &str) -> Result<SavePath> {
    build_save_path(name).map(|save| {
        commands.insert_resource(save.clone());
        save
    })
}

fn get_saved_games_folder() -> Result<PathBuf> {
    let user_dirs = UserDirs::new().ok_or_else(|| anyhow!("Failed to find user dirs"))?;
    let documents = user_dirs
        .document_dir()
        .ok_or_else(|| anyhow!("Failed to find document dir"))?;
    let saves = PathBuf::from(documents)
        .join("SavedGames")
        .join("WarlockRL");

    fs::create_dir_all(&saves)?;
    Ok(saves)
}

pub struct SaveEvent;
fn save_world(world: Res<Game>, save_path: Res<SavePath>) {
    let encoded = bincode::serialize(&world.state).unwrap();
    File::create(&save_path.0)
        .expect("Failed to create save file")
        .write_all(&encoded)
        .expect("Failed to write to save file");
}

pub fn load_saved_game(save_path: SavePath) -> Result<GameState> {
    let bytes = fs::read(&save_path.0)?;
    let decoded: GameState = bincode::deserialize(&bytes)?;
    Ok(decoded)
}
