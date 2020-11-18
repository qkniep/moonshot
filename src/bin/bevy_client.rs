// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use bevy::{
    input::{keyboard::KeyboardInput, ElementState, Input},
    prelude::*,
    render::{camera::Camera, pass::ClearColor},
    ui::camera::UI_CAMERA,
};

use moonshot::cursor_world_coords::*;
use moonshot::building::*;
use moonshot::components::*;
use moonshot::network::{NetworkPlugin, PlayerAction, Transport};

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(ClearColor(Color::hex("22265A").unwrap()))
            .add_resource(CursorInWorld::default())
            .add_resource(PlayerResources { pink: 30, green: 0 })
            .add_startup_system(game_setup)
            .add_system(cursor_world_coords)
            .add_system(camera_motion)
            .add_system(kepler_motion)
            .add_system(building)
            .add_system(combat)
            .add_system(resource_mining);
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
        .add_plugin(NetworkPlugin)
        .run();
}

fn game_setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/sprite_sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(256.0, 256.0), 4, 4);
    let texture_atlas_handle = texture_atlases.set("SPRITE_SHEET", texture_atlas);
    commands
        .spawn(Camera2dBundle::default())
        .spawn(UiCameraBundle::default())
        .spawn(TextBundle {
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
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..Default::default()
        })
        .with(Planet)
        .with_children(|parent| {
            parent
                // Moon 1
                .spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(1),
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform::from_scale(Vec3::splat(0.5)),
                    ..Default::default()
                })
                .with(Moon {
                    orbit_radius: 300.0,
                    speed: 1.0,
                    building: None,
                })
                // Moon 2
                .spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(1),
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform::from_scale(Vec3::splat(0.5)),
                    ..Default::default()
                })
                .with(Moon {
                    orbit_radius: 500.0,
                    speed: 0.5,
                    building: None,
                });
        })
        // Planet 2
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_translation(Vec3::splat(700.0)),
            ..Default::default()
        })
        .with(Planet)
        .with_children(|parent| {
            parent
                // Moon 1
                .spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(1),
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform::from_scale(Vec3::splat(0.5)),
                    ..Default::default()
                })
                .with(Moon {
                    orbit_radius: 300.0,
                    speed: 1.0,
                    building: None,
                })
                // Moon 2
                .spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(1),
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform::from_scale(Vec3::splat(0.5)),
                    ..Default::default()
                })
                .with(Moon {
                    orbit_radius: 500.0,
                    speed: 0.5,
                    building: None,
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
    current_rocket_base: Option<Entity>,
}

/// System for shooting rockets in mouse cursor direction.
fn combat(
    commands: &mut Commands,
    mut state: Local<CombatState>,
    time: Res<Time>,
    keyboard_inputs: Res<Events<KeyboardInput>>,
    mouse_input: Res<Input<MouseButton>>,
    cursor_in_world: Res<CursorInWorld>,
    mut transport: ResMut<Transport>,
    moon_query: Query<(Entity, &Moon, &GlobalTransform)>,
    mut rocket_query: Query<(Entity, &Rocket, Mut<Transform>)>,
) {
    for event in state.keyboard_event_reader.iter(&keyboard_inputs) {
        if event.key_code == Some(KeyCode::A) && event.state == ElementState::Pressed {
            let mut rocket_position = Vec3::splat(0.0);
            for (entity, _, trans) in moon_query.iter() {
                if state.current_rocket_base == Some(entity) {
                    rocket_position = trans.translation;
                }
            }
            let rocket_direction = cursor_in_world.position.normalize();

            let launch = PlayerAction::ShootRocket {
                pos: rocket_position.truncate(),
                dir: rocket_direction,
            };
            let serialized = bincode::serialize(&launch).unwrap();
            transport.send(serialized);
        }
    }

    let world_coords = cursor_in_world.position;
    if mouse_input.pressed(MouseButton::Left) {
        // check if cursor is inside of a moon
        // TODO: use actual sprite size instead of magic number
        for (entity, moon, trans) in moon_query.iter() {
            if trans.translation.x - 128.0 * trans.scale.x <= world_coords.x
                && trans.translation.x + 128.0 * trans.scale.x >= world_coords.x
                && trans.translation.y - 128.0 * trans.scale.y <= world_coords.y
                && trans.translation.y + 128.0 * trans.scale.y >= world_coords.y
                && moon.building == Some(BuildingType::Production)
            {
                //sprite.index = ...;
                state.current_rocket_base = Some(entity);
            }
        }
    }

    // move rockets according to their current velocity
    for (entity, rocket, mut trans) in rocket_query.iter_mut() {
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
            timer: Timer::from_seconds(1.0, true),
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
            if let Some(BuildingType::Mining) = moon.building {
                resources.pink += 1;
            }
        }
    }

    for (mut text, _) in text_query.iter_mut() {
        text.value = format!("{}, {}", resources.pink, resources.green);
    }
}
