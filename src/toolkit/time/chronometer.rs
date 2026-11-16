//! Fichier contenant la manipulation d'un chronomètre adapté à topi.

/// Chronomètre manipulant le temps sous forme de millisecondes.
pub struct Chronometer {
    begin: u64
}

impl Chronometer {
    /// Construit un nouveau chronomètre sur le temps courant.
    pub fn new() -> Self {
        Self { begin: super::now_as_milliseconds() }
    }

    /// Renvoie le temps écoulé depuis le lancement du chronomètre sous forme de millisecondes.
    pub fn get_elapsed_milliseconds(&self) -> u64 {
        super::now_as_milliseconds() - self.begin
    }

    /// Relance le chronomètre courant (place son temp écoulé à 0).
    pub fn restart(&mut self) {
        self.begin = super::now_as_milliseconds();
    }
}
