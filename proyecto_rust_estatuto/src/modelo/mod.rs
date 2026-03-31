//! # Módulo de dominio — modelo académico-administrativo
//!
//! Este módulo contiene todas las entidades del dominio del sistema de información
//! de la **Universidad Distrital Francisco José de Caldas**, implementadas conforme
//! al **Acuerdo 004 de 2025 — Estatuto General**.
//!
//! ## Estructura del modelo
//!
//! ### Entidades base (traits + structs)
//! - [`unidad_academica`]: Trait `UnidadAcademica` + struct `UnidadBase`
//! - [`persona`]: Trait `Persona` + struct `PersonaBase`
//!
//! ### Unidades académico-administrativas
//! - [`facultad`]: Unidad académica principal
//! - [`escuela`]: Subdivisión de la Facultad
//! - [`centro`]: Unidad de investigación y extensión
//! - [`instituto`]: Organismo académico-científico especializado
//! - [`caba`]: Comunidad Académica de Base
//!
//! ### Comunidad universitaria
//! - [`docente`]: Miembro del cuerpo profesoral
//! - [`estudiante`]: Miembro estudiantil
//! - [`personal_administrativo`]: Empleados de la gestión institucional
//! - [`egresado`]: Graduados de la Universidad
//!
//! ### Currículum y evaluación
//! - [`programa_academico`]: Oferta formativa de la Universidad
//! - [`plan_de_estudios`]: Malla curricular
//! - [`espacio_academico`]: Asignaturas del plan de estudios
//! - [`resultado_aprendizaje`]: Competencias del programa
//! - [`proposito_formacion`]: Misión formativa del programa
//! - [`rubrica`]: Instrumento de evaluación ponderado
//! - [`criterio_rubrica`]: Criterio individual de la rúbrica
//! - [`matricula`]: Registro formal de inscripción
//! - [`evidencia`]: Producto del aprendizaje estudiantil
//!
//! ### Soporte
//! - [`enums`]: Tipos enumerados del dominio
//! - [`validador`]: Utilidades de validación

pub mod caba;
pub mod centro;
pub mod criterio_rubrica;
pub mod docente;
pub mod egresado;
pub mod enums;
pub mod escuela;
pub mod espacio_academico;
pub mod estudiante;
pub mod evidencia;
pub mod facultad;
pub mod instituto;
pub mod matricula;
pub mod personal_administrativo;
pub mod persona;
pub mod plan_de_estudios;
pub mod programa_academico;
pub mod proposito_formacion;
pub mod resultado_aprendizaje;
pub mod rubrica;
pub mod unidad_academica;
pub mod validador;
