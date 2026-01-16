use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct EvalDefinition {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub dataset: DatasetConfig,
    pub solver: SolverConfig,
    pub scorer: Vec<ScorerConfig>,
    pub execution: ExecutionConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetConfig {
    pub source: String,
    pub path: String,
    #[serde(default)]
    pub split: Option<String>,
    #[serde(default)]
    pub limit: Option<u32>,
    #[serde(default)]
    pub shuffle: Option<bool>,
    #[serde(default)]
    pub seed: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolverConfig {
    #[serde(rename = "type")]
    pub solver_type: String,
    #[serde(default)]
    pub system_prompt: Option<String>,
    #[serde(default)]
    pub tools: Option<Vec<Value>>,
    #[serde(default)]
    pub max_turns: Option<u32>,
    #[serde(default)]
    pub sandbox: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScorerConfig {
    #[serde(rename = "type")]
    pub scorer_type: String,
    #[serde(flatten)]
    pub config: serde_json::Map<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionConfig {
    pub max_concurrent: u32,
    pub timeout_seconds: u32,
    pub retries: u32,
    pub model: ModelRef,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModelRef {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Serialize)]
pub struct EvalSummary {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub path: String,
}

pub fn load_eval_file(path: &Path) -> Result<EvalDefinition, String> {
    let contents =
        fs::read_to_string(path).map_err(|err| format!("Read error {}: {err}", path.display()))?;
    serde_yaml::from_str(&contents)
        .map_err(|err| format!("YAML error {}: {err}", path.display()))
}

pub fn list_eval_summaries(project_path: &Path) -> Result<Vec<EvalSummary>, String> {
    let evals_dir = project_path.join("evals");
    if !evals_dir.exists() {
        return Ok(Vec::new());
    }

    let entries =
        fs::read_dir(&evals_dir).map_err(|err| format!("Read evals dir error: {err}"))?;
    let mut summaries = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|err| format!("Dir entry error: {err}"))?;
        if !entry
            .file_type()
            .map_err(|err| format!("File type error: {err}"))?
            .is_file()
        {
            continue;
        }

        let path = entry.path();
        if !is_yaml_file(&path) {
            continue;
        }

        let eval = load_eval_file(&path)?;
        summaries.push(EvalSummary {
            id: eval.id,
            name: eval.name,
            description: eval.description,
            path: path.to_string_lossy().to_string(),
        });
    }

    summaries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(summaries)
}

fn is_yaml_file(path: &PathBuf) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("yaml") | Some("yml")
    )
}
