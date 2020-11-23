// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::building::*;

#[derive(Default)]
pub struct Planet {
    pub current_aura: Option<Aura>,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aura {
    MoonSpeed,
    ProductionSpeed,
    RocketDamage,
    RocketSpeed,
    Shield,
}

pub struct Moon {
    pub orbit_radius: f32,
    pub speed: f64,
    pub building: Option<BuildingType>,
}

pub struct Rocket {
    pub velocity: Vec2,
}

pub struct ResourcesText;
pub struct PlayerResources {
    pub pink: u32,
    pub green: u32,
}
