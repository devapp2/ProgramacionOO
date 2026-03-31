//! # Módulo de persistencia
//!
//! Implementa el patrón DAO (Data Access Object) para la persistencia de entidades
//! del sistema de información de la Universidad Distrital.
//!
//! ## Diseño
//! - [`dao`]: Define el trait genérico `GenericDao<T, Id>`
//! - [`json_dao`]: Implementación con archivos JSON (usando `serde_json`)
//!
//! ## Equivalencia Java → Rust
//! El patrón DAO de Java (`GenericDAO<T, ID>` + `JsonDAO<T, ID>` con Gson)
//! se traduce directamente a traits + structs genéricos en Rust.

pub mod dao;
pub mod json_dao;

pub use dao::GenericDao;
pub use json_dao::JsonDao;
