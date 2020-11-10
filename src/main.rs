use bevy::{
    input::{keyboard::KeyboardInput, ElementState, Input},
    prelude::*,
    render::{camera::{Camera, OrthographicProjection}, pass::ClearColor},
    ui::camera::UI_CAMERA,
};

struct Planet;

struct Moon {
    orbit_radius: f32,
    speed: f64,
    mining: bool,
}

struct Rocket {
    velocity: Vec2,
}

struct ResourcesText;
struct PlayerResources {
    pink: u32,
    green: u32,
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(ClearColor(Color::hex("22265A").unwrap()))
            .add_resource(PlayerResources { pink: 0, green: 0 })
            .add_startup_system(game_setup.system())
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
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
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
                },
            },
            ..Default::default()
        })
        .with(ResourcesText)
        // Planet
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
struct BuildingState {
    keyboard_event_reader: EventReader<KeyboardInput>,
    cursor_event_reader: EventReader<CursorMoved>,
    cursor_position: Vec2,
    cursor_follower: Option<Entity>,
    currently_building: bool,
}

struct CursorFollowing;

fn building(commands: &mut Commands,
    mut state: Local<BuildingState>,
    keyboard_inputs: Res<Events<KeyboardInput>>,
    mouse_input: Res<Input<MouseButton>>,
    cursor_inputs: Res<Events<CursorMoved>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    camera_query: Query<(&Camera, &Transform, &OrthographicProjection)>,
    mut moon_query: Query<(Mut<Moon>, Mut<TextureAtlasSprite>, &Transform)>,
    mut sprite_query: Query<(&CursorFollowing, Mut<Transform>)>,
) {
    for event in state.cursor_event_reader.iter(&cursor_inputs) {
        state.cursor_position = event.position;
    }

    // get the releveant attributes of the 2D orth. projection
    let mut camera_pos = Vec2::splat(0.0);
    let mut camera_width = 0.0;
    let mut camera_height = 0.0;
    for (camera, trans, orth) in camera_query.iter() {
        if camera.name == Some(UI_CAMERA.to_string()) {
            continue;
        }

        camera_pos = Vec2::new(trans.translation.x(), trans.translation.y());
        camera_width = orth.right - orth.left;
        camera_height = orth.top - orth.bottom;
    }

    // convert cursor position to world coordinates
    let x = state.cursor_position.x();
    let y = state.cursor_position.y();
    let screen_coords = Vec2::new(x - camera_width / 2.0, y - camera_height / 2.0);
    let world_coords = camera_pos + screen_coords;

    // change to building mode on button press
    let ta_id = texture_atlases.ids().next().unwrap();
    for event in state.keyboard_event_reader.iter(&keyboard_inputs) {
        if event.key_code == Some(KeyCode::B) && event.state == ElementState::Pressed {
            state.currently_building = true;
            state.cursor_follower = commands.spawn(SpriteSheetComponents {
                sprite: TextureAtlasSprite::new(6),
                texture_atlas: texture_atlases.get_handle(ta_id),
                transform: Transform {
                    translation: world_coords.extend(0.0),
                    rotation: Quat::default(),
                    scale: Vec3::splat(0.25),
                },
                ..Default::default()
            })
            .with(CursorFollowing)
            .current_entity();
        }
    }

    if state.currently_building {
        if mouse_input.pressed(MouseButton::Left) {
            // check if cursor is inside of a moon
            // TODO: use actual sprite size instead of magic number
            for (mut moon, mut sprite, trans) in moon_query.iter_mut() {
                if trans.translation.x() - 128.0 * trans.scale.x() <= world_coords.x()
                    && trans.translation.x() + 128.0 * trans.scale.x() >= world_coords.x()
                    && trans.translation.y() - 128.0 * trans.scale.y() <= world_coords.y()
                    && trans.translation.y() + 128.0 * trans.scale.y() >= world_coords.y() {
                    sprite.index = 4;
                    moon.mining = true;
                }
            }
            commands.despawn(state.cursor_follower.unwrap());
            state.currently_building = false;
        }

        // make the building indicator follow the mouse cursor
        for (_, mut trans) in sprite_query.iter_mut() {
            trans.translation = world_coords.extend(0.0);
        }
    }
}

#[derive(Default)]
struct CombatState {
    keyboard_event_reader: EventReader<KeyboardInput>,
    cursor_event_reader: EventReader<CursorMoved>,
    cursor_position: Vec2,
}

/// System for shooting rockets in mouse cursor direction.
fn combat(
    commands: &mut Commands,
    mut state: Local<CombatState>,
    time: Res<Time>,
    windows: Res<Windows>,
    keyboard_inputs: Res<Events<KeyboardInput>>,
    cursor_inputs: Res<Events<CursorMoved>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(Entity, &Rocket, Mut<Transform>)>,
) {
    let window = windows.get_primary().unwrap();

    for event in state.cursor_event_reader.iter(&cursor_inputs) {
        state.cursor_position =
            event.position - Vec2::new(window.width() as f32 / 2.0, window.height() as f32 / 2.0);
    }

    // TODO: find better way of getting the SpriteSheet handle
    let ta_id = texture_atlases.ids().next().unwrap();
    for event in state.keyboard_event_reader.iter(&keyboard_inputs) {
        if event.key_code == Some(KeyCode::A) && event.state == ElementState::Pressed {
            //let rocket_velocity = Vec3::new(300.0, 300.0, 0.0);
            let rocket_direction = state.cursor_position.normalize();
            let angle = rocket_direction.y().atan2(rocket_direction.x());
            commands
                .spawn(SpriteSheetComponents {
                    sprite: TextureAtlasSprite::new(7),
                    //texture_atlas: texture_atlases.get_handle("sprites/sprite_sheet.png"),
                    texture_atlas: texture_atlases.get_handle(ta_id),
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

fn resource_mining(
    time: Res<Time>,
    mut resources: ResMut<PlayerResources>,
    moon_query: Query<&Moon>,
    mut text_query: Query<(&mut Text, &ResourcesText)>,
) {
    for moon in moon_query.iter() {
        if moon.mining {
            resources.pink += 1;
            //resources.green += 2;
        }
    }

    for (mut text, _) in text_query.iter_mut() {
        text.value = format!("{}, {}", resources.pink, resources.green);
    }
}
