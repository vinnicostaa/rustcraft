use bevy::prelude::*;
use rc_voxel::BlockType;

/// Startup set for render asset preparation.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderStartupSet {
    PrepareAssets,
}

/// Simple visual settings for the prototype.
#[derive(Resource, Debug, Clone, Copy)]
pub struct RenderConfig {
    pub block_size: f32,
    pub sun_illuminance: f32,
    pub shadows_enabled: bool,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            block_size: 1.0,
            sun_illuminance: 10_000.0,
            shadows_enabled: true,
        }
    }
}

/// Shared handles for block rendering.
#[derive(Resource, Clone)]
pub struct BlockRenderAssets {
    block_mesh: Handle<Mesh>,
    grass_material: Handle<StandardMaterial>,
    dirt_material: Handle<StandardMaterial>,
    stone_material: Handle<StandardMaterial>,
}

impl BlockRenderAssets {
    pub fn block_mesh(&self) -> Handle<Mesh> {
        self.block_mesh.clone()
    }

    pub fn material_for(&self, block_type: BlockType) -> Option<Handle<StandardMaterial>> {
        match block_type {
            BlockType::Air => None,
            BlockType::Grass => Some(self.grass_material.clone()),
            BlockType::Dirt => Some(self.dirt_material.clone()),
            BlockType::Stone => Some(self.stone_material.clone()),
        }
    }
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

fn setup_lighting(mut commands: Commands, config: Res<RenderConfig>) {
    commands.spawn((
        DirectionalLight {
            illuminance: config.sun_illuminance,
            shadows_enabled: config.shadows_enabled,
            ..default()
        },
        Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn setup_block_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<RenderConfig>,
) {
    let block_size = config.block_size;

    let block_mesh = meshes.add(Cuboid::new(block_size, block_size, block_size));
    let grass_material = materials.add(block_material(Color::srgb(0.2, 0.6, 0.1)));
    let dirt_material = materials.add(block_material(Color::srgb(0.4, 0.25, 0.1)));
    let stone_material = materials.add(block_material(Color::srgb(0.5, 0.5, 0.5)));

    commands.insert_resource(BlockRenderAssets {
        block_mesh,
        grass_material,
        dirt_material,
        stone_material,
    });
}

fn block_material(color: Color) -> StandardMaterial {
    StandardMaterial {
        base_color: color,
        unlit: false,
        ..default()
    }
}
