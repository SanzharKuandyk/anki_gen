# Configuration System

## Priority Hierarchy

The application loads configuration in the following priority order (highest to lowest):

1. **Command-line arguments** - Explicitly passed flags (e.g., `--model gemma2`)
2. **Configuration file** - `config.yaml` or `config.json` in project root
3. **Default values** - Hard-coded defaults in the application

## Configuration Files

The application automatically searches for config files in this order:
- `config.yaml`
- `config.yml`
- `config.json`
- `.anki_gen.yaml`
- `.anki_gen.json`

### Creating a Config File

Generate an example configuration:

```bash
# Generate YAML format (recommended)
anki_gen config --format yaml > config.yaml

# Generate JSON format
anki_gen config --format json > config.json
```

Or copy from the examples:
```bash
cp config.example.yaml config.yaml
```

### Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `model` | string | `llama3` | Ollama model name |
| `ollama_url` | string | `http://localhost:11434` | Ollama API endpoint |
| `anki_url` | string | `http://localhost:8765` | AnkiConnect endpoint |
| `deck` | string? | `null` | Default Anki deck name |
| `note_type` | string | `Kiku` | Default note type |
| `fields` | array | `[]` | Default card fields |
| `storage_path` | string | `storage/used_grammar.json` | Storage file path |

## Examples

### Example 1: Config File Only

**config.yaml:**
```yaml
model: gemma2
deck: Japanese
note_type: Basic
fields:
  - Front
  - Back
```

**Usage:**
```bash
anki_gen generate "ておく grammar"
# Uses: gemma2, Japanese, Basic, [Front, Back] from config
```

### Example 2: Config + CLI Override

**config.yaml:**
```yaml
model: gemma2
deck: Japanese
```

**Usage:**
```bash
anki_gen generate "ておく grammar" --model llama3 -d Spanish
# Uses: llama3 (CLI override), Spanish (CLI override)
# Config file values are overridden by CLI args
```

### Example 3: No Config File

**Usage:**
```bash
anki_gen generate "test" -d Deck -f Front,Back
# Uses all defaults: llama3, localhost:11434, Kiku note type
```

## Testing the Configuration

To see which config file is loaded, run any command and check for the message:
```
✓ Loaded config from: config.yaml
```

If no message appears, the application is using defaults.

To verify your current configuration priority:
```bash
# Test 1: Config file used
anki_gen check

# Test 2: CLI overrides config
anki_gen check --model different-model

# Test 3: Generate config template
anki_gen config --format yaml
```

## Best Practices

1. **Use config files for stable settings** - Put your API URLs, default deck, and model in the config file
2. **Use CLI args for one-off changes** - Override specific values when needed without editing the config
3. **Version control** - Add `config.yaml` to `.gitignore`, commit `config.example.yaml` instead
4. **Multiple environments** - Use different config files (dev/prod) and symlink to `config.yaml`

## Troubleshooting

**Config file not loaded:**
- Check the filename matches one of the supported names
- Verify the file is in the project root directory (where you run `anki_gen`)
- Check for YAML/JSON syntax errors in the config file

**CLI args not working:**
- CLI args must be specified correctly (e.g., `--model llama3`, not `--model=llama3`)
- Global flags must come before the subcommand: `anki_gen --model llama3 generate ...`

**Unexpected values:**
- Remember priority: CLI > config > defaults
- Use `anki_gen check` to verify which model is being used
