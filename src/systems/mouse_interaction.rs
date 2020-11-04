// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use amethyst::{
    assets::AssetStorage,
    core::{
        geometry::Plane,
        math::{Point2, Vector2},
        Transform,
    },
    ecs::*,
    input::{InputHandler, StringBindings},
    renderer::{
        camera::{ActiveCamera, Camera},
        sprite::{SpriteRender, SpriteSheet},
    },
    shrev::EventChannel,
    window::ScreenDimensions,
    winit::MouseButton,
};

#[derive(Debug)]
pub struct MouseInteractionEvent {
    pub entity: Entity,
}

#[derive(Default)]
pub struct MouseInteractionSystem;

impl<'s> System<'s> for MouseInteractionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, SpriteRender>,
        Read<'s, AssetStorage<SpriteSheet>>,
        ReadExpect<'s, ScreenDimensions>,
        Read<'s, ActiveCamera>,
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, EventChannel<MouseInteractionEvent>>,
    );

    fn run(
        &mut self,
        (
            entities,
            transforms,
            cameras,
            sprites,
            sprite_sheets,
            screen_dimensions,
            active_camera,
            input,
            mut event_channel,
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
                        event_channel.single_write(MouseInteractionEvent { entity });
                    }
                }
            }
        }
    }
}
