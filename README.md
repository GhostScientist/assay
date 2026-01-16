# Assay: Product Requirements Document

**A Desktop Evaluation Workbench for Frontier AI Systems**

Version 1.0 | January 2026

---

## Executive Summary

Assay is a Tauri-based desktop application that provides a unified, fast, and extensible workbench for conducting evaluations, benchmarks, and capability assessments of AI models and agents. Built with Rust for performance-critical operations and React/TypeScript for a modern UI, Assay bridges the gap between heavyweight CLI-based evaluation harnesses (like lm-evaluation-harness, Inspect AI) and developer-friendly observability platforms (like Langfuse, Braintrust).

**The core insight**: Evaluation is not just about running benchmarks—it's about *understanding* AI systems through iterative experimentation. Assay treats evaluations as first-class development artifacts, with version control, collaborative annotation, and visual trace exploration.

---

## Problem Statement

### Current Landscape Gaps

1. **CLI Fragmentation**: SWE-bench, lm-evaluation-harness, HELM, and Inspect AI are powerful but require terminal expertise, custom scripting, and offer limited visual feedback during runs.

2. **Observability ≠ Evaluation**: Platforms like Langfuse and Braintrust excel at production monitoring but are not designed for systematic pre-deployment capability assessment.

3. **Agentic Evaluation Complexity**: Testing agents on SWE-bench or multi-step tasks requires Docker orchestration, trace collection, and environment management—all outside typical developer workflows.

4. **No Single Pane of Glass**: Researchers juggle multiple tools, losing context switching between running evals, analyzing traces, comparing models, and sharing results.

### Target Users

- **AI Researchers** conducting capability evaluations for papers or internal assessments
- **ML Engineers** validating fine-tunes, LoRA adapters, or prompt variations before deployment
- **Safety Teams** running red-team evaluations and safeguard stress-tests
- **Indie Developers** building agents who want SWE-bench-style feedback without infrastructure overhead

---

## Product Vision

> Assay is the IDE for AI evaluation—where running a benchmark is as natural as running a test suite, and understanding model behavior is as visual as using a debugger.

### Design Principles

1. **Local-First, Cloud-Optional**: All core functionality works offline. Cloud sync, team collaboration, and remote compute are opt-in.

2. **Eval-as-Code**: Evaluation definitions are TypeScript/YAML files that live alongside your codebase, versioned in git.

3. **Trace-Centric**: Every evaluation produces rich, explorable traces. The trace viewer is the heart of the app.

4. **Extensible by Default**: Plugin architecture for custom scorers, model providers, sandbox environments, and benchmark suites.

5. **Speed Matters**: Rust-powered task orchestration, parallel execution, and incremental evaluation caching.

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Assay Desktop                           │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                    React + TypeScript UI                     │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────────┐   │   │
│  │  │Dashboard │ │  Eval    │ │  Trace   │ │  Compare     │   │   │
│  │  │  Panel   │ │ Designer │ │  Viewer  │ │  & Analyze   │   │   │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                              │                                      │
│                        Tauri IPC Bridge                             │
│                              │                                      │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                    Rust Core Engine                          │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────────┐   │   │
│  │  │  Task    │ │ Sandbox  │ │  Model   │ │   Storage    │   │   │
│  │  │Scheduler │ │ Manager  │ │ Gateway  │ │   & Index    │   │   │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                              │                                      │
│           ┌──────────────────┼──────────────────┐                  │
│           ▼                  ▼                  ▼                  │
│     ┌──────────┐      ┌──────────┐       ┌──────────┐             │
│     │  Docker  │      │   LLM    │       │  Local   │             │
│     │ Sandbox  │      │   APIs   │       │  Models  │             │
│     └──────────┘      └──────────┘       └──────────┘             │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### Technology Stack

