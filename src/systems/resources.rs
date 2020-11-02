// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use crate::{Planet, ResourcesBoard};
use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::prelude::{Entity, Join, ReadExpect, System, SystemData, Write, WriteStorage},
    ui::UiText,
};

/// This system is responsible for counting the resources harvested by the player
#[derive(SystemDesc)]
pub struct ResourcesSystem;

impl<'s> System<'s> for ResourcesSystem {
    type SystemData = (
        WriteStorage<'s, Planet>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ResourcesBoard>,
        ReadExpect<'s, ResourcesText>,
    );

    fn run(
        &mut self,
        (
            mut planets,
            mut transforms,
            mut text,
            mut resources_board,
            resources_text,
        ): Self::SystemData,
    ) {
        //for (planet, transform) in (&mut planets, &mut transforms).join() {
        resources_board.red = (resources_board.red + 1).min(512);
        if let Some(text) = text.get_mut(resources_text.p_red) {
            text.text = resources_board.red.to_string();
        }
        //}
    }
}

/// Stores the entities that are displaying the player's resource count with UiText.
pub struct ResourcesText {
    pub p_red: Entity,
}
