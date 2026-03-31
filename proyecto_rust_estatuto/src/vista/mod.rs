//! # Módulo de vista
//!
//! Implementa la interfaz gráfica de usuario del sistema de información
//! académico-administrativo de la Universidad Distrital Francisco José de Caldas.
//!
//! ## Tecnología
//! Se utiliza **egui/eframe** — un framework de GUI inmediata (Immediate Mode GUI)
//! puro en Rust, equivalente a JavaFX en el proyecto Java.
//!
//! ## Equivalencia Java → Rust
//! - `VistaPrincipal extends Application` (JavaFX) → struct `VistaPrincipal` + `impl eframe::App`
//! - `TabPane` (JavaFX) → panel lateral con `selectable_label`
//! - `TextField` (JavaFX) → `egui::TextEdit`
//! - `ComboBox` (JavaFX) → `egui::ComboBox`

pub mod vista_principal;

pub use vista_principal::VistaPrincipal;
