<!-- Description: Compact version with essential sections only -->
# Blueprint: llama_anki_gen

> **Started:** 2026-02-16
> **Status:** Planning

## Layer 1: Intent Map

**PROJECT:** llama_anki_gen

**ONE-LINE:** [Tooling to generate Anki cards via llama model]

**ACTORS:**
- [USER/CLI] - Runs commands generate, next, batch
- [Ollama model (llama 3)] - Structured json output for card fields
- [AnkiConnect] - Receives notes and put's them to Anki
- [LocalStorage] - tracks history

**CORE FLOWS:**
1. User → CLI "check" → Ping Ollama + AnkiConnect → Report status
2. User → CLI "generate" → Ollama → JSON card → AnkiConnect → New note
3. User → CLI "next" → Reads history → Asks model for next grammar → Adds to Anki
4. User → CLI "batch" → Load list → Loop generation → Insert

**HARD PARTS:**
- [Getting the model to output strict, parseable JSON every time.]

- [Mapping flexible user-defined fields to Anki note fields safely.]

- [Handling network errors or malformed responses gracefully.]

- [Keeping iteration history consistent across sessions.]

**NON-GOALS:**
- [No GUI.]

- [No cloud LLM support.]

- [No card editing or deck browsing (Anki handles that).]

- [No scheduling algorithms (Anki handles that).]

- [Not a general NLP toolkit — only structured card generation.]

---

## Layer 2: Interface Contracts

### Data Shapes (Type A)

**[CardRequest]**
- description: [String]
- fields: [Vec<String>]
- note_type: [String]

**[CardJSON]**
- [HashMap<String, String> — key:value fields]
- [Used by: AnkiClient]
- [Produced by: ModelClient]

**[StoredHistory]**
- [used_items: Vec<String> — grammar already generated]
- [Used by: Engine]
- [Produced by: Storage]

### Capabilities (Type B)

**[PromptBuilder — constructs prompts]**
- [build(req: CardRequest) -> String]
- [Implementation: DefaultPromptBuilder]

**[ModelClient — talks to Ollama]**
- [ping() -> Result<Vec<String>> — list available models]
- [generate(prompt: &str) -> Result<CardJSON>]
- [Implementation: OllamaClient]

**[AnkiClient — talks to AnkiConnect]**
- [ping() -> Result<u64> — check connectivity, return version]
- [add_note(fields: CardJSON, note_type: &str, deck: &str) -> Result<()>]
- [Implementation: AnkiConnectClient]

**[Storage — read/write iteration history]**
- [load_history() -> StoredHistory]
- [save_history() -> Result<()>]
- [Implementation: FileStorage]

### Boundaries (Type C)

**[NetworkBoundary ("ModelAPI")]**
- [Touches: http://localhost:11434/api/generate (streaming NDJSON)]
- [Touches: http://localhost:11434/api/tags (health check)]
- [On fail: surface network errors; retry limited times.]
- [Async: tokio runtime, reqwest async client]

**[NetworkBoundary (“AnkiConnect”)]**
- [Touches: http://localhost:8765]
- [On fail: return structured error with full request payload.]

**[FileSystemBoundary (“HistoryStore”)]**
- [Reads: storage/used_grammar.json]
- [Writes: same file]
- [On fail: create file with empty history.]

---

## Layer 3: File Skeleton

```
llama_anki_gen/
├── src/
│   ├── main.rs            ← ENTRY: CLI dispatcher
│   ├── cli.rs             ← CLI commands & args
│   ├── engine.rs          ← Core workflow orchestration
│   ├── prompt_builder.rs  ← Prompt generation for model
│   ├── model_client.rs    ← Ollama API interface
│   ├── anki_client.rs     ← AnkiConnect interface
│   ├── storage.rs         ← Local history store
│   ├── types.rs           ← Shared Rust structs/enums
│   └── errors.rs          ← Unified error type
├── storage/
│   └── used_grammar.json  ← Iteration history
├── Cargo.toml
└── README.md
```

---

## Layer 4: Task Queue

### DONE ✓
- [x] Project init (Cargo.toml, file skeleton)
- [x] types.rs — CardRequest, CardFields, StoredHistory
- [x] errors.rs — AppError (Network, Parse, Anki, Storage, Model)
- [x] storage.rs — FileStorage load/save used_grammar.json
- [x] prompt_builder.rs — build() + build_next() with JSON forcing
- [x] model_client.rs — OllamaClient (POST /api/generate, format:"json")
- [x] anki_client.rs — AnkiConnectClient (addNote v6)
- [x] engine.rs — generate / next / batch workflows
- [x] cli.rs — clap derive, 4 subcommands (check, generate, next, batch)
- [x] model_client.rs — ping() with model availability check
- [x] anki_client.rs — ping() via "version" action
- [x] main.rs — full wiring + parse_items(@file support)
- [x] First clean build

### IN PROGRESS →
- (none)

### NEXT UP
- [ ] **End-to-end test with real Ollama + AnkiConnect**
  - **Depends on:** Ollama running, Anki open with AnkiConnect
  - **Approach:** `cargo run -- -d "Test" -n "Basic" -f "Front,Back" generate "test card"`

### ICEBOX
- [Add “templates” for different card styles]
- [Add “grammar dictionary sync” mode]
- [Allow user-defined global system prompts]
- [Optional Web UI wrapper]
- [Multi-model support (e.g., Mixtral, Llama 3 70B)]
