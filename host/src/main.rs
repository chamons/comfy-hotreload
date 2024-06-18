use std::path::PathBuf;

use anyhow::Result;
use binding::{
    exports::example::host::game_api::RenderCommand, WebAssemblyContext, WebAssemblyInstance,
};

use macroquad::prelude::*;
use watcher::FileWatcher;

// Generated wit code does not follow rust conventions completely
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
mod binding;

mod watcher;

fn wasm_path() -> Result<PathBuf> {
    let mut wasm_path = std::env::current_exe()?;
    wasm_path.pop();
    wasm_path.push("game.wasm");
    Ok(wasm_path)
}

#[macroquad::main("BasicShapes")]
async fn main() -> Result<()> {
    let font = load_ttf_font_from_bytes(include_bytes!("../../resources/Kreon-Regular.ttf"))
        .expect("Unable to load font");

    let context = WebAssemblyContext::load()?;
    let mut assembly = WebAssemblyInstance::load(context)?;
    let mut instance = assembly.create_game_instance()?;

    let file_watcher = FileWatcher::new(wasm_path()?)?;

    loop {
        if file_watcher.changed() {
            let context = WebAssemblyContext::load()?;
            assembly = WebAssemblyInstance::load(context)?;
            instance = assembly.create_game_instance()?;
        }

        let commands = instance.run_frame()?;
        for command in commands {
            match command {
                RenderCommand::Text(text) => {
                    draw_text_ex(
                        &text.text,
                        text.position.x,
                        text.position.y,
                        TextParams {
                            font: Some(&font),
                            font_size: text.size as u16,
                            color: Color {
                                r: text.color.r,
                                g: text.color.g,
                                b: text.color.b,
                                a: text.color.a,
                            },
                            ..Default::default()
                        },
                    );
                }
            }
        }

        next_frame().await
    }
}
