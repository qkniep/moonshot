// Copyright (C) 2020 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use amethyst::{
    ecs::Entity,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    ui::{Anchor, LineMode, TtfFormat, UiCreator, UiEvent, UiEventType, UiFinder},
    window::ScreenDimensions,
};
use log::debug;

#[derive(Default)]
pub struct PauseMenuState {
    resume_button: Option<Entity>,
    exit_button: Option<Entity>,
    root: Option<Entity>,
}

const RESUME_BUTTON_ID: &str = "resume";
const EXIT_BUTTON_ID: &str = "exit";

/// Contains the main state with the game logic.
impl<'a> SimpleState for PauseMenuState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        self.root =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/pause_menu.ron", ())));
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(root) = self.root {
            if data.world.delete_entity(root).is_ok() {
                self.root = None;
            }
        }
        self.resume_button = None;
    }

        fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    log::info!("[Trans::Quit] Quitting Application!");
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    log::info!("[Trans::Pop] Closing Pause Menu!");
                    Trans::Pop
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.resume_button {
                    log::info!("Resuming Game!");
                    Trans::Pop
                } else if Some(target) == self.exit_button {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        // once deferred creation of the root ui entity finishes, look up buttons
        if self.resume_button.is_none()
            || self.exit_button.is_none()
        {
            data.world.exec(|ui_finder: UiFinder<'_>| {
                self.resume_button = ui_finder.find(RESUME_BUTTON_ID);
                self.exit_button = ui_finder.find(EXIT_BUTTON_ID);
            });
        }
        Trans::None
    }
}
