/// Semantic actions consumed by gameplay systems.
///
/// Gameplay systems should not know about `KeyCode`; they read player intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerAction {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
}
