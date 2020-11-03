// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use amethyst::{
    core::{Time, Transform},
    ecs::*,
    input::{InputHandler, StringBindings},
    renderer::camera::Camera,
};

#[derive(Default)]
pub struct CameraMovementSystem;

impl<'s> System<'s> for CameraMovementSystem {
    type SystemData = (
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (cameras, mut transforms, input_handler, time): Self::SystemData) {
        let delta_time = time.delta_real_seconds();
        let move_factor = 128.0 * delta_time;
        for (_, transform) in (&cameras, &mut transforms).join() {
            if input_handler.action_is_down("CameraMoveUp").unwrap() {
                transform.move_up(move_factor);
            }
            if input_handler.action_is_down("CameraMoveDown").unwrap() {
                transform.move_down(move_factor);
            }
            if input_handler.action_is_down("CameraMoveLeft").unwrap() {
                transform.move_left(move_factor);
            }
            if input_handler.action_is_down("CameraMoveRight").unwrap() {
                transform.move_right(move_factor);
            }
        }
    }
}
