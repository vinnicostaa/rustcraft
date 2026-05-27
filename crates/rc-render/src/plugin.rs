use bevy::prelude::*;

use crate::{RenderConfig, assets::setup_block_assets, lighting::setup_lighting};

/// Startup set for render asset preparation.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderStartupSet {
    PrepareAssets,
}

/// Prepares scene lighting and block render assets.
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
