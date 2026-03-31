//! # Factory — patrón de creación de unidades académicas
//!
//! Implementa el **Patrón Factory Method** para centralizar la creación de
//! objetos `UnidadAcademica` según el tipo requerido.
//!
//! ## Equivalencia Java → Rust
//! La clase `UnidadAcademicaFactory` de Java se traduce como una función
//! libre `crear_unidad_academica` que utiliza un `enum` para el tipo de unidad.
//!
//! ## Propósito pedagógico
//! El Factory Method permite crear objetos sin acoplarse a su clase concreta,
//! lo que facilita la extensión del sistema (principio Open/Closed).
//!
//! En Rust, la función retorna un `Box<dyn UnidadAcademica>` que permite
//! tratar polimórficamente cualquier tipo de unidad — equivalente a
//! la interfaz `UnidadAcademica` del polimorfismo Java.

use crate::excepcion::errores::EstatutoResult;
use crate::modelo::centro::Centro;
use crate::modelo::enums::TipoUnidad;
use crate::modelo::escuela::Escuela;
use crate::modelo::facultad::Facultad;
use crate::modelo::instituto::Instituto;
use crate::modelo::unidad_academica::UnidadAcademica;
use chrono::NaiveDate;

/// Parámetros de configuración para crear una unidad académica.
///
/// Agrupa los datos necesarios para instanciar cualquier tipo de unidad,
/// evitando una firma de función con demasiados parámetros.
pub struct ConfigUnidad {
    pub nombre: String,
    pub codigo: String,
    pub fecha_creacion: NaiveDate,
    pub director: String,
    /// Campo adicional para Facultad (decano) o Instituto (área de especialidad).
    pub campo_extra: Option<String>,
    /// Para Centro: indica si es interfacultativo.
    pub interfacultativo: bool,
}

impl ConfigUnidad {
    /// Crea una configuración básica de unidad académica.
    pub fn basica(
        nombre: &str,
        codigo: &str,
        fecha_creacion: NaiveDate,
        director: &str,
    ) -> Self {
        ConfigUnidad {
            nombre: nombre.to_string(),
            codigo: codigo.to_string(),
            fecha_creacion,
            director: director.to_string(),
            campo_extra: None,
            interfacultativo: false,
        }
    }

    /// Configura el campo extra (decano para Facultad, área para Instituto).
    pub fn con_campo_extra(mut self, extra: &str) -> Self {
        self.campo_extra = Some(extra.to_string());
        self
    }

    /// Configura si el Centro es interfacultativo.
    pub fn interfacultativo(mut self) -> Self {
        self.interfacultativo = true;
        self
    }
}

/// Crea una unidad académica del tipo especificado.
///
/// Esta función implementa el **Patrón Factory Method**:
/// el cliente no necesita conocer la clase concreta, solo el tipo deseado.
///
/// ## Ejemplo
/// ```
/// use chrono::NaiveDate;
/// use estatuto_ud::modelo::enums::TipoUnidad;
/// use estatuto_ud::patron::factory::{crear_unidad_academica, ConfigUnidad};
///
/// let config = ConfigUnidad::basica(
///     "Facultad de Ingeniería",
///     "FAC-ING",
///     NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
///     "Dr. Pérez",
/// ).con_campo_extra("Dra. López");
///
/// let unidad = crear_unidad_academica(TipoUnidad::Facultad, config).unwrap();
/// assert_eq!(unidad.tipo(), "Facultad");
/// ```
pub fn crear_unidad_academica(
    tipo: TipoUnidad,
    config: ConfigUnidad,
) -> EstatutoResult<Box<dyn UnidadAcademica>> {
    match tipo {
        TipoUnidad::Facultad => {
            let decano = config.campo_extra.as_deref().unwrap_or(&config.director);
            let f = Facultad::nueva(
                &config.nombre,
                &config.codigo,
                config.fecha_creacion,
                &config.director,
                decano,
            )?;
            Ok(Box::new(f))
        }
        TipoUnidad::Escuela => {
            let e = Escuela::nueva(
                &config.nombre,
                &config.codigo,
                config.fecha_creacion,
                &config.director,
            )?;
            Ok(Box::new(e))
        }
        TipoUnidad::Centro => {
            let c = Centro::nuevo(
                &config.nombre,
                &config.codigo,
                config.fecha_creacion,
                &config.director,
                config.interfacultativo,
            )?;
            Ok(Box::new(c))
        }
        TipoUnidad::Instituto => {
            let area = config.campo_extra.as_deref().unwrap_or("General");
            let i = Instituto::nuevo(
                &config.nombre,
                &config.codigo,
                config.fecha_creacion,
                &config.director,
                area,
            )?;
            Ok(Box::new(i))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn fecha() -> NaiveDate {
        NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()
    }

    #[test]
    fn factory_crea_facultad() {
        let config = ConfigUnidad::basica("Facultad TIC", "FAC-TIC", fecha(), "Dir")
            .con_campo_extra("Decano");
        let unidad = crear_unidad_academica(TipoUnidad::Facultad, config).unwrap();
        assert_eq!(unidad.tipo(), "Facultad");
    }

    #[test]
    fn factory_crea_escuela() {
        let config = ConfigUnidad::basica("Escuela Sistemas", "ESC-SIS", fecha(), "Dir");
        let unidad = crear_unidad_academica(TipoUnidad::Escuela, config).unwrap();
        assert_eq!(unidad.tipo(), "Escuela");
    }

    #[test]
    fn factory_crea_centro() {
        let config = ConfigUnidad::basica("Centro Inv.", "CI-01", fecha(), "Dir");
        let unidad = crear_unidad_academica(TipoUnidad::Centro, config).unwrap();
        assert_eq!(unidad.tipo(), "Centro");
    }

    #[test]
    fn factory_crea_instituto() {
        let config = ConfigUnidad::basica("Instituto TIC", "INST-TIC", fecha(), "Dir")
            .con_campo_extra("Tecnologías");
        let unidad = crear_unidad_academica(TipoUnidad::Instituto, config).unwrap();
        assert_eq!(unidad.tipo(), "Instituto");
    }

    #[test]
    fn factory_error_codigo_vacio() {
        let config = ConfigUnidad::basica("Facultad", "", fecha(), "Dir");
        let r = crear_unidad_academica(TipoUnidad::Facultad, config);
        assert!(r.is_err());
    }
}
