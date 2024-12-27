use crate::exports::example::host::game_api::{Guest, GuestGameInstance};
use crate::exports::example::host::game_api::{KeyboardInfo, MouseInfo};

pub use crate::example::host::host_api::GameScreen;

use crate::Instance;

pub struct GameGuest;

impl Guest for GameGuest {
    type GameInstance = Instance;
}

impl GuestGameInstance for Instance {
    fn new() -> Instance {
        Instance::new()
    }

    fn save(&self) -> Vec<u8> {
        Instance::save(self)
    }

    fn restore(&self, data: Vec<u8>) {
        Instance::restore(self, data)
    }

    fn run_frame(&self, mouse: MouseInfo, key: KeyboardInfo, screen: &GameScreen) {
        Instance::run_frame(self, mouse, key, screen);
    }
}
