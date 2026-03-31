//! # Módulo de controladores
//!
//! Implementa la capa de control del patrón MVC para el sistema de información
//! académico-administrativo de la Universidad Distrital.
//!
//! ## Controladores disponibles
//! - [`facultad_controlador`]: CRUD de Facultades
//! - [`docente_controlador`]: CRUD de Docentes
//! - [`estudiante_controlador`]: CRUD de Estudiantes
//! - [`programa_controlador`]: CRUD de Programas Académicos

pub mod docente_controlador;
pub mod estudiante_controlador;
pub mod facultad_controlador;
pub mod programa_controlador;

pub use docente_controlador::DocenteControlador;
pub use estudiante_controlador::EstudianteControlador;
pub use facultad_controlador::FacultadControlador;
pub use programa_controlador::ProgramaControlador;
