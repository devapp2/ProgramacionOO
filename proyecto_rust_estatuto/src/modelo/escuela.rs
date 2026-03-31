//! # Escuela — subdivisión de la Facultad
//!
//! La Escuela es la unidad académica que organiza a los docentes en torno
//! a un campo disciplinar dentro de una Facultad.
//!
//! **Conforme al Artículo 20 del Acuerdo 004 de 2025**, cada escuela está
//! dirigida por un Director elegido por los docentes adscritos.

use crate::excepcion::errores::{EstatutoError, EstatutoResult};
use crate::modelo::caba::Caba;
use crate::modelo::docente::Docente;
use crate::modelo::unidad_academica::{UnidadAcademica, UnidadBase};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Unidad académica que agrupa docentes y CABA dentro de una Facultad.
///
/// Equivale a la clase `Escuela extends UnidadAcademica` del proyecto Java.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Escuela {
    /// Datos comunes de la unidad académica.
    pub base: UnidadBase,

    /// Docentes adscritos a esta escuela.
    pub docentes: Vec<Docente>,

    /// Comunidades Académicas de Base (CABA) organizadas en la escuela.
    pub cabas: Vec<Caba>,
}

impl Escuela {
    /// Crea una nueva Escuela con validación de campos.
    ///
    /// # Errores
    /// Propaga errores de validación de [`UnidadBase::nuevo`].
    pub fn nueva(
        nombre: &str,
        codigo: &str,
        fecha_creacion: NaiveDate,
        director: &str,
    ) -> EstatutoResult<Self> {
        let base = UnidadBase::nuevo(nombre, codigo, fecha_creacion, director)?;
        Ok(Escuela {
            base,
            docentes: Vec::new(),
            cabas: Vec::new(),
        })
    }

    /// Agrega un docente a la escuela si su identificación no está duplicada.
    ///
    /// # Errores
    /// Retorna [`EstatutoError::Duplicada`] si ya existe un docente con la misma identificación.
    pub fn agregar_docente(&mut self, docente: Docente) -> EstatutoResult<()> {
        if self.buscar_docente(&docente.base.identificacion).is_some() {
            return Err(EstatutoError::Duplicada(format!(
                "Ya existe un docente con identificación '{}' en la escuela '{}'",
                docente.base.identificacion, self.base.codigo
            )));
        }
        self.docentes.push(docente);
        Ok(())
    }

    /// Retira (elimina) un docente de la escuela por su identificación.
    ///
    /// # Retorna
    /// `true` si fue encontrado y eliminado, `false` si no existía.
    pub fn retirar_docente(&mut self, identificacion: &str) -> bool {
        let id = identificacion.trim();
        let len_antes = self.docentes.len();
        self.docentes.retain(|d| d.base.identificacion != id);
        self.docentes.len() < len_antes
    }

    /// Busca un docente por su número de identificación.
    ///
    /// Equivale a `Optional<Docente> buscarDocente(String id)` en Java.
    pub fn buscar_docente(&self, identificacion: &str) -> Option<&Docente> {
        let id = identificacion.trim();
        self.docentes.iter().find(|d| d.base.identificacion == id)
    }

    /// Agrega una CABA a la escuela.
    pub fn agregar_caba(&mut self, caba: Caba) -> EstatutoResult<()> {
        if self.cabas.iter().any(|c| c.codigo == caba.codigo) {
            return Err(EstatutoError::Duplicada(format!(
                "Ya existe una CABA con código '{}'",
                caba.codigo
            )));
        }
        self.cabas.push(caba);
        Ok(())
    }
}

impl UnidadAcademica for Escuela {
    fn tipo(&self) -> &str {
        "Escuela"
    }

    fn base(&self) -> &UnidadBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut UnidadBase {
        &mut self.base
    }
}

impl fmt::Display for Escuela {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Escuela [{}] {} — Director: {} | Docentes: {} | CABA: {}",
            self.base.codigo,
            self.base.nombre,
            self.base.director,
            self.docentes.len(),
            self.cabas.len()
        )
    }
}

// ============================================================
//  Pruebas unitarias
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modelo::docente::Docente;
    use crate::modelo::enums::TipoVinculacion;
    use chrono::NaiveDate;

    fn escuela_prueba() -> Escuela {
        Escuela::nueva(
            "Escuela de Sistemas",
            "ESC-SIS",
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            "Dr. Torres",
        )
        .unwrap()
    }

    fn docente_prueba(id: &str) -> Docente {
        Docente::nuevo("Pedro López", id, None, TipoVinculacion::Planta, "ESC-SIS", "Asociado")
    }

    #[test]
    fn escuela_crea_correctamente() {
        let e = escuela_prueba();
        assert_eq!(e.base.codigo, "ESC-SIS");
        assert!(e.docentes.is_empty());
    }

    #[test]
    fn agregar_docente_ok() {
        let mut e = escuela_prueba();
        assert!(e.agregar_docente(docente_prueba("CC-1001")).is_ok());
        assert_eq!(e.docentes.len(), 1);
    }

    #[test]
    fn agregar_docente_duplicado_error() {
        let mut e = escuela_prueba();
        e.agregar_docente(docente_prueba("CC-1001")).unwrap();
        assert!(e.agregar_docente(docente_prueba("CC-1001")).is_err());
    }

    #[test]
    fn buscar_docente_existente() {
        let mut e = escuela_prueba();
        e.agregar_docente(docente_prueba("CC-1001")).unwrap();
        assert!(e.buscar_docente("CC-1001").is_some());
    }

    #[test]
    fn buscar_docente_inexistente() {
        let e = escuela_prueba();
        assert!(e.buscar_docente("CC-9999").is_none());
    }

    #[test]
    fn retirar_docente_ok() {
        let mut e = escuela_prueba();
        e.agregar_docente(docente_prueba("CC-1001")).unwrap();
        assert!(e.retirar_docente("CC-1001"));
        assert!(e.docentes.is_empty());
    }

    #[test]
    fn tipo_retorna_escuela() {
        assert_eq!(escuela_prueba().tipo(), "Escuela");
    }
}
