// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use bevy::prelude::*;

pub struct Planet;

pub struct Moon {
    pub orbit_radius: f32,
    pub speed: f64,
    pub mining: bool,
}

pub struct Rocket {
    pub velocity: Vec2,
}

pub struct ResourcesText;
pub struct PlayerResources {
    pub pink: u32,
    pub green: u32,
}