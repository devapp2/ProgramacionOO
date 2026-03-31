//! # Módulo de manejo de errores
//!
//! Centraliza los tipos de error del sistema de información académico-administrativo
//! de la Universidad Distrital Francisco José de Caldas.
//!
//! ## Diseño
//! Se utiliza el crate `thiserror` para derivar implementaciones de
//! `std::error::Error`, siguiendo las convenciones de la comunidad Rust.

pub mod errores;

pub use errores::{EstatutoError, EstatutoResult};
