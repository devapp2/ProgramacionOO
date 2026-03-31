//! # ProgramaControlador — capa de control para Programas Académicos
//!
//! Gestiona las operaciones CRUD sobre los `ProgramaAcademico`s del sistema,
//! conforme al régimen académico del Acuerdo 004 de 2025.

use crate::excepcion::errores::EstatutoResult;
use crate::modelo::enums::NivelPrograma;
use crate::modelo::programa_academico::ProgramaAcademico;
use crate::persistencia::dao::GenericDao;
use crate::persistencia::json_dao::JsonDao;

/// Controlador para la gestión de Programas Académicos.
pub struct ProgramaControlador {
    dao: JsonDao<ProgramaAcademico>,
}

impl ProgramaControlador {
    /// Crea un nuevo controlador usando el archivo JSON en la ruta dada.
    pub fn nuevo(ruta_datos: &str) -> Self {
        let ruta = format!("{}/programas.json", ruta_datos);
        ProgramaControlador {
            dao: JsonDao::nuevo(ruta, |p: &ProgramaAcademico| p.codigo.clone()),
        }
    }

    /// Lista todos los programas académicos del sistema.
    pub fn listar_todos(&self) -> EstatutoResult<Vec<ProgramaAcademico>> {
        self.dao.buscar_todos()
    }

    /// Registra un nuevo Programa Académico.
    pub fn registrar(&mut self, programa: ProgramaAcademico) -> EstatutoResult<()> {
        self.dao.guardar(programa)
    }

    /// Busca un Programa por su código institucional.
    pub fn buscar(&self, codigo: &str) -> EstatutoResult<Option<ProgramaAcademico>> {
        self.dao.buscar_por_id(&codigo.to_uppercase())
    }

    /// Actualiza los datos de un Programa existente.
    pub fn actualizar(&mut self, programa: ProgramaAcademico) -> EstatutoResult<()> {
        self.dao.actualizar(programa)
    }

    /// Elimina un Programa del sistema.
    pub fn eliminar(&mut self, codigo: &str) -> EstatutoResult<bool> {
        self.dao.eliminar(&codigo.to_uppercase())
    }

    /// Lista programas por nivel de formación.
    pub fn listar_por_nivel(&self, nivel: NivelPrograma) -> EstatutoResult<Vec<ProgramaAcademico>> {
        let todos = self.dao.buscar_todos()?;
        Ok(todos.into_iter().filter(|p| p.nivel == nivel).collect())
    }

    /// Lista solo los programas de pregrado.
    pub fn listar_pregrado(&self) -> EstatutoResult<Vec<ProgramaAcademico>> {
        self.listar_por_nivel(NivelPrograma::Pregrado)
    }

    /// Lista programas de una facultad específica.
    pub fn listar_por_facultad(&self, facultad_codigo: &str) -> EstatutoResult<Vec<ProgramaAcademico>> {
        let todos = self.dao.buscar_todos()?;
        let codigo = facultad_codigo.to_uppercase();
        Ok(todos
            .into_iter()
            .filter(|p| p.facultad_codigo == codigo)
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modelo::enums::NivelPrograma;
    use std::env;

    fn controlador_temporal() -> (ProgramaControlador, std::path::PathBuf) {
        let dir = env::temp_dir().join(format!("ctrl_prog_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        (ProgramaControlador::nuevo(dir.to_str().unwrap()), dir)
    }

    fn programa(codigo: &str, nivel: NivelPrograma) -> ProgramaAcademico {
        ProgramaAcademico::nuevo(&format!("Programa {}", codigo), codigo, nivel, "Presencial", "FAC-ING")
    }

    #[test]
    fn registrar_y_listar_pregrado() {
        let (mut ctrl, dir) = controlador_temporal();
        ctrl.registrar(programa("P-01", NivelPrograma::Pregrado)).unwrap();
        ctrl.registrar(programa("P-02", NivelPrograma::Maestria)).unwrap();
        let pregrado = ctrl.listar_pregrado().unwrap();
        assert_eq!(pregrado.len(), 1);
        std::fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn buscar_programa_existente() {
        let (mut ctrl, dir) = controlador_temporal();
        ctrl.registrar(programa("P-03", NivelPrograma::Doctorado)).unwrap();
        assert!(ctrl.buscar("P-03").unwrap().is_some());
        std::fs::remove_dir_all(dir).ok();
    }
}
