wit_bindgen::generate!({
    world: "hotreload-example",
    path: "../wit"
});

use exports::example::host::game_api::{
    Color, DrawLineCommand, Guest, GuestGameInstance, ImageCommand, Position, RenderCommand,
    TextCommand,
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
        vec![
            RenderCommand::Text(TextCommand {
                text: "Hello".to_string(),
                position: Position { x: 40.0, y: 80.0 },
                size: 40.0,
                color: Color {
                    r: 0.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
            }),
            RenderCommand::Image(ImageCommand {
                filename: "resources/rustacean-flat-happy.png".to_string(),
                position: Position { x: 300.0, y: 180.0 },
            }),
            RenderCommand::Line(DrawLineCommand {
                first: Position { x: 100.0, y: 100.0 },
                second: Position { x: 200.0, y: 200.0 },
                thickness: 4.0,
                color: Color {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                },
            }),
        ]
    }
}

export!(GameGuest);
