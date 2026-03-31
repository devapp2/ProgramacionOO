//! # Rubrica — instrumento de evaluación con criterios ponderados
//!
//! La Rúbrica es el instrumento de evaluación que permite calificar el
//! desempeño estudiantil de forma objetiva y transparente.
//!
//! **Conforme al Acuerdo 004 de 2025**, la evaluación en la Universidad Distrital
//! debe ser formativa, integral y contextualizada, apoyada en instrumentos claros.

use crate::excepcion::errores::{EstatutoError, EstatutoResult};
use crate::modelo::criterio_rubrica::CriterioRubrica;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Instrumento de evaluación con criterios ponderados y niveles de desempeño.
///
/// Equivale a la clase `Rubrica` del proyecto Java.
/// El método `calcular_nota` implementa la suma ponderada de calificaciones,
/// equivalente a `calcularNota(Map<String, Double>)` en Java.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rubrica {
    /// Nombre identificador de la rúbrica.
    pub nombre: String,

    /// Criterios de evaluación con sus ponderaciones.
    pub criterios: Vec<CriterioRubrica>,

    /// Niveles de desempeño disponibles (ej: ["Insuficiente", "Básico", "Alto", "Superior"]).
    pub niveles: Vec<String>,
}

impl Rubrica {
    /// Crea una nueva Rúbrica.
    ///
    /// # Ejemplo
    /// ```
    /// use estatuto_ud::modelo::rubrica::Rubrica;
    ///
    /// let r = Rubrica::nueva(
    ///     "Rúbrica de Proyecto Final",
    ///     vec!["Insuficiente".to_string(), "Básico".to_string(), "Alto".to_string()],
    /// );
    /// assert_eq!(r.nombre, "Rúbrica de Proyecto Final");
    /// ```
    pub fn nueva(nombre: &str, niveles: Vec<String>) -> Self {
        Rubrica {
            nombre: nombre.trim().to_string(),
            criterios: Vec::new(),
            niveles,
        }
    }

    /// Agrega un criterio de evaluación a la rúbrica.
    pub fn agregar_criterio(&mut self, criterio: CriterioRubrica) {
        self.criterios.push(criterio);
    }

    /// Calcula la nota final ponderada a partir de las calificaciones por criterio.
    ///
    /// La función toma un slice de calificaciones (una por criterio, en el mismo orden),
    /// y retorna la suma ponderada: `∑(calificacion_i × ponderacion_i)`.
    ///
    /// # Argumentos
    /// - `calificaciones`: Slice de calificaciones (valores entre 0.0 y 5.0),
    ///   uno por cada criterio en el mismo orden que `self.criterios`.
    ///
    /// # Errores
    /// Retorna [`EstatutoError::Validacion`] si:
    /// - El número de calificaciones no coincide con el número de criterios.
    /// - Alguna calificación está fuera del rango [0.0, 5.0].
    ///
    /// # Ejemplo
    /// ```
    /// use std::collections::HashMap;
    /// use estatuto_ud::modelo::rubrica::Rubrica;
    /// use estatuto_ud::modelo::criterio_rubrica::CriterioRubrica;
    ///
    /// let mut r = Rubrica::nueva("Test", vec!["Bajo".to_string(), "Alto".to_string()]);
    /// r.agregar_criterio(CriterioRubrica::nuevo("C1", "Desc1", 0.6, HashMap::new()));
    /// r.agregar_criterio(CriterioRubrica::nuevo("C2", "Desc2", 0.4, HashMap::new()));
    /// let nota = r.calcular_nota(&[4.0, 3.5]).unwrap();
    /// assert!((nota - 3.8).abs() < 1e-6);
    /// ```
    pub fn calcular_nota(&self, calificaciones: &[f64]) -> EstatutoResult<f64> {
        if calificaciones.len() != self.criterios.len() {
            return Err(EstatutoError::Validacion(format!(
                "Se esperaban {} calificaciones, se recibieron {}",
                self.criterios.len(),
                calificaciones.len()
            )));
        }
        for (i, &nota) in calificaciones.iter().enumerate() {
            if nota < 0.0 || nota > 5.0 {
                return Err(EstatutoError::Validacion(format!(
                    "La calificación del criterio {} ({}) está fuera del rango [0.0, 5.0]",
                    i + 1,
                    nota
                )));
            }
        }
        let total: f64 = self
            .criterios
            .iter()
            .zip(calificaciones.iter())
            .map(|(c, &nota)| c.ponderacion * nota)
            .sum();
        Ok(total)
    }
}

impl fmt::Display for Rubrica {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Rúbrica '{}' — {} criterios | Niveles: {}",
            self.nombre,
            self.criterios.len(),
            self.niveles.join(", ")
        )
    }
}

// ============================================================
//  Pruebas unitarias
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modelo::criterio_rubrica::CriterioRubrica;
    use std::collections::HashMap;

    fn rubrica_con_criterios() -> Rubrica {
        let mut r = Rubrica::nueva(
            "Rúbrica POO",
            vec!["Bajo".to_string(), "Medio".to_string(), "Alto".to_string()],
        );
        r.agregar_criterio(CriterioRubrica::nuevo("Diseño", "Calidad del diseño", 0.5, HashMap::new()));
        r.agregar_criterio(CriterioRubrica::nuevo("Código", "Calidad del código", 0.3, HashMap::new()));
        r.agregar_criterio(CriterioRubrica::nuevo("Pruebas", "Cobertura de pruebas", 0.2, HashMap::new()));
        r
    }

    #[test]
    fn rubrica_crea_correctamente() {
        let r = rubrica_con_criterios();
        assert_eq!(r.criterios.len(), 3);
    }

    #[test]
    fn calcular_nota_ponderada() {
        let r = rubrica_con_criterios();
        // 0.5*4.0 + 0.3*3.5 + 0.2*5.0 = 2.0 + 1.05 + 1.0 = 4.05
        let nota = r.calcular_nota(&[4.0, 3.5, 5.0]).unwrap();
        assert!((nota - 4.05).abs() < 1e-6);
    }

    #[test]
    fn calcular_nota_error_cantidad_incorrecta() {
        let r = rubrica_con_criterios();
        assert!(r.calcular_nota(&[4.0, 3.0]).is_err()); // Solo 2 para 3 criterios
    }

    #[test]
    fn calcular_nota_error_fuera_de_rango() {
        let r = rubrica_con_criterios();
        assert!(r.calcular_nota(&[4.0, 3.0, 6.0]).is_err()); // 6.0 > 5.0
    }

    #[test]
    fn calcular_nota_todos_en_cinco() {
        let r = rubrica_con_criterios();
        let nota = r.calcular_nota(&[5.0, 5.0, 5.0]).unwrap();
        assert!((nota - 5.0).abs() < 1e-6);
    }

    #[test]
    fn rubrica_display() {
        let r = rubrica_con_criterios();
        assert!(r.to_string().contains("Rúbrica POO"));
        assert!(r.to_string().contains("3 criterios"));
    }
}
