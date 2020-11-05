// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

//! Moonshot, a 2020 GitHub Game Off submission.
//! This will be a strategy game with planets, moons, and rockets!
//! ...possibly also asteroids

mod bundle;
mod sprites;
mod states;
mod systems;

use std::time::Duration;

use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    ecs::{Component, DenseVecStorage},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
    LogLevelFilter, LoggerConfig,
};

use crate::bundle::GameplayBundle;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(LoggerConfig {
        level_filter: LogLevelFilter::Debug,
        ..Default::default()
    });

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let key_bindings_path = app_root.join("config").join("input.ron");
    let assets_dir = app_root.join("assets");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with_bundle(GameplayBundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.01027, 0.05736, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default()),
        )?;

    let mut game = Application::build(assets_dir, states::game::GameplayState::default())?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            144,
        )
        .build(game_data)?;
    game.run();

    Ok(())
}

pub struct Planet {
    scale: f32,
}

impl Component for Planet {
    type Storage = DenseVecStorage<Self>;
}

pub struct Moon {
    scale: f32,
    velocity: f32,
    mining: bool,
    orbit_radius: f32,
}

impl Component for Moon {
    type Storage = DenseVecStorage<Self>;
}

pub struct Rocket;

impl Component for Rocket {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct ResourcesBoard {
    red: i32,
}

impl ResourcesBoard {
    pub fn new() -> ResourcesBoard {
        ResourcesBoard { red: 0 }
    }
}
