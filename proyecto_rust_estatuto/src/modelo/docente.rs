//! # Docente — miembro del cuerpo profesoral
//!
//! El Docente es el actor principal de la función de enseñanza en la
//! Universidad Distrital Francisco José de Caldas.
//!
//! **Conforme al Artículo 68 del Acuerdo 004 de 2025**, los docentes se
//! clasifican según su tipo de vinculación laboral con la institución.

use crate::modelo::enums::TipoVinculacion;
use crate::modelo::persona::{Persona, PersonaBase};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Miembro del cuerpo profesoral de la Universidad Distrital.
///
/// Equivale a la clase `Docente extends Persona` del proyecto Java.
/// En Rust, la relación de herencia se implementa con composición:
/// el campo `base: PersonaBase` contiene los atributos comunes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Docente {
    /// Datos comunes de la persona (nombre, identificación, correo).
    pub base: PersonaBase,

    /// Modalidad de vinculación laboral con la Universidad.
    ///
    /// Conforme al Artículo 68 del Acuerdo 004 de 2025, determina los
    /// derechos y obligaciones del docente.
    pub tipo_vinculacion: TipoVinculacion,

    /// Código de la escuela a la que está adscrito el docente.
    pub escuela_codigo: String,

    /// Categoría docente según el escalafón (Auxiliar, Asistente, Asociado, Titular).
    pub categoria: String,
}

impl Docente {
    /// Crea un nuevo Docente con los datos requeridos.
    ///
    /// # Ejemplo
    /// ```
    /// use estatuto_ud::modelo::docente::Docente;
    /// use estatuto_ud::modelo::enums::TipoVinculacion;
    ///
    /// let d = Docente::nuevo(
    ///     "Carlos Gómez",
    ///     "CC-1002",
    ///     Some("cgomez@udistrital.edu.co".to_string()),
    ///     TipoVinculacion::Planta,
    ///     "ESC-SIS",
    ///     "Asociado",
    /// );
    /// assert!(d.es_planta());
    /// ```
    pub fn nuevo(
        nombre: &str,
        identificacion: &str,
        correo: Option<String>,
        tipo_vinculacion: TipoVinculacion,
        escuela_codigo: &str,
        categoria: &str,
    ) -> Self {
        Docente {
            base: PersonaBase::nueva(nombre, identificacion, correo),
            tipo_vinculacion,
            escuela_codigo: escuela_codigo.trim().to_uppercase(),
            categoria: categoria.trim().to_string(),
        }
    }

    /// Indica si el docente pertenece a la planta oficial de la Universidad.
    ///
    /// Los docentes de planta tienen nombramiento definitivo y gozan de
    /// estabilidad laboral, conforme al Acuerdo 004 de 2025.
    pub fn es_planta(&self) -> bool {
        self.tipo_vinculacion == TipoVinculacion::Planta
    }

    /// Indica si el docente está vinculado por horas de cátedra.
    pub fn es_catedra(&self) -> bool {
        self.tipo_vinculacion == TipoVinculacion::HoraCatedra
    }
}

impl Persona for Docente {
    fn rol(&self) -> &str {
        "Docente"
    }

    fn base(&self) -> &PersonaBase {
        &self.base
    }
}

impl fmt::Display for Docente {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Docente {} — {} | Categoría: {} | Escuela: {}",
            self.base, self.tipo_vinculacion, self.categoria, self.escuela_codigo
        )
    }
}

// ============================================================
//  Pruebas unitarias
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modelo::enums::TipoVinculacion;

    fn docente_planta() -> Docente {
        Docente::nuevo(
            "Ana Pérez",
            "CC-2001",
            Some("aperez@ud.edu.co".to_string()),
            TipoVinculacion::Planta,
            "ESC-SIS",
            "Titular",
        )
    }

    fn docente_catedra() -> Docente {
        Docente::nuevo(
            "Luis García",
            "CC-3001",
            None,
            TipoVinculacion::HoraCatedra,
            "ESC-SIS",
            "Auxiliar",
        )
    }

    #[test]
    fn docente_crea_correctamente() {
        let d = docente_planta();
        assert_eq!(d.base.nombre, "Ana Pérez");
        assert_eq!(d.escuela_codigo, "ESC-SIS");
    }

    #[test]
    fn es_planta_retorna_true() {
        assert!(docente_planta().es_planta());
    }

    #[test]
    fn es_planta_retorna_false_para_catedra() {
        assert!(!docente_catedra().es_planta());
    }

    #[test]
    fn es_catedra_retorna_true() {
        assert!(docente_catedra().es_catedra());
    }

    #[test]
    fn docente_tipo_visitante() {
        let d = Docente::nuevo("Inv. Externo", "PASS-001", None, TipoVinculacion::Visitante, "ESC-MAT", "Visitante");
        assert_eq!(d.tipo_vinculacion, TipoVinculacion::Visitante);
    }

    #[test]
    fn docente_rol() {
        assert_eq!(docente_planta().rol(), "Docente");
    }

    #[test]
    fn docente_display_contiene_nombre() {
        let d = docente_planta();
        assert!(d.to_string().contains("Ana Pérez"));
        assert!(d.to_string().contains("Titular"));
    }

    #[test]
    fn docente_serializa() {
        let d = docente_planta();
        let json = serde_json::to_string(&d).unwrap();
        let back: Docente = serde_json::from_str(&json).unwrap();
        assert_eq!(back.base.identificacion, "CC-2001");
    }
}
