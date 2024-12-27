use std::sync::Arc;

use async_mutex::Mutex;
use async_trait::async_trait;
use macroquad::{
    color::{Color, WHITE},
    math::Vec2,
    shapes::draw_line,
    text::{draw_text_ex, Font, TextParams},
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};

use crate::texture_cache::TextureCache;

#[derive(Clone)]
pub struct GameScreen {
    font: Font,
    texture_cache: Arc<Mutex<TextureCache>>,
}

impl GameScreen {
    pub fn new(font: Font, texture_cache: TextureCache) -> Self {
        Self {
            font,
            texture_cache: Arc::new(Mutex::new(texture_cache)),
        }
    }

    async fn fetch_texture(&self, filename: &str) -> Option<Texture2D> {
        let mut texture_cache = self.texture_cache.lock().await;
        texture_cache.get(filename).await.ok()
    }
}

#[async_trait]
impl game::GameScreenInterface for GameScreen {
    fn draw_text(
        &self,
        text: &str,
        position: game::example::host::host_api::Position,
        size: f32,
        color: game::example::host::host_api::GameColor,
    ) {
        draw_text_ex(
            &text,
            position.x,
            position.y,
            TextParams {
                font: Some(&self.font),
                font_size: size as u16,
                color: Color {
                    r: color.r,
                    g: color.g,
                    b: color.b,
                    a: color.a,
                },
                ..Default::default()
            },
        );
    }

    async fn draw_image(
        &self,
        filename: &str,
        position: game::example::host::host_api::Position,
        size: Option<game::example::host::host_api::Size>,
    ) {
        // Ignore image loading errors and just skip render
        if let Some(texture) = self.fetch_texture(&filename).await {
            let mut params = DrawTextureParams::default();
            if let Some(size) = size {
                params.dest_size = Some(Vec2 {
                    x: size.width,
                    y: size.height,
                })
            }
            draw_texture_ex(&texture, position.x, position.y, WHITE, params);
        }
    }

    fn draw_line(
        &self,
        first: game::example::host::host_api::Position,
        second: game::example::host::host_api::Position,
        thickness: f32,
        color: game::example::host::host_api::GameColor,
    ) {
        draw_line(
            first.x,
            first.y,
            second.x,
            second.y,
            thickness,
            Color {
                r: color.r,
                g: color.g,
                b: color.b,
                a: color.a,
            },
        )
    }
}
