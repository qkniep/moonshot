// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[derive(Debug)]
pub enum PlayerActions {
    Build(),
    ShootRocket {
        pos: Vec2,
        vel: Vec2,
    },
}
