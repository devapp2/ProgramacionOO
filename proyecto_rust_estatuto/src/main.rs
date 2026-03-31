//! # Sistema de Información Académico-Administrativo
//! ## Universidad Distrital Francisco José de Caldas
//!
//! Implementación en **Rust** del modelo de dominio basado en el
//! **Acuerdo 004 de 2025 — Estatuto General de la Universidad Distrital**.
//!
//! Este proyecto es la versión Rust del sistema equivalente en Java, desarrollado
//! como material didáctico para el curso de Programación Orientada a Objetos.
//!
//! ## Propósito educativo
//! El proyecto demuestra la traducción de conceptos POO de Java a Rust:
//! - Clases abstractas → traits + structs (composición)
//! - Herencia → composición con structs base
//! - Interfaces → traits
//! - Excepciones → `Result<T, EstatutoError>` con `thiserror`
//! - Colecciones → `Vec<T>`, `Option<T>` idiomáticos
//! - Patrones GoF → implementaciones idiomáticas en Rust
//!
//! ## Ejecución
//! ```bash
//! cargo run        # Lanza la aplicación con GUI
//! cargo test       # Ejecuta todas las pruebas unitarias
//! cargo clippy     # Verificación de estilo y calidad
//! ```
//!
//! ## Autor
//! Roberto Albeiro Pava-Díaz <rpavad@udistrital.edu.co>

// ── Declaración de módulos ────────────────────────────────────────────────────
pub mod controlador;
pub mod excepcion;
pub mod modelo;
pub mod patron;
pub mod persistencia;
pub mod vista;

use eframe::egui;
use vista::VistaPrincipal;

// ── Función principal ─────────────────────────────────────────────────────────

fn main() -> eframe::Result<()> {
    // Configuración de la ventana principal de la aplicación egui/eframe.
    // Equivale al método `start(Stage primaryStage)` de JavaFX.
    let opciones = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Sistema Académico — Universidad Distrital UD")
            .with_inner_size([900.0, 650.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    // Lanzar la aplicación con la vista principal.
    // eframe llama a `VistaPrincipal::update()` en cada frame del ciclo de renderizado.
    eframe::run_native(
        "Estatuto UD",
        opciones,
        Box::new(|_cc| Ok(Box::new(VistaPrincipal::nueva()))),
    )
}
