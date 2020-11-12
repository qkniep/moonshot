// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum PlayerAction {
    Build(),
    ShootRocket { pos: Vec2, vel: Vec2 },
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ServerTurn {
    actions: Vec<PlayerAction>,
}

impl ServerTurn {
    pub fn new(actions: Vec<PlayerAction>) -> Self {
        ServerTurn { actions }
    }
}
