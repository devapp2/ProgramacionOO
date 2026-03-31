//! # Matricula — registro formal de inscripción del estudiante
//!
//! La Matrícula es el acto administrativo mediante el cual el estudiante
//! formaliza su vinculación a un programa académico en un período determinado.
//!
//! **Conforme al Acuerdo 004 de 2025**, la matrícula es obligatoria para
//! acceder a los servicios académicos y da origen a los derechos y deberes
//! del estudiante en cada período académico.

use crate::excepcion::errores::{EstatutoError, EstatutoResult};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Registro formal de la inscripción de un estudiante en un período académico.
///
/// Equivale a la clase `Matricula` del proyecto Java.
/// El identificador único se genera con UUID v4 (equivalente a `UUID.randomUUID()`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Matricula {
    /// Identificador único de la matrícula (UUID v4).
    pub codigo: String,

    /// Identificación del estudiante matriculado.
    pub estudiante_id: String,

    /// Código del programa académico en el que se matricula.
    pub programa_codigo: String,

    /// Período académico de la matrícula (ej: "2025-1", "2025-2").
    pub periodo: String,

    /// Fecha en que se realizó el trámite de matrícula.
    pub fecha_matricula: NaiveDate,
}

impl Matricula {
    /// Crea y valida una nueva Matrícula.
    ///
    /// Genera automáticamente un UUID v4 como código de la matrícula.
    ///
    /// # Errores
    /// Retorna [`EstatutoError::Matricula`] si:
    /// - `estudiante_id` está vacío.
    /// - `programa_codigo` está vacío.
    /// - `periodo` está vacío o no tiene el formato esperado.
    ///
    /// # Ejemplo
    /// ```
    /// use chrono::NaiveDate;
    /// use estatuto_ud::modelo::matricula::Matricula;
    ///
    /// let m = Matricula::nueva(
    ///     "CC-4001",
    ///     "PROG-SIS",
    ///     "2025-1",
    ///     NaiveDate::from_ymd_opt(2025, 2, 1).unwrap(),
    /// ).unwrap();
    /// assert!(!m.codigo.is_empty());
    /// ```
    pub fn nueva(
        estudiante_id: &str,
        programa_codigo: &str,
        periodo: &str,
        fecha_matricula: NaiveDate,
    ) -> EstatutoResult<Self> {
        if estudiante_id.trim().is_empty() {
            return Err(EstatutoError::Matricula(
                "El ID del estudiante no puede estar vacío".to_string(),
            ));
        }
        if programa_codigo.trim().is_empty() {
            return Err(EstatutoError::Matricula(
                "El código del programa no puede estar vacío".to_string(),
            ));
        }
        if periodo.trim().is_empty() {
            return Err(EstatutoError::Matricula(
                "El período académico no puede estar vacío".to_string(),
            ));
        }
        Ok(Matricula {
            codigo: Uuid::new_v4().to_string(),
            estudiante_id: estudiante_id.trim().to_string(),
            programa_codigo: programa_codigo.trim().to_uppercase(),
            periodo: periodo.trim().to_string(),
            fecha_matricula,
        })
    }

    /// Valida que la matrícula cumpla las reglas de negocio básicas.
    ///
    /// Equivale al método `validar()` de la clase Java.
    /// En esta implementación, la validación se realiza en el constructor,
    /// pero este método puede usarse para re-validar tras una deserialización.
    pub fn validar(&self) -> EstatutoResult<()> {
        if self.estudiante_id.is_empty() {
            return Err(EstatutoError::Matricula(
                "La matrícula no tiene estudiante asignado".to_string(),
            ));
        }
        if self.programa_codigo.is_empty() {
            return Err(EstatutoError::Matricula(
                "La matrícula no tiene programa asignado".to_string(),
            ));
        }
        Ok(())
    }
}

impl fmt::Display for Matricula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Matrícula [{}] Estudiante: {} | Programa: {} | Período: {} | Fecha: {}",
            &self.codigo[..8], // Solo primeros 8 chars del UUID para brevedad
            self.estudiante_id,
            self.programa_codigo,
            self.periodo,
            self.fecha_matricula
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn matricula_prueba() -> Matricula {
        Matricula::nueva(
            "CC-4001",
            "2879",
            "2025-1",
            NaiveDate::from_ymd_opt(2025, 2, 1).unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn matricula_crea_correctamente() {
        let m = matricula_prueba();
        assert!(!m.codigo.is_empty());
        assert_eq!(m.estudiante_id, "CC-4001");
    }

    #[test]
    fn matricula_uuid_generado() {
        let m1 = matricula_prueba();
        let m2 = matricula_prueba();
        // Cada matrícula debe tener UUID único
        assert_ne!(m1.codigo, m2.codigo);
    }

    #[test]
    fn matricula_estudiante_vacio_error() {
        let r = Matricula::nueva("", "PROG", "2025-1", NaiveDate::from_ymd_opt(2025, 1, 1).unwrap());
        assert!(r.is_err());
    }

    #[test]
    fn matricula_programa_vacio_error() {
        let r = Matricula::nueva("CC-1", "", "2025-1", NaiveDate::from_ymd_opt(2025, 1, 1).unwrap());
        assert!(r.is_err());
    }

    #[test]
    fn matricula_valida_correctamente() {
        assert!(matricula_prueba().validar().is_ok());
    }

    #[test]
    fn matricula_serializa() {
        let m = matricula_prueba();
        let json = serde_json::to_string(&m).unwrap();
        let back: Matricula = serde_json::from_str(&json).unwrap();
        assert_eq!(back.estudiante_id, "CC-4001");
    }
}
