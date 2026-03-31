//! # GenericDao — trait de acceso a datos
//!
//! Define las operaciones CRUD (Crear, Leer, Actualizar, Eliminar) que debe
//! implementar cualquier repositorio de datos del sistema.
//!
//! ## Equivalencia Java → Rust
//! - `GenericDAO<T, ID>` (interface Java) → `trait GenericDao<T, Id>` (Rust)
//! - Los métodos retornan `Result<T, EstatutoError>` en vez de lanzar excepciones
//!
//! ## Polimorfismo en Rust
//! Al ser un trait, `GenericDao` puede implementarse de diferentes formas:
//! - [`super::json_dao::JsonDao`]: persistencia en archivos JSON
//! - (futuro) `PostgresDao`: persistencia en base de datos
//! - (futuro) `MemoryDao`: persistencia en memoria para pruebas

use crate::excepcion::errores::EstatutoResult;

/// Trait genérico para repositorios de datos.
///
/// `T` es el tipo de entidad (ej: `Facultad`, `Docente`).
/// `Id` es el tipo del identificador (generalmente `String`).
pub trait GenericDao<T, Id> {
    /// Retorna todas las entidades almacenadas.
    ///
    /// Equivale a `List<T> buscarTodos()` en Java.
    fn buscar_todos(&self) -> EstatutoResult<Vec<T>>;

    /// Guarda (inserta) una nueva entidad.
    ///
    /// Equivale a `void guardar(T entidad)` en Java.
    fn guardar(&mut self, entidad: T) -> EstatutoResult<()>;

    /// Busca una entidad por su identificador.
    ///
    /// Equivale a `Optional<T> buscarPorId(ID id)` en Java.
    /// Retorna `Ok(None)` si la entidad no existe.
    fn buscar_por_id(&self, id: &Id) -> EstatutoResult<Option<T>>;

    /// Actualiza una entidad existente.
    ///
    /// Equivale a `void actualizar(T entidad)` en Java.
    /// Retorna error si la entidad no existe.
    fn actualizar(&mut self, entidad: T) -> EstatutoResult<()>;

    /// Elimina una entidad por su identificador.
    ///
    /// Equivale a `void eliminar(ID id)` en Java.
    /// Retorna `true` si fue encontrada y eliminada, `false` si no existía.
    fn eliminar(&mut self, id: &Id) -> EstatutoResult<bool>;
}
