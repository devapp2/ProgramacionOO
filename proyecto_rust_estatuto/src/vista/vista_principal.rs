//! # VistaPrincipal — interfaz gráfica con egui/eframe
//!
//! Implementa la interfaz de usuario del sistema de información académico-administrativo
//! de la Universidad Distrital Francisco José de Caldas.
//!
//! ## Equivalencia Java → Rust
//! La clase `VistaPrincipal extends Application` (JavaFX) se implementa en Rust
//! como una struct que implementa el trait `eframe::App`.
//!
//! ## Estructura de la GUI
//! ```
//! ┌─────────────────────────────────────────────────────┐
//! │  Sistema Académico — Universidad Distrital UD       │
//! ├───────────────┬─────────────────────────────────────┤
//! │  Panel lateral│  Área de contenido                  │
//! │               │                                     │
//! │  [Facultades] │  Lista + Formulario de la pestaña   │
//! │  [Docentes  ] │  activa                             │
//! │  [Estudiantes]│                                     │
//! │  [Programas ] │                                     │
//! └───────────────┴─────────────────────────────────────┘
//! ```
//!
//! ## Propósito pedagógico
//! Este módulo demuestra:
//! 1. Cómo implementar un trait externo (`eframe::App`) en Rust.
//! 2. Cómo manejar estado mutable en una aplicación GUI (campos de formulario).
//! 3. Cómo integrar la capa de vista con los controladores (patrón MVC).

use crate::controlador::{
    DocenteControlador, EstudianteControlador, FacultadControlador, ProgramaControlador,
};
use crate::modelo::enums::{EstadoEstudiante, NivelPrograma, TipoVinculacion};
use crate::modelo::docente::Docente;
use crate::modelo::estudiante::Estudiante;
use crate::modelo::facultad::Facultad;
use crate::modelo::programa_academico::ProgramaAcademico;
use chrono::NaiveDate;
use eframe::egui;

// ============================================================
//  Enumeración de pestañas
// ============================================================

/// Pestañas disponibles en el panel lateral de la aplicación.
///
/// Equivale al `TabPane` de JavaFX con sus pestañas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabActiva {
    Facultades,
    Docentes,
    Estudiantes,
    Programas,
}

// ============================================================
//  Estado del formulario de Facultad
// ============================================================

/// Estado del formulario para crear/editar una Facultad.
///
/// En egui, el estado mutable de la UI se mantiene en la propia struct
/// de la aplicación (equivalente a los campos JavaFX `TextField`, etc.).
#[derive(Default, Debug, Clone)]
struct FormFacultad {
    nombre: String,
    codigo: String,
    director: String,
    decano: String,
    mensaje: String,
}

// ============================================================
//  Estado del formulario de Docente
// ============================================================

#[derive(Default, Debug, Clone)]
struct FormDocente {
    nombre: String,
    identificacion: String,
    correo: String,
    escuela_codigo: String,
    categoria: String,
    tipo_vinculacion: TipoVinculacion,
    mensaje: String,
}

impl Default for TipoVinculacion {
    fn default() -> Self {
        TipoVinculacion::Planta
    }
}

// ============================================================
//  Estado del formulario de Estudiante
// ============================================================

#[derive(Debug, Clone)]
struct FormEstudiante {
    nombre: String,
    identificacion: String,
    correo: String,
    codigo_estudiantil: String,
    programa_codigo: String,
    semestre: String,
    estado: EstadoEstudiante,
    mensaje: String,
}

impl Default for FormEstudiante {
    fn default() -> Self {
        FormEstudiante {
            nombre: String::new(),
            identificacion: String::new(),
            correo: String::new(),
            codigo_estudiantil: String::new(),
            programa_codigo: String::new(),
            semestre: "1".to_string(),
            estado: EstadoEstudiante::Activo,
            mensaje: String::new(),
        }
    }
}

// ============================================================
//  Estado del formulario de Programa
// ============================================================

#[derive(Debug, Clone)]
struct FormPrograma {
    nombre: String,
    codigo: String,
    modalidad: String,
    facultad_codigo: String,
    nivel: NivelPrograma,
    mensaje: String,
}

