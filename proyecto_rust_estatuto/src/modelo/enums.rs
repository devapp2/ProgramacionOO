//! # Enumeraciones del dominio académico-administrativo
//!
//! Este módulo define los tipos enumerados utilizados en el sistema de información
//! de la Universidad Distrital Francisco José de Caldas, conforme al
//! **Acuerdo 004 de 2025 — Estatuto General**.
//!
//! ## Equivalencia Java → Rust
//! Los enums de Java con campo `descripcion` se traducen como enums de Rust
//! con implementación del trait `Display`.

use serde::{Deserialize, Serialize};
use std::fmt;

// ============================================================
//  TipoVinculacion
// ============================================================

/// Tipo de vinculación laboral del docente con la Universidad Distrital.
///
/// Conforme al Artículo 68 del Acuerdo 004 de 2025, los docentes pueden
/// vincularse bajo distintas modalidades según el régimen de dedicación
/// y la naturaleza del cargo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TipoVinculacion {
    /// Docente de carrera con nombramiento definitivo en la planta de personal.
    Planta,
    /// Docente vinculado de forma temporal por necesidades del servicio.
    Ocasional,
    /// Docente vinculado por horas de clase sin dedicación exclusiva.
    HoraCatedra,
    /// Docente proveniente de otra institución invitado a colaborar.
    Visitante,
    /// Profesional externo con conocimiento especializado sin título docente.
    Experto,
}

impl fmt::Display for TipoVinculacion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let descripcion = match self {
            TipoVinculacion::Planta => "Docente de Planta",
            TipoVinculacion::Ocasional => "Docente Ocasional",
            TipoVinculacion::HoraCatedra => "Docente de Hora Cátedra",
            TipoVinculacion::Visitante => "Docente Visitante",
            TipoVinculacion::Experto => "Experto Temático",
        };
        write!(f, "{}", descripcion)
    }
}

// ============================================================
//  EstadoEstudiante
// ============================================================

/// Estado académico del estudiante dentro del ciclo de vida universitario.
///
/// Conforme al Artículo 92 del Acuerdo 004 de 2025, el estudiante pasa por
/// distintos estados desde su aspiración hasta la graduación o retiro.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EstadoEstudiante {
    /// Ha presentado solicitud de admisión pero no ha sido admitido aún.
    Aspirante,
    /// Ha sido admitido mediante proceso de selección y espera matricularse.
    Admitido,
    /// Estudiante con matrícula vigente participando activamente.
    Activo,
    /// Estudiante con matrícula suspendida por causas académicas o disciplinarias.
    Suspendido,
    /// Ha culminado exitosamente el plan de estudios y recibió su título.
    Graduado,
    /// Se desvinculó voluntaria o involuntariamente de la institución.
    Retirado,
}

impl fmt::Display for EstadoEstudiante {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let descripcion = match self {
            EstadoEstudiante::Aspirante => "Aspirante",
            EstadoEstudiante::Admitido => "Admitido",
            EstadoEstudiante::Activo => "Estudiante Activo",
            EstadoEstudiante::Suspendido => "Estudiante Suspendido",
            EstadoEstudiante::Graduado => "Graduado",
            EstadoEstudiante::Retirado => "Retirado",
        };
        write!(f, "{}", descripcion)
    }
}

// ============================================================
//  NivelPrograma
// ============================================================

/// Nivel de formación académica de un programa curricular.
///
/// Conforme al Artículo 54 del Acuerdo 004 de 2025, la Universidad Distrital
/// ofrece programas en distintos niveles de educación superior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NivelPrograma {
    /// Formación de educación superior conducente a título profesional.
    Pregrado,
    /// Posgrado de profundización en un área específica del conocimiento.
    Especializacion,
    /// Posgrado de investigación o profundización con tesis o proyecto.
    Maestria,
    /// Máximo nivel académico conducente al título de Doctor.
    Doctorado,
}

impl fmt::Display for NivelPrograma {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let descripcion = match self {
            NivelPrograma::Pregrado => "Pregrado",
            NivelPrograma::Especializacion => "Especialización",
            NivelPrograma::Maestria => "Maestría",
            NivelPrograma::Doctorado => "Doctorado",
        };
        write!(f, "{}", descripcion)
    }
}

// ============================================================
//  TipoUnidad (usado por el Factory)
// ============================================================

/// Tipo de unidad académica para el patrón Factory.
///
/// Conforme al Artículo 8 del Acuerdo 004 de 2025, la estructura académica
/// comprende facultades, escuelas, centros e institutos.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TipoUnidad {
    /// Unidad académica principal que agrupa escuelas y programas.
    Facultad,
    /// Subdivisión de la facultad que gestiona un área disciplinar.
    Escuela,
    /// Unidad de investigación y extensión temática.
    Centro,
    /// Organismo especializado de carácter académico-científico.
    Instituto,
}

impl fmt::Display for TipoUnidad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nombre = match self {
            TipoUnidad::Facultad => "Facultad",
            TipoUnidad::Escuela => "Escuela",
            TipoUnidad::Centro => "Centro",
            TipoUnidad::Instituto => "Instituto",
        };
        write!(f, "{}", nombre)
    }
}

// ============================================================
//  Pruebas unitarias
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tipo_vinculacion_display_planta() {
        assert_eq!(TipoVinculacion::Planta.to_string(), "Docente de Planta");
    }

    #[test]
    fn tipo_vinculacion_display_hora_catedra() {
        assert_eq!(
            TipoVinculacion::HoraCatedra.to_string(),
            "Docente de Hora Cátedra"
        );
    }

    #[test]
    fn estado_estudiante_display_activo() {
        assert_eq!(EstadoEstudiante::Activo.to_string(), "Estudiante Activo");
    }

    #[test]
    fn estado_estudiante_display_graduado() {
        assert_eq!(EstadoEstudiante::Graduado.to_string(), "Graduado");
    }

    #[test]
    fn nivel_programa_display_maestria() {
        assert_eq!(NivelPrograma::Maestria.to_string(), "Maestría");
    }

    #[test]
    fn tipo_unidad_display_facultad() {
        assert_eq!(TipoUnidad::Facultad.to_string(), "Facultad");
    }

    #[test]
    fn enums_serialize_deserialize() {
        let tv = TipoVinculacion::Ocasional;
        let json = serde_json::to_string(&tv).unwrap();
        let back: TipoVinculacion = serde_json::from_str(&json).unwrap();
        assert_eq!(tv, back);
    }
}
