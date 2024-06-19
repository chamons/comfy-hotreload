wasmtime::component::bindgen!({
    path: "../wit"
});

pub mod types {
    pub use super::exports::example::host::game_api::{
        ClickInfo, DrawLineCommand, GameColor, GuestGameInstance, ImageCommand, Key, KeyboardInfo,
        MouseInfo, Position, RenderCommand, Size, TextCommand,
    };

    pub use super::HotreloadExample;
}
