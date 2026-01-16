use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use uuid::Uuid;

const MANIFEST_FILE: &str = "Assay.toml";
const INTERNAL_DIR: &str = ".assay";
const DB_FILE: &str = "assay.db";

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectManifest {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct ProjectInfo {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    pub created_at: DateTime<Utc>,
    pub version: String,
    pub db_path: String,
}

pub fn create_project(path: &Path, name: &str) -> Result<ProjectInfo, String> {
    if name.trim().is_empty() {
        return Err("Project name is required".to_string());
    }

    fs::create_dir_all(path).map_err(|err| format!("Failed to create project directory: {err}"))?;
    let manifest_path = path.join(MANIFEST_FILE);
    if manifest_path.exists() {
        return Err("Assay.toml already exists in this directory".to_string());
    }

    let manifest = ProjectManifest {
        id: Uuid::new_v4(),
        name: name.trim().to_string(),
        created_at: Utc::now(),
        version: "0.1.0".to_string(),
    };

    let manifest_contents =
        toml::to_string_pretty(&manifest).map_err(|err| format!("TOML error: {err}"))?;
    fs::write(&manifest_path, manifest_contents)
        .map_err(|err| format!("Failed to write Assay.toml: {err}"))?;

    for dir in ["evals", "datasets", "results", "models", "plugins"] {
        fs::create_dir_all(path.join(dir))
            .map_err(|err| format!("Failed to create {dir} directory: {err}"))?;
    }

    let internal_dir = path.join(INTERNAL_DIR);
    fs::create_dir_all(&internal_dir)
        .map_err(|err| format!("Failed to create internal directory: {err}"))?;

    Ok(ProjectInfo {
        id: manifest.id,
        name: manifest.name,
        path: path.to_string_lossy().to_string(),
        created_at: manifest.created_at,
        version: manifest.version,
        db_path: internal_dir.join(DB_FILE).to_string_lossy().to_string(),
    })
}

pub fn open_project(path: &Path) -> Result<ProjectInfo, String> {
    let manifest_path = path.join(MANIFEST_FILE);
    let manifest_contents =
        fs::read_to_string(&manifest_path).map_err(|err| format!("Assay.toml error: {err}"))?;
    let manifest: ProjectManifest =
        toml::from_str(&manifest_contents).map_err(|err| format!("TOML error: {err}"))?;

    let internal_dir = path.join(INTERNAL_DIR);
    fs::create_dir_all(&internal_dir)
        .map_err(|err| format!("Failed to create internal directory: {err}"))?;

    Ok(ProjectInfo {
        id: manifest.id,
        name: manifest.name,
        path: path.to_string_lossy().to_string(),
        created_at: manifest.created_at,
        version: manifest.version,
        db_path: internal_dir.join(DB_FILE).to_string_lossy().to_string(),
    })
}

pub fn list_projects(root: &Path) -> Result<Vec<ProjectInfo>, String> {
    let mut projects = Vec::new();
    let entries = fs::read_dir(root)
        .map_err(|err| format!("Failed to read directory: {err}"))?;

    for entry in entries {
        let entry = entry.map_err(|err| format!("Directory entry error: {err}"))?;
        if !entry
            .file_type()
            .map_err(|err| format!("File type error: {err}"))?
            .is_dir()
        {
            continue;
        }

        let path = entry.path();
        let manifest_path = path.join(MANIFEST_FILE);
        if !manifest_path.exists() {
            continue;
        }

        if let Ok(project) = open_project(&path) {
            projects.push(project);
        }
    }

    projects.sort_by_key(|project| project.created_at);
    Ok(projects)
}

