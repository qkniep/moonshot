// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

mod building;
mod components;
mod cursor_world_coords;

use bevy::{
    input::{keyboard::KeyboardInput, ElementState, Input},
    prelude::*,
    render::{camera::Camera, pass::ClearColor},
    ui::camera::UI_CAMERA,
};

use building::*;
use components::*;
use cursor_world_coords::*;

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(ClearColor(Color::hex("22265A").unwrap()))
            .add_resource(CursorInWorld::default())
            .add_resource(PlayerResources { pink: 0, green: 0 })
            .add_startup_system(game_setup.system())
            .add_system(cursor_world_coords.system())
            .add_system(camera_motion.system())
            .add_system(kepler_motion.system())
            .add_system(building.system())
            .add_system(combat.system())
            .add_system(resource_mining.system());
    }
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Moonshot!".to_string(),
            width: 1920,
            height: 1080,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}

fn game_setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/sprite_sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(256.0, 256.0), 4, 2);
    let texture_atlas_handle = texture_atlases.set("SPRITE_SHEET", texture_atlas);
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        .spawn(TextComponents {
            style: Style {
                align_self: AlignSelf::FlexStart,
                ..Default::default()
            },
            text: Text {
                value: "0, 0".to_string(),
                font: asset_server.load("fonts/Nunito-Regular.ttf"),
                style: TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    alignment: TextAlignment::default(),
                },
            },
            ..Default::default()
        })
        .with(ResourcesText)
        // Planet 1
        .spawn(SpriteSheetComponents {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..Default::default()
        })
        .with(Planet)
        .with_children(|parent| {
            parent
                // Moon 1
                .spawn(SpriteSheetComponents {
                    sprite: TextureAtlasSprite::new(1),
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform::from_scale(Vec3::splat(0.5)),
                    ..Default::default()
                })
                .with(Moon {
                    orbit_radius: 300.0,
                    speed: 1.0,
                    mining: false,
                })
                // Moon 2
                .spawn(SpriteSheetComponents {
                    sprite: TextureAtlasSprite::new(1),
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform::from_scale(Vec3::splat(0.5)),
                    ..Default::default()
                })
                .with(Moon {
                    orbit_radius: 500.0,
                    speed: 0.5,
                    mining: false,
                });
        })
        // Planet 2
        .spawn(SpriteSheetComponents {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_translation(Vec3::splat(700.0)),
            ..Default::default()
        })
        .with(Planet)
        .with_children(|parent| {
            parent
                // Moon 1
                .spawn(SpriteSheetComponents {
                    sprite: TextureAtlasSprite::new(1),
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform::from_scale(Vec3::splat(0.5)),
                    ..Default::default()
                })
                .with(Moon {
                    orbit_radius: 300.0,
                    speed: 1.0,
                    mining: false,
                })
                // Moon 2
                .spawn(SpriteSheetComponents {
                    sprite: TextureAtlasSprite::new(1),
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform::from_scale(Vec3::splat(0.5)),
                    ..Default::default()
                })
                .with(Moon {
                    orbit_radius: 500.0,
                    speed: 0.5,
                    mining: false,
                });
        });
}

fn camera_motion(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Camera, Mut<Transform>)>,
) {
    for (camera, mut trans) in query.iter_mut() {
        if camera.name == Some(UI_CAMERA.to_string()) {
            continue;
        }

        // determine direction based on keyboard input
        let mut direction = Vec3::splat(0.0);
        if keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 1.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction += Vec3::new(0.0, -1.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Left) {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }

        // move the camera at constant speed in determined direction
        let camera_speed = 500.0;
        let ds = camera_speed * time.delta_seconds;
        if direction.length() > 0.0 {
            trans.translation += direction.normalize() * ds;
        }
    }
}

fn kepler_motion(time: Res<Time>, mut query: Query<(&Moon, Mut<Transform>)>) {
    for (moon, mut trans) in query.iter_mut() {
        let ds = moon.speed * time.seconds_since_startup;
        let x = moon.orbit_radius * ds.cos() as f32;
        let y = moon.orbit_radius * ds.sin() as f32;
        trans.translation = Vec3::new(x, y, 0.0);
    }
}

#[derive(Default)]
struct CombatState {
    keyboard_event_reader: EventReader<KeyboardInput>,
}

/// System for shooting rockets in mouse cursor direction.
fn combat(
    commands: &mut Commands,
    mut state: Local<CombatState>,
    time: Res<Time>,
    keyboard_inputs: Res<Events<KeyboardInput>>,
    cursor_in_world: Res<CursorInWorld>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(Entity, &Rocket, Mut<Transform>)>,
) {
    for event in state.keyboard_event_reader.iter(&keyboard_inputs) {
        if event.key_code == Some(KeyCode::A) && event.state == ElementState::Pressed {
            let rocket_direction = cursor_in_world.position.normalize();
            let angle = rocket_direction.y().atan2(rocket_direction.x());
            commands
                .spawn(SpriteSheetComponents {
                    sprite: TextureAtlasSprite::new(7),
                    texture_atlas: texture_atlases.get_handle("SPRITE_SHEET"),
                    transform: Transform {
                        translation: Vec3::splat(0.0),
                        rotation: Quat::from_rotation_z(angle),
                        scale: Vec3::splat(0.25),
                    },
                    ..Default::default()
                })
                .with(Rocket {
                    velocity: 300.0 * rocket_direction,
                });
        }
    }

    for (entity, rocket, mut trans) in query.iter_mut() {
        trans.translation += rocket.velocity.extend(0.0) * time.delta_seconds;
        // despawn if out of bounds
        if trans.translation.length() > 800.0 {
            commands.despawn(entity);
        }
    }
}

struct ResourceMiningState {
    timer: Timer,
}

impl Default for ResourceMiningState {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, true)
        }
    }
}

fn resource_mining(
    mut state: Local<ResourceMiningState>,
    time: Res<Time>,
    mut resources: ResMut<PlayerResources>,
    moon_query: Query<&Moon>,
    mut text_query: Query<(&mut Text, &ResourcesText)>,
) {
    if state.timer.tick(time.delta_seconds).finished {
        for moon in moon_query.iter() {
            if moon.mining {
                resources.pink += 1;
            }
        }
    }

    for (mut text, _) in text_query.iter_mut() {
        text.value = format!("{}, {}", resources.pink, resources.green);
    }
}
