use std::path::PathBuf;

use anyhow::Result;
use binding::{
    exports::example::host::game_api::{DrawLineCommand, ImageCommand, RenderCommand, TextCommand},
    WebAssemblyContext, WebAssemblyInstance,
};

use frontend::TextureCache;
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
    let mut image_cache = TextureCache::default();

    let context = WebAssemblyContext::load()?;
    let mut assembly = WebAssemblyInstance::load(context)?;
    let mut instance = assembly.create_game_instance()?;

    let file_watcher = FileWatcher::new(wasm_path()?)?;

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

        let commands = instance.run_frame()?;
        for command in commands {
            match command {
                RenderCommand::Text(text) => handle_text_command(text, &font),
                RenderCommand::Image(image) => handle_image_command(image, &mut image_cache).await,
                RenderCommand::Line(line) => handle_draw_line(line),
            }
        }

        next_frame().await
    }
}

fn handle_draw_line(line: DrawLineCommand) {
    draw_line(
        line.first.x,
        line.first.y,
        line.second.x,
        line.second.y,
        line.thickness,
        Color {
            r: line.color.r,
            g: line.color.g,
            b: line.color.b,
            a: line.color.a,
        },
    )
}

async fn handle_image_command(image: ImageCommand, texture_cache: &mut TextureCache) {
    // Ignore image loading errors and just skip render
    if let Ok(texture) = texture_cache.get(&image.filename).await {
        let mut params = DrawTextureParams::default();
        if let Some(size) = image.size {
            params.dest_size = Some(Vec2 {
                x: size.width,
                y: size.height,
            })
        }
        draw_texture_ex(&texture, image.position.x, image.position.y, WHITE, params);
    }
}

fn handle_text_command(text: TextCommand, font: &Font) {
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