impl Default for FormPrograma {
    fn default() -> Self {
        FormPrograma {
            nombre: String::new(),
            codigo: String::new(),
            modalidad: "Presencial".to_string(),
            facultad_codigo: String::new(),
            nivel: NivelPrograma::Pregrado,
            mensaje: String::new(),
        }
    }
}

// ============================================================
//  Aplicación principal
// ============================================================

/// Aplicación principal del sistema de información académico-administrativo.
///
/// Equivale a la clase `VistaPrincipal extends Application` de JavaFX.
/// Implementa el trait `eframe::App` que requiere el método `update`.
pub struct VistaPrincipal {
    /// Pestaña activa en el panel lateral.
    tab_activa: TabActiva,

    /// Controladores de dominio (capa MVC).
    ctrl_facultad: FacultadControlador,
    ctrl_docente: DocenteControlador,
    ctrl_estudiante: EstudianteControlador,
    ctrl_programa: ProgramaControlador,

    /// Formularios de entrada de datos.
    form_facultad: FormFacultad,
    form_docente: FormDocente,
    form_estudiante: FormEstudiante,
    form_programa: FormPrograma,

    /// Caché de entidades para mostrar en la lista (se recarga desde el DAO).
    facultades: Vec<crate::modelo::facultad::Facultad>,
    docentes: Vec<crate::modelo::docente::Docente>,
    estudiantes: Vec<crate::modelo::estudiante::Estudiante>,
    programas: Vec<crate::modelo::programa_academico::ProgramaAcademico>,
}

impl VistaPrincipal {
    /// Crea una nueva instancia de la aplicación.
    ///
    /// Inicializa los controladores apuntando al directorio `datos/`.
    pub fn nueva() -> Self {
        let ruta = "datos";
        let mut app = VistaPrincipal {
            tab_activa: TabActiva::Facultades,
            ctrl_facultad: FacultadControlador::nuevo(ruta),
            ctrl_docente: DocenteControlador::nuevo(ruta),
            ctrl_estudiante: EstudianteControlador::nuevo(ruta),
            ctrl_programa: ProgramaControlador::nuevo(ruta),
            form_facultad: FormFacultad::default(),
            form_docente: FormDocente::default(),
            form_estudiante: FormEstudiante::default(),
            form_programa: FormPrograma::default(),
            facultades: Vec::new(),
            docentes: Vec::new(),
            estudiantes: Vec::new(),
            programas: Vec::new(),
        };
        app.recargar_facultades();
        app.recargar_docentes();
        app.recargar_estudiantes();
        app.recargar_programas();
        app
    }

    fn recargar_facultades(&mut self) {
        self.facultades = self.ctrl_facultad.listar_todas().unwrap_or_default();
    }

    fn recargar_docentes(&mut self) {
        self.docentes = self.ctrl_docente.listar_todos().unwrap_or_default();
    }

    fn recargar_estudiantes(&mut self) {
        self.estudiantes = self.ctrl_estudiante.listar_todos().unwrap_or_default();
    }

    fn recargar_programas(&mut self) {
        self.programas = self.ctrl_programa.listar_todos().unwrap_or_default();
    }

    // ----------------------------------------------------------
    //  Renderizado por pestaña
    // ----------------------------------------------------------

