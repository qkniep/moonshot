// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use amethyst::{
    assets::{AssetStorage, Handle, Loader, PrefabData, PrefabLoader, ProgressCounter, RonFormat},
    derive::PrefabData,
    ecs::Entity,
    prelude::*,
    renderer::{
        sprite::prefab::SpriteScenePrefab, ImageFormat, SpriteSheet, SpriteSheetFormat, Texture,
    },
    ui::UiCreator,
    Error,
};
use log::debug;
use serde::Deserialize;

use crate::sprites::init_sprite_resource;
use crate::{states::game::GameplayState, Moon, Planet};

#[derive(Debug, Clone, Deserialize, PrefabData)]
pub struct MyPrefabData {
    sprite_scene: SpriteScenePrefab,
    moon_data: Option<Moon>,
    planet_data: Option<Planet>,
}

#[derive(Default)]
pub struct LoadingState {
    root: Option<Entity>,
    loading_progress: ProgressCounter,
}

/// Contains the main state with the game logic.
impl<'a> SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        self.root = Some(
            world.exec(|mut creator: UiCreator<'_>| creator.create("ui/loading_screen.ron", ())),
        );

        let sprite_sheet_handle = load_sprite_sheet(world, &mut self.loading_progress);
        let map_prefab = world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
            loader.load("maps/default.ron", RonFormat, &mut self.loading_progress)
        });
        world.create_entity().with(map_prefab).build();
        init_sprite_resource(world, sprite_sheet_handle);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(root) = self.root {
            if data.world.delete_entity(root).is_ok() {
                self.root = None;
            }
        }
    }

    fn update(&mut self, _data: &mut StateData<GameData>) -> SimpleTrans {
        if self.loading_progress.is_complete() {
            Trans::Switch(Box::new(GameplayState::default()))
        } else {
            Trans::None
        }
    }
}

/// Loads and splits the `sprites.png` image asset into 3 sprites,
/// which will then be assigned to entities for rendering them.
///
/// The provided `world` is used to retrieve the resource loader.
fn load_sprite_sheet(
    world: &mut World,
    progress_counter: &mut ProgressCounter,
) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/planets.png",
            ImageFormat::default(),
            &mut *progress_counter,
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

    loader.load(
        "sprites/planets.ron",
        SpriteSheetFormat(texture_handle),
        progress_counter,
        &sheet_storage,
    )
}
