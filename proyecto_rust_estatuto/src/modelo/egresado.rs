//! # Egresado — graduado de la Universidad Distrital
//!
//! El Egresado es el profesional formado por la Universidad Distrital
//! Francisco José de Caldas que hace parte de la comunidad universitaria
//! de manera permanente.
//!
//! **Conforme al Artículo 96 del Acuerdo 004 de 2025**, los egresados tienen
//! derechos de participación y acceso a servicios institucionales.

use crate::modelo::persona::{Persona, PersonaBase};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Graduado de la Universidad Distrital Francisco José de Caldas.
///
/// Equivale a la clase `Egresado extends Persona` del proyecto Java.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Egresado {
    /// Datos comunes de la persona.
    pub base: PersonaBase,

    /// Año en que se graduó el egresado.
    pub anio_graduacion: u16,

    /// Código del programa del que se graduó.
    pub programa_codigo: String,

    /// Título académico obtenido al graduarse.
    pub titulo_obtenido: String,
}

impl Egresado {
    /// Crea un nuevo registro de Egresado.
    ///
    /// # Ejemplo
    /// ```
    /// use estatuto_ud::modelo::egresado::Egresado;
    ///
    /// let e = Egresado::nuevo(
    ///     "Camila Torres",
    ///     "CC-7001",
    ///     Some("ctorres@gmail.com".to_string()),
    ///     2023,
    ///     "PROG-SIS",
    ///     "Ingeniera de Sistemas",
    /// );
    /// assert_eq!(e.anio_graduacion, 2023);
    /// ```
    pub fn nuevo(
        nombre: &str,
        identificacion: &str,
        correo: Option<String>,
        anio_graduacion: u16,
        programa_codigo: &str,
        titulo_obtenido: &str,
    ) -> Self {
        Egresado {
            base: PersonaBase::nueva(nombre, identificacion, correo),
            anio_graduacion,
            programa_codigo: programa_codigo.trim().to_uppercase(),
            titulo_obtenido: titulo_obtenido.trim().to_string(),
        }
    }
}

impl Persona for Egresado {
    fn rol(&self) -> &str {
        "Egresado"
    }

    fn base(&self) -> &PersonaBase {
        &self.base
    }
}

impl fmt::Display for Egresado {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Egresado {} — Título: {} ({}) | Programa: {}",
            self.base, self.titulo_obtenido, self.anio_graduacion, self.programa_codigo
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn egresado_prueba() -> Egresado {
        Egresado::nuevo("Camila Torres", "CC-7001", None, 2023, "PROG-SIS", "Ing. de Sistemas")
    }

    #[test]
    fn egresado_crea_correctamente() {
        let e = egresado_prueba();
        assert_eq!(e.anio_graduacion, 2023);
        assert_eq!(e.titulo_obtenido, "Ing. de Sistemas");
    }

    #[test]
    fn egresado_rol() {
        assert_eq!(egresado_prueba().rol(), "Egresado");
    }

    #[test]
    fn egresado_display() {
        let e = egresado_prueba();
        assert!(e.to_string().contains("Camila Torres"));
        assert!(e.to_string().contains("2023"));
    }

    #[test]
    fn egresado_serializa() {
        let e = egresado_prueba();
        let json = serde_json::to_string(&e).unwrap();
        let back: Egresado = serde_json::from_str(&json).unwrap();
        assert_eq!(back.anio_graduacion, 2023);
    }
}
