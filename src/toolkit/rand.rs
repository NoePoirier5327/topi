//! Contient des fonctions régissant l'aléatoire de topi.
//! Ce ne sont que des wrappers sur des fonctions de la crate rand 0.10.3.

/// Renvoie un nombre aléatoire entre 0 et 1.
///
/// # Return
/// f64 aléatoire entre 0 et 1.
pub fn random() -> f64 {
    rand::random()
}

/// Renvoie un f64 aléatoire entre les deux bornes inclus en paramètre.
///
/// # Arguments
/// * `lower_bound` borne inférieur du nombre aléatoire.
/// * `upper_bound` borne supérieur du nombre aléatoire.
///
/// # Return
/// Nombre aléatoire entre les bornes en paramètre.
pub fn randrange(lower_bound: f64, upper_bound: f64) -> f64 {
    rand::random_range(lower_bound..=upper_bound)
}
