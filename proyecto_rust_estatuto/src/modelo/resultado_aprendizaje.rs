//! # ResultadoDeAprendizaje — competencias del programa académico
//!
//! Los Resultados de Aprendizaje describen lo que el estudiante debe saber,
//! saber hacer y saber ser al finalizar un espacio académico o el programa.
//!
//! **Conforme al Acuerdo 004 de 2025**, los resultados de aprendizaje articulan
//! el plan de estudios con el perfil del egresado y los propósitos de formación.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Resultado de aprendizaje esperado en un espacio académico o programa.
///
/// Equivale a la clase `ResultadoDeAprendizaje` del proyecto Java.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultadoDeAprendizaje {
    /// Descripción detallada de la competencia o logro esperado.
    pub descripcion: String,

    /// Nivel de dominio requerido (Básico, Intermedio, Avanzado, etc.).
    pub nivel_dominio: String,

    /// Código del programa al que pertenece este resultado.
    pub programa_codigo: String,
}

impl ResultadoDeAprendizaje {
    /// Crea un nuevo Resultado de Aprendizaje.
    pub fn nuevo(descripcion: &str, nivel_dominio: &str, programa_codigo: &str) -> Self {
        ResultadoDeAprendizaje {
            descripcion: descripcion.trim().to_string(),
            nivel_dominio: nivel_dominio.trim().to_string(),
            programa_codigo: programa_codigo.trim().to_uppercase(),
        }
    }
}

impl fmt::Display for ResultadoDeAprendizaje {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RA [Nivel: {}] {} (Programa: {})",
            self.nivel_dominio, self.descripcion, self.programa_codigo
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ra_crea_correctamente() {
        let ra = ResultadoDeAprendizaje::nuevo("Diseñar algoritmos eficientes", "Avanzado", "PROG-SIS");
        assert_eq!(ra.nivel_dominio, "Avanzado");
    }

    #[test]
    fn ra_display() {
        let ra = ResultadoDeAprendizaje::nuevo("Modelar sistemas", "Intermedio", "PROG-SIS");
        assert!(ra.to_string().contains("Modelar sistemas"));
    }

    #[test]
    fn ra_serializa() {
        let ra = ResultadoDeAprendizaje::nuevo("Programar en Rust", "Avanzado", "PROG-SIS");
        let json = serde_json::to_string(&ra).unwrap();
        let back: ResultadoDeAprendizaje = serde_json::from_str(&json).unwrap();
        assert_eq!(back.nivel_dominio, "Avanzado");
    }
}
