use bevy::prelude::{Color, Component};
use bevy_inspector_egui::Inspectable;
use derive_more::{From, Into, Display};
use rstar::Point;
use rule_system::register_components;

#[derive(
    Debug, Clone, Copy, Eq, PartialEq, Hash, Component, Inspectable, Serialize, Deserialize, Display
)]
pub struct EntityId(pub u64);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl Point for Position {
    type Scalar = i64;

    const DIMENSIONS: usize = 2;

    fn generate(mut generator: impl FnMut(usize) -> Self::Scalar) -> Self {
        Position {
            x: generator(0),
            y: generator(1),
        }
    }

    fn nth(&self, index: usize) -> Self::Scalar {
        match index {
            0 => self.x,
            1 => self.y,
            _ => unreachable!(),
        }
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Display, Into, From)]
pub struct Attack(pub i64);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Display, Into, From)]
pub struct Health(pub i64);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Display, Into, From)]
pub struct Initiative(pub u32);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Display, Into, From)]
pub struct Name(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Glyph {
    pub character: char,
    pub color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Component, Serialize, Deserialize)]
pub struct Player;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Solid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Item;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CarriedBy(pub EntityId);

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Display, Into, From)]
pub struct Energy(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Display, Into, From)]
pub struct ActionCost(pub u32);

register_components!(
    index EntityId,
    components {
        Health, Attack, Initiative, Glyph, Name, Player, Solid, Item, CarriedBy, Energy, ActionCost
    }
    spatial {
        Position
    }
);
