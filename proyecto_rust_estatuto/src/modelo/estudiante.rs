//! # Estudiante — miembro estudiantil de la comunidad universitaria
//!
//! El Estudiante es el actor principal del proceso de formación en la
//! Universidad Distrital Francisco José de Caldas.
//!
//! **Conforme al Artículo 92 del Acuerdo 004 de 2025**, los estudiantes
//! tienen derechos y deberes específicos dentro de la institución.

use crate::excepcion::errores::{EstatutoError, EstatutoResult};
use crate::modelo::enums::EstadoEstudiante;
use crate::modelo::persona::{Persona, PersonaBase};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Miembro estudiantil de la Universidad Distrital Francisco José de Caldas.
///
/// Equivale a la clase `Estudiante extends Persona` del proyecto Java.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Estudiante {
    /// Datos comunes de la persona.
    pub base: PersonaBase,

    /// Código estudiantil único asignado al momento de la admisión.
    pub codigo_estudiantil: String,

    /// Código del programa académico al que está inscrito el estudiante.
    pub programa_codigo: String,

    /// Número de semestre actual del estudiante (debe ser > 0).
    pub semestre: u8,

    /// Estado académico actual del estudiante.
    pub estado: EstadoEstudiante,
}

impl Estudiante {
    /// Crea un nuevo Estudiante con validación del semestre.
    ///
    /// # Errores
    /// Retorna [`EstatutoError::Matricula`] si el semestre es 0.
    ///
    /// # Ejemplo
    /// ```
    /// use estatuto_ud::modelo::estudiante::Estudiante;
    /// use estatuto_ud::modelo::enums::EstadoEstudiante;
    ///
    /// let e = Estudiante::nuevo(
    ///     "Sara Romero",
    ///     "CC-4001",
    ///     None,
    ///     "20221020",
    ///     "PROG-SIS",
    ///     3,
    ///     EstadoEstudiante::Activo,
    /// ).unwrap();
    /// assert!(e.esta_activo());
    /// ```
    pub fn nuevo(
        nombre: &str,
        identificacion: &str,
        correo: Option<String>,
        codigo_estudiantil: &str,
        programa_codigo: &str,
        semestre: u8,
        estado: EstadoEstudiante,
    ) -> EstatutoResult<Self> {
        if semestre == 0 {
            return Err(EstatutoError::Matricula(
                "El semestre debe ser mayor que cero".to_string(),
            ));
        }
        Ok(Estudiante {
            base: PersonaBase::nueva(nombre, identificacion, correo),
            codigo_estudiantil: codigo_estudiantil.trim().to_string(),
            programa_codigo: programa_codigo.trim().to_uppercase(),
            semestre,
            estado,
        })
    }

    /// Indica si el estudiante tiene matrícula vigente.
    ///
    /// Un estudiante está activo cuando su estado es [`EstadoEstudiante::Activo`].
    pub fn esta_activo(&self) -> bool {
        self.estado == EstadoEstudiante::Activo
    }

    /// Avanza al siguiente semestre si el estudiante está activo.
    ///
    /// # Errores
    /// Retorna [`EstatutoError::Matricula`] si el estudiante no está activo.
    pub fn avanzar_semestre(&mut self) -> EstatutoResult<()> {
        if !self.esta_activo() {
            return Err(EstatutoError::Matricula(format!(
                "El estudiante {} no está activo para avanzar de semestre",
                self.base.identificacion
            )));
        }
        self.semestre += 1;
        Ok(())
    }

    /// Cambia el estado del estudiante (transición de ciclo de vida).
    pub fn cambiar_estado(&mut self, nuevo_estado: EstadoEstudiante) {
        self.estado = nuevo_estado;
    }
}

impl Persona for Estudiante {
    fn rol(&self) -> &str {
        "Estudiante"
    }

    fn base(&self) -> &PersonaBase {
        &self.base
    }
}

impl fmt::Display for Estudiante {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Estudiante {} — Código: {} | Semestre: {} | Estado: {} | Programa: {}",
            self.base,
            self.codigo_estudiantil,
            self.semestre,
            self.estado,
            self.programa_codigo
        )
    }
}

// ============================================================
//  Pruebas unitarias
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modelo::enums::EstadoEstudiante;

    fn estudiante_activo() -> Estudiante {
        Estudiante::nuevo(
            "Sara Romero",
            "CC-4001",
            None,
            "20221020",
            "PROG-SIS",
            3,
            EstadoEstudiante::Activo,
        )
        .unwrap()
    }

    #[test]
    fn estudiante_crea_correctamente() {
        let e = estudiante_activo();
        assert_eq!(e.base.nombre, "Sara Romero");
        assert_eq!(e.semestre, 3);
    }

    #[test]
    fn semestre_cero_retorna_error() {
        let r = Estudiante::nuevo("X", "CC-0", None, "EST-0", "PROG", 0, EstadoEstudiante::Activo);
        assert!(r.is_err());
    }

    #[test]
    fn esta_activo_true() {
        assert!(estudiante_activo().esta_activo());
    }

    #[test]
    fn esta_activo_false_para_suspendido() {
        let mut e = estudiante_activo();
        e.cambiar_estado(EstadoEstudiante::Suspendido);
        assert!(!e.esta_activo());
    }

    #[test]
    fn avanzar_semestre_ok() {
        let mut e = estudiante_activo();
        assert!(e.avanzar_semestre().is_ok());
        assert_eq!(e.semestre, 4);
    }

    #[test]
    fn avanzar_semestre_suspendido_error() {
        let mut e = estudiante_activo();
        e.cambiar_estado(EstadoEstudiante::Suspendido);
        assert!(e.avanzar_semestre().is_err());
    }

    #[test]
    fn estudiante_rol() {
        assert_eq!(estudiante_activo().rol(), "Estudiante");
    }

    #[test]
    fn estudiante_serializa() {
        let e = estudiante_activo();
        let json = serde_json::to_string(&e).unwrap();
        let back: Estudiante = serde_json::from_str(&json).unwrap();
        assert_eq!(back.codigo_estudiantil, "20221020");
    }
}
