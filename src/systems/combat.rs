// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use amethyst::{
    assets::{AssetStorage, Handle},
    core::{
        geometry::Plane,
        math::{Point2, Vector2, Vector3},
        Transform,
    },
    ecs::*,
    input::{InputHandler, StringBindings},
    renderer::{
        camera::{ActiveCamera, Camera},
        sprite::{SpriteRender, SpriteSheet},
        Transparent,
    },
    window::ScreenDimensions,
    winit::MouseButton,
};

use crate::{sprites::SpriteResource, Moon, Rocket};

#[derive(Default)]
pub struct CombatSystem;

impl<'s> System<'s> for CombatSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Moon>,
        WriteStorage<'s, Rocket>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, SpriteRender>,
        Read<'s, AssetStorage<SpriteSheet>>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, ScreenDimensions>,
        Read<'s, ActiveCamera>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(
        &mut self,
        (
            entities,
            moons,
            mut rockets,
            transforms,
            cameras,
            sprites,
            sprite_sheets,
            sprite_resource,
            screen_dimensions,
            active_camera,
            input,
            lazy_update,
        ): Self::SystemData,
    ) {
        if !input.mouse_button_is_down(MouseButton::Left) {
            return;
        }
        if let Some(mouse_position) = input.mouse_position() {
            // Get the active camera if it is spawned and ready
            let mut camera_join = (&cameras, &transforms).join();
            if let Some((camera, camera_transform)) = active_camera
                .entity
                .and_then(|a| camera_join.get(a, &entities))
                .or_else(|| camera_join.next())
            {
                // Project a ray from the camera to the z=0 plane
                let ray = camera.screen_ray(
                    Point2::new(mouse_position.0, mouse_position.1),
                    Vector2::new(screen_dimensions.width(), screen_dimensions.height()),
                    camera_transform,
                );
                let distance = ray.intersect_plane(&Plane::with_z(0.0)).unwrap();
                let mouse_world_position = ray.at_distance(distance);

                // Find any moons which the mouse is currently inside
                for (sprite, transform, entity) in (&sprites, &transforms, &entities).join() {
                    if moons.get(entity).is_some() {
                        // the clicked entity is a moon
                        let sprite_sheet = sprite_sheets.get(&sprite.sprite_sheet).unwrap();
                        let sprite = &sprite_sheet.sprites[sprite.sprite_number];
                        let (min_x, max_x, min_y, max_y) = {
                            (
                                transform.translation().x - (sprite.width * 0.5),
                                transform.translation().x + (sprite.width * 0.5),
                                transform.translation().y - (sprite.height * 0.5),
                                transform.translation().y + (sprite.height * 0.5),
                            )
                        };
                        if mouse_world_position.x > min_x
                            && mouse_world_position.x < max_x
                            && mouse_world_position.y > min_y
                            && mouse_world_position.y < max_y
                        {
                            spawn_rocket(
                                &entities,
                                sprite_resource.sprite_sheet.clone(),
                                Vector3::new(100.0, 100.0, 0.0),
                                &lazy_update,
                            );
                        }
                    }
                }
            }
        }
    }
}

pub fn spawn_rocket(
    entities: &Entities,
    sprite_sheet: Handle<SpriteSheet>,
    spawn_position: Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) -> Entity {
    let sprite_render = SpriteRender::new(sprite_sheet, 3);

    let mut local_transform = Transform::default();
    local_transform.set_translation(spawn_position);
    local_transform.set_scale(Vector3::new(0.25, 0.25, 0.25));

    let enemy_entity = lazy_update
        .create_entity(entities)
        .with(sprite_render)
        .with(local_transform)
        .with(Transparent)
        .build();

    enemy_entity
}
