import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

type ProjectInfo = {
  id: string;
  name: string;
  path: string;
  created_at: string;
  version: string;
  db_path: string;
};

export default function App() {
  const [projectPath, setProjectPath] = useState("");
  const [projectName, setProjectName] = useState("");
  const [rootPath, setRootPath] = useState("");
  const [status, setStatus] = useState("Ready.");

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
              onChange={(event) => setProjectPath(event.target.value)}
              placeholder="/Users/you/Dev/my-assay-project"
            />
          </label>
          <label>
            Project Name
            <input
              value={projectName}
              onChange={(event) => setProjectName(event.target.value)}
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
              onChange={(event) => setRootPath(event.target.value)}
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
        <h2>Status</h2>
        <pre className="status">{status}</pre>
      </section>
    </div>
  );
}
