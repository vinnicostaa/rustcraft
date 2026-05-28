use bevy::{
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    prelude::*,
};

/// Diagnósticos básicos do app em tempo de execução.
///
/// Esta camada usa plugins oficiais do Bevy para imprimir métricas no console.
/// Métricas próprias de chunk/faces ficam registradas no domínio de mundo.
pub struct RustcraftDiagnosticsPlugin;

impl Plugin for RustcraftDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
            EntityCountDiagnosticsPlugin::default(),
            SystemInformationDiagnosticsPlugin,
        ));
    }
}
