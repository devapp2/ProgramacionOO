//! # Observer — notificación de decisiones de Consejos
//!
//! Implementa el **Patrón Observer** para notificar a las entidades interesadas
//! cuando se toma una decisión en un Consejo universitario.
//!
//! ## Contexto institucional
//! **Conforme al Acuerdo 004 de 2025**, la Universidad Distrital tiene varios
//! órganos de gobierno colegiado (Consejo Académico, Consejo de Facultad,
//! Consejo Curricular) que toman decisiones que afectan a múltiples actores.
//!
//! ## Equivalencia Java → Rust
//! - `ConsejoObserver` (interface Java) → trait `ConsejoObserver`
//! - `ConsejoCurricular` → struct con impl del trait
//! - `ConsejoFacultad` → struct con impl del trait
//!
//! ## Propósito pedagógico
//! En Rust, el patrón Observer se implementa con traits y `Vec<Box<dyn ConsejoObserver>>`.
//! La clave es que cada observador recibe una notificación sin que el sujeto
//! conozca los detalles de implementación de cada uno.

// ============================================================
//  Trait ConsejoObserver
// ============================================================

/// Interfaz del observador de decisiones de consejo.
///
/// Equivale a la interface `ConsejoObserver` de Java.
/// Cualquier entidad que quiera ser notificada de las decisiones de un
/// órgano colegiado debe implementar este trait.
pub trait ConsejoObserver: Send + Sync {
    /// Se invoca cuando el órgano de gobierno toma una decisión.
    ///
    /// # Argumentos
    /// - `decision`: Texto que describe la decisión tomada.
    ///
    /// Equivale al método `onDecision(String decision)` de Java.
    fn on_decision(&self, decision: &str);

    /// Nombre identificador del observador.
    fn nombre_observador(&self) -> &str;
}

// ============================================================
//  ConsejoCurricular — observador del currículo
// ============================================================

/// Observador que representa al Consejo Curricular de un programa.
///
/// Recibe notificaciones cuando el Consejo Académico toma decisiones
/// que afectan el currículo del programa.
pub struct ConsejoCurricular {
    pub programa_codigo: String,
}

impl ConsejoCurricular {
    pub fn nuevo(programa_codigo: &str) -> Self {
        ConsejoCurricular {
            programa_codigo: programa_codigo.to_uppercase(),
        }
    }
}

impl ConsejoObserver for ConsejoCurricular {
    fn on_decision(&self, decision: &str) {
        // En una aplicación real, esto podría actualizar el estado del programa,
        // registrar en un log, enviar notificación, etc.
        println!(
            "[ConsejoCurricular - {}] Notificación recibida: {}",
            self.programa_codigo, decision
        );
    }

    fn nombre_observador(&self) -> &str {
        "Consejo Curricular"
    }
}

// ============================================================
//  ConsejoFacultad — observador de la facultad
// ============================================================

/// Observador que representa al Consejo de Facultad.
///
/// Conforme al Artículo 17 del Acuerdo 004 de 2025, el Consejo de Facultad
/// es el máximo órgano de dirección de la Facultad y debe ser notificado
/// de las decisiones del Consejo Académico que lo afecten.
pub struct ConsejoFacultad {
    pub facultad_codigo: String,
}

impl ConsejoFacultad {
    pub fn nuevo(facultad_codigo: &str) -> Self {
        ConsejoFacultad {
            facultad_codigo: facultad_codigo.to_uppercase(),
        }
    }
}

impl ConsejoObserver for ConsejoFacultad {
    fn on_decision(&self, decision: &str) {
        println!(
            "[ConsejoFacultad - {}] Decisión del Consejo Académico registrada: {}",
            self.facultad_codigo, decision
        );
    }

    fn nombre_observador(&self) -> &str {
        "Consejo de Facultad"
    }
}

// ============================================================
//  GestorNotificaciones — el sujeto (Subject) del patrón Observer
// ============================================================

/// Gestor de notificaciones de decisiones de órganos colegiados.
///
/// Este es el **sujeto (Subject)** del patrón Observer. Mantiene la lista
/// de observadores y les notifica cuando se toma una decisión.
pub struct GestorNotificaciones {
    observadores: Vec<Box<dyn ConsejoObserver>>,
}

impl GestorNotificaciones {
    /// Crea un nuevo gestor de notificaciones sin observadores.
    pub fn nuevo() -> Self {
        GestorNotificaciones {
            observadores: Vec::new(),
        }
    }

    /// Registra un nuevo observador (equivale a `addObserver` en Java).
    pub fn suscribir(&mut self, observador: Box<dyn ConsejoObserver>) {
        self.observadores.push(observador);
    }

    /// Notifica a todos los observadores de una decisión.
    ///
    /// Equivale al método `notifyObservers(String)` del Subject en Java.
    pub fn notificar(&self, decision: &str) {
        for obs in &self.observadores {
            obs.on_decision(decision);
        }
    }

    /// Retorna el número de observadores suscritos.
    pub fn num_observadores(&self) -> usize {
        self.observadores.len()
    }
}

impl Default for GestorNotificaciones {
    fn default() -> Self {
        Self::nuevo()
    }
}

// ============================================================
//  Pruebas unitarias
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    /// Observador de prueba que registra las notificaciones recibidas.
    struct ObservadorRegistrador {
        decisiones: Arc<Mutex<Vec<String>>>,
    }

    impl ConsejoObserver for ObservadorRegistrador {
        fn on_decision(&self, decision: &str) {
            self.decisiones.lock().unwrap().push(decision.to_string());
        }
        fn nombre_observador(&self) -> &str {
            "Registrador de Prueba"
        }
    }

    #[test]
    fn gestor_notifica_a_todos_los_observadores() {
        let mut gestor = GestorNotificaciones::nuevo();
        let decisiones1 = Arc::new(Mutex::new(Vec::new()));
        let decisiones2 = Arc::new(Mutex::new(Vec::new()));

        gestor.suscribir(Box::new(ObservadorRegistrador {
            decisiones: decisiones1.clone(),
        }));
        gestor.suscribir(Box::new(ObservadorRegistrador {
            decisiones: decisiones2.clone(),
        }));

        gestor.notificar("Aprobación del Plan de Estudios 2025");

        assert_eq!(decisiones1.lock().unwrap().len(), 1);
        assert_eq!(decisiones2.lock().unwrap().len(), 1);
    }

    #[test]
    fn consejo_curricular_nombre() {
        let cc = ConsejoCurricular::nuevo("PROG-SIS");
        assert_eq!(cc.nombre_observador(), "Consejo Curricular");
    }

    #[test]
    fn consejo_facultad_nombre() {
        let cf = ConsejoFacultad::nuevo("FAC-ING");
        assert_eq!(cf.nombre_observador(), "Consejo de Facultad");
    }

    #[test]
    fn gestor_cuenta_observadores() {
        let mut g = GestorNotificaciones::nuevo();
        g.suscribir(Box::new(ConsejoCurricular::nuevo("P1")));
        g.suscribir(Box::new(ConsejoFacultad::nuevo("F1")));
        assert_eq!(g.num_observadores(), 2);
    }
}
