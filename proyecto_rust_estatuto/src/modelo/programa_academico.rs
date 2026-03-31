//! # ProgramaAcademico — oferta formativa de la Universidad
//!
//! El Programa Académico es la unidad curricular que estructura un proceso
//! formativo conducente a título en la Universidad Distrital Francisco José de Caldas.
//!
//! **Conforme al Artículo 54 del Acuerdo 004 de 2025**, los programas académicos
//! deben estar registrados ante el MEN y aprobados por el Consejo Académico.

use crate::modelo::enums::NivelPrograma;
use crate::modelo::plan_de_estudios::PlanDeEstudios;
use crate::modelo::proposito_formacion::PropositoFormacion;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Programa curricular de pregrado o posgrado de la Universidad Distrital.
///
/// Equivale a la clase `ProgramaAcademico` del proyecto Java.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramaAcademico {
    /// Nombre oficial del programa académico.
    pub nombre: String,

    /// Código institucional del programa (ej: "2879" para Ingeniería de Sistemas).
    pub codigo: String,

    /// Nivel de formación del programa.
    pub nivel: NivelPrograma,

    /// Modalidad del programa: "Presencial", "Distancia", "Virtual", etc.
    pub modalidad: String,

    /// Código de la Facultad que oferta el programa.
    pub facultad_codigo: String,

    /// Plan de estudios vigente del programa.
    pub plan_de_estudios: Option<PlanDeEstudios>,

    /// Propósitos de formación declarados por el programa.
    pub propositos_formacion: Vec<PropositoFormacion>,
}

impl ProgramaAcademico {
    /// Crea un nuevo Programa Académico.
    ///
    /// # Ejemplo
    /// ```
    /// use estatuto_ud::modelo::programa_academico::ProgramaAcademico;
    /// use estatuto_ud::modelo::enums::NivelPrograma;
    ///
    /// let p = ProgramaAcademico::nuevo(
    ///     "Ingeniería de Sistemas",
    ///     "2879",
    ///     NivelPrograma::Pregrado,
    ///     "Presencial",
    ///     "FAC-ING",
    /// );
    /// assert_eq!(p.nivel, NivelPrograma::Pregrado);
    /// ```
    pub fn nuevo(
        nombre: &str,
        codigo: &str,
        nivel: NivelPrograma,
        modalidad: &str,
        facultad_codigo: &str,
    ) -> Self {
        ProgramaAcademico {
            nombre: nombre.trim().to_string(),
            codigo: codigo.trim().to_uppercase(),
            nivel,
            modalidad: modalidad.trim().to_string(),
            facultad_codigo: facultad_codigo.trim().to_uppercase(),
            plan_de_estudios: None,
            propositos_formacion: Vec::new(),
        }
    }

    /// Asigna o reemplaza el plan de estudios del programa.
    pub fn asignar_plan(&mut self, plan: PlanDeEstudios) {
        self.plan_de_estudios = Some(plan);
    }

    /// Agrega un propósito de formación al programa.
    pub fn agregar_proposito(&mut self, proposito: PropositoFormacion) {
        self.propositos_formacion.push(proposito);
    }

    /// Retorna true si el programa es de posgrado.
    pub fn es_posgrado(&self) -> bool {
        matches!(
            self.nivel,
            NivelPrograma::Especializacion | NivelPrograma::Maestria | NivelPrograma::Doctorado
        )
    }
}

impl fmt::Display for ProgramaAcademico {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Programa [{}] {} — {} | {} | Facultad: {}",
            self.codigo, self.nombre, self.nivel, self.modalidad, self.facultad_codigo
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modelo::enums::NivelPrograma;

    fn programa_prueba() -> ProgramaAcademico {
        ProgramaAcademico::nuevo(
            "Ingeniería de Sistemas",
            "2879",
            NivelPrograma::Pregrado,
            "Presencial",
            "FAC-ING",
        )
    }

    #[test]
    fn programa_crea_correctamente() {
        let p = programa_prueba();
        assert_eq!(p.codigo, "2879");
        assert_eq!(p.nivel, NivelPrograma::Pregrado);
    }

    #[test]
    fn es_posgrado_false_para_pregrado() {
        assert!(!programa_prueba().es_posgrado());
    }

    #[test]
    fn es_posgrado_true_para_maestria() {
        let p = ProgramaAcademico::nuevo("Maestría en TIC", "MTIC-01", NivelPrograma::Maestria, "Virtual", "FAC-ING");
        assert!(p.es_posgrado());
    }

    #[test]
    fn asignar_plan_ok() {
        let mut p = programa_prueba();
        let plan = PlanDeEstudios::nuevo("Plan 2025", "2025", 165);
        p.asignar_plan(plan);
        assert!(p.plan_de_estudios.is_some());
    }

    #[test]
    fn agregar_proposito_ok() {
        let mut p = programa_prueba();
        p.agregar_proposito(PropositoFormacion::nuevo("Liderar", "Actitudinal", "2879"));
        assert_eq!(p.propositos_formacion.len(), 1);
    }

    #[test]
    fn programa_display() {
        let p = programa_prueba();
        assert!(p.to_string().contains("2879"));
        assert!(p.to_string().contains("Pregrado"));
    }
}
