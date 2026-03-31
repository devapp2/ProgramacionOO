//! # EstudianteControlador — capa de control para Estudiantes
//!
//! Gestiona las operaciones CRUD sobre los `Estudiante`s del sistema,
//! conforme al régimen estudiantil del Acuerdo 004 de 2025.

use crate::excepcion::errores::EstatutoResult;
use crate::modelo::enums::EstadoEstudiante;
use crate::modelo::estudiante::Estudiante;
use crate::persistencia::dao::GenericDao;
use crate::persistencia::json_dao::JsonDao;

/// Controlador para la gestión de Estudiantes.
pub struct EstudianteControlador {
    dao: JsonDao<Estudiante>,
}

impl EstudianteControlador {
    /// Crea un nuevo controlador usando el archivo JSON en la ruta dada.
    pub fn nuevo(ruta_datos: &str) -> Self {
        let ruta = format!("{}/estudiantes.json", ruta_datos);
        EstudianteControlador {
            dao: JsonDao::nuevo(ruta, |e: &Estudiante| e.codigo_estudiantil.clone()),
        }
    }

    /// Lista todos los estudiantes del sistema.
    pub fn listar_todos(&self) -> EstatutoResult<Vec<Estudiante>> {
        self.dao.buscar_todos()
    }

    /// Registra un nuevo Estudiante.
    pub fn registrar(&mut self, estudiante: Estudiante) -> EstatutoResult<()> {
        self.dao.guardar(estudiante)
    }

    /// Busca un Estudiante por su código estudiantil.
    pub fn buscar(&self, codigo_estudiantil: &str) -> EstatutoResult<Option<Estudiante>> {
        self.dao.buscar_por_id(&codigo_estudiantil.to_string())
    }

    /// Actualiza los datos de un Estudiante existente.
    pub fn actualizar(&mut self, estudiante: Estudiante) -> EstatutoResult<()> {
        self.dao.actualizar(estudiante)
    }

    /// Elimina un Estudiante del sistema.
    pub fn eliminar(&mut self, codigo_estudiantil: &str) -> EstatutoResult<bool> {
        self.dao.eliminar(&codigo_estudiantil.to_string())
    }

    /// Lista solo los estudiantes con estado `Activo`.
    pub fn listar_activos(&self) -> EstatutoResult<Vec<Estudiante>> {
        let todos = self.dao.buscar_todos()?;
        Ok(todos.into_iter().filter(|e| e.esta_activo()).collect())
    }

    /// Lista estudiantes de un programa específico.
    pub fn listar_por_programa(&self, programa_codigo: &str) -> EstatutoResult<Vec<Estudiante>> {
        let todos = self.dao.buscar_todos()?;
        let codigo = programa_codigo.to_uppercase();
        Ok(todos
            .into_iter()
            .filter(|e| e.programa_codigo == codigo)
            .collect())
    }

    /// Cambia el estado de un estudiante por su código estudiantil.
    pub fn cambiar_estado(
        &mut self,
        codigo_estudiantil: &str,
        nuevo_estado: EstadoEstudiante,
    ) -> EstatutoResult<()> {
        let mut estudiante = self
            .buscar(codigo_estudiantil)?
            .ok_or_else(|| {
                crate::excepcion::errores::EstatutoError::NoEncontrada(
                    codigo_estudiantil.to_string(),
                )
            })?;
        estudiante.cambiar_estado(nuevo_estado);
        self.actualizar(estudiante)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modelo::enums::EstadoEstudiante;
    use std::env;

    fn controlador_temporal() -> (EstudianteControlador, std::path::PathBuf) {
        let dir = env::temp_dir().join(format!("ctrl_est_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        (EstudianteControlador::nuevo(dir.to_str().unwrap()), dir)
    }

    fn estudiante(codigo: &str) -> Estudiante {
        Estudiante::nuevo("Nombre", "CC-001", None, codigo, "PROG-SIS", 3, EstadoEstudiante::Activo).unwrap()
    }

    #[test]
    fn registrar_y_listar_activos() {
        let (mut ctrl, dir) = controlador_temporal();
        ctrl.registrar(estudiante("EST-001")).unwrap();
        let activos = ctrl.listar_activos().unwrap();
        assert_eq!(activos.len(), 1);
        std::fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn cambiar_estado_ok() {
        let (mut ctrl, dir) = controlador_temporal();
        ctrl.registrar(estudiante("EST-002")).unwrap();
        ctrl.cambiar_estado("EST-002", EstadoEstudiante::Graduado).unwrap();
        let e = ctrl.buscar("EST-002").unwrap().unwrap();
        assert_eq!(e.estado, EstadoEstudiante::Graduado);
        std::fs::remove_dir_all(dir).ok();
    }
}