    /// Renderiza la pestaña de Facultades.
    ///
    /// Muestra la lista de facultades y un formulario para agregar nuevas.
    fn render_facultades(&mut self, ui: &mut egui::Ui) {
        ui.heading("Facultades");
        ui.separator();

        // ── Lista de facultades ──────────────────────────────
        ui.label("Facultades registradas:");
        egui::ScrollArea::vertical()
            .max_height(200.0)
            .show(ui, |ui| {
                for f in &self.facultades {
                    ui.label(format!("[{}] {} — Decano: {}", f.base.codigo, f.base.nombre, f.decano));
                }
                if self.facultades.is_empty() {
                    ui.label("(No hay facultades registradas)");
                }
            });

        ui.separator();
        ui.heading("Registrar nueva Facultad");

        // ── Formulario ───────────────────────────────────────
        egui::Grid::new("form_facultad")
            .num_columns(2)
            .spacing([10.0, 6.0])
            .show(ui, |ui| {
                ui.label("Nombre:");
                ui.text_edit_singleline(&mut self.form_facultad.nombre);
                ui.end_row();

                ui.label("Código:");
                ui.text_edit_singleline(&mut self.form_facultad.codigo);
                ui.end_row();

                ui.label("Director:");
                ui.text_edit_singleline(&mut self.form_facultad.director);
                ui.end_row();

                ui.label("Decano:");
                ui.text_edit_singleline(&mut self.form_facultad.decano);
                ui.end_row();
            });

        if ui.button("Registrar Facultad").clicked() {
            let fecha = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
            let resultado = Facultad::nueva(
                &self.form_facultad.nombre,
                &self.form_facultad.codigo,
                fecha,
                &self.form_facultad.director,
                &self.form_facultad.decano,
            )
            .and_then(|f| self.ctrl_facultad.registrar(f));

            match resultado {
                Ok(()) => {
                    self.form_facultad.mensaje = "✓ Facultad registrada correctamente".to_string();
                    self.form_facultad.nombre.clear();
                    self.form_facultad.codigo.clear();
                    self.form_facultad.director.clear();
                    self.form_facultad.decano.clear();
                    self.recargar_facultades();
                }
                Err(e) => {
                    self.form_facultad.mensaje = format!("✗ Error: {}", e);
                }
            }
        }

        if !self.form_facultad.mensaje.is_empty() {
            ui.label(&self.form_facultad.mensaje.clone());
        }
    }

