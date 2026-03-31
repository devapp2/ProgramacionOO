//! # Módulo de patrones de diseño
//!
//! Implementa los patrones de diseño GoF utilizados en el sistema de información
//! académico-administrativo de la Universidad Distrital Francisco José de Caldas.
//!
//! ## Patrones implementados
//!
//! ### [`factory`] — Factory Method
//! Centraliza la creación de objetos [`crate::modelo::unidad_academica::UnidadAcademica`]
//! según el tipo requerido, sin acoplar el cliente a las clases concretas.
//!
//! ### [`strategy`] — Strategy
//! Define una familia de algoritmos de evaluación académica intercambiables.
//! Las implementaciones `EvaluacionSimple` y `EvaluacionPonderada` comparten la
//! misma interfaz `EvaluacionStrategy`.
//!
//! ### [`observer`] — Observer
//! Permite a los órganos colegiados notificar a múltiples observadores cuando
//! toman una decisión institucional, sin acoplamiento directo.

pub mod factory;
pub mod observer;
pub mod strategy;
