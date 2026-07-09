# AEGIS Pivot Plan: Inference Gateway → Agent Orchestration Runtime

## Executive Summary

Pivot AEGIS from a distributed AI inference scheduler to an **AI Agent Orchestration Runtime** — a production-grade Rust framework for orchestrating AI agents that chain LLM calls with tools, memory, and reasoning loops.

**Differentiator**: No one has a production-grade Rust agent framework (LangChain = Python, CrewAI = Python, AutoGen = Python).

---

## Phase 0: Foundation (Restructure Workspace)

### 0.1 — Update project identity
- Rename workspace description in `Cargo.toml`: "AEGIS - Agent Orchestration Runtime"
- Update `ARCHITECTURE.md` with new agent-focused architecture
- Update `README.md` with new positioning and features
- Update `PROJECT_OVERVIEW.md`

### 0.2 — Restructure Cargo workspace
**Keep as-is** (directly reusable):
| Crate | Why |
|-------|-----|
| `security` | Multi-tenant auth, JWT, API keys, RBAC, TLS — perfect for agent deployments |
| `resilience` | Circuit breakers, retry, timeout, graceful degradation — wrap tool calls |
| `audit` | Cryptographic tamper-proof logs — agent action compliance (HIPAA, SOC2) |
| `telemetry` | Prometheus + OpenTelemetry — reuse for agent metrics |
| `observability` | Health checks, distributed tracing — reuse for agent observability |

**Rewrite** (repurpose skeleton):
| Crate | From | To |
|-------|------|----|
| `gateway` | Inference HTTP handlers | Agent API gateway (REST + WebSocket for streaming) |
| `runtime` | Inference orchestrator | Agent runtime orchestrator |
| `proto` | Inference protobuf | Agent/tool protobuf definitions |

**Remove** (inference-specific, not needed):
| Crate | Reason |
|-------|--------|
| `scheduler` | Inference node scheduling — replaced by agent task scheduling |
| `speculative` | Speculative decoding — not relevant to agents |
| `consensus` | Inference state replication — replaced by multi-agent coordination |
| `inference-backends` | llama.cpp, vLLM, Ollama — replaced by tool execution |
| `safety` | Inference safety — replaced by agent guardrails |
| `aegis-scheduler` | Separate inference scheduler binary |
| `benchmarks` | Inference benchmarks — replaced by agent benchmarks |

**Add** (new crates):
| Crate | Purpose |
|-------|---------|
| `agent` | Agent state machine, lifecycle, configuration |
| `tools` | Tool execution framework (API calls, code execution, DB queries) |
| `memory` | Short-term context window + long-term vector store |
| `coordination` | Multi-agent coordination, delegation, handoff |
| `session` | Conversation/session management, history |

### 0.3 — Update dependencies
Remove from `Cargo.toml`:
- `llama-cpp-2` (native inference)
- `quic` (inference transport)

Add to `Cargo.toml`:
- `reqwest` (HTTP client for tool calls)
- `sqlx` (async SQL for session/memory persistence)
- `uuid` with `v7` feature (time-ordered IDs for agent events)
- `tokio-tungstenite` (WebSocket for streaming)

---

## Phase 1: Agent Core (`agent` crate)

### 1.1 — Agent state machine
```
┌─────────┐     ┌─────┐     ┌─────────┐     ┌─────────┐
│  Think   │────▶│ Act  │────▶│ Observe │────▶│  Think  │
└─────────┘     └─────┘     └─────────┘     └─────────┘
     ▲                                              │
     └──────────────────────────────────────────────┘
```

Types:
```rust
pub enum AgentState {
    Idle,
    Thinking { prompt: String },
    Acting { tool_call: ToolCall },
    Observing { result: ToolResult },
    Completed { output: String },
    Error { error: AgentError },
}

pub struct Agent {
    id: Uuid,
    name: String,
    config: AgentConfig,
    state: AgentState,
    memory: MemoryManager,
    tools: ToolRegistry,
    audit: Arc<AuditEngine>,
}
```

### 1.2 — Agent configuration
```rust
pub struct AgentConfig {
    pub model: String,
    pub system_prompt: String,
    pub max_iterations: usize,
    pub max_tokens: usize,
    pub temperature: f32,
    pub tools: Vec<String>,
    pub permissions: Vec<String>,
}
```

