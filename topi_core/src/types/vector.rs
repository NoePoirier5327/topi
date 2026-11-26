//! Contient les structures de vecteur sous formes de n-uplets dans des espaces réels de dimensions 2 et 3.

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};

/// Représente un vecteur de dimension 2 composés de f64.
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Vector2D {
    x: f64,
    y: f64
}

impl Vector2D {
    /// Construit un nouveau vecteur en deux dimensions à partir des quantitées de ses dimensions.
    pub fn new(x: f64, y: f64) -> Self {
        Self {x, y}
    }

    /// Calcul et renvoie la norme du vecteur courant.
    #[inline]
    pub fn magnitude(&self) -> f64 {
        (self.x.powf(2f64) + self.y.powf(2f64)).sqrt()
    }

    /// Mutateur en x du vecteur courant.
    #[inline]
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    /// Mutateur en y du vecteur courant.
    #[inline]
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    /// Accesseur en x du vecteur courant.
    #[inline]
    pub fn get_x(&self) -> f64 {
        self.x
    }

    /// Accesseur en y du vecteur courant.
    #[inline]
    pub fn get_y(&self) -> f64 {
        self.y
    }
}

/// Additionne deux vecteur entre eux.
impl Add for Vector2D {
    type Output = Vector2D;
    fn add(self, v: Vector2D) -> Self::Output {
        Vector2D::new(self.x + v.x, self.y + v.y)
    }
}

/// Additionne des références de vecteurs entre elles.
impl Add for &Vector2D {
    type Output = Vector2D;
    fn add(self, v: &Vector2D) -> Self::Output {
        Vector2D::new(self.x + v.x, self.y + v.y)
    }
}

/// Additionne deux vecteurs et assigne le résultat à celui de gauche.
impl AddAssign for Vector2D {
    fn add_assign(&mut self, v: Vector2D) {
        self.x += v.x;
        self.y += v.y;
    }
}

/// Soustrait deux vecteurs entre eux.
impl Sub for Vector2D {
    type Output = Vector2D;
    fn sub(self, v: Vector2D) -> Self::Output {
        Vector2D::new(self.x - v.x, self.y - v.y)
    }
}

/// Soustrait des références de vecteurs entre elles.
impl Sub for &Vector2D {
    type Output = Vector2D;

    fn sub(self, v: &Vector2D) -> Self::Output {
        Vector2D::new(self.x - v.x, self.y - v.y)
    }
}

/// Soustrait deux vecteurs et assigne le résultat à celui de gauche.
impl SubAssign for Vector2D {
    fn sub_assign(&mut self, v: Vector2D) {
        self.x -= v.x;
        self.y -= v.y;
    }
}

/// Multiplie un vecteur par un scalaire.
impl Mul<f64> for Vector2D {
    type Output = Vector2D;
    fn mul(self, scal: f64) -> Self::Output {
        Vector2D::new(self.x * scal, self.y * scal)
    }
}

/// Multiplie une référence vers un vecteur par un scalaire.
impl Mul<f64> for &Vector2D {
    type Output = Vector2D;
    fn mul(self, scal: f64) -> Self::Output {
        Vector2D::new(self.x * scal, self.y * scal)
    }
}

/// Multiplie un scalaire par un vecteur.
impl Mul<Vector2D> for f64 {
    type Output = Vector2D;
    fn mul(self, v: Vector2D) -> Self::Output {
        Vector2D::new(v.x * self, v.y * self)
    }
}

/// Multiplie un scalaire par une référence vers un vecteur.
impl Mul<&Vector2D> for f64 {
    type Output = Vector2D;
    fn mul(self, v: &Vector2D) -> Self::Output {
        Vector2D::new(v.x * self, v.y * self)
    }
}

/// Multiplie un vecteur par un scalaire et assigne le résultat dans le membre de gauche.
impl MulAssign<f64> for Vector2D {
    fn mul_assign(&mut self, scal: f64) {
        self.x *= scal;
        self.y *= scal;
    }
}

#[cfg(test)]
mod vector_test {
    use super::*;

    #[test]
    fn vector_sum() {
        assert_eq!(
            Vector2D::new(6f64, -78f64) + Vector2D::new(-7f64, -2f64),
            Vector2D::new(-1f64, -80f64)
        )
    }

    #[test]
    fn vector_sub() {
        assert_eq!(
            Vector2D::new(6f64, -78f64) - Vector2D::new(-7f64, -2f64),
            Vector2D::new(13f64, -76f64)
        )
    }

    #[test]
    fn vector_scal_mul() {
        let a = Vector2D::new(3f64, -6f64);
        let b = Vector2D::new(1.5f64, -3f64);

        assert_eq!(a * 0.5f64, b);
        assert_eq!(0.5f64 * a, b);
    }

    #[test]
    fn vector2d_magnitude() {
        let v = Vector2D::new(0f64, -3f64);
        assert_eq!(v.magnitude(), 3f64);
    }
}
