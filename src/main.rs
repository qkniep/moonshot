// Copyright (C) 2020 qkniep <qkniep@qkmac>
// Distributed under terms of the MIT license.

use amethyst::{
    core::transform::TransformBundle,
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

mod state;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let key_bindings_path = app_root.join("config").join("input.ron");

    let (r, g, b, a) = Srgba::new(0. / 255.,  26. / 255., 68. / 255., 1.)
        .into_linear()
        .into_components();

    let game_data = GameDataBuilder::default()
    .with_bundle(TransformBundle::new())?
    .with_bundle(
        InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
    )?
    .with_bundle(UiBundle::<StringBindings>::new())?
    .with_bundle(
        RenderingBundle::<DefaultBackend>::new()
            // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                    .with_clear([r, g, b, a]),
            )
            .with_plugin(RenderUi::default())
            // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
            .with_plugin(RenderFlat2D::default()),
    )?;

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, state::GameplayState, game_data)?;
    game.run();

    Ok(())
}
