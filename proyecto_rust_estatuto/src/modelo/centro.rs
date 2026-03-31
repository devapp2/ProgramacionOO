//! # Centro — unidad académica de investigación y extensión
//!
//! Los Centros son unidades académicas especializadas de la Universidad Distrital
//! orientadas a la investigación, la extensión y la proyección social.
//!
//! **Conforme al Artículo 22 del Acuerdo 004 de 2025**, los Centros pueden
//! ser interfacultativos o adscritos a una Facultad específica.

use crate::excepcion::errores::EstatutoResult;
use crate::modelo::unidad_academica::{UnidadAcademica, UnidadBase};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Unidad académica de investigación y extensión.
///
/// Equivale a la clase `Centro extends UnidadAcademica` del proyecto Java.
/// Es una subclase simple sin campos adicionales (solo hereda la base).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Centro {
    /// Datos comunes de la unidad académica.
    pub base: UnidadBase,

    /// Indica si el centro es interfacultativo (transversal a varias facultades).
    pub interfacultativo: bool,
}

impl Centro {
    /// Crea un nuevo Centro con validación de campos.
    ///
    /// # Errores
    /// Propaga errores de [`UnidadBase::nuevo`].
    ///
    /// # Ejemplo
    /// ```
    /// use chrono::NaiveDate;
    /// use estatuto_ud::modelo::centro::Centro;
    ///
    /// let c = Centro::nuevo(
    ///     "Centro de Investigación Tecnológica",
    ///     "CIT-01",
    ///     NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
    ///     "Dr. Ramírez",
    ///     false,
    /// ).unwrap();
    /// assert_eq!(c.tipo(), "Centro");
    /// ```
    pub fn nuevo(
        nombre: &str,
        codigo: &str,
        fecha_creacion: NaiveDate,
        director: &str,
        interfacultativo: bool,
    ) -> EstatutoResult<Self> {
        let base = UnidadBase::nuevo(nombre, codigo, fecha_creacion, director)?;
        Ok(Centro {
            base,
            interfacultativo,
        })
    }
}

impl UnidadAcademica for Centro {
    fn tipo(&self) -> &str {
        "Centro"
    }

    fn base(&self) -> &UnidadBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut UnidadBase {
        &mut self.base
    }
}

impl fmt::Display for Centro {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let alcance = if self.interfacultativo {
            "Interfacultativo"
        } else {
            "Adscrito"
        };
        write!(
            f,
            "Centro [{}] {} — Director: {} ({})",
            self.base.codigo, self.base.nombre, self.base.director, alcance
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modelo::unidad_academica::UnidadAcademica;
    use chrono::NaiveDate;

    #[test]
    fn centro_crea_correctamente() {
        let c = Centro::nuevo(
            "Centro de Investigación",
            "CI-01",
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            "Dir",
            false,
        )
        .unwrap();
        assert_eq!(c.tipo(), "Centro");
        assert_eq!(c.base.codigo, "CI-01");
    }

    #[test]
    fn centro_display_interfacultativo() {
        let c = Centro::nuevo(
            "Centro Tecnológico",
            "CT-01",
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            "Dir",
            true,
        )
        .unwrap();
        assert!(c.to_string().contains("Interfacultativo"));
    }

    #[test]
    fn centro_codigo_vacio_error() {
        let r = Centro::nuevo("Centro", "", NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(), "Dir", false);
        assert!(r.is_err());
    }
}
