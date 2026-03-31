//! # CABA — Comunidad Académica de Base
//!
//! Las CABA son comunidades académicas internas de las escuelas de la
//! Universidad Distrital. Agrupan docentes en torno a un área temática
//! específica para articular la investigación y la docencia.
//!
//! **Conforme al Acuerdo 004 de 2025**, las CABA constituyen el núcleo
//! básico de organización académica dentro de las escuelas.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Comunidad Académica de Base (CABA).
///
/// Equivale a la clase `Caba` del proyecto Java.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Caba {
    /// Nombre descriptivo de la comunidad académica.
    pub nombre: String,

    /// Código único identificador de la CABA.
    pub codigo: String,

    /// Área temática o disciplinar que caracteriza a la comunidad.
    pub area_tematica: String,

    /// Código de la escuela a la que pertenece esta CABA.
    pub escuela_codigo: String,
}

impl Caba {
    /// Crea una nueva CABA con los campos requeridos.
    ///
    /// # Ejemplo
    /// ```
    /// use estatuto_ud::modelo::caba::Caba;
    /// let c = Caba::nueva("CABA Sistemas", "CABA-SIS", "Ingeniería de Software", "ESC-SIS");
    /// assert_eq!(c.codigo, "CABA-SIS");
    /// ```
    pub fn nueva(
        nombre: &str,
        codigo: &str,
        area_tematica: &str,
        escuela_codigo: &str,
    ) -> Self {
        Caba {
            nombre: nombre.trim().to_string(),
            codigo: codigo.trim().to_uppercase(),
            area_tematica: area_tematica.trim().to_string(),
            escuela_codigo: escuela_codigo.trim().to_uppercase(),
        }
    }
}

impl fmt::Display for Caba {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CABA [{}] {} — Área: {} (Escuela: {})",
            self.codigo, self.nombre, self.area_tematica, self.escuela_codigo
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caba_crea_correctamente() {
        let c = Caba::nueva("CABA Sistemas", "caba-sis", "Software", "ESC-SIS");
        assert_eq!(c.codigo, "CABA-SIS");
        assert_eq!(c.nombre, "CABA Sistemas");
    }

    #[test]
    fn caba_display() {
        let c = Caba::nueva("CABA Redes", "CABA-RED", "Redes", "ESC-SIS");
        let texto = c.to_string();
        assert!(texto.contains("CABA-RED"));
        assert!(texto.contains("Redes"));
    }

    #[test]
    fn caba_serializa() {
        let c = Caba::nueva("CABA BD", "CABA-BD", "Bases de Datos", "ESC-SIS");
        let json = serde_json::to_string(&c).unwrap();
        let back: Caba = serde_json::from_str(&json).unwrap();
        assert_eq!(back.codigo, "CABA-BD");
    }
}
