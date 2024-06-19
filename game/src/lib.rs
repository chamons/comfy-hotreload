wit_bindgen::generate!({
    world: "hotreload-example",
    path: "../wit"
});

use std::cell::RefCell;

use exports::example::host::game_api::{
    DrawLineCommand, GameColor, Guest, GuestGameInstance, ImageCommand, KeyboardInfo, MouseInfo,
    Position, RenderCommand, Size, TextCommand,
};
use serde::{Deserialize, Serialize};

struct GameGuest;

impl Guest for GameGuest {
    type GameInstance = Instance;
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct GameState {
    count: u32,
}

pub struct Instance {
    state: RefCell<GameState>,
}

impl Instance {
    pub fn new() -> Instance {
        Instance {
            state: RefCell::new(GameState { count: 0 }),
        }
    }

    pub fn save(&self) -> Vec<u8> {
        bincode::serialize(&*self.state.borrow()).expect("Unable to save state")
    }

    pub fn restore(&self, data: Vec<u8>) {
        *self.state.borrow_mut() = bincode::deserialize(&data).expect("Unable to restore state");
    }

    pub fn run_frame(&self, mouse: MouseInfo, key: KeyboardInfo) -> Vec<RenderCommand> {
        if mouse.left.pressed {
            let mut state = self.state.borrow_mut();
            state.count += 1;
        }

        vec![
            RenderCommand::Text(TextCommand {
                text: "Hot Reloading with Rust!".to_string(),
                position: Position { x: 40.0, y: 80.0 },
                size: 40.0,
                color: GameColor {
                    r: 0.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
            }),
            RenderCommand::Image(ImageCommand {
                filename: "resources/rustacean-flat-happy.png".to_string(),
                position: Position { x: 500.0, y: 25.0 },
                size: Some(Size {
                    width: 150.0,
                    height: 90.0,
                }),
            }),
            RenderCommand::Text(TextCommand {
                text: format!("Count: {}", self.state.borrow().count),
                position: Position { x: 40.0, y: 120.0 },
                size: 20.0,
                color: GameColor {
                    r: 0.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
            }),
            RenderCommand::Text(TextCommand {
                text: format!("Key Down: ({:?})", key.down),
                position: Position { x: 40.0, y: 160.0 },
                size: 20.0,
                color: GameColor {
                    r: 0.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
            }),
            RenderCommand::Text(TextCommand {
                text: format!("Mouse: ({}, {})", mouse.position.x, mouse.position.y),
                position: Position { x: 40.0, y: 185.0 },
                size: 20.0,
                color: GameColor {
                    r: 0.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
            }),
            RenderCommand::Line(DrawLineCommand {
                first: Position { x: 625.0, y: 125.0 },
                second: Position { x: 675.0, y: 200.0 },
                thickness: 4.0,
                color: GameColor {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                },
            }),
            RenderCommand::Line(DrawLineCommand {
                first: Position { x: 700.0, y: 125.0 },
                second: Position { x: 700.0, y: 200.0 },
                thickness: 4.0,
                color: GameColor {
                    r: 0.0,
                    g: 0.0,
                    b: 1.0,
                    a: 1.0,
                },
            }),
        ]
    }
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

    fn run_frame(&self, mouse: MouseInfo, key: KeyboardInfo) -> Vec<RenderCommand> {
        Instance::run_frame(self, mouse, key)
    }
}

export!(GameGuest);
