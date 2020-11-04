// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::math::Vector3,
    core::transform::Transform,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{
        Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture, Transparent,
    },
    ui::{Anchor, LineMode, TtfFormat, UiImage, UiText, UiTransform},
    window::ScreenDimensions,
};
use amethyst_rendy::palette::Srgba;

use crate::{
    sprites::init_sprite_resource, states::pause::PauseMenuState, systems::ResourcesText, Moon,
    Planet,
};

#[derive(Default)]
pub struct GameplayState;

/// Contains the main state with the game logic.
impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        let sprite_sheet_handle = load_sprite_sheet(world);
        init_camera(world, &dimensions);
        init_planet(world, sprite_sheet_handle.clone(), &dimensions);
        init_ui(world, sprite_sheet_handle.clone());
        init_sprite_resource(world, sprite_sheet_handle);
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    log::info!("[Trans::Push] Pausing Game!");
                    Trans::Push(Box::new(PauseMenuState::default()))
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }

    fn fixed_update(&mut self, _data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        SimpleTrans::None
    }
}

/// Creates a camera entity in the `world`.
///
/// The `dimensions` are used to center the camera in the middle
/// of the screen, as well as make it cover the entire screen.
fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() / 2.0, dimensions.height() / 2.0, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

/// Loads and splits the `sprites.png` image asset into 3 sprites,
/// which will then be assigned to entities for rendering them.
///
/// The provided `world` is used to retrieve the resource loader.
fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the texture for our sprites. We'll later need to
    // add a handle to this texture to our `SpriteRender`s, so
    // we need to keep a reference to it.
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/planets.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    // Load the spritesheet definition file, which contains metadata on our
    // spritesheet texture.
    let loader = world.read_resource::<Loader>();
    let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

    loader.load(
        "sprites/planets.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sheet_storage,
    )
}

/// Creates a single planet without a moon.
fn init_planet(
    world: &mut World,
    sprite_sheet_handle: Handle<SpriteSheet>,
    dimensions: &ScreenDimensions,
) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(dimensions.width() / 2.0, dimensions.height() / 2.0, 0.0);

    // Assign the first sprite on the sprite sheet, as this is the planet
    let sprite_render = SpriteRender::new(sprite_sheet_handle.clone(), 0);

    world
        .create_entity()
        .with(sprite_render)
        .with(Planet)
        .with(local_transform.clone())
        .build();

    local_transform.set_scale(Vector3::new(0.25, 0.25, 0.25));
    let moon_sprite = SpriteRender::new(sprite_sheet_handle, 1);

    world
        .create_entity()
        .with(moon_sprite)
        .with(Moon {
            velocity: 0.25,
            mining: false,
        })
        .with(local_transform)
        .build();
}

/// Creates the UI that shows the resources of the player.
fn init_ui(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let (r, g, b, a) = Srgba::new(37. / 255., 205. / 255., 227. / 255., 0.8)
        .into_linear()
        .into_components();
    // this creates the simple gray background UI element.
    world
        .create_entity()
        .with(UiImage::SolidColor([r, g, b, a]))
        .with(UiTransform::new(
            "".to_string(),
            Anchor::TopLeft,
            Anchor::TopLeft,
            30.0,
            -30.,
            0.,
            250.,
            50.,
        ))
        .with(Transparent)
        .build();

    // Assign the third sprite on the sprite sheet, as this is the minerals icon
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 2);

    world
        .create_entity()
        .with(UiImage::Sprite(sprite_render))
        .with(UiTransform::new(
            "".to_string(),
            Anchor::TopLeft,
            Anchor::TopLeft,
            35.,
            -30.,
            1.,
            50.,
            50.,
        ))
        .with(Transparent)
        .build();

    let font = world.read_resource::<Loader>().load(
        "fonts/Nunito-SemiBold.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    // Creates the actual label and places it on the screen.
    let p_resources = world
        .create_entity()
        .with(UiTransform::new(
            "".to_string(),
            Anchor::TopLeft,
            Anchor::TopLeft,
            90.0,
            -31.,
            1.,
            200.,
            50.,
        ))
        .with(UiText::new(
            font,
            "0".to_string(),
            [1., 1., 1., 1.],
            48.,
            LineMode::Single,
            Anchor::TopLeft,
        ))
        .build();

    world.insert(ResourcesText { p_red: p_resources });
}
