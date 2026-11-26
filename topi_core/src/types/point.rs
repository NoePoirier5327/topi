//! Contient les structures de données représentant des points dans des espaces.

/// Représente un point dans un plan en deux dimensions sous forme d'un 2-uplet de flottant.
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Point2f {
    x: f64,
    y: f64
}

impl Point2f {
    /// Construit un nouveau point en deux dimensions.
    pub fn new(x: f64, y: f64) -> Self {
        Self {x, y}
    }

    /// Calcul la distance entre les deux points en paramètres.
    #[inline]
    pub fn dist(&self, p: &Point2f) -> f64 {
        crate::toolkit::dist::dist_2d(self.x, p.x, self.y, p.y)
    }

    /// Mutateur de la position en x du point courant.
    #[inline]
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    /// Mutateur de la position en y du point courant.
    #[inline]
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    /// Accesseur de la position en x du point courant.
    #[inline]
    pub fn get_x(&self) -> f64 {
        self.x
    }

    /// Accesseur de la position en y du point courant.
    #[inline]
    pub fn get_y(&self) -> f64 {
        self.y
    }

    /// Afficheur du point courant dans la console.
    pub fn display(&self) {
        println!("({},{})", self.x, self.y);
    }
}

#[cfg(test)]
mod point2f_test {
    use super::Point2f;

    #[test]
    fn dist_point_2d() {
        let a = Point2f::new(4f64, 0f64);
        let b = Point2f::new(2f64, 0f64);
        assert_eq!(Point2f::dist(&a, &b), 2f64);
    }
}

/// Représente un point sur un plan deux dimensions sous-forme de 2-uplets d'entiers.
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Point2i {
    x: i32,
    y: i32
}

impl Point2i {
    /// Construit un nouveau point en deux dimensions.
    pub fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }

    /// Calcul la distance entre le point courant et un autre point.
    #[inline]
    pub fn dist(&self, p: &Point2i) -> f64 {
        crate::toolkit::dist::dist_2d(self.x as f64, p.x as f64, self.y as f64, p.y as f64)
    }

    /// Mutateur de la position en x du point courant.
    #[inline]
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    /// Mutateur de la position en y du point courant.
    #[inline]
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    /// Accesseur de la position en x du point courant.
    #[inline]
    pub fn get_x(&self) -> i32 {
        self.x
    }

    /// Accesseur de la position en y du point courant.
    #[inline]
    pub fn get_y(&self) -> i32 {
        self.y
    }

    /// Afficheur du point courant dans la console.
    pub fn display(&self) {
        println!("({},{})", self.x, self.y);
    }
}

#[cfg(test)]
mod point2i_test {
    use super::Point2i;

    #[test]
    fn dist_point_2d() {
        let a = Point2i::new(7, -6);
        let b = Point2i::new(-8, 9);
        assert_eq!(Point2i::dist(&a, &b), 0f64);
    }
}
