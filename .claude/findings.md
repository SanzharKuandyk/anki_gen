# Findings & Decisions

## Requirements
- Rust CLI tool with 3 subcommands: `generate`, `next`, `batch`
- `generate`: User describes a card → model generates fields → inserted into Anki
- `next`: Reads history of already-generated items → asks model for next grammar point → inserts
- `batch`: Load a list of items → loop generate each → insert all
- Talks to local Ollama (Llama 3) at `localhost:11434/api/generate`
- Talks to AnkiConnect at `localhost:8765`
- Tracks generated items in `storage/used_grammar.json`
- User specifies note type, deck name, and field names
- Primary use case: JLPT Japanese grammar cards (N5–N1)
- No GUI, no cloud LLM, no card editing/browsing

## Research Findings

### Ollama API (`/api/generate`)
- POST `http://localhost:11434/api/generate`
- Request body: `{ "model": "llama3", "prompt": "...", "stream": false, "format": "json" }`
- `format: "json"` forces JSON output mode
- Response: `{ "response": "...", "done": true, ... }` — the `response` field contains the generated text (JSON string)
- When `stream: false`, returns a single response object

### AnkiConnect API (`addNote`)
- POST `http://localhost:8765`
- Request body:
```json
{
  "action": "addNote",
  "version": 6,
  "params": {
    "note": {
      "deckName": "Japanese::Grammar",
      "modelName": "JapaneseGrammar",
      "fields": { "Front": "...", "Back": "...", ... },
      "options": { "allowDuplicate": false }
    }
  }
}
```
- Response: `{ "result": <note_id>, "error": null }` on success

### Crate Versions (as of 2026)
- `clap` 4.x with derive feature
- `reqwest` 0.12.x with blocking feature
- `serde` 1.x + `serde_json` 1.x
- `thiserror` 2.x

## Technical Decisions
| Decision | Rationale |
|----------|-----------|
| Blocking reqwest, no async runtime | CLI tool doesn't benefit from async; simpler code |
| Ollama `format: "json"` | Forces structured JSON, addresses "hard part" from niraprint |
| `thiserror` over `anyhow` for AppError | Want typed errors (Network, Parse, Anki, Storage) |
| clap derive subcommands | Clean `generate`, `next`, `batch` dispatch |
| Store history as simple Vec<String> in JSON | Matches niraprint StoredHistory spec |

## Issues Encountered
| Issue | Resolution |
|-------|------------|
| (none yet) | - |

## Resources
- Ollama API docs: https://github.com/ollama/ollama/blob/main/docs/api.md
- AnkiConnect docs: https://foosoft.net/projects/anki-connect/
- niraprint.md — project architecture blueprint

---
*Update this file after every 2 view/browser/search operations*
