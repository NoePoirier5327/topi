//! Fichier contenant les interfaces vers la manipulation du temps dans topi (timer, chronomètre
//! etc...).

use std::time::{SystemTime, UNIX_EPOCH};

mod timer;
mod chronometer;

pub use timer::Timer;
pub use chronometer::Chronometer;

/// Renvoie le temps courant écoulé en milisecondes.
pub fn now_as_milliseconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System clock is earlier than unix epoch.")
        .as_millis() as u64
}
