#[derive(Clone, Default)]
pub struct GameScreen {}

#[cfg(not(feature = "hotreload"))]
impl game::GameScreenInterface for GameScreen {
    fn draw_text(
        &self,
        text: &str,
        position: game::example::host::host_api::Position,
        size: f32,
        color: game::example::host::host_api::GameColor,
    ) {
    }

    fn draw_image(
        &self,
        filename: &str,
        position: game::example::host::host_api::Position,
        size: Option<game::example::host::host_api::Size>,
    ) {
    }

    fn draw_line(
        &self,
        first: game::example::host::host_api::Position,
        second: game::example::host::host_api::Position,
        thickness: f32,
        color: game::example::host::host_api::GameColor,
    ) {
    }
}
