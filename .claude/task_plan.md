# Task Plan: llama_anki_gen — Full Implementation

## Goal
Build a working Rust CLI tool that generates Anki flashcards (especially language learning / JLPT grammar) by prompting a local Ollama (Llama 3) model for structured JSON, then inserting the cards into Anki via AnkiConnect.

## Current Phase
Phase 6 — COMPLETE

## Phases

### Phase 1: Examine & Plan
- [x] Read niraprint.md architecture
- [x] Examine all existing source files (all empty except hello-world main.rs)
- [x] Identify dependencies needed
- [x] Map out implementation order
- [x] Create planning files
- **Status:** complete

### Phase 2: Foundation — Cargo.toml + types.rs + errors.rs
- [x] Add dependencies via `cargo add` (clap, reqwest, serde, serde_json, thiserror)
- [x] Implement `types.rs` — CardRequest, CardFields (HashMap), StoredHistory
- [x] Implement `errors.rs` — unified AppError enum (Network, Parse, Anki, Storage, Model)
- **Status:** complete

### Phase 3: Infrastructure — storage.rs + prompt_builder.rs
- [x] Implement `storage.rs` — FileStorage: load/save used_grammar.json
- [x] Implement `prompt_builder.rs` — build() and build_next() with JSON-forcing prompts
- **Status:** complete

### Phase 4: Clients — model_client.rs + anki_client.rs
- [x] Implement `model_client.rs` — OllamaClient: POST to /api/generate with format:"json"
- [x] Implement `anki_client.rs` — AnkiConnectClient: addNote action via HTTP
- **Status:** complete

### Phase 5: Orchestration — engine.rs + cli.rs + main.rs
- [x] Implement `engine.rs` — generate / next / batch workflows
- [x] Implement `cli.rs` — clap derive with 3 subcommands + global config args
- [x] Wire everything in `main.rs` with parse_items() for batch @file support
- **Status:** complete

### Phase 6: Build & Verify
- [x] `cargo build` — clean build (0 errors, 0 warnings)
- [x] Fixed clap global+required conflict
- [x] `--help` output verified
- **Status:** complete

## Decisions Made
| Decision | Rationale |
|----------|-----------|
| Use `clap` with derive API | Clean subcommand parsing, idiomatic Rust CLI |
| Use blocking `reqwest` (not async) | CLI tool, no concurrency needed, simpler code |
| Use `serde` + `serde_json` | Required for JSON serialization/deserialization |
| Use `thiserror` for AppError | Ergonomic custom error types with `?` propagation |
| Use `HashMap<String, String>` for card fields | Flexible — any note type with any fields |
| Default Ollama endpoint `localhost:11434` | Standard Ollama default |
| Default AnkiConnect endpoint `localhost:8765` | Standard AnkiConnect default |
| Remove `global = true` from required CLI args | clap panics if global args are also required |

## Errors Encountered
| Error | Attempt | Resolution |
|-------|---------|------------|
| clap panic: "Global arguments cannot be required" | 1 | Removed `global = true` from deck, note_type, fields args |

## Notes
- Implementation order follows dependency graph: types → errors → storage → prompt → clients → engine → cli → main
- niraprint.md is the source of truth for architecture
- All 8 source files fully implemented