    /// Renderiza la pestaña de Docentes.
    fn render_docentes(&mut self, ui: &mut egui::Ui) {
        ui.heading("Docentes");
        ui.separator();

        egui::ScrollArea::vertical()
            .max_height(200.0)
            .show(ui, |ui| {
                for d in &self.docentes {
                    ui.label(format!(
                        "[{}] {} — {} | {}",
                        d.base.identificacion, d.base.nombre, d.tipo_vinculacion, d.categoria
                    ));
                }
                if self.docentes.is_empty() {
                    ui.label("(No hay docentes registrados)");
                }
            });

        ui.separator();
        ui.heading("Registrar nuevo Docente");

        egui::Grid::new("form_docente")
            .num_columns(2)
            .spacing([10.0, 6.0])
            .show(ui, |ui| {
                ui.label("Nombre:");
                ui.text_edit_singleline(&mut self.form_docente.nombre);
                ui.end_row();

                ui.label("Identificación:");
                ui.text_edit_singleline(&mut self.form_docente.identificacion);
                ui.end_row();

                ui.label("Correo (opcional):");
                ui.text_edit_singleline(&mut self.form_docente.correo);
                ui.end_row();

                ui.label("Código Escuela:");
                ui.text_edit_singleline(&mut self.form_docente.escuela_codigo);
                ui.end_row();

                ui.label("Categoría:");
                ui.text_edit_singleline(&mut self.form_docente.categoria);
                ui.end_row();

                ui.label("Tipo Vinculación:");
                egui::ComboBox::from_id_salt("combo_vinculacion")
                    .selected_text(self.form_docente.tipo_vinculacion.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.form_docente.tipo_vinculacion, TipoVinculacion::Planta, "Docente de Planta");
                        ui.selectable_value(&mut self.form_docente.tipo_vinculacion, TipoVinculacion::Ocasional, "Docente Ocasional");
                        ui.selectable_value(&mut self.form_docente.tipo_vinculacion, TipoVinculacion::HoraCatedra, "Docente de Hora Cátedra");
                        ui.selectable_value(&mut self.form_docente.tipo_vinculacion, TipoVinculacion::Visitante, "Docente Visitante");
                        ui.selectable_value(&mut self.form_docente.tipo_vinculacion, TipoVinculacion::Experto, "Experto Temático");
                    });
                ui.end_row();
            });

        if ui.button("Registrar Docente").clicked() {
            let correo = if self.form_docente.correo.is_empty() {
                None
            } else {
                Some(self.form_docente.correo.clone())
            };
            let docente = Docente::nuevo(
                &self.form_docente.nombre,
                &self.form_docente.identificacion,
                correo,
                self.form_docente.tipo_vinculacion,
                &self.form_docente.escuela_codigo,
                &self.form_docente.categoria,
            );
            match self.ctrl_docente.registrar(docente) {
                Ok(()) => {
                    self.form_docente.mensaje = "✓ Docente registrado correctamente".to_string();
                    self.recargar_docentes();
                }
                Err(e) => {
                    self.form_docente.mensaje = format!("✗ Error: {}", e);
                }
            }
        }

        if !self.form_docente.mensaje.is_empty() {
            ui.label(&self.form_docente.mensaje.clone());
        }
    }

    /// Renderiza la pestaña de Estudiantes.
    fn render_estudiantes(&mut self, ui: &mut egui::Ui) {
        ui.heading("Estudiantes");
        ui.separator();

        egui::ScrollArea::vertical()
            .max_height(200.0)
            .show(ui, |ui| {
                for e in &self.estudiantes {
                    ui.label(format!(
                        "[{}] {} — Sem: {} | {} | {}",
                        e.codigo_estudiantil, e.base.nombre, e.semestre, e.estado, e.programa_codigo
                    ));
                }
                if self.estudiantes.is_empty() {
                    ui.label("(No hay estudiantes registrados)");
                }
            });

        ui.separator();
        ui.heading("Registrar nuevo Estudiante");

        egui::Grid::new("form_estudiante")
            .num_columns(2)
            .spacing([10.0, 6.0])
            .show(ui, |ui| {
                ui.label("Nombre:");
                ui.text_edit_singleline(&mut self.form_estudiante.nombre);
                ui.end_row();

                ui.label("Identificación:");
                ui.text_edit_singleline(&mut self.form_estudiante.identificacion);
                ui.end_row();

                ui.label("Código Estudiantil:");
                ui.text_edit_singleline(&mut self.form_estudiante.codigo_estudiantil);
                ui.end_row();

                ui.label("Código Programa:");
                ui.text_edit_singleline(&mut self.form_estudiante.programa_codigo);
                ui.end_row();

                ui.label("Semestre:");
                ui.text_edit_singleline(&mut self.form_estudiante.semestre);
                ui.end_row();

                ui.label("Estado:");
                egui::ComboBox::from_id_salt("combo_estado")
                    .selected_text(self.form_estudiante.estado.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.form_estudiante.estado, EstadoEstudiante::Admitido, "Admitido");
                        ui.selectable_value(&mut self.form_estudiante.estado, EstadoEstudiante::Activo, "Activo");
                        ui.selectable_value(&mut self.form_estudiante.estado, EstadoEstudiante::Suspendido, "Suspendido");
                    });
                ui.end_row();
            });

        if ui.button("Registrar Estudiante").clicked() {
            let semestre: u8 = self.form_estudiante.semestre.parse().unwrap_or(0);
            let correo = if self.form_estudiante.correo.is_empty() {
                None
            } else {
                Some(self.form_estudiante.correo.clone())
            };
            let resultado = Estudiante::nuevo(
                &self.form_estudiante.nombre,
                &self.form_estudiante.identificacion,
                correo,
                &self.form_estudiante.codigo_estudiantil,
                &self.form_estudiante.programa_codigo,
                semestre,
                self.form_estudiante.estado,
            )
            .and_then(|e| self.ctrl_estudiante.registrar(e));

            match resultado {
                Ok(()) => {
                    self.form_estudiante.mensaje = "✓ Estudiante registrado correctamente".to_string();
                    self.recargar_estudiantes();
                }
                Err(e) => {
                    self.form_estudiante.mensaje = format!("✗ Error: {}", e);
                }
            }
        }

        if !self.form_estudiante.mensaje.is_empty() {
            ui.label(&self.form_estudiante.mensaje.clone());
        }
    }

    /// Renderiza la pestaña de Programas Académicos.
    fn render_programas(&mut self, ui: &mut egui::Ui) {
        ui.heading("Programas Académicos");
        ui.separator();

        egui::ScrollArea::vertical()
            .max_height(200.0)
            .show(ui, |ui| {
                for p in &self.programas {
                    ui.label(format!(
                        "[{}] {} — {} | {} | Fac: {}",
                        p.codigo, p.nombre, p.nivel, p.modalidad, p.facultad_codigo
                    ));
                }
                if self.programas.is_empty() {
                    ui.label("(No hay programas registrados)");
                }
            });

        ui.separator();
        ui.heading("Registrar nuevo Programa");

        egui::Grid::new("form_programa")
            .num_columns(2)
            .spacing([10.0, 6.0])
            .show(ui, |ui| {
                ui.label("Nombre:");
                ui.text_edit_singleline(&mut self.form_programa.nombre);
                ui.end_row();

                ui.label("Código:");
                ui.text_edit_singleline(&mut self.form_programa.codigo);
                ui.end_row();

                ui.label("Modalidad:");
                ui.text_edit_singleline(&mut self.form_programa.modalidad);
                ui.end_row();

                ui.label("Código Facultad:");
                ui.text_edit_singleline(&mut self.form_programa.facultad_codigo);
                ui.end_row();

                ui.label("Nivel:");
                egui::ComboBox::from_id_salt("combo_nivel")
                    .selected_text(self.form_programa.nivel.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.form_programa.nivel, NivelPrograma::Pregrado, "Pregrado");
                        ui.selectable_value(&mut self.form_programa.nivel, NivelPrograma::Especializacion, "Especialización");
                        ui.selectable_value(&mut self.form_programa.nivel, NivelPrograma::Maestria, "Maestría");
                        ui.selectable_value(&mut self.form_programa.nivel, NivelPrograma::Doctorado, "Doctorado");
                    });
                ui.end_row();
            });

        if ui.button("Registrar Programa").clicked() {
            let programa = ProgramaAcademico::nuevo(
                &self.form_programa.nombre,
                &self.form_programa.codigo,
                self.form_programa.nivel,
                &self.form_programa.modalidad,
                &self.form_programa.facultad_codigo,
            );
            match self.ctrl_programa.registrar(programa) {
                Ok(()) => {
                    self.form_programa.mensaje = "✓ Programa registrado correctamente".to_string();
                    self.recargar_programas();
                }
                Err(e) => {
                    self.form_programa.mensaje = format!("✗ Error: {}", e);
                }
            }
        }

        if !self.form_programa.mensaje.is_empty() {
            ui.label(&self.form_programa.mensaje.clone());
        }
    }
}

