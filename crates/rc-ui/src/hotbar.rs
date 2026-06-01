use bevy::prelude::*;
use rc_inventory::{HOTBAR_SLOTS, SelectedBlock};
use rc_voxel::{BlockState, DIRT, GRASS, STONE};

const SLOT_SIZE: f32 = 52.0;
const SLOT_BORDER: f32 = 3.0;

#[derive(Component)]
pub(crate) struct HotbarRoot;

#[derive(Component)]
pub(crate) struct HotbarSlot {
    block: BlockState,
}

pub(crate) fn spawn_hotbar(mut commands: Commands) {
    commands
        .spawn((
            HotbarRoot,
            Visibility::Visible,
            Node {
                position_type: PositionType::Absolute,
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                padding: UiRect::bottom(px(24)),
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: px(6),
                    padding: UiRect::all(px(5)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.04, 0.04, 0.04)),
            ))
            .with_children(|bar| {
                for slot in HOTBAR_SLOTS {
                    bar.spawn((
                        HotbarSlot { block: slot.block },
                        Node {
                            width: px(SLOT_SIZE),
                            height: px(SLOT_SIZE),
                            border: UiRect::all(px(SLOT_BORDER)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(slot_background_color(slot.block)),
                        BorderColor::all(normal_border_color()),
                    ))
                    .with_child((
                        Text::new(slot.slot.to_string()),
                        TextFont {
                            font_size: 22.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                }
            });
        });
}

pub(crate) fn update_hotbar_selection(
    selected_block: Res<SelectedBlock>,
    mut slots: Query<(&HotbarSlot, &mut BorderColor)>,
) {
    let selected = selected_block.block();

    for (slot, mut border) in &mut slots {
        let color = if is_selected_slot(slot.block, selected) {
            selected_border_color()
        } else {
            normal_border_color()
        };

        *border = BorderColor::all(color);
    }
}

pub(crate) fn show_hotbar(mut roots: Query<&mut Visibility, With<HotbarRoot>>) {
    for mut visibility in &mut roots {
        *visibility = Visibility::Visible;
    }
}

pub(crate) fn hide_hotbar(mut roots: Query<&mut Visibility, With<HotbarRoot>>) {
    for mut visibility in &mut roots {
        *visibility = Visibility::Hidden;
    }
}

fn is_selected_slot(slot_block: BlockState, selected_block: BlockState) -> bool {
    slot_block == selected_block
}

fn slot_background_color(block: BlockState) -> Color {
    match block.id {
        GRASS => Color::srgb(0.2, 0.6, 0.1),
        DIRT => Color::srgb(0.4, 0.25, 0.1),
        STONE => Color::srgb(0.5, 0.5, 0.5),
        _ => Color::srgb(0.1, 0.1, 0.1),
    }
}

fn selected_border_color() -> Color {
    Color::srgb(1.0, 0.85, 0.2)
}

fn normal_border_color() -> Color {
    Color::srgb(0.12, 0.12, 0.12)
}

#[cfg(test)]
mod tests {
    use super::{is_selected_slot, slot_background_color};
    use rc_voxel::{BlockState, DIRT, GRASS, STONE};

    #[test]
    fn selected_slot_matches_same_block_state() {
        let block = BlockState::new(GRASS);

        assert!(is_selected_slot(block, block));
    }

    #[test]
    fn known_blocks_have_distinct_background_colors() {
        let grass = slot_background_color(BlockState::new(GRASS));
        let dirt = slot_background_color(BlockState::new(DIRT));
        let stone = slot_background_color(BlockState::new(STONE));

        assert_ne!(grass, dirt);
        assert_ne!(grass, stone);
        assert_ne!(dirt, stone);
    }

    #[test]
    fn selected_slot_rejects_different_block_state() {
        assert!(!is_selected_slot(
            BlockState::new(GRASS),
            BlockState::new(STONE),
        ));
    }
}
