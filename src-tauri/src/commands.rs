use crate::db;
use crate::eval;
use crate::project;
use std::path::PathBuf;

#[tauri::command]
pub fn create_project(path: String, name: String) -> Result<project::ProjectInfo, String> {
    let project_path = PathBuf::from(path);
    let project_info = project::create_project(&project_path, &name)?;
    db::init_db(&PathBuf::from(&project_info.db_path))?;
    Ok(project_info)
}

#[tauri::command]
pub fn open_project(path: String) -> Result<project::ProjectInfo, String> {
    let project_path = PathBuf::from(path);
    let project_info = project::open_project(&project_path)?;
    db::init_db(&PathBuf::from(&project_info.db_path))?;
    Ok(project_info)
}

#[tauri::command]
pub fn list_projects(root_path: String) -> Result<Vec<project::ProjectInfo>, String> {
    let root = PathBuf::from(root_path);
    project::list_projects(&root)
}

#[tauri::command]
pub fn list_evals(project_path: String) -> Result<Vec<eval::EvalSummary>, String> {
    let path = PathBuf::from(project_path);
    eval::list_eval_summaries(&path)
}