| Layer | Technology | Rationale |
|-------|------------|-----------|
| **Desktop Shell** | Tauri 2.0 | 95% smaller than Electron, Rust security, native performance |
| **Frontend** | React 19 + TypeScript | Mature ecosystem, type safety, component library compatibility |
| **State Management** | Zustand + React Query | Lightweight, TypeScript-first, excellent caching |
| **Styling** | Tailwind CSS + shadcn/ui | Rapid iteration, consistent design system |
| **Rust Backend** | Tokio async runtime | High-concurrency task scheduling, efficient I/O |
| **Type Bridge** | tauri-specta | Auto-generated TypeScript bindings from Rust |
| **Database** | SQLite + libSQL | Local-first, fast queries, optional Turso sync |
| **Sandbox** | Docker/Podman | Isolation for untrusted code execution |
| **Model Gateway** | Rust HTTP client | Unified API for OpenAI, Anthropic, local models |

---

## Feature Specification

### F1: Project Workspace

**Description**: Assay organizes work into Projects, each containing evaluation definitions, datasets, model configurations, and results.

**Components**:
- `Assay.toml` - Project configuration file
- `/evals/` - Evaluation definition files (TypeScript or YAML)
- `/datasets/` - Local dataset files or HuggingFace references
- `/results/` - Evaluation logs and traces (SQLite + JSON)
- `/models/` - Model configuration profiles

**User Stories**:
- As a researcher, I want to create a new project from a template (SWE-bench, MMLU, Custom) so I can start evaluating quickly.
- As an engineer, I want to import an existing lm-evaluation-harness task as an Assay eval so I can use the visual tooling.
- As a team lead, I want to clone a project from git and run the same evaluations locally.

**Implementation Notes**:
- Rust backend handles file watching for hot-reload of eval definitions
- Project state stored in SQLite for fast querying across runs
- Git integration via `git2` crate for version info in traces

---

### F2: Evaluation Designer (Visual + Code)

**Description**: A hybrid editor for creating and modifying evaluations. Supports both visual configuration (for common patterns) and direct TypeScript/YAML editing (for power users).

**Evaluation Schema** (TypeScript):
```typescript
interface EvalDefinition {
  id: string;
  name: string;
  description?: string;
  
  // Data source
  dataset: DatasetConfig;
  
  // How to run each sample
  solver: SolverConfig;
  
  // How to score outputs
  scorer: ScorerConfig[];
  
  // Execution settings
  execution: ExecutionConfig;
}

interface DatasetConfig {
  source: 'huggingface' | 'local' | 'inline';
  path: string;
  split?: string;
  limit?: number;
  shuffle?: boolean;
  seed?: number;
}

interface SolverConfig {
  type: 'generate' | 'chain_of_thought' | 'agent' | 'custom';
  system_prompt?: string;
  tools?: ToolConfig[];
  max_turns?: number;
  sandbox?: SandboxConfig;
}

interface ScorerConfig {
  type: 'exact_match' | 'contains' | 'llm_judge' | 'code_execution' | 'custom';
  // Type-specific config
  [key: string]: unknown;
}

interface ExecutionConfig {
  max_concurrent: number;
  timeout_seconds: number;
  retries: number;
  model: ModelRef | ModelRef[]; // Run against multiple models
}
```

**Visual Designer Features**:
- Drag-and-drop dataset selection (browse HuggingFace, local files)
- Template-based solver configuration with live preview
- Scorer builder with LLM-as-judge prompt editor
- Model selector with cost estimation

**User Stories**:
- As a researcher, I want to create a custom MMLU-style eval by selecting a HuggingFace dataset and configuring multiple-choice scoring.
- As an agent developer, I want to define a multi-turn eval where the agent can use tools, with execution in a Docker sandbox.
- As a safety engineer, I want to clone an existing eval and modify the system prompt to test a new safeguard.

---

### F3: Model Registry & Gateway

**Description**: Unified interface for configuring and accessing AI models from multiple providers.

**Supported Providers** (v1.0):
- **Cloud APIs**: OpenAI, Anthropic, Google (Vertex/AI Studio), Mistral, Cohere
- **Local Inference**: Ollama, llama.cpp, vLLM, HuggingFace Transformers
- **Custom Endpoints**: Any OpenAI-compatible API

