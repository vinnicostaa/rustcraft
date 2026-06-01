use crate::{
    diagnostics::RustcraftDiagnosticsPlugin,
    state::{GameState, RustcraftStatePlugin},
};
use bevy::prelude::*;
use rc_input::InputPlugin;
use rc_interaction::InteractionPlugin;
use rc_inventory::InventoryPlugin;
use rc_player::PlayerPlugin;
use rc_render::RenderPlugin;
use rc_ui::UiPlugin;
use rc_world::WorldPlugin;

/// Plugin raiz do jogo.
///
/// Este é o ponto de composição da arquitetura: módulos de domínio entram como
/// plugins independentes e expõem apenas os recursos/sistemas necessários.
pub struct RustcraftPlugin;

impl Plugin for RustcraftPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RustcraftStatePlugin,
            RustcraftDiagnosticsPlugin,
            RenderPlugin,
            WorldPlugin,
            InputPlugin,
            PlayerPlugin,
            InventoryPlugin::active_in(GameState::InGame),
            InteractionPlugin::active_in(GameState::InGame),
            UiPlugin::visible_in(GameState::InGame, GameState::Paused),
        ));
    }
}

/// Executa o app Bevy do `rustcraft`.
pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RustcraftPlugin)
        .run();
}
