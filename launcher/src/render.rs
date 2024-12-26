use macroquad::prelude::*;

use crate::texture_cache::TextureCache;

pub struct Screen {
    texture_cache: TextureCache,
}

// impl ScreenAPI for Screen {}

// pub fn handle_draw_line(line: DrawLineCommand) {
//     draw_line(
//         line.first.x,
//         line.first.y,
//         line.second.x,
//         line.second.y,
//         line.thickness,
//         Color {
//             r: line.color.r,
//             g: line.color.g,
//             b: line.color.b,
//             a: line.color.a,
//         },
//     )
// }

// pub async fn handle_image_command(image: ImageCommand, texture_cache: &mut TextureCache) {
//     // Ignore image loading errors and just skip render
//     if let Ok(texture) = texture_cache.get(&image.filename).await {
//         let mut params = DrawTextureParams::default();
//         if let Some(size) = image.size {
//             params.dest_size = Some(Vec2 {
//                 x: size.width,
//                 y: size.height,
//             })
//         }
//         draw_texture_ex(&texture, image.position.x, image.position.y, WHITE, params);
//     }
// }

// pub fn handle_text_command(text: TextCommand, font: &Font) {
//     draw_text_ex(
//         &text.text,
//         text.position.x,
//         text.position.y,
//         TextParams {
//             font: Some(&font),
//             font_size: text.size as u16,
//             color: Color {
//                 r: text.color.r,
//                 g: text.color.g,
//                 b: text.color.b,
//                 a: text.color.a,
//             },
//             ..Default::default()
//         },
//     );
// }