**Model Profile Schema**:
```yaml
# models/claude-sonnet.yaml
id: claude-sonnet-4
provider: anthropic
model_name: claude-sonnet-4-20250514
parameters:
  temperature: 0
  max_tokens: 4096
cost:
  input_per_1k: 0.003
  output_per_1k: 0.015
rate_limits:
  requests_per_minute: 50
  tokens_per_minute: 100000
```

**Gateway Features**:
- **Request Queuing**: Respects rate limits, retries with exponential backoff
- **Cost Tracking**: Real-time cost accumulation per eval run
- **Response Caching**: Optional semantic caching to avoid redundant API calls
- **Fallback Chains**: Automatically switch to backup model on failure

**User Stories**:
- As a researcher, I want to compare GPT-4o, Claude Sonnet, and Llama 3.3 70B on the same eval without changing my eval definition.
- As a budget-conscious developer, I want to see estimated cost before running an eval and set a cost limit.
- As an enterprise user, I want to route all requests through my company's API gateway.

---

### F4: Execution Engine

**Description**: Rust-powered task scheduler that runs evaluations with configurable parallelism, progress tracking, and graceful interruption.

**Execution Modes**:

| Mode | Description | Use Case |
|------|-------------|----------|
| **Interactive** | Run single samples, see results immediately | Debugging, iteration |
| **Batch** | Run full eval with progress UI | Standard evaluation |
| **Headless** | CLI invocation, JSON output | CI/CD integration |
| **Distributed** | Fan out to remote workers | Large-scale benchmarks |

**Progress & Observability**:
- Real-time progress bar with ETA
- Live sample completion feed
- Token usage and cost accumulator
- Error aggregation and retry status

**Sandbox Integration**:
- Docker-based sandbox for code execution tasks
- File system isolation with configurable mounts
- Network isolation with allowlist for specific domains
- Resource limits (CPU, memory, time)

**Implementation Details**:
```rust
// Simplified execution loop
async fn run_eval(eval: EvalDefinition, model: ModelGateway) -> EvalResult {
    let samples = load_dataset(&eval.dataset).await?;
    let semaphore = Semaphore::new(eval.execution.max_concurrent);
    
    let results: Vec<SampleResult> = stream::iter(samples)
        .map(|sample| {
            let permit = semaphore.acquire().await;
            async move {
                let _permit = permit;
                let output = run_solver(&eval.solver, &sample, &model).await?;
                let scores = run_scorers(&eval.scorers, &sample, &output).await?;
                Ok(SampleResult { sample, output, scores })
            }
        })
        .buffer_unordered(eval.execution.max_concurrent)
        .collect()
        .await;
    
    aggregate_results(results)
}
```

**User Stories**:
- As a researcher, I want to pause a long-running eval, close my laptop, and resume tomorrow.
- As an engineer, I want to run a quick 10-sample test to validate my eval before running the full dataset.
- As a CI pipeline, I want to run evals on every PR and fail if accuracy drops below threshold.

---

### F5: Trace Viewer

**Description**: The flagship feature—a rich, interactive viewer for exploring evaluation traces at multiple levels of detail.

**Trace Hierarchy**:
```
EvalRun
├── Metadata (model, config, timestamp, cost)
├── Aggregated Metrics (accuracy, latency, token usage)
└── Samples[]
    ├── Input (prompt, context, expected output)
    ├── Trajectory (for multi-turn/agents)
    │   ├── Turn 1: User message → Model response
    │   ├── Turn 2: Tool call → Tool result
    │   └── Turn N: ...
    ├── Output (final response)
    ├── Scores (per-scorer breakdown)
    └── Debug Info (latency, tokens, cache hit)
```

**Viewer Features**:

| Feature | Description |
|---------|-------------|
| **Sample Table** | Filterable, sortable table of all samples with inline scores |
| **Trajectory Timeline** | Visual timeline of multi-turn conversations and tool calls |
| **Diff View** | Side-by-side comparison of expected vs actual output |
| **Token Highlighter** | Visualize which tokens contributed to scoring |
| **Score Distribution** | Histogram of scores across samples |
| **Error Clustering** | Group failed samples by error type |
| **Annotation System** | Add notes, tags, and ratings to individual samples |

