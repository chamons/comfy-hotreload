wit_bindgen::generate!({
    world: "hotreload-example",
    path: "../wit"
});

use exports::example::host::game_api::{
    Color, Guest, GuestGameInstance, Position, RenderCommand, TextCommand,
};

struct GameGuest;

impl Guest for GameGuest {
    type GameInstance = Instance;
}

struct Instance {}

impl GuestGameInstance for Instance {
    fn new() -> Instance {
        Instance {}
    }

    fn run_frame(&self) -> Vec<RenderCommand> {
        vec![RenderCommand::Text(TextCommand {
            position: Position { x: 1.0, y: 3.0 },
            size: 10.0,
            color: Color {
                r: 1.0,
                g: 0.0,
                b: 0.5,
                a: 1.0,
            },
        })]
    }
}

export!(GameGuest);
