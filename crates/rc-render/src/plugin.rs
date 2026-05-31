use bevy::prelude::*;

use crate::{RenderConfig, assets::setup_block_assets, lighting::setup_lighting};

/// Set de startup para preparação dos assets de render.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderStartupSet {
    /// Cria materiais, meshes compartilhadas e iluminação antes do spawn do mundo.
    PrepareAssets,
}

/// Prepara iluminação da cena e assets visuais de blocos.
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RenderConfig>()
            .configure_sets(Startup, RenderStartupSet::PrepareAssets)
            .add_systems(
                Startup,
                (setup_lighting, setup_block_assets).in_set(RenderStartupSet::PrepareAssets),
            );
    }
}
