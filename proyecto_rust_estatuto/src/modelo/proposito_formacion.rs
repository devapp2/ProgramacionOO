//! # PropositoFormacion — misión formativa del programa
//!
//! El Propósito de Formación describe la intención educativa general del programa
//! académico, orientando el perfil del egresado y el currículo.
//!
//! **Conforme al Acuerdo 004 de 2025**, cada programa académico debe declarar
//! explícitamente sus propósitos de formación alineados con la misión institucional.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Propósito de formación de un programa académico.
///
/// Equivale a la clase `PropositoFormacion` del proyecto Java.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropositoFormacion {
    /// Descripción del propósito o intención educativa.
    pub descripcion: String,

    /// Nivel taxonómico o de profundidad del propósito (Cognitivo, Procedimental, Actitudinal).
    pub nivel: String,

    /// Código del programa al que pertenece este propósito.
    pub programa_codigo: String,
}

impl PropositoFormacion {
    /// Crea un nuevo Propósito de Formación.
    pub fn nuevo(descripcion: &str, nivel: &str, programa_codigo: &str) -> Self {
        PropositoFormacion {
            descripcion: descripcion.trim().to_string(),
            nivel: nivel.trim().to_string(),
            programa_codigo: programa_codigo.trim().to_uppercase(),
        }
    }
}

impl fmt::Display for PropositoFormacion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PF [{}] ({}) — {}",
            self.programa_codigo, self.nivel, self.descripcion
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proposito_crea_correctamente() {
        let pf = PropositoFormacion::nuevo("Desarrollar pensamiento crítico", "Cognitivo", "PROG-SIS");
        assert_eq!(pf.nivel, "Cognitivo");
    }

    #[test]
    fn proposito_display() {
        let pf = PropositoFormacion::nuevo("Liderar proyectos", "Actitudinal", "PROG-SIS");
        assert!(pf.to_string().contains("Liderar proyectos"));
    }

    #[test]
    fn proposito_serializa() {
        let pf = PropositoFormacion::nuevo("Aplicar metodologías", "Procedimental", "PROG-SIS");
        let json = serde_json::to_string(&pf).unwrap();
        let back: PropositoFormacion = serde_json::from_str(&json).unwrap();
        assert_eq!(back.nivel, "Procedimental");
    }
}
