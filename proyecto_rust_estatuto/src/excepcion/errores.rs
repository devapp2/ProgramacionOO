//! # Tipos de error del sistema de estatuto
//!
//! Define los errores de dominio utilizando el crate [`thiserror`],
//! que genera implementaciones del trait `std::error::Error` automáticamente.
//!
//! ## Equivalencia Java → Rust
//! - `EstatutoException` → `EstatutoError` (enum con variantes)
//! - `MatriculaException` → variante `EstatutoError::Matricula`
//!
//! En Rust, los errores se propagan con `?` y se representan como valores
//! del tipo `Result<T, EstatutoError>`, en lugar de las excepciones chequeadas
//! de Java.

use thiserror::Error;

/// Error principal del dominio académico-administrativo.
///
/// Cada variante corresponde a una categoría de fallo específica dentro
/// del sistema de información de la Universidad Distrital,
/// conforme al Acuerdo 004 de 2025.
#[derive(Debug, Error)]
pub enum EstatutoError {
    /// Error producido por datos inválidos o incompletos en las entidades.
    ///
    /// # Ejemplo
    /// ```
    /// use estatuto_ud::excepcion::errores::EstatutoError;
    /// let e = EstatutoError::Validacion("El código no puede estar vacío".to_string());
    /// ```
    #[error("Error de validación: {0}")]
    Validacion(String),

    /// Error relacionado con el proceso de matrícula de un estudiante.
    ///
    /// Equivale a `MatriculaException` en el proyecto Java.
    #[error("Error de matrícula: {0}")]
    Matricula(String),

    /// La entidad solicitada no fue encontrada en el repositorio.
    #[error("Entidad no encontrada: {0}")]
    NoEncontrada(String),

    /// Intento de insertar una entidad que ya existe (clave duplicada).
    #[error("Entidad duplicada: {0}")]
    Duplicada(String),

    /// Error de entrada/salida al leer o escribir archivos JSON.
    ///
    /// Se convierte automáticamente desde `std::io::Error` usando `#[from]`.
    #[error("Error de persistencia: {0}")]
    Persistencia(#[from] std::io::Error),

    /// Error al serializar o deserializar JSON con `serde_json`.
    ///
    /// Se convierte automáticamente desde `serde_json::Error` usando `#[from]`.
    #[error("Error de serialización: {0}")]
    Serializacion(#[from] serde_json::Error),
}

/// Alias de conveniencia para `Result<T, EstatutoError>`.
pub type EstatutoResult<T> = Result<T, EstatutoError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_validacion_mensaje() {
        let e = EstatutoError::Validacion("campo vacío".to_string());
        assert_eq!(e.to_string(), "Error de validación: campo vacío");
    }

    #[test]
    fn error_matricula_mensaje() {
        let e = EstatutoError::Matricula("semestre inválido".to_string());
        assert_eq!(e.to_string(), "Error de matrícula: semestre inválido");
    }

    #[test]
    fn error_no_encontrada() {
        let e = EstatutoError::NoEncontrada("FAC001".to_string());
        assert!(e.to_string().contains("FAC001"));
    }

    #[test]
    fn error_duplicada() {
        let e = EstatutoError::Duplicada("DOC-42".to_string());
        assert!(e.to_string().contains("DOC-42"));
    }

    #[test]
    fn estatuto_result_ok() {
        let r: EstatutoResult<i32> = Ok(42);
        assert_eq!(r.unwrap(), 42);
    }
}
