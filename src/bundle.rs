// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use crate::systems::{CameraMovementSystem, CombatSystem, KeplerSystem, ResourcesSystem};
use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    error::Error,
};

/// A bundle is a convenient way to initialise related resources, components and systems in a
/// world. This bundle prepares the main systems for gameplay.
pub struct GameplayBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameplayBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(CombatSystem, "combat_system", &[]);
        //builder.add(BuildingSystem, "building_system", &[]);
        builder.add(CameraMovementSystem, "camera_movement_system", &[]);
        builder.add(KeplerSystem, "kepler_system", &[]);
        builder.add(ResourcesSystem, "resources_system", &[]);
        Ok(())
    }
}
