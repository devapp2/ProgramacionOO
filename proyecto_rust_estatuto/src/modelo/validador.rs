//! # Validador — utilidades de validación de dominio
//!
//! Proporciona funciones estáticas (libres en Rust) para validar datos de
//! las entidades del sistema, equivalente a la clase `Validador` en el
//! paquete `util` del proyecto Java.
//!
//! Todas las funciones retornan `EstatutoResult<()>`, propagando los errores
//! con el operador `?` en vez de lanzar excepciones.

use crate::excepcion::errores::{EstatutoError, EstatutoResult};

/// Verifica que una cadena de texto no esté vacía ni compuesta solo de espacios.
///
/// # Argumentos
/// - `valor`: La cadena a validar.
/// - `campo`: Nombre del campo (para mensajes de error descriptivos).
///
/// # Errores
/// Retorna [`EstatutoError::Validacion`] si la cadena está vacía o es solo espacios.
///
/// # Ejemplo
/// ```
/// use estatuto_ud::modelo::validador::no_vacio;
/// assert!(no_vacio("FAC-001", "codigo").is_ok());
/// assert!(no_vacio("   ", "codigo").is_err());
/// ```
pub fn no_vacio(valor: &str, campo: &str) -> EstatutoResult<()> {
    if valor.trim().is_empty() {
        Err(EstatutoError::Validacion(format!(
            "El campo '{}' no puede estar vacío",
            campo
        )))
    } else {
        Ok(())
    }
}

/// Verifica que un número entero sea positivo (> 0).
///
/// # Argumentos
/// - `valor`: Número a validar.
/// - `campo`: Nombre del campo.
///
/// # Errores
/// Retorna [`EstatutoError::Validacion`] si el valor es cero o negativo.
pub fn positivo(valor: i32, campo: &str) -> EstatutoResult<()> {
    if valor <= 0 {
        Err(EstatutoError::Validacion(format!(
            "El campo '{}' debe ser mayor que cero, se recibió: {}",
            campo, valor
        )))
    } else {
        Ok(())
    }
}

/// Verifica que un número de punto flotante esté dentro de un rango.
///
/// # Argumentos
/// - `valor`: Número a validar.
/// - `campo`: Nombre del campo.
/// - `min`: Límite inferior (inclusivo).
/// - `max`: Límite superior (inclusivo).
pub fn en_rango_f64(valor: f64, campo: &str, min: f64, max: f64) -> EstatutoResult<()> {
    if valor < min || valor > max {
        Err(EstatutoError::Validacion(format!(
            "El campo '{}' debe estar entre {} y {}, se recibió: {}",
            campo, min, max, valor
        )))
    } else {
        Ok(())
    }
}

/// Verifica que la suma de ponderaciones de una colección sea exactamente 1.0
/// (con tolerancia de flotante).
///
/// Útil para validar criterios de rúbrica donde los pesos deben sumar 100 %.
pub fn ponderaciones_validas(ponderaciones: &[f64]) -> EstatutoResult<()> {
    let suma: f64 = ponderaciones.iter().sum();
    if (suma - 1.0_f64).abs() > 1e-6 {
        Err(EstatutoError::Validacion(format!(
            "Las ponderaciones deben sumar 1.0, suma actual: {:.4}",
            suma
        )))
    } else {
        Ok(())
    }
}

/// Normaliza un código: elimina espacios y convierte a mayúsculas.
///
/// Equivale al comportamiento del constructor de `UnidadAcademica` en Java.
pub fn normalizar_codigo(codigo: &str) -> String {
    codigo.trim().to_uppercase()
}

// ============================================================
//  Pruebas unitarias
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_vacio_ok_con_texto() {
        assert!(no_vacio("FAC-001", "codigo").is_ok());
    }

    #[test]
    fn no_vacio_error_con_vacio() {
        assert!(no_vacio("", "codigo").is_err());
    }

    #[test]
    fn no_vacio_error_con_espacios() {
        assert!(no_vacio("   ", "codigo").is_err());
    }

    #[test]
    fn positivo_ok() {
        assert!(positivo(3, "semestre").is_ok());
    }

    #[test]
    fn positivo_error_cero() {
        assert!(positivo(0, "semestre").is_err());
    }

    #[test]
    fn en_rango_f64_ok() {
        assert!(en_rango_f64(0.5, "nota", 0.0, 5.0).is_ok());
    }

    #[test]
    fn en_rango_f64_error_fuera() {
        assert!(en_rango_f64(6.0, "nota", 0.0, 5.0).is_err());
    }

    #[test]
    fn ponderaciones_validas_suma_uno() {
        assert!(ponderaciones_validas(&[0.5, 0.3, 0.2]).is_ok());
    }

    #[test]
    fn ponderaciones_invalidas_suma_distinta() {
        assert!(ponderaciones_validas(&[0.5, 0.3]).is_err());
    }

    #[test]
    fn normalizar_codigo_mayusculas() {
        assert_eq!(normalizar_codigo("  fac-001  "), "FAC-001");
    }
}
