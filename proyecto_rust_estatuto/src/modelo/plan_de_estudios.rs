//! # PlanDeEstudios — malla curricular del programa académico
//!
//! El Plan de Estudios organiza los espacios académicos en una secuencia
//! lógica y coherente que conduce a la formación integral del estudiante.
//!
//! **Conforme al Acuerdo 004 de 2025**, los planes de estudio son aprobados
//! por el Consejo Académico y deben cumplir con los lineamientos del MEN.

use crate::modelo::espacio_academico::EspacioAcademico;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Malla curricular de un programa académico.
///
/// Equivale a la clase `PlanDeEstudios` del proyecto Java.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanDeEstudios {
    /// Nombre o denominación del plan de estudios.
    pub nombre: String,

    /// Versión del plan (ej: "Plan 2020", "Plan 2025").
    pub version: String,

    /// Número total de créditos académicos requeridos para graduarse.
    pub creditos_totales: u16,

    /// Espacios académicos (asignaturas) que conforman el plan.
    pub espacios_academicos: Vec<EspacioAcademico>,
}

impl PlanDeEstudios {
    /// Crea un nuevo Plan de Estudios.
    ///
    /// # Ejemplo
    /// ```
    /// use estatuto_ud::modelo::plan_de_estudios::PlanDeEstudios;
    ///
    /// let plan = PlanDeEstudios::nuevo("Plan Ingeniería Sistemas", "2025", 165);
    /// assert_eq!(plan.creditos_totales, 165);
    /// ```
    pub fn nuevo(nombre: &str, version: &str, creditos_totales: u16) -> Self {
        PlanDeEstudios {
            nombre: nombre.trim().to_string(),
            version: version.trim().to_string(),
            creditos_totales,
            espacios_academicos: Vec::new(),
        }
    }

    /// Agrega un Espacio Académico al plan de estudios.
    pub fn agregar_espacio(&mut self, espacio: EspacioAcademico) {
        self.espacios_academicos.push(espacio);
    }

    /// Calcula el total de créditos de los espacios registrados.
    pub fn creditos_registrados(&self) -> u16 {
        self.espacios_academicos
            .iter()
            .map(|e| e.creditos as u16)
            .sum()
    }

    /// Retorna los espacios académicos de un semestre específico.
    pub fn espacios_por_semestre(&self, semestre: u8) -> Vec<&EspacioAcademico> {
        self.espacios_academicos
            .iter()
            .filter(|e| e.semestre == semestre)
            .collect()
    }
}

impl fmt::Display for PlanDeEstudios {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Plan de Estudios: {} ({}) — {} créditos | {} espacios académicos",
            self.nombre,
            self.version,
            self.creditos_totales,
            self.espacios_academicos.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modelo::espacio_academico::EspacioAcademico;

    #[test]
    fn plan_crea_correctamente() {
        let plan = PlanDeEstudios::nuevo("Plan Sistemas", "2025", 165);
        assert_eq!(plan.creditos_totales, 165);
        assert!(plan.espacios_academicos.is_empty());
    }

    #[test]
    fn agregar_espacio_ok() {
        let mut plan = PlanDeEstudios::nuevo("Plan", "2025", 100);
        plan.agregar_espacio(EspacioAcademico::nuevo("Cálculo", "CAL-1", 4, 8, "Teórico", 1));
        assert_eq!(plan.espacios_academicos.len(), 1);
    }

    #[test]
    fn creditos_registrados() {
        let mut plan = PlanDeEstudios::nuevo("Plan", "2025", 100);
        plan.agregar_espacio(EspacioAcademico::nuevo("E1", "E1", 3, 6, "T", 1));
        plan.agregar_espacio(EspacioAcademico::nuevo("E2", "E2", 4, 8, "T", 1));
        assert_eq!(plan.creditos_registrados(), 7);
    }

    #[test]
    fn espacios_por_semestre_filtro() {
        let mut plan = PlanDeEstudios::nuevo("Plan", "2025", 100);
        plan.agregar_espacio(EspacioAcademico::nuevo("S1A", "S1A", 3, 6, "T", 1));
        plan.agregar_espacio(EspacioAcademico::nuevo("S2A", "S2A", 3, 6, "T", 2));
        plan.agregar_espacio(EspacioAcademico::nuevo("S1B", "S1B", 3, 6, "T", 1));
        let sem1 = plan.espacios_por_semestre(1);
        assert_eq!(sem1.len(), 2);
    }

    #[test]
    fn plan_display() {
        let plan = PlanDeEstudios::nuevo("Plan Sistemas", "2025", 165);
        assert!(plan.to_string().contains("165"));
    }
}
