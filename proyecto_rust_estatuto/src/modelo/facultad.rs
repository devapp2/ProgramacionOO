//! # Facultad — unidad académica principal
//!
//! La `Facultad` es la unidad académico-administrativa más importante de la
//! Universidad Distrital Francisco José de Caldas. Agrupa escuelas, programas
//! y la gestión curricular de un campo disciplinar.
//!
//! **Conforme al Artículo 16 del Acuerdo 004 de 2025**, cada facultad está
//! dirigida por un Decano elegido por el Consejo de Facultad.

use crate::excepcion::errores::{EstatutoError, EstatutoResult};
use crate::modelo::escuela::Escuela;
use crate::modelo::unidad_academica::{UnidadAcademica, UnidadBase};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Unidad académica principal que agrupa escuelas y programas curriculares.
///
/// Equivale a la clase `Facultad extends UnidadAcademica` del proyecto Java.
/// En Rust, la herencia se reemplaza por composición: `base: UnidadBase`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Facultad {
    /// Datos comunes de la unidad académica (nombre, código, etc.).
    pub base: UnidadBase,

    /// Nombre del Decano actual de la Facultad.
    ///
    /// El Decano preside el Consejo de Facultad y representa la unidad
    /// ante el Consejo Académico, conforme al Artículo 17 del Acuerdo 004 de 2025.
    pub decano: String,

    /// Escuelas que pertenecen a esta Facultad.
    pub escuelas: Vec<Escuela>,
}

impl Facultad {
    /// Crea una nueva Facultad con validación de campos.
    ///
    /// # Argumentos
    /// - `nombre`: Nombre oficial de la Facultad.
    /// - `codigo`: Código institucional único (se normaliza a mayúsculas).
    /// - `fecha_creacion`: Fecha de creación o reconocimiento formal.
    /// - `director`: Nombre del director/decano asignado inicialmente.
    /// - `decano`: Nombre del Decano actual (puede coincidir con `director`).
    ///
    /// # Errores
    /// Propaga errores de validación de [`UnidadBase::nuevo`].
    ///
    /// # Ejemplo
    /// ```
    /// use chrono::NaiveDate;
    /// use estatuto_ud::modelo::facultad::Facultad;
    ///
    /// let f = Facultad::nueva(
    ///     "Facultad de Ingeniería",
    ///     "FAC-ING",
    ///     NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
    ///     "Dr. Pérez",
    ///     "Dra. López",
    /// ).unwrap();
    /// assert_eq!(f.decano, "Dra. López");
    /// ```
    pub fn nueva(
        nombre: &str,
        codigo: &str,
        fecha_creacion: NaiveDate,
        director: &str,
        decano: &str,
    ) -> EstatutoResult<Self> {
        let base = UnidadBase::nuevo(nombre, codigo, fecha_creacion, director)?;
        Ok(Facultad {
            base,
            decano: decano.trim().to_string(),
            escuelas: Vec::new(),
        })
    }

    /// Agrega una escuela a la facultad.
    ///
    /// Verifica que no exista una escuela con el mismo código antes de agregar
    /// (equivale al comportamiento con `List.contains` de Java).
    ///
    /// # Errores
    /// Retorna [`EstatutoError::Duplicada`] si ya existe una escuela con ese código.
    pub fn agregar_escuela(&mut self, escuela: Escuela) -> EstatutoResult<()> {
        if self.buscar_escuela(&escuela.base.codigo).is_some() {
            return Err(EstatutoError::Duplicada(format!(
                "Ya existe una escuela con código '{}' en la facultad '{}'",
                escuela.base.codigo, self.base.codigo
            )));
        }
        self.escuelas.push(escuela);
        Ok(())
    }

