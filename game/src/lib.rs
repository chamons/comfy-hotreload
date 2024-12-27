wit_bindgen::generate!({
    world: "hotreload-example",
    path: "../wit",
});

#[cfg(feature = "hotreload")]
mod hotreload;
#[cfg(feature = "hotreload")]
use hotreload::{GameGuest, GameScreen};
#[cfg(feature = "hotreload")]
type Screen = GameScreen;
#[cfg(feature = "hotreload")]
export!(GameGuest);

#[cfg(not(feature = "hotreload"))]
mod direct;
#[cfg(not(feature = "hotreload"))]
use direct::GameScreen;
#[cfg(not(feature = "hotreload"))]
type Screen = GameScreen;
#[cfg(not(feature = "hotreload"))]
pub use direct::GameScreenInterface;

mod colors;
pub use colors::*;

mod ui;

mod game;
pub use game::Game;
