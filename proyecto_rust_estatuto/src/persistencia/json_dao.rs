//! # JsonDao — persistencia en archivos JSON con serde_json
//!
//! Implementación de [`GenericDao`] que almacena las entidades como archivos JSON
//! en el directorio `datos/` del proyecto.
//!
//! ## Equivalencia Java → Rust
//! - `JsonDAO<T, ID>` con Gson → `JsonDao<T>` con `serde_json`
//! - Los adaptadores `LocalDateAdapter` de Java no son necesarios porque
//!   `chrono::NaiveDate` ya implementa `Serialize`/`Deserialize` con la feature `serde`.
//!
//! ## Diseño
//! Cada instancia de `JsonDao<T>` gestiona UN archivo JSON que contiene
//! un arreglo JSON de entidades `[{...}, {...}, ...]`.

use crate::excepcion::errores::EstatutoResult;
use crate::persistencia::dao::GenericDao;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Repositorio JSON genérico.
///
/// El parámetro de tipo `T` debe implementar `Serialize + DeserializeOwned + Clone`.
/// El campo `id_extractor` es una función que extrae el identificador de la entidad.
///
/// # Ejemplo de uso
/// ```no_run
/// use estatuto_ud::persistencia::json_dao::JsonDao;
/// use estatuto_ud::modelo::facultad::Facultad;
///
/// let dao: JsonDao<Facultad> = JsonDao::nuevo(
///     "datos/facultades.json",
///     |f: &Facultad| f.base.codigo.clone(),
/// );
/// ```
pub struct JsonDao<T> {
    /// Ruta al archivo JSON donde se almacenan las entidades.
    ruta_archivo: PathBuf,

    /// Función que extrae el identificador único de una entidad.
    /// Equivale a la key usada en el `Map<ID, T>` de la implementación Java.
    id_extractor: Box<dyn Fn(&T) -> String + Send + Sync>,
}

impl<T> JsonDao<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
{
    /// Crea un nuevo `JsonDao` para el archivo y la función extractora dados.
    ///
    /// Si el archivo no existe, se creará al hacer la primera operación de escritura.
    pub fn nuevo<F>(ruta_archivo: impl AsRef<Path>, id_extractor: F) -> Self
    where
        F: Fn(&T) -> String + Send + Sync + 'static,
    {
        JsonDao {
            ruta_archivo: ruta_archivo.as_ref().to_path_buf(),
            id_extractor: Box::new(id_extractor),
        }
    }

    /// Lee todas las entidades del archivo JSON.
    ///
    /// Si el archivo no existe, retorna un Vec vacío (equivale a una colección vacía).
    fn leer_todo(&self) -> EstatutoResult<Vec<T>> {
        if !self.ruta_archivo.exists() {
            return Ok(Vec::new());
        }
        let contenido = fs::read_to_string(&self.ruta_archivo)?;
        if contenido.trim().is_empty() {
            return Ok(Vec::new());
        }
        let entidades: Vec<T> = serde_json::from_str(&contenido)?;
        Ok(entidades)
    }

    /// Escribe todas las entidades al archivo JSON (sobrescribe el contenido).
    fn escribir_todo(&self, entidades: &[T]) -> EstatutoResult<()> {
        // Crear el directorio padre si no existe
        if let Some(dir) = self.ruta_archivo.parent() {
            fs::create_dir_all(dir)?;
        }
        let json = serde_json::to_string_pretty(entidades)?;
        fs::write(&self.ruta_archivo, json)?;
        Ok(())
    }

    /// Extrae el ID de una entidad usando la función configurada.
    fn get_id(&self, entidad: &T) -> String {
        (self.id_extractor)(entidad)
    }
}

impl<T> GenericDao<T, String> for JsonDao<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
{
    fn buscar_todos(&self) -> EstatutoResult<Vec<T>> {
        self.leer_todo()
    }

    fn guardar(&mut self, entidad: T) -> EstatutoResult<()> {
        let mut entidades = self.leer_todo()?;
        let id = self.get_id(&entidad);
        // Verificar duplicados
        if entidades.iter().any(|e| self.get_id(e) == id) {
            return Err(crate::excepcion::errores::EstatutoError::Duplicada(
                format!("Ya existe una entidad con ID '{}'", id),
            ));
        }
        entidades.push(entidad);
        self.escribir_todo(&entidades)
    }