### 1.3 — Agent execution loop
```rust
impl Agent {
    pub async fn run(&mut self, input: &str) -> Result<String> {
        self.state = AgentState::Thinking { prompt: input.to_string() };
        
        for _ in 0..self.config.max_iterations {
            let response = self.think().await?;
            
            match response.action {
                Action::Respond(text) => {
                    self.state = AgentState::Completed { output: text.clone() };
                    return Ok(text);
                }
                Action::ToolCall(tool_call) => {
                    self.state = AgentState::Acting { tool_call: tool_call.clone() };
                    let result = self.act(&tool_call).await?;
                    
                    self.state = AgentState::Observing { result: result.clone() };
                    self.observe(result).await?;
                }
            }
        }
        
        Err(AgentError::MaxIterationsExceeded)
    }
}
```

---

## Phase 2: Tool Execution (`tools` crate)

### 2.1 — Tool trait
```rust
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters_schema(&self) -> serde_json::Value;
    async fn execute(&self, params: serde_json::Value, ctx: &ToolContext) -> Result<ToolResult>;
}

pub struct ToolContext {
    pub agent_id: Uuid,
    pub session_id: Uuid,
    pub permissions: Vec<String>,
    pub circuit_breaker: Arc<CircuitBreaker>,
    pub audit: Arc<AuditEngine>,
}
```

### 2.2 — Built-in tools
- `http_request` — call external APIs
- `code_execution` — run code in sandboxed environment
- `database_query` — read/write to databases
- `file_operations` — read/write files
- `shell_command` — execute shell commands (with safety guardrails)

### 2.3 — Tool execution with resilience
```rust
pub struct ToolExecutor {
    tools: HashMap<String, Arc<dyn Tool>>,
    circuit_breakers: HashMap<String, Arc<CircuitBreaker>>,
}

impl ToolExecutor {
    pub async fn execute(&self, tool_call: ToolCall, ctx: &ToolContext) -> Result<ToolResult> {
        let tool = self.tools.get(&tool_call.name).ok_or(ToolError::NotFound)?;
        let cb = self.circuit_breakers.get(&tool_call.name).ok_or(ToolError::NoCircuitBreaker)?;
        
        cb.can_request()?;
        let result = tool.execute(tool_call.params, ctx).await;
        match &result {
            Ok(_) => cb.record_success(),
            Err(_) => cb.record_failure(),
        }
        
        ctx.audit.record(AuditEvent { /* ... */ })?;
        result
    }
}
```

---

## Phase 3: Memory System (`memory` crate)

### 3.1 — Short-term memory (context window)
```rust
pub struct ShortTermMemory {
    messages: Vec<Message>,
    max_tokens: usize,
    summarizer: Option<Box<dyn Summarizer>>,
}
```

### 3.2 — Long-term memory (vector store)
```rust
pub struct LongTermMemory {
    store: Box<dyn VectorStore>,
    embedder: Box<dyn Embedder>,
}

#[async_trait]
pub trait VectorStore: Send + Sync {
    async fn insert(&self, id: Uuid, embedding: Vec<f32>, metadata: Value) -> Result<()>;
    async fn search(&self, query: Vec<f32>, top_k: usize) -> Result<Vec<MemoryEntry>>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}
```

### 3.3 — Memory manager
```rust
pub struct MemoryManager {
    short_term: ShortTermMemory,
    long_term: LongTermMemory,
}
```

---

## Phase 4: Multi-Agent Coordination (`coordination` crate)

Patterns:
- **Sequential pipeline**: Agent A → Agent B → Agent C
- **Parallel fan-out**: Agent A delegates to B, C, D simultaneously
- **Supervisor pattern**: Supervisor agent monitors workers
- **Handoff**: Agent A transfers context to Agent B

---

## Phase 5: Session Management (`session` crate)

- Session lifecycle (create, update, delete)
- Session store trait (swappable backends)
- Message history with pagination

---

## Phase 6: Gateway Rebuild

### New API endpoints
| Method | Path | Purpose |
|--------|------|---------|
| POST | `/agents` | Create agent |
| GET | `/agents/:id` | Get agent |
| POST | `/agents/:id/run` | Execute agent |
| POST | `/agents/:id/chat` | Streaming chat (WebSocket) |
| GET | `/sessions` | List sessions |
| POST | `/tools` | Register tool |
| GET | `/health` | Health check |

---

## Execution Order

| Phase | What | Dependencies |
|-------|------|-------------|
| 0 | Restructure workspace | None |
| 1 | Agent core (state machine) | Phase 0 |
| 2 | Tool execution framework | Phase 1 |
| 3 | Memory system | Phase 1 |
| 4 | Multi-agent coordination | Phase 1, 2, 3 |
| 5 | Session management | Phase 1, 3 |
| 6 | Gateway rebuild | Phase 1, 2, 4, 5 |
| 7 | Integration & testing | All phases |
