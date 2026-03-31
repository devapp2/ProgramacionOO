//! # PersonalAdministrativo — empleados de la gestión institucional
//!
//! El personal administrativo apoya la gestión académica y operativa de la
//! Universidad Distrital Francisco José de Caldas.
//!
//! **Conforme al Acuerdo 004 de 2025**, el personal administrativo hace parte
//! de la comunidad universitaria con derechos y deberes propios.

use crate::modelo::persona::{Persona, PersonaBase};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Empleado administrativo de la Universidad Distrital.
///
/// Equivale a la clase `PersonalAdministrativo extends Persona` del proyecto Java.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalAdministrativo {
    /// Datos comunes de la persona.
    pub base: PersonaBase,

    /// Cargo o denominación del empleo dentro de la institución.
    pub cargo: String,

    /// Dependencia o unidad organizacional donde presta sus servicios.
    pub dependencia: String,
}

impl PersonalAdministrativo {
    /// Crea un nuevo registro de personal administrativo.
    ///
    /// # Ejemplo
    /// ```
    /// use estatuto_ud::modelo::personal_administrativo::PersonalAdministrativo;
    ///
    /// let p = PersonalAdministrativo::nuevo(
    ///     "Roberto Salcedo",
    ///     "CC-5001",
    ///     Some("rsalcedo@ud.edu.co".to_string()),
    ///     "Auxiliar Administrativo",
    ///     "Registro y Control",
    /// );
    /// assert_eq!(p.cargo, "Auxiliar Administrativo");
    /// ```
    pub fn nuevo(
        nombre: &str,
        identificacion: &str,
        correo: Option<String>,
        cargo: &str,
        dependencia: &str,
    ) -> Self {
        PersonalAdministrativo {
            base: PersonaBase::nueva(nombre, identificacion, correo),
            cargo: cargo.trim().to_string(),
            dependencia: dependencia.trim().to_string(),
        }
    }
}

impl Persona for PersonalAdministrativo {
    fn rol(&self) -> &str {
        "Personal Administrativo"
    }

    fn base(&self) -> &PersonaBase {
        &self.base
    }
}

impl fmt::Display for PersonalAdministrativo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Admin. {} — Cargo: {} | Dependencia: {}",
            self.base, self.cargo, self.dependencia
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn personal_administrativo_crea_correctamente() {
        let p = PersonalAdministrativo::nuevo(
            "Roberto Salcedo",
            "CC-5001",
            None,
            "Auxiliar",
            "Registro y Control",
        );
        assert_eq!(p.cargo, "Auxiliar");
        assert_eq!(p.dependencia, "Registro y Control");
    }

    #[test]
    fn personal_administrativo_rol() {
        let p = PersonalAdministrativo::nuevo("X", "CC-0", None, "C", "D");
        assert_eq!(p.rol(), "Personal Administrativo");
    }

    #[test]
    fn personal_administrativo_display() {
        let p = PersonalAdministrativo::nuevo("Ana", "CC-1", None, "Secretaria", "Rectoría");
        assert!(p.to_string().contains("Secretaria"));
    }

    #[test]
    fn personal_administrativo_serializa() {
        let p = PersonalAdministrativo::nuevo("Luis", "CC-9", None, "Jefe", "Finanzas");
        let json = serde_json::to_string(&p).unwrap();
        let back: PersonalAdministrativo = serde_json::from_str(&json).unwrap();
        assert_eq!(back.cargo, "Jefe");
    }
}
