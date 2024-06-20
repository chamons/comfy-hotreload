use anyhow::Result;
use macroquad::prelude::*;

mod texture_cache;

#[cfg(feature = "hotreload")]
mod hotreload;

#[cfg(feature = "hotreload")]
use crate::hotreload::binding::{
    exports::example::host::game_api::{
        ClickInfo, DrawLineCommand, ImageCommand, Key, KeyboardInfo, MouseInfo, Position,
        RenderCommand, TextCommand,
    },
    WebAssemblyContext, WebAssemblyInstance,
};

#[cfg(not(feature = "hotreload"))]
pub use game::{
    exports::example::host::game_api::{
        ClickInfo, DrawLineCommand, ImageCommand, Key, KeyboardInfo, MouseInfo, Position,
        RenderCommand, TextCommand,
    },
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

fn get_key_info() -> KeyboardInfo {
    KeyboardInfo {
        pressed: get_keys_pressed().into_iter().map(map_key).collect(),
        released: get_keys_released().into_iter().map(map_key).collect(),
        down: get_keys_down().into_iter().map(map_key).collect(),
    }
}

fn map_key(key: KeyCode) -> Key {
    match key {
        KeyCode::Space => Key::Space,
        KeyCode::Apostrophe => Key::Apostrophe,
        KeyCode::Comma => Key::Comma,
        KeyCode::Minus => Key::Minus,
        KeyCode::Period => Key::Period,
        KeyCode::Slash => Key::Slash,
        KeyCode::Key0 => Key::KeyZero,
        KeyCode::Key1 => Key::KeyOne,
        KeyCode::Key2 => Key::KeyTwo,
        KeyCode::Key3 => Key::KeyThree,
        KeyCode::Key4 => Key::KeyFour,
        KeyCode::Key5 => Key::KeyFive,
        KeyCode::Key6 => Key::KeySix,
        KeyCode::Key7 => Key::KeySeven,
        KeyCode::Key8 => Key::KeyEight,
        KeyCode::Key9 => Key::KeyNine,
        KeyCode::Semicolon => Key::Semicolon,
        KeyCode::Equal => Key::Equal,
        KeyCode::A => Key::A,
        KeyCode::B => Key::B,
        KeyCode::C => Key::C,
        KeyCode::D => Key::D,
        KeyCode::E => Key::E,
        KeyCode::F => Key::F,
        KeyCode::G => Key::G,
        KeyCode::H => Key::H,
        KeyCode::I => Key::I,
        KeyCode::J => Key::J,
        KeyCode::K => Key::K,
        KeyCode::L => Key::L,
        KeyCode::M => Key::M,
        KeyCode::N => Key::N,
        KeyCode::O => Key::O,
        KeyCode::P => Key::P,
        KeyCode::Q => Key::Q,
        KeyCode::R => Key::R,
        KeyCode::S => Key::S,
        KeyCode::T => Key::T,
        KeyCode::U => Key::U,
        KeyCode::V => Key::V,
        KeyCode::W => Key::W,
        KeyCode::X => Key::X,
        KeyCode::Y => Key::Y,
        KeyCode::Z => Key::Z,
        KeyCode::LeftBracket => Key::LeftBracket,
        KeyCode::Backslash => Key::Backslash,
        KeyCode::RightBracket => Key::RightBracket,
        KeyCode::GraveAccent => Key::GraveAccent,
        KeyCode::World1 => Key::WorldOne,
        KeyCode::World2 => Key::WorldTwo,
        KeyCode::Escape => Key::Escape,
        KeyCode::Enter => Key::Enter,
        KeyCode::Tab => Key::Tab,
        KeyCode::Backspace => Key::Backspace,
        KeyCode::Insert => Key::Insert,
        KeyCode::Delete => Key::Delete,
        KeyCode::Right => Key::Right,
        KeyCode::Left => Key::Left,
        KeyCode::Down => Key::Down,
        KeyCode::Up => Key::Up,
        KeyCode::PageUp => Key::PageUp,
        KeyCode::PageDown => Key::PageDown,
        KeyCode::Home => Key::Home,
        KeyCode::End => Key::End,
        KeyCode::CapsLock => Key::CapsLock,
        KeyCode::ScrollLock => Key::ScrollLock,
        KeyCode::NumLock => Key::NumLock,
        KeyCode::PrintScreen => Key::PrintScreen,
        KeyCode::Pause => Key::Pause,
        KeyCode::F1 => Key::F1,
        KeyCode::F2 => Key::F2,
        KeyCode::F3 => Key::F3,
        KeyCode::F4 => Key::F4,
        KeyCode::F5 => Key::F5,
        KeyCode::F6 => Key::F6,
        KeyCode::F7 => Key::F7,
        KeyCode::F8 => Key::F8,
        KeyCode::F9 => Key::F9,
        KeyCode::F10 => Key::F10,
        KeyCode::F11 => Key::F11,
        KeyCode::F12 => Key::F12,
        KeyCode::F13 => Key::F13,
        KeyCode::F14 => Key::F14,
        KeyCode::F15 => Key::F15,
        KeyCode::F16 => Key::F16,
        KeyCode::F17 => Key::F17,
        KeyCode::F18 => Key::F18,
        KeyCode::F19 => Key::F19,
        KeyCode::F20 => Key::F20,
        KeyCode::F21 => Key::F21,
        KeyCode::F22 => Key::F22,
        KeyCode::F23 => Key::F23,
        KeyCode::F24 => Key::F24,
        KeyCode::F25 => Key::F25,
        KeyCode::Kp0 => Key::Kp0,
        KeyCode::Kp1 => Key::Kp1,
        KeyCode::Kp2 => Key::Kp2,
        KeyCode::Kp3 => Key::Kp3,
        KeyCode::Kp4 => Key::Kp4,
        KeyCode::Kp5 => Key::Kp5,
        KeyCode::Kp6 => Key::Kp6,
        KeyCode::Kp7 => Key::Kp7,
        KeyCode::Kp8 => Key::Kp8,
        KeyCode::Kp9 => Key::Kp9,
        KeyCode::KpDecimal => Key::KpDecimal,
        KeyCode::KpDivide => Key::KpDivide,
        KeyCode::KpMultiply => Key::KpMultiply,
        KeyCode::KpSubtract => Key::KpSubtract,
        KeyCode::KpAdd => Key::KpAdd,
        KeyCode::KpEnter => Key::KpEnter,
        KeyCode::KpEqual => Key::KpEqual,
        KeyCode::LeftShift => Key::LeftShift,
        KeyCode::LeftControl => Key::LeftControl,
        KeyCode::LeftAlt => Key::LeftAlt,
        KeyCode::LeftSuper => Key::LeftSuper,
        KeyCode::RightShift => Key::RightShift,
        KeyCode::RightControl => Key::RightControl,
        KeyCode::RightAlt => Key::RightAlt,
        KeyCode::RightSuper => Key::RightSuper,
        KeyCode::Menu => Key::Menu,
        KeyCode::Unknown => Key::Unknown,
    }
}

fn get_mouse_state() -> MouseInfo {
    let mouse_position = mouse_position();
    MouseInfo {
        position: Position {
            x: mouse_position.0,
            y: mouse_position.1,
        },
        left: ClickInfo {
            pressed: is_mouse_button_pressed(MouseButton::Left),
            released: is_mouse_button_released(MouseButton::Left),
            down: is_mouse_button_down(MouseButton::Left),
        },
        right: ClickInfo {
            pressed: is_mouse_button_pressed(MouseButton::Right),
            released: is_mouse_button_released(MouseButton::Right),
            down: is_mouse_button_down(MouseButton::Right),
        },
        middle: ClickInfo {
            pressed: is_mouse_button_pressed(MouseButton::Middle),
            released: is_mouse_button_released(MouseButton::Middle),
            down: is_mouse_button_down(MouseButton::Middle),
        },
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
