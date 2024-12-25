use anyhow::Result;
use macroquad::prelude::*;

mod render;
use render::*;

mod input;
use input::*;

mod texture_cache;

#[cfg(feature = "hotreload")]
mod hotreload;

#[cfg(feature = "hotreload")]
use crate::hotreload::binding::{
    exports::example::host::game_api::{KeyboardInfo, MouseInfo, RenderCommand},
    WebAssemblyContext, WebAssemblyInstance,
};

#[cfg(not(feature = "hotreload"))]
pub use game::{
    exports::example::host::game_api::{KeyboardInfo, MouseInfo, RenderCommand},
    Instance,
};

use texture_cache::TextureCache;

pub trait RunnableGameInstance {
    fn run_frame(&self, mouse: MouseInfo, key: KeyboardInfo) -> Vec<RenderCommand>;
}

#[cfg(not(feature = "hotreload"))]
impl RunnableGameInstance for Instance {
    fn run_frame(&self, mouse: MouseInfo, key: KeyboardInfo) -> Vec<RenderCommand> {
        Instance::run_frame(self, mouse, key)
    }
}

async fn run_frame<R: RunnableGameInstance>(
    instance: &R,
    font: &Font,
    texture_cache: &mut TextureCache,
) {
    let mouse = get_mouse_state();
    let key = get_key_info();

    let commands = instance.run_frame(mouse, key);
    for command in commands {
        match command {
            RenderCommand::Text(text) => handle_text_command(text, font),
            RenderCommand::Image(image) => handle_image_command(image, texture_cache).await,
            RenderCommand::Line(line) => handle_draw_line(line),
        }
    }

    next_frame().await
}

#[cfg(not(feature = "hotreload"))]
async fn run(font: Font, mut texture_cache: TextureCache) -> Result<()> {
    let instance = Instance::new();
    loop {
        run_frame(&instance, &font, &mut texture_cache).await;
    }
}

#[cfg(feature = "hotreload")]
async fn run(font: Font, mut texture_cache: TextureCache) -> Result<()> {
    let context = WebAssemblyContext::load()?;
    let mut assembly = WebAssemblyInstance::load(context)?;
    let mut instance = assembly.create_game_instance()?;

    let file_watcher = crate::hotreload::watcher::FileWatcher::new(crate::hotreload::wasm_path()?)?;

    loop {
        if file_watcher.changed() {
            let save_data = instance.save();
            let context = WebAssemblyContext::load()?;
            assembly = WebAssemblyInstance::load(context)?;
            instance = assembly.create_game_instance()?;
            if let Ok(save_data) = save_data {
                let _ = instance.load(save_data);
            }
        }

        run_frame(&instance, &font, &mut texture_cache).await;
    }
}

#[macroquad::main("BasicShapes")]
async fn main() -> Result<()> {
    let font = load_ttf_font_from_bytes(include_bytes!("../../resources/Kreon-Regular.ttf"))
        .expect("Unable to load font");
    let texture_cache = TextureCache::default();

    run(font, texture_cache).await
}
