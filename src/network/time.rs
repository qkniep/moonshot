// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::ops::RangeInclusive;

use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NetworkSimulationTime {
    /// The current simulation frame
    frame_number: u32,
    /// Accumulated duration since last simulation frame (in seconds)
    elapsed_duration: f32,
    /// Duration per frame (in seconds)
    per_frame_duration: f32,
    /// Number of frames the game lags behind the server simulation
    frame_lag: u32,
}

impl NetworkSimulationTime {
    /// Returns the simulation frame numbers needed to be run this game frame.
    pub fn sim_frames_to_run(&self) -> RangeInclusive<u32> {
        (self.frame_number + 1 - self.frame_lag)..=self.frame_number
    }

    /// Bumps the frame number
    pub fn increment_frame_number(&mut self) {
        self.frame_number += 1;
        self.elapsed_duration -= self.per_frame_duration;
        self.frame_lag += 1;
    }

    /// Resets the frame lag
    pub fn reset_frame_lag(&mut self) {
        self.frame_lag = 0;
    }

    /// Increases the `elapsed_duration` by the given duration in seconds
    pub fn update_elapsed(&mut self, seconds: f32) {
        self.elapsed_duration += seconds;
    }

    /// Returns the current simulation frame number
    pub fn frame_number(&self) -> u32 {
        self.frame_number
    }

    /// Sets the frame number to the given frame number. This is useful when synchronizing frames
    /// with a server for example.
    pub fn set_frame_number(&mut self, new_frame: u32) {
        self.frame_number = new_frame;
    }

    /// Returns the total duration since the last simulation frame
    pub fn elapsed_duration(&self) -> f32 {
        self.elapsed_duration
    }

    /// Returns the duration between each simulation frame. This number is calculated when a frame rate
    /// is set
    pub fn per_frame_duration(&self) -> f32 {
        self.per_frame_duration
    }

    /// Returns the number of frames the game lags behind the server simulation.
    pub fn frame_lag(&self) -> u32 {
        self.frame_lag
    }
}

impl Default for NetworkSimulationTime {
    fn default() -> Self {
        Self {
            frame_number: 0,
            elapsed_duration: 0.0,
            // 30 frames / second
            per_frame_duration: 1.0 / 30.0,
            frame_lag: 1,
        }
    }
}

pub fn update_simulation_time(mut sim_time: ResMut<NetworkSimulationTime>, time: Res<Time>) {
    sim_time.update_elapsed(time.delta_seconds);
    sim_time.reset_frame_lag();
    while sim_time.elapsed_duration() > sim_time.per_frame_duration() {
        sim_time.increment_frame_number();
    }
}
