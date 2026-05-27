use bevy::prelude::*;
use rc_input::InputPlugin;
use rc_player::PlayerPlugin;
use rc_render::RenderPlugin;
use rc_world::WorldPlugin;

/// Plugin raiz do jogo.
///
/// Este é o ponto de composição da arquitetura: módulos de domínio entram como
/// plugins independentes e expõem apenas os recursos/sistemas necessários.
pub struct RustcraftPlugin;

impl Plugin for RustcraftPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RenderPlugin, WorldPlugin, InputPlugin, PlayerPlugin));
    }
}

/// Executa o app Bevy do `rustcraft`.
pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RustcraftPlugin)
        .run();
}
