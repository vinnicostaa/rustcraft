/// Ações semânticas consumidas por sistemas de gameplay.
///
/// Sistemas de gameplay não devem conhecer `KeyCode`; eles leem intenção do
/// jogador.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerAction {
    /// Move o player para frente no referencial da câmera.
    MoveForward,
    /// Move o player para trás no referencial da câmera.
    MoveBackward,
    /// Move o player para a esquerda no referencial da câmera.
    MoveLeft,
    /// Move o player para a direita no referencial da câmera.
    MoveRight,
    /// Move o player para cima no mundo.
    MoveUp,
    /// Move o player para baixo no mundo.
    MoveDown,
}
