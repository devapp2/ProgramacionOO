//! # Instituto — organismo académico-científico especializado
//!
//! Los Institutos son organismos de carácter académico-científico que integran
//! docencia, investigación y extensión en una disciplina o problemática
//! de conocimiento específica.
//!
//! **Conforme al Artículo 24 del Acuerdo 004 de 2025**, los Institutos
//! tienen reglamentación especial aprobada por el Consejo Superior Universitario.

use crate::excepcion::errores::EstatutoResult;
use crate::modelo::unidad_academica::{UnidadAcademica, UnidadBase};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Organismo académico-científico especializado de la Universidad Distrital.
///
/// Equivale a la clase `Instituto extends UnidadAcademica` del proyecto Java.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instituto {
    /// Datos comunes de la unidad académica.
    pub base: UnidadBase,

    /// Área científica o temática de especialización del instituto.
    pub area_especialidad: String,
}

impl Instituto {
    /// Crea un nuevo Instituto con validación de campos.
    ///
    /// # Errores
    /// Propaga errores de [`UnidadBase::nuevo`].
    ///
    /// # Ejemplo
    /// ```
    /// use chrono::NaiveDate;
    /// use estatuto_ud::modelo::instituto::Instituto;
    ///
    /// let i = Instituto::nuevo(
    ///     "Instituto de Estudios Ambientales",
    ///     "IDEA",
    ///     NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
    ///     "Dra. Vargas",
    ///     "Ciencias Ambientales",
    /// ).unwrap();
    /// assert_eq!(i.tipo(), "Instituto");
    /// ```
    pub fn nuevo(
        nombre: &str,
        codigo: &str,
        fecha_creacion: NaiveDate,
        director: &str,
        area_especialidad: &str,
    ) -> EstatutoResult<Self> {
        let base = UnidadBase::nuevo(nombre, codigo, fecha_creacion, director)?;
        Ok(Instituto {
            base,
            area_especialidad: area_especialidad.trim().to_string(),
        })
    }
}

impl UnidadAcademica for Instituto {
    fn tipo(&self) -> &str {
        "Instituto"
    }

    fn base(&self) -> &UnidadBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut UnidadBase {
        &mut self.base
    }
}

impl fmt::Display for Instituto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Instituto [{}] {} — Director: {} | Especialidad: {}",
            self.base.codigo, self.base.nombre, self.base.director, self.area_especialidad
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modelo::unidad_academica::UnidadAcademica;
    use chrono::NaiveDate;

    #[test]
    fn instituto_crea_correctamente() {
        let i = Instituto::nuevo(
            "Instituto Ambiental",
            "INST-AMB",
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            "Dir",
            "Medio Ambiente",
        )
        .unwrap();
        assert_eq!(i.tipo(), "Instituto");
        assert_eq!(i.area_especialidad, "Medio Ambiente");
    }

    #[test]
    fn instituto_display() {
        let i = Instituto::nuevo(
            "Instituto de TIC",
            "INST-TIC",
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            "Dir",
            "Tecnologías de la Información",
        )
        .unwrap();
        assert!(i.to_string().contains("INST-TIC"));
        assert!(i.to_string().contains("Tecnologías"));
    }

    #[test]
    fn instituto_codigo_vacio_error() {
        let r = Instituto::nuevo("Instituto", "", NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(), "Dir", "TIC");
        assert!(r.is_err());
    }
}
