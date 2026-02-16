# Progress Log

## Session: 2026-02-16

### Phase 1: Examine & Plan
- **Status:** complete
- **Started:** 2026-02-16
- Actions taken:
  - Read niraprint.md — 4-layer architecture (Intent, Contracts, Skeleton, Tasks)
  - Examined all source files — all empty except main.rs (hello world)
  - Cargo.toml had edition 2024, zero deps
  - Created task_plan.md, findings.md, progress.md
- Files created/modified:
  - task_plan.md (created)
  - findings.md (created)
  - progress.md (created)

### Phase 2: Foundation
- **Status:** complete
- Actions taken:
  - `cargo add clap --features derive`
  - `cargo add reqwest --features blocking,json`
  - `cargo add serde --features derive`
  - `cargo add serde_json`
  - `cargo add thiserror`
  - Wrote types.rs (CardRequest, CardFields, StoredHistory)
  - Wrote errors.rs (AppError with 5 variants)
- Files created/modified:
  - Cargo.toml (updated by cargo add)
  - Cargo.lock (generated)
  - src/types.rs (written)
  - src/errors.rs (written)

### Phase 3: Infrastructure
- **Status:** complete
- Actions taken:
  - Wrote storage.rs (FileStorage with load/save, auto-create on missing)
  - Wrote prompt_builder.rs (build + build_next with used-items exclusion)
- Files created/modified:
  - src/storage.rs (written)
  - src/prompt_builder.rs (written)

### Phase 4: Clients
- **Status:** complete
- Actions taken:
  - Wrote model_client.rs (OllamaClient, POST /api/generate, format:"json")
  - Wrote anki_client.rs (AnkiConnectClient, addNote action v6)
- Files created/modified:
  - src/model_client.rs (written)
  - src/anki_client.rs (written)

### Phase 5: Orchestration
- **Status:** complete
- Actions taken:
  - Wrote engine.rs (generate, next, batch workflows with history tracking)
  - Wrote cli.rs (clap derive, 3 subcommands, config args)
  - Wrote main.rs (mod declarations, CLI dispatch, parse_items for batch)
- Files created/modified:
  - src/engine.rs (written)
  - src/cli.rs (written)
  - src/main.rs (rewritten)

### Phase 6: Build & Verify
- **Status:** complete
- Actions taken:
  - First `cargo build` — clean compile (0 errors)
  - `cargo run -- --help` — panicked: clap global+required conflict
  - Fixed cli.rs: removed `global = true` from deck, note_type, fields
  - Rebuild — clean
  - `--help` output verified: all 3 subcommands, all options shown correctly
- Files created/modified:
  - src/cli.rs (fixed)

### Phase 7: Check command + async/streaming
- **Status:** complete
- Actions taken:
  - Added `check` subcommand — pings Ollama + AnkiConnect
  - Added `ping()` to OllamaClient (GET /api/tags, verifies model exists)
  - Added `ping()` to AnkiConnectClient ("version" action)
  - Made deck/note_type Optional so `check` doesn't require them
  - Switched from blocking reqwest to async (tokio runtime)
  - Ollama streaming: `stream: true`, read NDJSON chunks, print tokens live
  - Added key/value trimming after model response (model adds leading spaces)
  - Added field validation in engine (missing fields, all-empty check)
- Files created/modified:
  - Cargo.toml (added tokio, removed blocking from reqwest)
  - src/model_client.rs (async, streaming, key trimming)
  - src/anki_client.rs (async)
  - src/engine.rs (async, field validation)
  - src/cli.rs (check command, optional deck/note_type)
  - src/main.rs (#[tokio::main], check dispatch)

### Phase 8: Fuzzy field matching + prompt preamble
- **Status:** complete
- Actions taken:
  - Added `strsim` crate for Levenshtein distance
  - Added `fix_field_names()` in engine — remaps model keys within edit distance 2
  - Prints warning when fuzzy remapping occurs (e.g. `Fuzzy fix: 'SentenceFurigama' -> 'SentenceFurigana'`)
  - Rewrote prompt_builder with `SYSTEM_PREAMBLE` — explains the task, rules for JSON, field naming, and content quality
  - Added example JSON template in prompt so model sees exact expected format
- Files created/modified:
  - Cargo.toml (added strsim)
  - src/engine.rs (fix_field_names with levenshtein)
  - src/prompt_builder.rs (full rewrite with preamble + example template)

## Test Results
| Test | Input | Expected | Actual | Status |
|------|-------|----------|--------|--------|
| cargo build | N/A | Clean compile | Clean compile (0 errors) | PASS |
| --help | `cargo run -- --help` | Help text | Correct help with 4 subcommands | PASS |

## Error Log
| Timestamp | Error | Attempt | Resolution |
|-----------|-------|---------|------------|
| 2026-02-16 | clap panic: Global arguments cannot be required | 1 | Removed `global = true` from required args in cli.rs |
| 2026-02-16 | AnkiConnect: "cannot create note because it is empty" | 1 | Model returned keys with leading spaces; added key/value trimming + validation |
| 2026-02-16 | reqwest `stream` feature broken (wasm-streams 0.5 missing) | 1 | Removed stream feature, use resp.chunk() instead |

## 5-Question Reboot Check
| Question | Answer |
|----------|--------|
| Where am I? | All phases complete — async streaming build done |
| Where am I going? | Ready for real testing |
| What's the goal? | Rust CLI to generate Anki cards via local Llama + AnkiConnect |
| What have I learned? | Model adds whitespace to keys; reqwest stream feature broken on 0.13.2; chunk() works without it |
| What have I done? | Full async rewrite with streaming output, check command, field validation |

---
*Update after completing each phase or encountering errors*