**Keyboard Navigation**:
- `j/k` - Navigate between samples
- `Enter` - Expand sample detail
- `t` - Toggle trajectory view
- `d` - Open diff view
- `a` - Add annotation
- `f` - Filter to similar samples

**User Stories**:
- As a researcher, I want to quickly find all samples where the model hallucinated, sorted by confidence.
- As a safety engineer, I want to see the full conversation trajectory when the agent took an unsafe action.
- As an annotator, I want to mark samples as "interesting" and export them for a paper.

---

### F6: Comparison & Analysis

**Description**: Tools for comparing evaluation results across models, prompts, or configurations.

**Comparison Types**:

1. **Model Comparison**: Same eval, different models
   - Side-by-side accuracy breakdown
   - Cost-performance scatter plot
   - Statistical significance testing (bootstrap CI)

2. **Prompt Comparison**: Same model, different system prompts
   - A/B test visualization
   - Per-category improvement analysis

3. **Temporal Comparison**: Same config, different runs over time
   - Regression detection
   - Trend visualization

**Analysis Features**:
- **Auto-generated Report**: One-click summary with key findings
- **Export to Markdown/PDF**: For papers, presentations, stakeholder updates
- **Interactive Charts**: Recharts-powered visualizations, exportable as SVG

**User Stories**:
- As a researcher, I want to generate a comparison table showing Model A vs Model B across 5 eval suites for my paper.
- As an ML engineer, I want to see if my fine-tune improved accuracy on the specific categories I targeted.
- As a team lead, I want a weekly report showing model performance trends.

---

### F7: Benchmark Suite Manager

**Description**: Pre-packaged evaluation suites for common benchmarks, with one-click setup.

**Built-in Suites** (v1.0):

| Suite | Tasks | Description |
|-------|-------|-------------|
| **LM Harness Core** | MMLU, HellaSwag, ARC, Winogrande | Standard LLM capability benchmarks |
| **Reasoning** | GSM8K, MATH, BBH | Mathematical and logical reasoning |
| **Coding** | HumanEval, MBPP, SWE-bench Lite | Code generation and repair |
| **Safety** | TruthfulQA, XSTest, ToxiGen | Safety and alignment benchmarks |
| **Agentic** | GAIA, WebArena (lite), ToolBench | Agent capability evaluation |

**Suite Features**:
- **Subset Selection**: Run specific categories or difficulties
- **Progress Tracking**: Resume interrupted benchmark runs
- **Leaderboard Submission**: Export results in standard formats

**User Stories**:
- As a researcher, I want to run MMLU on my model and get results comparable to the Open LLM Leaderboard.
- As an agent developer, I want to run SWE-bench Lite on my agent and see which issue types it struggles with.
- As a safety team, I want a "safety audit" suite that runs TruthfulQA, XSTest, and our custom red-team evals.

---

### F8: Plugin System

**Description**: Extensible architecture allowing community contributions for custom scorers, model providers, and benchmark suites.

**Plugin Types**:

| Type | Description | Example |
|------|-------------|---------|
| **Scorer** | Custom scoring logic | Domain-specific rubric grader |
| **Model Provider** | New API integration | Fireworks AI, Together AI |
| **Solver** | Custom execution strategy | RAG pipeline, multi-agent |
| **Dataset Loader** | New data sources | Company-internal datasets |
| **Sandbox** | Alternative isolation | Kubernetes pods, Firecracker |
| **Exporter** | Output formats | LaTeX tables, Weights & Biases |

**Plugin API** (TypeScript):
```typescript
// plugins/my-scorer/index.ts
import { defineScorer, ScorerInput, ScorerOutput } from '@Assay/sdk';

export default defineScorer({
  id: 'domain-rubric',
  name: 'Domain-Specific Rubric',
  
  async score(input: ScorerInput): Promise<ScorerOutput> {
    // Custom scoring logic
    const rubricScore = evaluateAgainstRubric(input.output, input.expected);
    return {
      score: rubricScore.total / rubricScore.maxPoints,
      breakdown: rubricScore.criteria,
      explanation: rubricScore.feedback,
    };
  },
  
  configSchema: {
    rubricPath: { type: 'string', description: 'Path to rubric definition' },
  },
});
```

