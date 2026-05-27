/// Função protótipo simples para altura do terreno.
///
/// Retorna uma altura entre `0` e `altura_máxima`.
pub fn terrain_height(x: i32, z: i32, max_height: i32) -> i32 {
    if max_height <= 0 {
        return 0;
    }

    let fx = x as f32 * 0.3;
    let fz = z as f32 * 0.3;

    let noise = (fx.sin() + fz.cos() + (fx * 0.5).cos()) / 3.0;
    let normalized = (noise + 1.0) / 2.0;

    (normalized * max_height as f32) as i32
}

#[cfg(test)]
mod tests {
    use super::terrain_height;

    #[test]
    fn terrain_height_stays_within_expected_range() {
        let max_height = 6;

        for x in -16..16 {
            for z in -16..16 {
                let height = terrain_height(x, z, max_height);
                assert!(
                    (0..=max_height).contains(&height),
                    "height {height} was outside 0..={max_height} at {x},{z}"
                );
            }
        }
    }

    #[test]
    fn terrain_height_handles_non_positive_max_height() {
        assert_eq!(terrain_height(0, 0, 0), 0);
        assert_eq!(terrain_height(0, 0, -10), 0);
    }
}
