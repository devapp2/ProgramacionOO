//! # Trait `UnidadAcademica` y struct `UnidadBase`
//!
//! Define la abstracción base para todas las unidades académico-administrativas
//! de la Universidad Distrital Francisco José de Caldas, conforme al
//! **Artículo 8 del Acuerdo 004 de 2025**.
//!
//! ## Equivalencia Java → Rust
//! La clase abstracta `UnidadAcademica` de Java se convierte en:
//! - `trait UnidadAcademica`: define la interfaz polimórfica.
//! - `struct UnidadBase`: contiene los campos comunes (composición).
//!
//! Las estructuras concretas (`Facultad`, `Escuela`, etc.) contienen un campo
//! `base: UnidadBase` y deben implementar el trait `UnidadAcademica`.

use crate::excepcion::errores::EstatutoResult;
use crate::modelo::validador;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt;

// ============================================================
//  UnidadBase — campos compartidos por todas las unidades
// ============================================================

/// Estructura con los campos comunes a todas las unidades académicas.
///
/// Equivale a los campos de la clase abstracta `UnidadAcademica` en Java.
/// Se usa mediante composición: cada unidad concreta tiene un campo `base`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnidadBase {
    /// Nombre oficial de la unidad académica.
    pub nombre: String,

    /// Código único identificador (normalizado en mayúsculas, sin espacios).
    ///
    /// Conforme al Acuerdo 004 de 2025, cada unidad tiene un código institucional
    /// único que sirve como clave de negocio.
    pub codigo: String,

    /// Fecha en que la unidad fue creada o reconocida formalmente.
    pub fecha_creacion: NaiveDate,

    /// Nombre del director o decano actual de la unidad.
    pub director: String,
}

impl UnidadBase {
    /// Crea una nueva `UnidadBase` con validación del código.
    ///
    /// El código es normalizado (mayúsculas, sin espacios extremos) antes de
    /// almacenarse, lo que equivale al comportamiento del constructor Java.
    ///
    /// # Errores
    /// Retorna [`EstatutoError::Validacion`] si:
    /// - `codigo` está vacío o solo contiene espacios.
    /// - `nombre` está vacío.
    ///
    /// # Ejemplo
    /// ```
    /// use chrono::NaiveDate;
    /// use estatuto_ud::modelo::unidad_academica::UnidadBase;
    ///
    /// let fecha = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    /// let base = UnidadBase::nuevo("Facultad de Ingeniería", "FAC-ING", fecha, "Dr. Pérez").unwrap();
    /// assert_eq!(base.codigo, "FAC-ING");
    /// ```
    pub fn nuevo(
        nombre: &str,
        codigo: &str,
        fecha_creacion: NaiveDate,
        director: &str,
    ) -> EstatutoResult<Self> {
        validador::no_vacio(nombre, "nombre")?;
        validador::no_vacio(codigo, "codigo")?;
        validador::no_vacio(director, "director")?;

        Ok(UnidadBase {
            nombre: nombre.trim().to_string(),
            codigo: validador::normalizar_codigo(codigo),
            fecha_creacion,
            director: director.trim().to_string(),
        })
    }
}

impl fmt::Display for UnidadBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} — Director: {} (desde {})",
            self.codigo, self.nombre, self.director, self.fecha_creacion
        )
    }
}

// ============================================================
//  Trait UnidadAcademica — interfaz polimórfica
// ============================================================

/// Trait que define la interfaz de las unidades académico-administrativas.
///
/// Equivale a la clase abstracta `UnidadAcademica` de Java (con sus métodos
/// abstractos `getTipo()` y `getNombre()`).
///
/// ## Implementaciones
/// - [`crate::modelo::facultad::Facultad`]
/// - [`crate::modelo::escuela::Escuela`]
/// - [`crate::modelo::centro::Centro`]
/// - [`crate::modelo::instituto::Instituto`]
pub trait UnidadAcademica: fmt::Display {
    /// Retorna el tipo de unidad como cadena descriptiva.
    ///
    /// Equivale al método abstracto `getTipo()` de Java.
    fn tipo(&self) -> &str;

    /// Retorna una referencia inmutable al `UnidadBase` subyacente.
    fn base(&self) -> &UnidadBase;

    /// Retorna una referencia mutable al `UnidadBase` subyacente.
    fn base_mut(&mut self) -> &mut UnidadBase;

    /// Retorna el código único de la unidad (delegado a `UnidadBase`).
    fn codigo(&self) -> &str {
        &self.base().codigo
    }

    /// Retorna el nombre de la unidad (delegado a `UnidadBase`).
    fn nombre(&self) -> &str {
        &self.base().nombre
    }
}

// ============================================================
//  Pruebas unitarias
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn fecha_prueba() -> NaiveDate {
        NaiveDate::from_ymd_opt(2025, 3, 1).unwrap()
    }

    #[test]
    fn unidad_base_crea_correctamente() {
        let base = UnidadBase::nuevo("Facultad de Ingeniería", "FAC-ING", fecha_prueba(), "Dr. Pérez");
        assert!(base.is_ok());
        let base = base.unwrap();
        assert_eq!(base.codigo, "FAC-ING");
        assert_eq!(base.nombre, "Facultad de Ingeniería");
    }

    #[test]
    fn unidad_base_normaliza_codigo() {
        let base = UnidadBase::nuevo("Escuela", "  esc-01  ", fecha_prueba(), "Dir").unwrap();
        assert_eq!(base.codigo, "ESC-01");
    }

    #[test]
    fn unidad_base_rechaza_codigo_vacio() {
        let resultado = UnidadBase::nuevo("Facultad", "", fecha_prueba(), "Dir");
        assert!(resultado.is_err());
    }

    #[test]
    fn unidad_base_rechaza_nombre_vacio() {
        let resultado = UnidadBase::nuevo("", "FAC-001", fecha_prueba(), "Dir");
        assert!(resultado.is_err());
    }

    #[test]
    fn unidad_base_display() {
        let base = UnidadBase::nuevo("Instituto", "INST-01", fecha_prueba(), "Dr. Gómez").unwrap();
        let texto = base.to_string();
        assert!(texto.contains("INST-01"));
        assert!(texto.contains("Instituto"));
    }

    #[test]
    fn unidad_base_serializa() {
        let base = UnidadBase::nuevo("Escuela", "ESC-01", fecha_prueba(), "Dir").unwrap();
        let json = serde_json::to_string(&base).unwrap();
        let back: UnidadBase = serde_json::from_str(&json).unwrap();
        assert_eq!(back.codigo, "ESC-01");
    }
}
