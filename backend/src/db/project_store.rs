use redb::{ReadableTable, TableDefinition};

use super::error::DbError;
use super::init::DbState;
use crate::models::project::Project;

const PROJECT_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("projects");
const BINCODE_CONFIG: bincode::config::Configuration = bincode::config::standard();

pub struct ProjectStore {
    encode_buffer: Vec<u8>,
}

impl ProjectStore {
    pub fn new() -> Self {
        Self { encode_buffer: Vec::new() }
    }

    pub fn save_project(&mut self, state: &DbState, project: &Project) -> Result<(), DbError> {
        let tx = state.db.begin_write()?;
        {
            let mut table = tx.open_table(PROJECT_TABLE)?;

            self.encode_buffer.clear();
            bincode::serde::encode_into_std_write(project, &mut self.encode_buffer, BINCODE_CONFIG)?;
            table.insert(project.uuid.as_str(), self.encode_buffer.as_slice())?;
        }
        tx.commit()?;
        Ok(())
    }

    pub fn get_project(&self, state: &DbState, project_uuid: &str) -> Result<Project, DbError> {
        let tx = state.db.begin_read()?;
        let table = tx.open_table(PROJECT_TABLE)?;

        let value = table.get(project_uuid)?;

        match value {
            Some(v) => {
                let bytes = v.value();
                let (project, _): (Project, usize) =
                    bincode::serde::decode_from_slice(bytes, BINCODE_CONFIG)?;
                Ok(project)
            },
            None => Err(DbError::EntityNotFound),
        }
    }

    pub fn update_project(&mut self, state: &DbState, updated_project: &Project) -> Result<(), DbError> {
        let tx = state.db.begin_write()?;
        {
            let mut table = tx.open_table(PROJECT_TABLE)?;
            let project_key = updated_project.uuid.as_str();

            // check if entity exists
            if table.get(project_key)?.is_none() {
                return Err(DbError::EntityNotFound);
            }

            self.encode_buffer.clear();
            bincode::serde::encode_into_std_write(updated_project, &mut self.encode_buffer, BINCODE_CONFIG)?;

            table.insert(project_key, self.encode_buffer.as_slice())?;
        }
        tx.commit()?;
        Ok(())
    }

    pub fn delete_project(&self, state: &DbState, project_uuid: &str) -> Result<(), DbError> {
        let tx = state.db.begin_write()?;

        {
            let mut table = tx.open_table(PROJECT_TABLE)?;

            // Check if the entity key exists before removal
            if table.remove(project_uuid)?.is_none() {
                return Err(DbError::EntityNotFound);
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn list_projects(&self, state: &DbState) -> Result<Vec<Project>, DbError> {
        let tx = state.db.begin_read()?;
        let table = tx.open_table(PROJECT_TABLE)?;
        
        let mut projects = Vec::new();
        for result in table.iter()? {
            let (_, value) = result?;
            let bytes = value.value();
            let (project, _): (Project, usize) = 
                bincode::serde::decode_from_slice(bytes, BINCODE_CONFIG)?;
            projects.push(project);
        }
        Ok(projects)
    }
}