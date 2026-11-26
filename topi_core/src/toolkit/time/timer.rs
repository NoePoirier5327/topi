//! Fichier contenant un minuteur adapté à topi et son fonctionnement interne.

/// Minuteur manipulant le temps en milisecondes.
pub struct Timer {
    beginning: u64,
    milliseconds_to_wait: u64
}

impl Timer {
    /// Construit un nouveau minuteur à partir du temps qu'il doit patienter.
    ///
    /// # Argument
    /// * `milliseconds_to_wait` temps à attendre en milisecondes.
    ///
    /// # Return
    /// Instance vers le nouveau minuteur.
    pub fn new(milliseconds_to_wait: u64) -> Self {
        Self {
            beginning: super::now_as_milliseconds(),
            milliseconds_to_wait
        }
    }

    /// Renvoie le temps que doit patienter le minuteur courante en milisecondes.
    pub fn get_milliseconds_to_wait(&self) -> u64 {
        self.milliseconds_to_wait
    }

    /// Modifie le temps à attendre dans le minuteur courant.
    pub fn set_milliseconds_to_wait(&mut self, milliseconds_to_wait: u64) {
        self.milliseconds_to_wait = milliseconds_to_wait;
    }

    /// Renvoie le temps écoulé en milisecondes depuis le début du minuteur courant.
    pub fn get_elasped_milliseconds(&self) -> u64 {
        super::now_as_milliseconds() - self.beginning
    }

    /// Vérifie si le minuteur courant a fini d'attendre.
    pub fn finished(&self) -> bool {
        self.get_elasped_milliseconds() >= self.get_milliseconds_to_wait()
    }

    /// Relance le minuteur courant.
    pub fn restart(&mut self) {
        self.beginning = super::now_as_milliseconds();
    }

    /// Attend que le minuteur courant finisse d'attendre.
    pub fn wait(&self) {
        while !self.finished() {}
    }
}
