import { invoke } from "@tauri-apps/api/core";
import { useState, type ChangeEvent } from "react";

type ProjectInfo = {
  id: string;
  name: string;
  path: string;
  created_at: string;
  version: string;
  db_path: string;
};

type EvalSummary = {
  id: string;
  name: string;
  description?: string;
  path: string;
};

export default function App() {
  const [projectPath, setProjectPath] = useState("");
  const [projectName, setProjectName] = useState("");
  const [rootPath, setRootPath] = useState("");
  const [status, setStatus] = useState("Ready.");
  const [evals, setEvals] = useState<EvalSummary[]>([]);

  const createProject = async () => {
    setStatus("Creating project...");
    try {
      const result = await invoke<ProjectInfo>("create_project", {
        path: projectPath,
        name: projectName
      });
      setStatus(JSON.stringify(result, null, 2));
    } catch (error) {
      setStatus(String(error));
    }
  };

  const openProject = async () => {
    setStatus("Opening project...");
    try {
      const result = await invoke<ProjectInfo>("open_project", {
        path: projectPath
      });
      setStatus(JSON.stringify(result, null, 2));
    } catch (error) {
      setStatus(String(error));
    }
  };

  const listProjects = async () => {
    setStatus("Listing projects...");
    try {
      const result = await invoke<ProjectInfo[]>("list_projects", {
        root_path: rootPath
      });
      setStatus(JSON.stringify(result, null, 2));
    } catch (error) {
      setStatus(String(error));
    }
  };

  const listEvals = async () => {
    setStatus("Listing evals...");
    try {
      const result = await invoke<EvalSummary[]>("list_evals", {
        project_path: projectPath
      });
      setEvals(result);
      setStatus(JSON.stringify(result, null, 2));
    } catch (error) {
      setStatus(String(error));
    }
  };

  return (
    <div className="app">
      <header className="app__header">
        <h1>Assay</h1>
        <p>Desktop evaluation workbench for frontier AI systems.</p>
      </header>

      <section className="app__section">
        <h2>Project Workspace</h2>
        <div className="form">
          <label>
            Project Path
            <input
              value={projectPath}
              onChange={(event: ChangeEvent<HTMLInputElement>) =>
                setProjectPath(event.target.value)
              }
              placeholder="/Users/you/Dev/my-assay-project"
            />
          </label>
          <label>
            Project Name
            <input
              value={projectName}
              onChange={(event: ChangeEvent<HTMLInputElement>) =>
                setProjectName(event.target.value)
              }
              placeholder="Safety Audit"
            />
          </label>
          <div className="button-row">
            <button type="button" onClick={createProject}>
              Create Project
            </button>
            <button type="button" onClick={openProject}>
              Open Project
            </button>
          </div>
        </div>
      </section>

      <section className="app__section">
        <h2>Discover Projects</h2>
        <div className="form">
          <label>
            Root Folder
            <input
              value={rootPath}
              onChange={(event: ChangeEvent<HTMLInputElement>) =>
                setRootPath(event.target.value)
              }
              placeholder="/Users/you/Dev"
            />
          </label>
          <div className="button-row">
            <button type="button" onClick={listProjects}>
              List Projects
            </button>
          </div>
        </div>
      </section>

      <section className="app__section">
        <h2>Eval Definitions</h2>
        <p>Reads YAML files from the project's `evals/` directory.</p>
        <div className="button-row">
          <button type="button" onClick={listEvals}>
            List Evals
          </button>
        </div>
        {evals.length > 0 ? (
          <div className="eval-list">
            {evals.map((evalItem: EvalSummary) => (
              <div className="eval-card" key={evalItem.id}>
                <strong>{evalItem.name}</strong>
                <span className="eval-id">{evalItem.id}</span>
                {evalItem.description ? (
                  <p>{evalItem.description}</p>
                ) : (
                  <p className="muted">No description.</p>
                )}
                <code>{evalItem.path}</code>
              </div>
            ))}
          </div>
        ) : (
          <p className="muted">No evals loaded.</p>
        )}
      </section>

      <section className="app__section">
        <h2>Status</h2>
        <pre className="status">{status}</pre>
      </section>
    </div>
  );
}
