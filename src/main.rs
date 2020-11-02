// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

//! Moonshot, a 2020 GitHub Game Off submission.
//! This will be a strategy game with planets, moons, and rockets!
//! ...possibly also asteroids

mod bundle;
mod state;
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
};
use amethyst_rendy::palette::Srgba;

use crate::bundle::GameplayBundle;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let key_bindings_path = app_root.join("config").join("input.ron");

    let (r, g, b, a) = Srgba::new(0. / 255., 26. / 255., 68. / 255., 1.)
        .into_linear()
        .into_components();

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with_bundle(GameplayBundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?.with_clear([r, g, b, a]),
                )
                .with_plugin(RenderUi::default())
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default()),
        )?;

    let assets_dir = app_root.join("assets");
    let mut game = Application::build(assets_dir, state::GameplayState::default())?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            144,
        )
        .build(game_data)?;
    game.run();

    Ok(())
}

pub struct Planet;

impl Component for Planet {
    type Storage = DenseVecStorage<Self>;
}

pub struct Moon {
    velocity: f32,
}

impl Component for Moon {
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