    /// Elimina una escuela por su código.
    ///
    /// # Retorna
    /// `true` si la escuela fue encontrada y eliminada, `false` si no existía.
    pub fn eliminar_escuela(&mut self, codigo: &str) -> bool {
        let codigo_norm = codigo.trim().to_uppercase();
        let len_antes = self.escuelas.len();
        self.escuelas.retain(|e| e.base.codigo != codigo_norm);
        self.escuelas.len() < len_antes
    }

    /// Busca una escuela por su código.
    ///
    /// # Retorna
    /// `Some(&Escuela)` si existe, `None` si no.
    ///
    /// Equivale a `Optional<Escuela> buscarEscuela(String codigo)` en Java.
    pub fn buscar_escuela(&self, codigo: &str) -> Option<&Escuela> {
        let codigo_norm = codigo.trim().to_uppercase();
        self.escuelas.iter().find(|e| e.base.codigo == codigo_norm)
    }

    /// Busca una escuela por su código y retorna referencia mutable.
    pub fn buscar_escuela_mut(&mut self, codigo: &str) -> Option<&mut Escuela> {
        let codigo_norm = codigo.trim().to_uppercase();
        self.escuelas.iter_mut().find(|e| e.base.codigo == codigo_norm)
    }

    /// Retorna el total de docentes en todas las escuelas de la facultad.
    pub fn total_docentes(&self) -> usize {
        self.escuelas.iter().map(|e| e.docentes.len()).sum()
    }
}

impl UnidadAcademica for Facultad {
    fn tipo(&self) -> &str {
        "Facultad"
    }

    fn base(&self) -> &UnidadBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut UnidadBase {
        &mut self.base
    }
}

impl fmt::Display for Facultad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Facultad [{}] {} — Decano: {} | Escuelas: {}",
            self.base.codigo,
            self.base.nombre,
            self.decano,
            self.escuelas.len()
        )
    }
}

// ============================================================
//  Pruebas unitarias
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modelo::escuela::Escuela;
    use chrono::NaiveDate;

    fn facultad_prueba() -> Facultad {
        Facultad::nueva(
            "Facultad de Ingeniería",
            "FAC-ING",
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            "Dr. Pérez",
            "Dra. López",
        )
        .unwrap()
    }

    fn escuela_prueba(codigo: &str) -> Escuela {
        Escuela::nueva(
            "Escuela de Prueba",
            codigo,
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            "Dir",
        )
        .unwrap()
    }

    #[test]
    fn facultad_crea_correctamente() {
        let f = facultad_prueba();
        assert_eq!(f.base.codigo, "FAC-ING");
        assert_eq!(f.decano, "Dra. López");
        assert!(f.escuelas.is_empty());
    }

    #[test]
    fn agregar_escuela_ok() {
        let mut f = facultad_prueba();
        let e = escuela_prueba("ESC-01");
        assert!(f.agregar_escuela(e).is_ok());
        assert_eq!(f.escuelas.len(), 1);
    }

    #[test]
    fn agregar_escuela_duplicada_error() {
        let mut f = facultad_prueba();
        f.agregar_escuela(escuela_prueba("ESC-01")).unwrap();
        let resultado = f.agregar_escuela(escuela_prueba("ESC-01"));
        assert!(resultado.is_err());
    }

    #[test]
    fn buscar_escuela_existente() {
        let mut f = facultad_prueba();
        f.agregar_escuela(escuela_prueba("ESC-01")).unwrap();
        assert!(f.buscar_escuela("ESC-01").is_some());
    }

    #[test]
    fn buscar_escuela_inexistente() {
        let f = facultad_prueba();
        assert!(f.buscar_escuela("ESC-99").is_none());
    }

    #[test]
    fn eliminar_escuela_ok() {
        let mut f = facultad_prueba();
        f.agregar_escuela(escuela_prueba("ESC-01")).unwrap();
        assert!(f.eliminar_escuela("ESC-01"));
        assert!(f.escuelas.is_empty());
    }

    #[test]
    fn tipo_retorna_facultad() {
        let f = facultad_prueba();
        assert_eq!(f.tipo(), "Facultad");
    }
}
