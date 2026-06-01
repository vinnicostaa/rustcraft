use bevy::prelude::*;
use rc_interaction::AimedBlock;

const CROSSHAIR_SIZE: f32 = 8.0;
const CROSSHAIR_BORDER: f32 = 1.0;

#[derive(Component)]
pub(crate) struct CrosshairRoot;

#[derive(Component)]
pub(crate) struct CrosshairMarker;

pub(crate) fn spawn_crosshair(mut commands: Commands) {
    commands
        .spawn((
            CrosshairRoot,
            Visibility::Visible,
            Node {
                position_type: PositionType::Absolute,
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_child((
            CrosshairMarker,
            Node {
                width: px(CROSSHAIR_SIZE),
                height: px(CROSSHAIR_SIZE),
                border: UiRect::all(px(CROSSHAIR_BORDER)),
                border_radius: BorderRadius::MAX,
                ..default()
            },
            BackgroundColor(idle_crosshair_color()),
            BorderColor::all(Color::srgb(0.02, 0.02, 0.02)),
        ));
}

pub(crate) fn update_crosshair_target(
    aimed_block: Res<AimedBlock>,
    mut markers: Query<&mut BackgroundColor, With<CrosshairMarker>>,
) {
    let color = if aimed_block.hit().is_some() {
        target_crosshair_color()
    } else {
        idle_crosshair_color()
    };

    for mut background in &mut markers {
        *background = BackgroundColor(color);
    }
}

pub(crate) fn show_crosshair(mut roots: Query<&mut Visibility, With<CrosshairRoot>>) {
    for mut visibility in &mut roots {
        *visibility = Visibility::Visible;
    }
}

pub(crate) fn hide_crosshair(mut roots: Query<&mut Visibility, With<CrosshairRoot>>) {
    for mut visibility in &mut roots {
        *visibility = Visibility::Hidden;
    }
}

fn idle_crosshair_color() -> Color {
    Color::srgb(0.85, 0.85, 0.85)
}

fn target_crosshair_color() -> Color {
    Color::srgb(1.0, 0.85, 0.2)
}

#[cfg(test)]
mod tests {
    use super::{idle_crosshair_color, target_crosshair_color};

    #[test]
    fn target_and_idle_crosshair_colors_are_distinct() {
        assert_ne!(idle_crosshair_color(), target_crosshair_color());
    }
}
