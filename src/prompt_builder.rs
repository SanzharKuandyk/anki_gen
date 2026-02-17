use crate::types::CardRequest;

const SYSTEM_PREAMBLE_STRICT: &str = "\
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

const SYSTEM_PREAMBLE_OPTIONAL: &str = "\
You are an expert language learning flashcard generator for Anki. \
Your job is to create high-quality, accurate flashcards that help learners study effectively.

Rules you MUST follow:
1. Output ONLY a single valid JSON object. No markdown, no explanation, no extra text.
2. Use the EXACT field names provided — do not rename, abbreviate, or misspell them.
3. Fill fields with useful content when relevant. You MAY omit or leave empty fields that are not crucial or not applicable to this specific card.
4. For Japanese content: use proper kanji/kana, provide accurate furigana readings, and natural example sentences when applicable.
5. For grammar points: include the grammatical structure, its meaning, JLPT level if applicable, and a natural example when possible.
6. For vocabulary: include the word, reading, meaning, part of speech, and a contextual example sentence when relevant.
7. Keep content concise but complete — only include fields that serve the learner for this specific topic.";

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
        let (preamble, field_instruction) = if req.optional_fields {
            (
                SYSTEM_PREAMBLE_OPTIONAL,
                "Available JSON keys (include only relevant ones): [{fields}]\n\n\
                 Respond with a single JSON object. Include only the fields that are relevant and have meaningful content for this topic.",
            )
        } else {
            (
                SYSTEM_PREAMBLE_STRICT,
                "Required JSON keys: [{fields}]\n\n\
                 Respond with a single JSON object using exactly those keys. Every value must be a non-empty string with real content.",
            )
        };

        format!(
            "{preamble}\n\n\
             ---\n\
             Task: Generate a flashcard.\n\
             Topic: {description}\n\
             Note type: {note_type}\n\
             {instruction}\n\
             Now generate the JSON:",
            preamble = preamble,
            description = req.description,
            note_type = req.note_type,
            instruction = field_instruction.replace("{fields}", &fields_list),
        )
    }

    pub fn build_next(req: &CardRequest, used: &[String]) -> String {
        let fields_list = Self::format_fields(&req.fields);

        let used_list = if used.is_empty() {
            "none yet".to_string()
        } else {
            used.join(", ")
        };

        let (preamble, field_instruction) = if req.optional_fields {
            (
                SYSTEM_PREAMBLE_OPTIONAL,
                "Available JSON keys (include only relevant ones): [{fields}]\n\n\
                 Respond with a single JSON object. Include only the fields that are relevant and have meaningful content for this topic.",
            )
        } else {
            (
                SYSTEM_PREAMBLE_STRICT,
                "Required JSON keys: [{fields}]\n\n\
                 Respond with a single JSON object using exactly those keys. Every value must be a non-empty string with real content.",
            )
        };

        format!(
            "{preamble}\n\n\
             ---\n\
             Task: Generate the NEXT item in a series. Pick one that has NOT been generated yet.\n\
             Topic: {description}\n\
             Note type: {note_type}\n\
             {instruction}\n\n\
             Already generated (DO NOT repeat any of these):\n{used}\n\n\
             Now generate the JSON for the next item:",
            preamble = preamble,
            description = req.description,
            note_type = req.note_type,
            instruction = field_instruction.replace("{fields}", &fields_list),
            used = used_list,
        )
    }
}
