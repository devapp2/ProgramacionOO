//! # DocenteControlador — capa de control para Docentes
//!
//! Gestiona las operaciones CRUD sobre los `Docente`s del sistema.

use crate::excepcion::errores::EstatutoResult;
use crate::modelo::docente::Docente;
use crate::modelo::enums::TipoVinculacion;
use crate::persistencia::dao::GenericDao;
use crate::persistencia::json_dao::JsonDao;

/// Controlador para la gestión de Docentes.
pub struct DocenteControlador {
    dao: JsonDao<Docente>,
}

impl DocenteControlador {
    /// Crea un nuevo controlador usando el archivo JSON en la ruta dada.
    pub fn nuevo(ruta_datos: &str) -> Self {
        let ruta = format!("{}/docentes.json", ruta_datos);
        DocenteControlador {
            dao: JsonDao::nuevo(ruta, |d: &Docente| d.base.identificacion.clone()),
        }
    }

    /// Lista todos los docentes del sistema.
    pub fn listar_todos(&self) -> EstatutoResult<Vec<Docente>> {
        self.dao.buscar_todos()
    }

    /// Registra un nuevo Docente.
    pub fn registrar(&mut self, docente: Docente) -> EstatutoResult<()> {
        self.dao.guardar(docente)
    }

    /// Busca un Docente por su número de identificación.
    pub fn buscar(&self, identificacion: &str) -> EstatutoResult<Option<Docente>> {
        self.dao.buscar_por_id(&identificacion.to_string())
    }

    /// Actualiza los datos de un Docente existente.
    pub fn actualizar(&mut self, docente: Docente) -> EstatutoResult<()> {
        self.dao.actualizar(docente)
    }

    /// Elimina un Docente del sistema.
    pub fn eliminar(&mut self, identificacion: &str) -> EstatutoResult<bool> {
        self.dao.eliminar(&identificacion.to_string())
    }

    /// Lista docentes filtrados por tipo de vinculación.
    pub fn listar_por_vinculacion(&self, tipo: TipoVinculacion) -> EstatutoResult<Vec<Docente>> {
        let todos = self.dao.buscar_todos()?;
        Ok(todos
            .into_iter()
            .filter(|d| d.tipo_vinculacion == tipo)
            .collect())
    }

    /// Lista docentes de planta.
    pub fn listar_planta(&self) -> EstatutoResult<Vec<Docente>> {
        self.listar_por_vinculacion(TipoVinculacion::Planta)
    }

    /// Lista docentes de una escuela específica.
    pub fn listar_por_escuela(&self, escuela_codigo: &str) -> EstatutoResult<Vec<Docente>> {
        let todos = self.dao.buscar_todos()?;
        let codigo = escuela_codigo.to_uppercase();
        Ok(todos
            .into_iter()
            .filter(|d| d.escuela_codigo == codigo)
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modelo::enums::TipoVinculacion;
    use std::env;

    fn controlador_temporal() -> (DocenteControlador, std::path::PathBuf) {
        let dir = env::temp_dir().join(format!("ctrl_doc_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        let ctrl = DocenteControlador::nuevo(dir.to_str().unwrap());
        (ctrl, dir)
    }

    fn docente(id: &str, tipo: TipoVinculacion) -> Docente {
        Docente::nuevo("Nombre", id, None, tipo, "ESC-SIS", "Titular")
    }

    #[test]
    fn registrar_y_buscar_docente() {
        let (mut ctrl, dir) = controlador_temporal();
        ctrl.registrar(docente("CC-100", TipoVinculacion::Planta)).unwrap();
        assert!(ctrl.buscar("CC-100").unwrap().is_some());
        std::fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn listar_planta() {
        let (mut ctrl, dir) = controlador_temporal();
        ctrl.registrar(docente("CC-201", TipoVinculacion::Planta)).unwrap();
        ctrl.registrar(docente("CC-202", TipoVinculacion::HoraCatedra)).unwrap();
        let planta = ctrl.listar_planta().unwrap();
        assert_eq!(planta.len(), 1);
        std::fs::remove_dir_all(dir).ok();
    }
}