**Plugin Distribution**:
- Local plugins in project `/plugins/` directory
- NPM packages with `Assay-plugin-*` naming convention
- Built-in plugin browser for discovery

---

### F9: CI/CD Integration

**Description**: Headless mode and GitHub Actions integration for automated evaluation in CI pipelines.

**CLI Interface**:
```bash
# Run eval and output JSON
Assay run --project ./my-project --eval mmlu --model gpt-4o --output results.json

# Compare to baseline
Assay compare results.json baseline.json --threshold 0.95

# Generate report
Assay report results.json --format markdown --output report.md
```

**GitHub Action**:
```yaml
# .github/workflows/eval.yml
name: Model Evaluation
on: [pull_request]

jobs:
  eval:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: Assay/action@v1
        with:
          project: ./evals
          eval: safety-suite
          model: ${{ secrets.MODEL_API_KEY }}
          baseline: main
          fail-on-regression: true
```

**Features**:
- PR comment with eval results and comparison to base branch
- Configurable quality gates (fail if accuracy < X%)
- Artifact upload for trace files
- Cost budget enforcement

---

### F10: Collaboration (Cloud Tier)

**Description**: Optional cloud features for team collaboration, centralized result storage, and remote compute.

**Cloud Features** (v2.0 roadmap):
- **Team Workspaces**: Shared projects with role-based access
- **Result Sync**: Automatic upload of eval results to cloud database
- **Remote Compute**: Dispatch evals to cloud GPU instances
- **Annotation Queue**: Collaborative human evaluation workflows
- **Dashboards**: Team-wide visibility into model performance trends

**Privacy Model**:
- All data encrypted at rest and in transit
- Option to self-host cloud components
- No training on customer data

---

## User Interface Mockups

### Dashboard View
```
┌──────────────────────────────────────────────────────────────────────┐
│  Assay                                           ⚙️ Settings     │
├──────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  📁 Projects                      📊 Recent Runs                     │
│  ┌─────────────────────────┐     ┌────────────────────────────────┐ │
│  │ 🔵 Agent Capability      │     │ Run #42 - MMLU - claude-4       │ │
│  │ 🟢 Safety Audit          │     │ ████████████░░░░ 78% - Running │ │
│  │ ⚪ SWE-bench Experiments │     ├────────────────────────────────┤ │
│  │                         │     │ Run #41 - Safety - gpt-4o      │ │
│  │ + New Project           │     │ ██████████████████ 94% ✓       │ │
│  └─────────────────────────┘     ├────────────────────────────────┤ │
│                                  │ Run #40 - Coding - llama-3.3   │ │
│  🚀 Quick Actions                │ ██████████████████ 67% ✓       │ │
│  ┌─────────────────────────┐     └────────────────────────────────┘ │
│  │ [Run Benchmark Suite]   │                                        │
│  │ [Compare Models]        │     📈 7-Day Trend                     │
│  │ [Import from lm-eval]   │     ┌────────────────────────────────┐ │
│  └─────────────────────────┘     │     ╭───────╮                  │ │
│                                  │    ╱         ╲    ╭──╮         │ │
│                                  │───╯           ╲──╯  ╰───       │ │
│                                  │  M  T  W  T  F  S  S           │ │
│                                  └────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────────────┘
```

