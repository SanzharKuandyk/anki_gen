use crate::types::CardRequest;

const SYSTEM_PREAMBLE: &str = "\
You are an expert language learning flashcard generator for Anki. \
Your job is to create high-quality, accurate flashcards that help learners study effectively.

Rules you MUST follow:
1. Output ONLY a single valid JSON object. No markdown, no explanation, no extra text.
2. Use the EXACT field names provided — do not rename, abbreviate, or misspell them.
3. Every field must be filled with useful content. Never leave a field empty.
4. For Japanese content: use proper kanji/kana, provide accurate furigana readings, and natural example sentences.
5. For grammar points: include the grammatical structure, its meaning, JLPT level if applicable, and a natural example.
6. For vocabulary: include the word, reading, meaning, part of speech, and a contextual example sentence.
7. Keep content concise but complete — each field should serve the learner.";

pub struct PromptBuilder;

impl PromptBuilder {
    fn format_fields(fields: &[String]) -> String {
        fields
            .iter()
            .map(|f| format!("\"{}\"", f))
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn build(req: &CardRequest) -> String {
        let fields_list = Self::format_fields(&req.fields);

        format!(
            "{preamble}\n\n\
             ---\n\
             Task: Generate a flashcard.\n\
             Topic: {description}\n\
             Note type: {note_type}\n\
             Required JSON keys: [{fields}]\n\n\
             Respond with a single JSON object using exactly those keys. Every value must be a non-empty string with real content.\n\
             Now generate the JSON:",
            preamble = SYSTEM_PREAMBLE,
            description = req.description,
            note_type = req.note_type,
            fields = fields_list,
        )
    }

    pub fn build_next(req: &CardRequest, used: &[String]) -> String {
        let fields_list = Self::format_fields(&req.fields);

        let used_list = if used.is_empty() {
            "none yet".to_string()
        } else {
            used.join(", ")
        };

        format!(
            "{preamble}\n\n\
             ---\n\
             Task: Generate the NEXT item in a series. Pick one that has NOT been generated yet.\n\
             Topic: {description}\n\
             Note type: {note_type}\n\
             Required JSON keys: [{fields}]\n\n\
             Already generated (DO NOT repeat any of these):\n{used}\n\n\
             Respond with a single JSON object using exactly those keys. Every value must be a non-empty string with real content.\n\
             Now generate the JSON for the next item:",
            preamble = SYSTEM_PREAMBLE,
            description = req.description,
            note_type = req.note_type,
            fields = fields_list,
            used = used_list,
        )
    }
}
