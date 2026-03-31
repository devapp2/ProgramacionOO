//! # Trait `Persona` y struct `PersonaBase`
//!
//! Define la abstracción base para todos los actores del sistema universitario:
//! docentes, estudiantes, personal administrativo y egresados.
//!
//! **Conforme al Artículo 88 del Acuerdo 004 de 2025**, la comunidad universitaria
//! está integrada por docentes, estudiantes, egresados y personal administrativo.
//!
//! ## Equivalencia Java → Rust
//! - Clase abstracta `Persona` → trait `Persona` + struct `PersonaBase`
//! - `equals(Object)` basado en `identificacion` → comparación directa de campos

use serde::{Deserialize, Serialize};
use std::fmt;

// ============================================================
//  PersonaBase — campos comunes a todos los actores
// ============================================================

/// Estructura con los campos comunes a todos los miembros de la comunidad universitaria.
///
/// Equivale a los campos de la clase abstracta `Persona` en Java.
/// Se utiliza mediante composición: cada tipo concreto tiene un campo `base: PersonaBase`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaBase {
    /// Nombre completo de la persona.
    pub nombre: String,

    /// Número único de identificación (cédula, pasaporte, etc.).
    ///
    /// Sirve como clave de negocio: dos personas son iguales si
    /// tienen la misma `identificacion` (equivale a `equals` de Java).
    pub identificacion: String,

    /// Correo electrónico institucional o personal (opcional).
    ///
    /// Equivale a `Optional<String>` en Java, representado como `Option<String>` en Rust.
    pub correo: Option<String>,
}

impl PersonaBase {
    /// Crea una nueva `PersonaBase`.
    ///
    /// # Ejemplo
    /// ```
    /// use estatuto_ud::modelo::persona::PersonaBase;
    /// let p = PersonaBase::nueva("Ana García", "CC-12345", Some("ana@udistrital.edu.co".to_string()));
    /// assert_eq!(p.identificacion, "CC-12345");
    /// ```
    pub fn nueva(
        nombre: &str,
        identificacion: &str,
        correo: Option<String>,
    ) -> Self {
        PersonaBase {
            nombre: nombre.trim().to_string(),
            identificacion: identificacion.trim().to_string(),
            correo,
        }
    }
}

impl fmt::Display for PersonaBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.correo {
            Some(c) => write!(f, "{} [{}] <{}>", self.nombre, self.identificacion, c),
            None => write!(f, "{} [{}]", self.nombre, self.identificacion),
        }
    }
}

// ============================================================
//  Trait Persona — interfaz polimórfica
// ============================================================

/// Trait que define la interfaz de todos los miembros de la comunidad universitaria.
///
/// Equivale a la clase abstracta `Persona` de Java.
///
/// ## Implementaciones
/// - [`crate::modelo::docente::Docente`]
/// - [`crate::modelo::estudiante::Estudiante`]
/// - [`crate::modelo::personal_administrativo::PersonalAdministrativo`]
/// - [`crate::modelo::egresado::Egresado`]
pub trait Persona: fmt::Display {
    /// Retorna el rol de la persona en la institución.
    ///
    /// Equivale al método abstracto `getRol()` de Java.
    fn rol(&self) -> &str;

    /// Retorna referencia al `PersonaBase` subyacente.
    fn base(&self) -> &PersonaBase;

    /// Retorna el nombre de la persona (delegado a `PersonaBase`).
    fn nombre(&self) -> &str {
        &self.base().nombre
    }

    /// Retorna la identificación de la persona (delegado a `PersonaBase`).
    fn identificacion(&self) -> &str {
        &self.base().identificacion
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persona_base_crea_correctamente() {
        let p = PersonaBase::nueva("Luis Martínez", "CC-9876", None);
        assert_eq!(p.nombre, "Luis Martínez");
        assert_eq!(p.identificacion, "CC-9876");
        assert!(p.correo.is_none());
    }

    #[test]
    fn persona_base_con_correo() {
        let p = PersonaBase::nueva("María López", "CC-1111", Some("mlopez@ud.edu.co".to_string()));
        assert!(p.correo.is_some());
    }

    #[test]
    fn persona_base_display_sin_correo() {
        let p = PersonaBase::nueva("Juan Pérez", "CC-5555", None);
        assert_eq!(p.to_string(), "Juan Pérez [CC-5555]");
    }

    #[test]
    fn persona_base_display_con_correo() {
        let p = PersonaBase::nueva("Ana García", "CC-1234", Some("a@ud.edu.co".to_string()));
        let s = p.to_string();
        assert!(s.contains("a@ud.edu.co"));
    }

    #[test]
    fn persona_base_serializa() {
        let p = PersonaBase::nueva("Carlos Ruiz", "CC-6789", None);
        let json = serde_json::to_string(&p).unwrap();
        let back: PersonaBase = serde_json::from_str(&json).unwrap();
        assert_eq!(back.identificacion, "CC-6789");
    }
}
