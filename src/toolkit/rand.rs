//! Contient des fonctions régissant l'aléatoire de topi.
//! Ce ne sont que des wrappers sur des fonctions de la crate rand 0.10.3.

use rand::distr::uniform::{SampleRange, SampleUniform};

/// Renvoie un nombre aléatoire entre 0 et 1.
///
/// # Return
/// f64 aléatoire entre 0 et 1.
#[inline]
pub fn random() -> f64 {
    rand::random()
}

/// Renvoie un nombre aléatoire entre dans le range en paramètre.
///
/// # Argument
/// * `range` bornes entre lesquelles le nombre aléatoire doit être généré.
///
/// # Return
/// Nombre aléatoire entre les bornes en paramètre.
#[inline]
pub fn randrange<T, R>(range: R) -> T
where 
    T: SampleUniform,
    R: SampleRange<T>
{
    rand::random_range(range)
}
