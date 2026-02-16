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

## Configuration

Default note type is "Kiku" (check out [youyoumu/kiku](https://github.com/youyoumu/kiku) - very cool note). Specify your own with `-n`:

```bash
anki_gen generate "..." -d "Deck" -f "Front,Back" -n "Basic"
```

Use any model via `--ollama-url` and `--model` flags:

```bash
anki_gen generate "..." --ollama-url http://your-model-api:1234 --model your-model
```

Tested with llama3 (usually bugs). YMMV with other models.

## Requirements

- Running Anki with [AnkiConnect](https://ankiweb.net/shared/info/2055492159)
- LLM API (default: Ollama on localhost:11434)