    fn buscar_por_id(&self, id: &String) -> EstatutoResult<Option<T>> {
        let entidades = self.leer_todo()?;
        Ok(entidades.into_iter().find(|e| &self.get_id(e) == id))
    }

    fn actualizar(&mut self, entidad: T) -> EstatutoResult<()> {
        let mut entidades = self.leer_todo()?;
        let id = self.get_id(&entidad);
        let pos = entidades
            .iter()
            .position(|e| self.get_id(e) == id)
            .ok_or_else(|| {
                crate::excepcion::errores::EstatutoError::NoEncontrada(format!(
                    "No existe entidad con ID '{}' para actualizar",
                    id
                ))
            })?;
        entidades[pos] = entidad;
        self.escribir_todo(&entidades)
    }

    fn eliminar(&mut self, id: &String) -> EstatutoResult<bool> {
        let mut entidades = self.leer_todo()?;
        let len_antes = entidades.len();
        entidades.retain(|e| &self.get_id(e) != id);
        let eliminado = entidades.len() < len_antes;
        if eliminado {
            self.escribir_todo(&entidades)?;
        }
        Ok(eliminado)
    }
}

// ============================================================
//  Pruebas unitarias (usan archivos temporales)
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modelo::docente::Docente;
    use crate::modelo::enums::TipoVinculacion;
    use crate::persistencia::dao::GenericDao;
    use std::env;

    fn dao_temporal() -> (JsonDao<Docente>, PathBuf) {
        let ruta = env::temp_dir().join(format!(
            "estatuto_test_{}.json",
            uuid::Uuid::new_v4()
        ));
        let dao = JsonDao::nuevo(ruta.clone(), |d: &Docente| d.base.identificacion.clone());
        (dao, ruta)
    }

    fn docente_ejemplo(id: &str) -> Docente {
        Docente::nuevo("Prueba", id, None, TipoVinculacion::Planta, "ESC-SIS", "Titular")
    }

    #[test]
    fn guardar_y_buscar_todos() {
        let (mut dao, ruta) = dao_temporal();
        dao.guardar(docente_ejemplo("CC-1")).unwrap();
        dao.guardar(docente_ejemplo("CC-2")).unwrap();
        let todos = dao.buscar_todos().unwrap();
        assert_eq!(todos.len(), 2);
        let _ = fs::remove_file(ruta);
    }

    #[test]
    fn buscar_por_id_encontrado() {
        let (mut dao, ruta) = dao_temporal();
        dao.guardar(docente_ejemplo("CC-10")).unwrap();
        let encontrado = dao.buscar_por_id(&"CC-10".to_string()).unwrap();
        assert!(encontrado.is_some());
        let _ = fs::remove_file(ruta);
    }

    #[test]
    fn buscar_por_id_no_encontrado() {
        let (dao, _ruta) = dao_temporal();
        let result = dao.buscar_por_id(&"CC-99".to_string()).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn actualizar_entidad() {
        let (mut dao, ruta) = dao_temporal();
        dao.guardar(docente_ejemplo("CC-20")).unwrap();
        let mut actualizado = docente_ejemplo("CC-20");
        actualizado.categoria = "Asociado".to_string();
        dao.actualizar(actualizado).unwrap();
        let encontrado = dao.buscar_por_id(&"CC-20".to_string()).unwrap().unwrap();
        assert_eq!(encontrado.categoria, "Asociado");
        let _ = fs::remove_file(ruta);
    }

    #[test]
    fn eliminar_entidad() {
        let (mut dao, ruta) = dao_temporal();
        dao.guardar(docente_ejemplo("CC-30")).unwrap();
        let eliminado = dao.eliminar(&"CC-30".to_string()).unwrap();
        assert!(eliminado);
        let todos = dao.buscar_todos().unwrap();
        assert!(todos.is_empty());
        let _ = fs::remove_file(ruta);
    }

    #[test]
    fn guardar_duplicado_retorna_error() {
        let (mut dao, ruta) = dao_temporal();
        dao.guardar(docente_ejemplo("CC-40")).unwrap();
        let r = dao.guardar(docente_ejemplo("CC-40"));
        assert!(r.is_err());
        let _ = fs::remove_file(ruta);
    }
}
