mod texture_cache;
pub use texture_cache::TextureCache;

// Generated wit code does not follow rust conventions completely
#[cfg(feature = "webassembly")]
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
mod webassembly;

pub mod types {
    #[cfg(feature = "direct")]
    pub use game::{
        exports::example::host::game_api::{
            ClickInfo, DrawLineCommand, GameColor, ImageCommand, Key, KeyboardInfo, MouseInfo,
            Position, RenderCommand, Size, TextCommand,
        },
        Instance,
    };

    #[cfg(feature = "webassembly")]
    pub use crate::webassembly::types::*;
}