// ============================================================
//  Implementación del trait eframe::App
// ============================================================

impl eframe::App for VistaPrincipal {
    /// Método principal del ciclo de renderizado de egui.
    ///
    /// Se invoca en cada frame (aprox. 60 fps). Equivale al método `start(Stage)`
    /// de JavaFX, pero con un modelo de renderizado inmediato (Immediate Mode GUI).
    ///
    /// ## Immediate Mode vs Retained Mode
    /// JavaFX usa un modelo de árbol de escena retenido (Retained Mode):
    /// los nodos existen en memoria y se actualizan cuando cambian.
    ///
    /// egui usa Immediate Mode: la UI se "redibuja" completa en cada frame,
    /// lo que simplifica el manejo de estado pero cambia el paradigma.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ── Panel lateral (menú de navegación) ───────────────
        egui::SidePanel::left("panel_navegacion")
            .default_width(160.0)
            .show(ctx, |ui| {
                ui.heading("Sistema UD");
                ui.separator();
                ui.label("Acuerdo 004 de 2025");
                ui.separator();

                // Botones de navegación — equivalen a las pestañas del TabPane de JavaFX
                if ui
                    .selectable_label(self.tab_activa == TabActiva::Facultades, "Facultades")
                    .clicked()
                {
                    self.tab_activa = TabActiva::Facultades;
                    self.recargar_facultades();
                }
                if ui
                    .selectable_label(self.tab_activa == TabActiva::Docentes, "Docentes")
                    .clicked()
                {
                    self.tab_activa = TabActiva::Docentes;
                    self.recargar_docentes();
                }
                if ui
                    .selectable_label(self.tab_activa == TabActiva::Estudiantes, "Estudiantes")
                    .clicked()
                {
                    self.tab_activa = TabActiva::Estudiantes;
                    self.recargar_estudiantes();
                }
                if ui
                    .selectable_label(self.tab_activa == TabActiva::Programas, "Programas")
                    .clicked()
                {
                    self.tab_activa = TabActiva::Programas;
                    self.recargar_programas();
                }

                ui.separator();
                ui.small("Universidad Distrital\nFrancisco José de Caldas");
            });

        // ── Panel central (contenido de la pestaña activa) ───
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.tab_activa {
                TabActiva::Facultades => self.render_facultades(ui),
                TabActiva::Docentes => self.render_docentes(ui),
                TabActiva::Estudiantes => self.render_estudiantes(ui),
                TabActiva::Programas => self.render_programas(ui),
            }
        });
    }
}
