use bevy::{
    diagnostic::{Diagnostic, DiagnosticPath, RegisterDiagnostic},
    prelude::*,
};

/// Quantidade de chunks renderizáveis criados no mundo atual.
pub const CHUNKS_RENDERED: DiagnosticPath = DiagnosticPath::const_new("rustcraft/chunks_rendered");

/// Quantidade de faces expostas geradas para a mesh do chunk inicial.
pub const CHUNK_FACES: DiagnosticPath = DiagnosticPath::const_new("rustcraft/chunk_faces");

/// Quantidade de vértices gerados para a mesh do chunk inicial.
pub const CHUNK_VERTICES: DiagnosticPath = DiagnosticPath::const_new("rustcraft/chunk_vertices");

/// Registra os diagnósticos próprios do mundo voxel.
pub(crate) fn register_world_diagnostics(app: &mut App) {
    app.register_diagnostic(Diagnostic::new(CHUNKS_RENDERED))
        .register_diagnostic(Diagnostic::new(CHUNK_FACES))
        .register_diagnostic(Diagnostic::new(CHUNK_VERTICES));
}
