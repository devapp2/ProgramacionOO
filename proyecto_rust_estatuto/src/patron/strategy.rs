//! # Strategy — estrategias de cálculo de evaluaciones
//!
//! Implementa el **Patrón Strategy** para el cálculo de notas académicas.
//! Permite cambiar el algoritmo de evaluación sin modificar el código cliente.
//!
//! ## Equivalencia Java → Rust
//! - `EvaluacionStrategy` (interface) → trait `EvaluacionStrategy`
//! - `EvaluacionSimpleStrategy` → struct `EvaluacionSimple`
//! - `EvaluacionPonderadaStrategy` → struct `EvaluacionPonderada`
//!
//! ## Propósito pedagógico
//! El patrón Strategy permite encapsular familias de algoritmos intercambiables.
//! En Rust, se implementa mediante trait objects (`Box<dyn EvaluacionStrategy>`),
//! equivalente al polimorfismo de Java con interfaces.

use crate::excepcion::errores::{EstatutoError, EstatutoResult};
use crate::modelo::rubrica::Rubrica;

// ============================================================
//  Trait EvaluacionStrategy
// ============================================================

/// Estrategia de cálculo de nota para un conjunto de calificaciones.
///
/// Equivale a la interface `EvaluacionStrategy` de Java.
/// El método `calcular` recibe la rúbrica y las calificaciones, y retorna la nota final.
pub trait EvaluacionStrategy: Send + Sync {
    /// Calcula la nota final usando la estrategia específica.
    ///
    /// # Argumentos
    /// - `rubrica`: Instrumento de evaluación con criterios y ponderaciones.
    /// - `calificaciones`: Nota obtenida en cada criterio (0.0 a 5.0).
    ///
    /// # Retorna
    /// La nota final calculada como `f64` en el rango [0.0, 5.0].
    fn calcular(&self, rubrica: &Rubrica, calificaciones: &[f64]) -> EstatutoResult<f64>;

    /// Nombre descriptivo de la estrategia (para registro y depuración).
    fn nombre(&self) -> &str;
}

// ============================================================
//  EvaluacionSimple — promedio aritmético simple
// ============================================================

/// Estrategia de evaluación simple: promedio aritmético de calificaciones.
///
/// Ignora las ponderaciones y calcula la media de todas las notas.
/// Útil para evaluaciones informales o de diagnóstico.
pub struct EvaluacionSimple;

impl EvaluacionStrategy for EvaluacionSimple {
    fn calcular(&self, rubrica: &Rubrica, calificaciones: &[f64]) -> EstatutoResult<f64> {
        if calificaciones.len() != rubrica.criterios.len() {
            return Err(EstatutoError::Validacion(format!(
                "[EvaluacionSimple] Se esperaban {} calificaciones, se recibieron {}",
                rubrica.criterios.len(),
                calificaciones.len()
            )));
        }
        if calificaciones.is_empty() {
            return Err(EstatutoError::Validacion(
                "No se proporcionaron calificaciones".to_string(),
            ));
        }
        for (i, &nota) in calificaciones.iter().enumerate() {
            if nota < 0.0 || nota > 5.0 {
                return Err(EstatutoError::Validacion(format!(
                    "Calificación {} fuera de rango: {}",
                    i + 1,
                    nota
                )));
            }
        }
        let suma: f64 = calificaciones.iter().sum();
        Ok(suma / calificaciones.len() as f64)
    }

    fn nombre(&self) -> &str {
        "Evaluación Simple (Promedio Aritmético)"
    }
}

// ============================================================
//  EvaluacionPonderada — suma ponderada con pesos de los criterios
// ============================================================

/// Estrategia de evaluación ponderada: suma ponderada usando los pesos de los criterios.
///
/// Multiplica cada calificación por la ponderación del criterio correspondiente
/// y suma los productos. Es la estrategia estándar para rúbricas analíticas.
pub struct EvaluacionPonderada;

impl EvaluacionStrategy for EvaluacionPonderada {
    fn calcular(&self, rubrica: &Rubrica, calificaciones: &[f64]) -> EstatutoResult<f64> {
        // Delega en el método `calcular_nota` de la Rúbrica, que ya implementa
        // la suma ponderada con validaciones.
        rubrica.calcular_nota(calificaciones)
    }

    fn nombre(&self) -> &str {
        "Evaluación Ponderada (Suma Ponderada)"
    }
}

// ============================================================
//  Pruebas unitarias
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modelo::criterio_rubrica::CriterioRubrica;
    use crate::modelo::rubrica::Rubrica;
    use std::collections::HashMap;

    fn rubrica_prueba() -> Rubrica {
        let mut r = Rubrica::nueva("Test", vec!["Bajo".to_string(), "Alto".to_string()]);
        r.agregar_criterio(CriterioRubrica::nuevo("C1", "D1", 0.6, HashMap::new()));
        r.agregar_criterio(CriterioRubrica::nuevo("C2", "D2", 0.4, HashMap::new()));
        r
    }

    #[test]
    fn estrategia_simple_promedio() {
        let r = rubrica_prueba();
        let e = EvaluacionSimple;
        // (4.0 + 3.0) / 2 = 3.5
        let nota = e.calcular(&r, &[4.0, 3.0]).unwrap();
        assert!((nota - 3.5).abs() < 1e-9);
    }

    #[test]
    fn estrategia_ponderada_suma() {
        let r = rubrica_prueba();
        let e = EvaluacionPonderada;
        // 0.6*4.0 + 0.4*3.0 = 2.4 + 1.2 = 3.6
        let nota = e.calcular(&r, &[4.0, 3.0]).unwrap();
        assert!((nota - 3.6).abs() < 1e-9);
    }

    #[test]
    fn estrategia_simple_error_cantidad_incorrecta() {
        let r = rubrica_prueba();
        let e = EvaluacionSimple;
        assert!(e.calcular(&r, &[4.0]).is_err());
    }

    #[test]
    fn estrategia_simple_error_fuera_de_rango() {
        let r = rubrica_prueba();
        let e = EvaluacionSimple;
        assert!(e.calcular(&r, &[6.0, 3.0]).is_err());
    }

    #[test]
    fn estrategia_nombre() {
        assert!(EvaluacionSimple.nombre().contains("Simple"));
        assert!(EvaluacionPonderada.nombre().contains("Ponderada"));
    }
}
