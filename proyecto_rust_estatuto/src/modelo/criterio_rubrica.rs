//! # CriterioRubrica — criterio de evaluación con ponderación
//!
//! Un Criterio de Rúbrica define un aspecto evaluable del desempeño estudiantil,
//! con su peso relativo en la calificación final y descriptores por nivel.
//!
//! En el modelo de evaluación de la Universidad Distrital, las rúbricas
//! articulan los resultados de aprendizaje con los criterios de evaluación.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Criterio de evaluación dentro de una rúbrica.
///
/// Equivale a la clase `CriterioRubrica` del proyecto Java.
/// El campo `descriptores` (Java `Map<String,String>`) se implementa
/// como `HashMap<String, String>` en Rust.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriterioRubrica {
    /// Nombre del criterio evaluado (ej: "Claridad del código").
    pub nombre: String,

    /// Descripción detallada del criterio.
    pub descripcion: String,

    /// Peso del criterio en la nota final (valor entre 0.0 y 1.0).
    ///
    /// La suma de ponderaciones de todos los criterios de una rúbrica debe ser 1.0.
    pub ponderacion: f64,

    /// Descriptores por nivel de desempeño.
    ///
    /// Ejemplo: `{"Excelente" => "El código es legible y...", "Insuficiente" => ...}`
    pub descriptores: HashMap<String, String>,
}

impl CriterioRubrica {
    /// Crea un nuevo Criterio de Rúbrica.
    ///
    /// # Ejemplo
    /// ```
    /// use std::collections::HashMap;
    /// use estatuto_ud::modelo::criterio_rubrica::CriterioRubrica;
    ///
    /// let c = CriterioRubrica::nuevo("Claridad", "Legibilidad del código", 0.4, HashMap::new());
    /// assert_eq!(c.ponderacion, 0.4);
    /// ```
    pub fn nuevo(
        nombre: &str,
        descripcion: &str,
        ponderacion: f64,
        descriptores: HashMap<String, String>,
    ) -> Self {
        CriterioRubrica {
            nombre: nombre.trim().to_string(),
            descripcion: descripcion.trim().to_string(),
            ponderacion,
            descriptores,
        }
    }
}

impl fmt::Display for CriterioRubrica {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Criterio '{}' (ponderación: {:.0}%) — {}",
            self.nombre,
            self.ponderacion * 100.0,
            self.descripcion
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn criterio_simple() -> CriterioRubrica {
        let mut desc = HashMap::new();
        desc.insert("Excelente".to_string(), "Cumple todos los requisitos".to_string());
        desc.insert("Insuficiente".to_string(), "No cumple los requisitos básicos".to_string());
        CriterioRubrica::nuevo("Corrección", "Correctitud del programa", 0.5, desc)
    }

    #[test]
    fn criterio_crea_correctamente() {
        let c = criterio_simple();
        assert_eq!(c.nombre, "Corrección");
        assert!((c.ponderacion - 0.5).abs() < 1e-9);
    }

    #[test]
    fn criterio_tiene_descriptores() {
        let c = criterio_simple();
        assert!(c.descriptores.contains_key("Excelente"));
    }

    #[test]
    fn criterio_display() {
        let c = criterio_simple();
        assert!(c.to_string().contains("Corrección"));
        assert!(c.to_string().contains("50%"));
    }

    #[test]
    fn criterio_serializa() {
        let c = criterio_simple();
        let json = serde_json::to_string(&c).unwrap();
        let back: CriterioRubrica = serde_json::from_str(&json).unwrap();
        assert_eq!(back.nombre, "Corrección");
    }
}