### Trace Viewer
```
┌──────────────────────────────────────────────────────────────────────┐
│  ← Back │ Run #42: MMLU (claude-sonnet-4)        │ 🔍 Filter │ Export│
├──────────────────────────────────────────────────────────────────────┤
│ Summary │ Samples │ Analysis │                                       │
├─────────────────────┬────────────────────────────────────────────────┤
│                     │                                                │
│  Samples (578/578)  │  Sample #127: Abstract Algebra Q23            │
│  ───────────────────│  ────────────────────────────────────────────  │
│  🔍 ____________    │                                                │
│                     │  📥 Input                                      │
│  ✅ #001 Biology    │  ┌────────────────────────────────────────┐   │
│  ✅ #002 Biology    │  │ Question: Let G be a group of order 15. │   │
│  ✅ #003 Chemistry  │  │ Which of the following is true?         │   │
│  ❌ #127 Math    ◀──│  │                                         │   │
│  ✅ #128 Math       │  │ (A) G is cyclic                         │   │
│  ✅ #129 Physics    │  │ (B) G has a normal subgroup of order 3  │   │
│  ...                │  │ (C) G has a normal subgroup of order 5  │   │
│                     │  │ (D) All of the above                    │   │
│  Showing 578 samples│  └────────────────────────────────────────┘   │
│                     │                                                │
│  Filters:           │  📤 Output                                     │
│  [x] Show failures  │  ┌────────────────────────────────────────┐   │
│  [ ] Show passes    │  │ Model: (A) G is cyclic                  │   │
│  Category: [All ▼]  │  │ Expected: (D) All of the above          │   │
│                     │  │                                         │   │
│  Score Range:       │  │ Score: 0.0 (incorrect)                  │   │
│  [0]━━━━━━━━━━[1]  │  └────────────────────────────────────────┘   │
│                     │                                                │
│                     │  💭 Trajectory (1 turn)                        │
│                     │  ┌────────────────────────────────────────┐   │
│                     │  │ Turn 1: Let me analyze this...          │   │
│                     │  │ [Expand full response]                  │   │
│                     │  └────────────────────────────────────────┘   │
│                     │                                                │
│                     │  📝 Annotations                                │
│                     │  + Add annotation                              │
│                     │                                                │
└─────────────────────┴────────────────────────────────────────────────┘
```

---

## Technical Deep Dives

### T1: Rust-TypeScript Bridge via tauri-specta

Assay uses `tauri-specta` to generate fully typed TypeScript bindings from Rust command definitions. This eliminates serialization bugs and provides excellent IDE support.

```rust
// src-tauri/src/commands/eval.rs
use specta::Type;
use tauri_specta::Event;

#[derive(Serialize, Type)]
pub struct EvalProgress {
    pub completed: u32,
    pub total: u32,
    pub current_sample: Option<String>,
    pub elapsed_seconds: f64,
    pub estimated_remaining: Option<f64>,
}

#[tauri::command]
#[specta::specta]
pub async fn run_eval(
    project_path: String,
    eval_id: String,
    model_id: String,
) -> Result<String, String> {
    // Returns run_id for tracking
}

#[derive(Clone, Serialize, Type, Event)]
pub struct EvalProgressEvent(pub EvalProgress);
```

Generated TypeScript:
```typescript
// src/bindings.ts (auto-generated)
export async function runEval(
  projectPath: string,
  evalId: string,
  modelId: string
): Promise<string>;

export type EvalProgress = {
  completed: number;
  total: number;
  currentSample: string | null;
  elapsedSeconds: number;
  estimatedRemaining: number | null;
};
```

### T2: Sandbox Architecture

For agentic evaluations requiring code execution, Assay provides isolated sandbox environments.

```rust
// src-tauri/src/sandbox/docker.rs
pub struct DockerSandbox {
    container_id: String,
    network_mode: NetworkMode,
    resource_limits: ResourceLimits,
}

impl Sandbox for DockerSandbox {
    async fn execute(&self, command: &str) -> Result<ExecResult, SandboxError> {
        let output = Command::new("docker")
            .args(["exec", &self.container_id, "bash", "-c", command])
            .timeout(self.resource_limits.timeout)
            .output()
            .await?;
        
        Ok(ExecResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
        })
    }
    
    async fn write_file(&self, path: &str, content: &[u8]) -> Result<(), SandboxError>;
    async fn read_file(&self, path: &str) -> Result<Vec<u8>, SandboxError>;
}
```

### T3: Trace Storage Schema

Traces are stored in SQLite with full-text search capability via FTS5.

