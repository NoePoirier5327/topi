/// Calcul la distance entre deux points sur un plan en deux dimensions.
///
/// # Arguments
/// * `x1` coordonnée en x du premier point.
/// * `x2` coordonnée en x du second point.
/// * `y1` coordonnée en y du premier point.
/// * `y2` coordonnée en y du second point.
///
/// # Return
/// Distance entre les points en paramètres.
pub fn dist_2d(x1: f64, x2: f64, y1: f64, y2: f64) -> f64 {
    ((x1 - x2).powf(2f64) + (y1 - y2).powf(2f64)).sqrt()
}

#[cfg(test)]
mod dist_test {
    #[test]
    fn dist_2d() {
        assert_eq!(super::dist_2d(4f64, 2f64, 0f64, 0f64), 2f64);
    }
}
