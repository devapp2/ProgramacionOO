//! # EspacioAcademico — asignatura del plan de estudios
//!
//! El Espacio Académico es la unidad básica del plan de estudios, equivalente
//! a una asignatura o materia. Define los créditos, horas y semestre de cursado.
//!
//! **Conforme al Acuerdo 004 de 2025**, los espacios académicos articulan
//! los resultados de aprendizaje con el currículo del programa.

use crate::modelo::resultado_aprendizaje::ResultadoDeAprendizaje;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Asignatura o espacio de aprendizaje dentro del plan de estudios.
///
/// Equivale a la clase `EspacioAcademico` del proyecto Java.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EspacioAcademico {
    /// Nombre oficial del espacio académico.
    pub nombre: String,

    /// Código único del espacio académico.
    pub codigo: String,

    /// Número de créditos académicos que vale la asignatura.
    pub creditos: u8,

    /// Horas de trabajo semanal (presencial + independiente).
    pub horas_semanales: u8,

    /// Tipo de espacio: "Teórico", "Práctico", "Teórico-Práctico", etc.
    pub tipo: String,

    /// Semestre del plan de estudios en que se cursa normalmente.
    pub semestre: u8,

    /// Resultados de aprendizaje asociados a este espacio.
    pub resultados_aprendizaje: Vec<ResultadoDeAprendizaje>,
}

impl EspacioAcademico {
    /// Crea un nuevo Espacio Académico.
    ///
    /// # Ejemplo
    /// ```
    /// use estatuto_ud::modelo::espacio_academico::EspacioAcademico;
    ///
    /// let ea = EspacioAcademico::nuevo("Algoritmos I", "ALG-101", 3, 6, "Teórico-Práctico", 1);
    /// assert_eq!(ea.creditos, 3);
    /// ```
    pub fn nuevo(
        nombre: &str,
        codigo: &str,
        creditos: u8,
        horas_semanales: u8,
        tipo: &str,
        semestre: u8,
    ) -> Self {
        EspacioAcademico {
            nombre: nombre.trim().to_string(),
            codigo: codigo.trim().to_uppercase(),
            creditos,
            horas_semanales,
            tipo: tipo.trim().to_string(),
            semestre,
            resultados_aprendizaje: Vec::new(),
        }
    }

    /// Agrega un Resultado de Aprendizaje a este espacio académico.
    pub fn agregar_resultado(&mut self, ra: ResultadoDeAprendizaje) {
        self.resultados_aprendizaje.push(ra);
    }
}

impl fmt::Display for EspacioAcademico {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "EA [{}] {} — {} créditos | {} h/sem | Semestre {} | {}",
            self.codigo, self.nombre, self.creditos, self.horas_semanales, self.semestre, self.tipo
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn espacio_crea_correctamente() {
        let ea = EspacioAcademico::nuevo("Cálculo I", "CAL-101", 4, 8, "Teórico", 1);
        assert_eq!(ea.codigo, "CAL-101");
        assert_eq!(ea.creditos, 4);
    }

    #[test]
    fn agregar_resultado_ok() {
        let mut ea = EspacioAcademico::nuevo("POO", "POO-201", 3, 6, "Teórico-Práctico", 2);
        let ra = ResultadoDeAprendizaje::nuevo("Diseñar clases", "Intermedio", "PROG-SIS");
        ea.agregar_resultado(ra);
        assert_eq!(ea.resultados_aprendizaje.len(), 1);
    }

    #[test]
    fn espacio_display() {
        let ea = EspacioAcademico::nuevo("Estructuras", "EST-301", 3, 6, "Práctico", 3);
        assert!(ea.to_string().contains("EST-301"));
    }

    #[test]
    fn espacio_serializa() {
        let ea = EspacioAcademico::nuevo("DB I", "DB-401", 3, 6, "Teórico", 4);
        let json = serde_json::to_string(&ea).unwrap();
        let back: EspacioAcademico = serde_json::from_str(&json).unwrap();
        assert_eq!(back.codigo, "DB-401");
    }
}
