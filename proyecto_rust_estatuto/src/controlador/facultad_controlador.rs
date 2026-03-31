//! # FacultadControlador — capa de control para Facultades
//!
//! Gestiona las operaciones CRUD sobre las `Facultad`es del sistema,
//! coordinando entre la capa de dominio y la de persistencia.
//!
//! ## Equivalencia Java → Rust
//! La clase `FacultadControlador` de Java usa composición con un `JsonDAO<Facultad>`.
//! En Rust, se usa composición con un `JsonDao<Facultad>` que implementa
//! el trait `GenericDao<Facultad, String>`.

use crate::excepcion::errores::EstatutoResult;
use crate::modelo::facultad::Facultad;
use crate::persistencia::dao::GenericDao;
use crate::persistencia::json_dao::JsonDao;

/// Controlador para la gestión de Facultades.
///
/// Provee métodos de negocio que usan el DAO para persistir y recuperar
/// entidades `Facultad`.
pub struct FacultadControlador {
    dao: JsonDao<Facultad>,
}

impl FacultadControlador {
    /// Crea un nuevo controlador usando el archivo JSON en la ruta dada.
    pub fn nuevo(ruta_datos: &str) -> Self {
        let ruta = format!("{}/facultades.json", ruta_datos);
        FacultadControlador {
            dao: JsonDao::nuevo(ruta, |f: &Facultad| f.base.codigo.clone()),
        }
    }

    /// Lista todas las facultades registradas en el sistema.
    pub fn listar_todas(&self) -> EstatutoResult<Vec<Facultad>> {
        self.dao.buscar_todos()
    }

    /// Registra una nueva Facultad en el sistema.
    ///
    /// # Errores
    /// Retorna error si ya existe una facultad con el mismo código.
    pub fn registrar(&mut self, facultad: Facultad) -> EstatutoResult<()> {
        self.dao.guardar(facultad)
    }

    /// Busca una Facultad por su código institucional.
    pub fn buscar(&self, codigo: &str) -> EstatutoResult<Option<Facultad>> {
        self.dao.buscar_por_id(&codigo.to_uppercase())
    }

    /// Actualiza los datos de una Facultad existente.
    pub fn actualizar(&mut self, facultad: Facultad) -> EstatutoResult<()> {
        self.dao.actualizar(facultad)
    }

    /// Elimina una Facultad del sistema por su código.
    ///
    /// # Retorna
    /// `true` si fue encontrada y eliminada, `false` si no existía.
    pub fn eliminar(&mut self, codigo: &str) -> EstatutoResult<bool> {
        self.dao.eliminar(&codigo.to_uppercase())
    }

    /// Busca facultades cuyo nombre contenga el término dado (búsqueda parcial).
    pub fn buscar_por_nombre(&self, termino: &str) -> EstatutoResult<Vec<Facultad>> {
        let todas = self.dao.buscar_todos()?;
        let termino_lower = termino.to_lowercase();
        Ok(todas
            .into_iter()
            .filter(|f| f.base.nombre.to_lowercase().contains(&termino_lower))
            .collect())
    }

    /// Verifica si existe una Facultad con el código dado.
    pub fn existe(&self, codigo: &str) -> EstatutoResult<bool> {
        Ok(self.buscar(codigo)?.is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modelo::facultad::Facultad;
    use chrono::NaiveDate;
    use std::env;

    fn controlador_temporal() -> (FacultadControlador, std::path::PathBuf) {
        let dir = env::temp_dir().join(format!("ctrl_fac_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        let ctrl = FacultadControlador::nuevo(dir.to_str().unwrap());
        (ctrl, dir)
    }

    fn facultad(codigo: &str) -> Facultad {
        Facultad::nueva(
            &format!("Facultad {}", codigo),
            codigo,
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            "Dir",
            "Decano",
        )
        .unwrap()
    }

    #[test]
    fn registrar_y_listar() {
        let (mut ctrl, dir) = controlador_temporal();
        ctrl.registrar(facultad("FAC-01")).unwrap();
        let todas = ctrl.listar_todas().unwrap();
        assert_eq!(todas.len(), 1);
        std::fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn buscar_existente() {
        let (mut ctrl, dir) = controlador_temporal();
        ctrl.registrar(facultad("FAC-02")).unwrap();
        assert!(ctrl.buscar("FAC-02").unwrap().is_some());
        std::fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn eliminar_facultad() {
        let (mut ctrl, dir) = controlador_temporal();
        ctrl.registrar(facultad("FAC-03")).unwrap();
        assert!(ctrl.eliminar("FAC-03").unwrap());
        std::fs::remove_dir_all(dir).ok();
    }
}
