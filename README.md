# anki_gen

Generate Anki flashcards using LLMs.

> 🛋️ Under lazy development. Any help would be good!

![Example](example.png)

## Usage

```bash
# Check connectivity
anki_gen check

# Generate a single card
anki_gen generate "ておく grammar point" -d "Japanese" -f "Grammar,Meaning,Example"

# Generate next in sequence (skips duplicates)
anki_gen next "JLPT N3 grammar" -d "Japanese" -f "Grammar,Meaning,Example"

# Batch from list or file
anki_gen batch "item1,item2,item3" -d "Deck" -f "Front,Back"
anki_gen batch "@items.txt" -d "Deck" -f "Front,Back"
```

### Batch Processing

Generate multiple cards from a file (one item per line):

**Create a file (e.g., `items.txt`):**
```
ておく
てしまう
ながら
ばかり
```

**Important:**
- One item per line
- Empty lines are skipped
- Whitespace is trimmed
- Use `@filename` syntax to read from file

**Command:**
```bash
anki_gen batch "@items.txt" -d "Japanese" -f "Grammar,Meaning,Example"
```

Or inline:
```bash
anki_gen batch "ておく,てしまう,ながら" -d "Japanese" -f "Grammar,Meaning,Example"
```

**Output:**
```
[1/4] Generating: ておく
  ✓ Added to Anki
[2/4] Generating: てしまう
  ✓ Added to Anki
[3/4] Generating: ながら
  ✗ Failed: Model error
[4/4] Generating: ばかり
  ✓ Added to Anki

Batch complete: 3 succeeded, 1 failed out of 4
```

Batch processing continues even if individual items fail. Failed items are reported at the end.

## Configuration

**Priority:** CLI args > config file > defaults

Create `config.yaml` or `config.json` in project root:

```yaml
model: llama3                              # LLM model name
ollama_url: http://localhost:11434         # Ollama API endpoint
anki_url: http://localhost:8765            # AnkiConnect endpoint
deck: Japanese                             # Default deck
note_type: Kiku                            # Default note type
fields: [Grammar, Meaning, Example]        # Default fields
storage_path: storage/used_grammar.json    # History tracking
optional_fields: false                     # Allow skipping non-crucial fields
```

Generate template: `anki_gen config --format yaml > config.yaml`

### Options

| Option | Default | Description |
|--------|---------|-------------|
| `model` | `llama3` | Ollama model name |
| `ollama_url` | `http://localhost:11434` | LLM API endpoint |
| `anki_url` | `http://localhost:8765` | AnkiConnect endpoint |
| `deck` | - | Default Anki deck |
| `note_type` | `Kiku` | Default note type ([youyoumu/kiku](https://github.com/youyoumu/kiku)) |
| `fields` | `[]` | Default card fields |
| `storage_path` | `storage/used_grammar.json` | Storage file path |
| `optional_fields` | `false` | Allow model to skip non-crucial fields |

### Auto-Detect Fields

If you don't specify `--fields`, the app will **auto-detect ALL fields** from your note type:

```bash
# Auto-detect all fields from "Kiku" note type
anki_gen generate "ておく" -d "Japanese"
# Detects: Expression, ExpressionFurigana, Meaning, Sentence, etc.

# Best combined with optional-fields
anki_gen generate "ておく" -d "Japanese" --optional-fields
# Model fills only relevant fields, skips the rest
```

**Without --fields:**
- ✅ Detects all fields from note type automatically
- ✅ Perfect for note types with many fields
- ✅ No need to list fields manually

**With --optional-fields:**
- ✅ Model decides which fields are relevant
- ✅ Skips non-applicable fields
- ✅ Maximum flexibility

### Optional Fields

**Strict mode (default):** All fields required and must be filled.
**Optional mode:** Model can omit fields that aren't relevant.

```bash
# Enable in config
optional_fields: true

# Or via CLI flag (overrides config)
anki_gen generate "..." --optional-fields
```

**Combinations:**

| Fields | Optional | Behavior |
|--------|----------|----------|
| Specified | No | Must fill ALL specified fields (strict) |
| Specified | Yes | Fill RELEVANT specified fields (flexible) |
| Auto-detect | No | Must fill ALL note type fields (complete) |
| Auto-detect | Yes | Fill RELEVANT note type fields (maximum flexibility) ✨ |

### CLI Examples

```bash
# Override config values
anki_gen generate "..." -d "CustomDeck" --model gemma2

# Use different note type
anki_gen generate "..." -n "Basic" -f "Front,Back"

# Enable optional fields for one command
anki_gen generate "slang word" -f "Word,Meaning,JLPT" --optional-fields
```

## Requirements

- Running Anki with [AnkiConnect](https://ankiweb.net/shared/info/2055492159)
- LLM API (default: Ollama on localhost:11434)
