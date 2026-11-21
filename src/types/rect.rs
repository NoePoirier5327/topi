//! Fichier contenant les interfaces de manipulation de rectangle pour le moteur topi, ils peuvent
//! être utilisé pour la gestion des boites de collision, le ciblage d'une portion d'une image à
//! afficher etc...

pub struct Rect(sdl2::rect::Rect);

impl Rect {
    /// Construit un nouveau rectangle ayant les propriétées en paramètre.
    ///
    /// # Arguments
    /// * `x` position du bord gauche du nouveau rectangle.
    /// * `y` position du bord haut du nouveau rectangle.
    /// * `w` largeur du nouveau rectangle.
    /// * `h` hauteur du nouveau rectangle.
    ///
    /// # Return
    /// Instance vers le nouveau rectangle.
    #[inline]
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Self(sdl2::rect::Rect::new(x, y, w, h))
    }

    /// Mutateur de la position en x du rectangle courant.
    #[inline]
    pub fn set_x(&mut self, x: i32) {
        self.0.x = x;
    }

    /// Mutateur de la position en y du rectangle courant.
    #[inline]
    pub fn set_y(&mut self, y: i32) {
        self.0.y = y;
    }

    /// Mutateur de la largeur du rectangle courant.
    #[inline]
    pub fn set_w(&mut self, w: u32) {
        self.0.w = w as i32;
    }

    /// Mutateur de la hauteur du rectangle courant.
    #[inline]
    pub fn set_h(&mut self, h: u32) {
        self.0.h = h as i32;
    }

    /// Accesseur de la position en x du rectangle courant.
    #[inline]
    pub fn get_x(&self) -> i32 {
        self.0.x
    }

    /// Accesseur de la position en y du rectangle courant.
    #[inline]
    pub fn get_y(&self) -> i32 {
        self.0.y
    }

    /// Accesseur de la largeur du rectangle courant.
    #[inline]
    pub fn get_w(&self) -> u32 {
        self.0.width()
    }

    /// Accesseur de la hauteur du rectangle courant.
    #[inline]
    pub fn get_h(&self) -> u32 {
        self.0.height()
    }
}