```sql
-- Core tables
CREATE TABLE eval_runs (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    eval_id TEXT NOT NULL,
    model_id TEXT NOT NULL,
    started_at DATETIME NOT NULL,
    completed_at DATETIME,
    status TEXT NOT NULL, -- 'running', 'completed', 'failed', 'cancelled'
    config_json TEXT NOT NULL,
    metrics_json TEXT
);

CREATE TABLE samples (
    id TEXT PRIMARY KEY,
    run_id TEXT NOT NULL REFERENCES eval_runs(id),
    index_num INTEGER NOT NULL,
    input_json TEXT NOT NULL,
    output_json TEXT,
    scores_json TEXT,
    trajectory_json TEXT,
    status TEXT NOT NULL,
    latency_ms INTEGER,
    tokens_input INTEGER,
    tokens_output INTEGER
);

-- Full-text search on sample content
CREATE VIRTUAL TABLE samples_fts USING fts5(
    input_text,
    output_text,
    content='samples',
    content_rowid='rowid'
);

-- Annotations
CREATE TABLE annotations (
    id TEXT PRIMARY KEY,
    sample_id TEXT NOT NULL REFERENCES samples(id),
    author TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    annotation_type TEXT NOT NULL, -- 'note', 'tag', 'rating'
    content TEXT NOT NULL
);
```

---

## Development Roadmap

### Phase 1: Foundation (Months 1-3)

**Milestone 1.1**: Core Infrastructure
- [ ] Tauri 2.0 project scaffold with React 19
- [ ] tauri-specta type bridge setup
- [ ] SQLite database layer with migrations
- [ ] Basic project management (create, open, list)

**Milestone 1.2**: Evaluation Engine
- [ ] Eval definition schema and parser
- [ ] Single-model execution with progress events
- [ ] Basic scorers (exact_match, contains, llm_judge)
- [ ] Result storage and retrieval

**Milestone 1.3**: Model Gateway
- [ ] OpenAI and Anthropic provider integration
- [ ] Request queuing with rate limiting
- [ ] Cost tracking per run
- [ ] Response caching (optional)

**Deliverable**: Run a custom eval on an API model and view results.

### Phase 2: Trace & Compare (Months 4-6)

**Milestone 2.1**: Trace Viewer
- [ ] Sample table with filtering and sorting
- [ ] Sample detail view with input/output/scores
- [ ] Multi-turn trajectory visualization
- [ ] Full-text search across samples

**Milestone 2.2**: Comparison Tools
- [ ] Model vs model comparison view
- [ ] Statistical significance calculation
- [ ] Auto-generated comparison reports
- [ ] Export to Markdown/PDF

**Milestone 2.3**: Benchmark Suites
- [ ] MMLU, HellaSwag, ARC integration
- [ ] GSM8K, HumanEval integration
- [ ] Suite runner with progress tracking
- [ ] Leaderboard-compatible output

**Deliverable**: Compare two models on MMLU and generate a report.

### Phase 3: Agentic & Advanced (Months 7-9)

**Milestone 3.1**: Docker Sandbox
- [ ] Docker sandbox provider
- [ ] Code execution scorer
- [ ] File system operations
- [ ] Network isolation controls

**Milestone 3.2**: Agentic Evaluations
- [ ] Multi-turn solver with tool calling
- [ ] SWE-bench Lite integration
- [ ] Trajectory analysis tools
- [ ] Step-by-step debugging

**Milestone 3.3**: Local Models
- [ ] Ollama integration
- [ ] HuggingFace Transformers (Python subprocess)
- [ ] vLLM integration
- [ ] Batch inference optimization

**Deliverable**: Run SWE-bench Lite on a local model with sandboxed execution.

### Phase 4: Ecosystem (Months 10-12)

**Milestone 4.1**: Plugin System
- [ ] Plugin API and SDK
- [ ] Plugin discovery and loading
- [ ] Documentation and examples
- [ ] Community plugin showcase

**Milestone 4.2**: CI/CD Integration
- [ ] Headless CLI mode
- [ ] GitHub Action
- [ ] Quality gates and thresholds
- [ ] Artifact management

