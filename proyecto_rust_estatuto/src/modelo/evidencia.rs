//! # Evidencia — producto del aprendizaje estudiantil
//!
//! La Evidencia es el artefacto o producto que el estudiante presenta como
//! demostración del logro de un resultado de aprendizaje.
//!
//! En el contexto del modelo pedagógico de la Universidad Distrital, las
//! evidencias son el insumo principal para la evaluación por competencias.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Producto o artefacto que evidencia el aprendizaje de un estudiante.
///
/// Equivale a la clase `Evidencia` del proyecto Java.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidencia {
    /// Descripción del artefacto o producto presentado.
    pub descripcion: String,

    /// Tipo de evidencia: "Informe", "Código", "Video", "Presentación", etc.
    pub tipo: String,

    /// URL o ruta donde se puede acceder al artefacto (opcional).
    pub url: Option<String>,

    /// Identificación del estudiante que presenta la evidencia.
    pub estudiante_id: String,

    /// Código del espacio académico en el contexto del cual se presenta.
    pub espacio_academico_codigo: String,
}

impl Evidencia {
    /// Crea una nueva Evidencia.
    ///
    /// # Ejemplo
    /// ```
    /// use estatuto_ud::modelo::evidencia::Evidencia;
    ///
    /// let e = Evidencia::nueva(
    ///     "Proyecto final de POO en Rust",
    ///     "Código",
    ///     Some("https://github.com/estudiante/proyecto".to_string()),
    ///     "CC-4001",
    ///     "POO-201",
    /// );
    /// assert_eq!(e.tipo, "Código");
    /// ```
    pub fn nueva(
        descripcion: &str,
        tipo: &str,
        url: Option<String>,
        estudiante_id: &str,
        espacio_academico_codigo: &str,
    ) -> Self {
        Evidencia {
            descripcion: descripcion.trim().to_string(),
            tipo: tipo.trim().to_string(),
            url,
            estudiante_id: estudiante_id.trim().to_string(),
            espacio_academico_codigo: espacio_academico_codigo.trim().to_uppercase(),
        }
    }

    /// Indica si la evidencia tiene un recurso en línea asociado.
    pub fn tiene_url(&self) -> bool {
        self.url.is_some()
    }
}

impl fmt::Display for Evidencia {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let url_str = self.url.as_deref().unwrap_or("Sin URL");
        write!(
            f,
            "Evidencia [{}] {} — Estudiante: {} | Espacio: {} | URL: {}",
            self.tipo, self.descripcion, self.estudiante_id, self.espacio_academico_codigo, url_str
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evidencia_crea_correctamente() {
        let e = Evidencia::nueva("Proyecto", "Código", None, "CC-4001", "POO-201");
        assert_eq!(e.tipo, "Código");
        assert!(!e.tiene_url());
    }

    #[test]
    fn evidencia_con_url() {
        let e = Evidencia::nueva("Informe", "Documento", Some("http://link".to_string()), "CC-4001", "ALG-101");
        assert!(e.tiene_url());
    }

    #[test]
    fn evidencia_display() {
        let e = Evidencia::nueva("Examen", "Evaluación", None, "CC-001", "CAL-101");
        assert!(e.to_string().contains("Examen"));
    }

    #[test]
    fn evidencia_serializa() {
        let e = Evidencia::nueva("Video demo", "Video", None, "CC-4001", "POO-201");
        let json = serde_json::to_string(&e).unwrap();
        let back: Evidencia = serde_json::from_str(&json).unwrap();
        assert_eq!(back.tipo, "Video");
    }
}