**Milestone 4.3**: Polish & Performance
- [ ] Performance optimization pass
- [ ] Accessibility audit
- [ ] Documentation site
- [ ] Cross-platform testing (macOS, Windows, Linux)

**Deliverable**: v1.0 release with stable API and plugin ecosystem.

---

## Success Metrics

| Metric | Target (v1.0) | Measurement |
|--------|---------------|-------------|
| **Eval Startup Time** | < 2 seconds | Time from click to first sample running |
| **Trace Query Latency** | < 100ms | P95 for 10K sample dataset |
| **Model Comparison Report** | < 5 clicks | User flow analysis |
| **MMLU Full Run** | < 30 minutes | With GPT-4o, parallel execution |
| **Binary Size** | < 50MB | macOS universal binary |
| **Memory Usage** | < 500MB | With 10K samples loaded |

---

## Competitive Analysis

| Feature | Assay | Inspect AI | lm-eval-harness | Braintrust | Langfuse |
|---------|-----------|------------|-----------------|------------|----------|
| **Desktop App** | ✅ Native | ❌ CLI only | ❌ CLI only | ❌ Web only | ❌ Web only |
| **Visual Trace Viewer** | ✅ Rich | ✅ Web viewer | ❌ JSON only | ✅ Good | ✅ Good |
| **Offline-First** | ✅ Full | ⚠️ Partial | ✅ Full | ❌ Cloud | ⚠️ Self-host |
| **Agentic Evals** | ✅ Native | ✅ Excellent | ❌ Limited | ⚠️ Basic | ⚠️ Basic |
| **CI/CD Integration** | ✅ Native | ⚠️ Custom | ⚠️ Custom | ✅ Excellent | ⚠️ Custom |
| **Plugin System** | ✅ TypeScript | ✅ Python | ✅ Python | ❌ No | ❌ No |
| **Multi-Model Compare** | ✅ Native | ✅ Good | ✅ Good | ✅ Excellent | ⚠️ Basic |
| **Cost Tracking** | ✅ Built-in | ⚠️ Basic | ❌ No | ✅ Excellent | ✅ Good |

**Assay's Differentiation**:
1. **Desktop-native** with offline-first architecture
2. **Visual-first** with trace viewer as the core experience
3. **Tauri/Rust performance** for fast, responsive UI even with large datasets
4. **TypeScript plugin system** accessible to frontend developers
5. **Unified workflow** from eval design to comparison to reporting

---

## Open Questions

1. **Pricing Model**: Should cloud features be freemium, subscription, or usage-based?

2. **Inspect AI Compatibility**: Should we aim for full compatibility with Inspect task definitions, or design a cleaner schema that imports from Inspect?

3. **SWE-bench Scale**: Full SWE-bench (2,294 instances) requires significant compute. Should we include cloud compute in v1.0, or focus on SWE-bench Lite?

4. **Human Evaluation**: Many safety evals require human judgment. Should human annotation be a core feature, or a plugin/cloud feature?

5. **Multi-Platform Testing**: How much effort to invest in Windows support for v1.0, given developer preference for macOS/Linux?

---

## Appendix A: Name Alternatives Considered

| Name | Pros | Cons |
|------|------|------|
| **Assay** | Action-oriented, memorable | Slightly generic |
| **Benchmark Studio** | Clear purpose | Too long |
| **ModelScope** | Modern feel | Name collision risk |
| **Calibrate** | Elegant, technical | Too abstract |
| **Assay** | Short, scientific | Obscure meaning |

**Decision**: Assay—conveys both evaluation purpose and tool-building philosophy.

---

## Appendix B: Technology Alternatives Considered

| Component | Chosen | Alternative | Reasoning |
|-----------|--------|-------------|-----------|
| Shell | Tauri | Electron | 95% smaller, Rust performance |
| Frontend | React | Svelte | Larger ecosystem, hiring pool |
| State | Zustand | Redux | Simpler, TypeScript-first |
| DB | SQLite | PostgreSQL | Local-first, portable |
| Sandbox | Docker | Firecracker | Wider availability |

---

*Document authored for Assay v1.0 planning. Subject to revision based on user research and technical spikes.*
