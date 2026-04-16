//! Intent router for determining post-processing mode
//!
//! This module analyzes transcripts and selection context to determine
//! the appropriate post-processing intent.

use crate::post_processing::selection_resolver::SelectionContext;
use serde::{Deserialize, Serialize};

/// Represents the detected intent from transcript analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Intent {
    /// Plain transcription - sanitize and clean the transcript
    PlainTranscription,
    /// Text transformation - modify selected text
    TextTransformation,
    /// Text generation - create new content
    TextGeneration,
    /// Image analysis - analyze selected image
    ImageAnalysis,
    /// Web-assisted answering - search for information
    WebAssistedAnswering,
}

impl Default for Intent {
    fn default() -> Self {
        Intent::PlainTranscription
    }
}

/// Keywords that indicate text transformation intent
const TRANSFORMATION_KEYWORDS: &[&str] = &[
    "fix",
    "correct",
    "improve",
    "make it",
    "change",
    "convert",
    "transform",
    "rewrite",
    "rephrase",
    "paraphrase",
    "summarize",
    "shorten",
    "expand",
    "elaborate",
    "condense",
    "format",
    "capitalize",
    "uppercase",
    "lowercase",
    "punctuate",
    "grammar",
    "spell check",
];

/// Keywords that indicate text generation intent
const GENERATION_KEYWORDS: &[&str] = &[
    "write",
    "create",
    "draft",
    "compose",
    "generate",
    "tell me about",
    "explain",
    "describe",
    "outline",
    "brainstorm",
    "suggest",
    "recommend",
    "email",
    "message",
    "letter",
    "note",
    "report",
    "summary",
    "blog",
    "article",
];

/// Keywords that indicate image-related intent
const IMAGE_KEYWORDS: &[&str] = &[
    "what is this",
    "what's this",
    "describe this",
    "explain this image",
    "what does this show",
    "what's in this",
    "analyze this",
    "look at this",
    "see this",
    "image",
    "picture",
    "photo",
    "screenshot",
    "graph",
    "chart",
    "diagram",
];

/// Keywords that indicate factual/current information needs
const FACTUAL_KEYWORDS: &[&str] = &[
    "what is",
    "what's",
    "who is",
    "who's",
    "where is",
    "where's",
    "when is",
    "when's",
    "how is",
    "how's",
    "why is",
    "why's",
    "current",
    "latest",
    "recent",
    "news",
    "today",
    "weather",
    "price",
    "stock",
    "score",
    "update",
    "比分",
    "价格",
    "最新",
    "天气",
    "新闻",
    "现在",
    "今天",
];

/// Keywords that indicate uncertainty or need for verification
const UNCERTAINTY_KEYWORDS: &[&str] = &[
    "i think",
    "i believe",
    "maybe",
    "perhaps",
    "probably",
    "possibly",
    "not sure",
    "uncertain",
    "i don't know",
    "i'm not sure",
    "verify",
    "check",
    "confirm",
    "look up",
    "find out",
    "不确定",
    "可能",
    "也许",
];

/// Intent router that analyzes transcripts to determine processing mode
pub struct IntentRouter;

impl IntentRouter {
    /// Detect intent from transcript and selection context
    ///
    /// Priority order:
    /// 1. ImageAnalysis - if image is selected and transcript mentions image-related keywords
    /// 2. WebAssistedAnswering - if transcript asks for factual/current/uncertain information
    /// 3. TextTransformation - if transcript contains transformation commands and text is selected
    /// 4. TextGeneration - if transcript requests generating new content
    /// 5. PlainTranscription - default fallback
    pub fn detect(transcript: &str, selection: &SelectionContext) -> Intent {
        let transcript_lower = transcript.to_lowercase();
        let _transcript_trimmed = transcript.trim();

        // Rule 1: Image analysis - if image selected and transcript is about the image
        if selection.has_image() && Self::contains_any(&transcript_lower, IMAGE_KEYWORDS) {
            return Intent::ImageAnalysis;
        }

        // Rule 2: Web-assisted answering - factual, current, or uncertain queries
        if Self::is_factual_query(&transcript_lower)
            || Self::contains_any(&transcript_lower, FACTUAL_KEYWORDS)
            || Self::contains_any(&transcript_lower, UNCERTAINTY_KEYWORDS)
        {
            return Intent::WebAssistedAnswering;
        }

        // Rule 3: Text transformation - command detected and text is selected
        if selection.has_text() && Self::is_transformation_command(&transcript_lower) {
            return Intent::TextTransformation;
        }

        // Rule 4: Text generation - clear generation request
        if Self::is_generation_request(&transcript_lower) {
            return Intent::TextGeneration;
        }

        // Rule 5: Default to plain transcription
        Intent::PlainTranscription
    }

    /// Check if transcript contains any of the keywords
    fn contains_any(text: &str, keywords: &[&str]) -> bool {
        keywords.iter().any(|kw| text.contains(kw))
    }

    /// Check if transcript is a factual query requiring web search
    fn is_factual_query(transcript: &str) -> bool {
        // Only match actual question-word prefixes.
        // Auxiliary verbs (is, can, will, do, should, etc.) are intentionally
        // excluded because they start normal declarative sentences too often
        // (e.g. "I should go", "Can you believe", "Do the right thing").
        // The FACTUAL_KEYWORDS list already catches "what is", "who is", etc.
        let question_prefixes = [
            "what ",
            "what's ",
            "who ",
            "who's ",
            "where ",
            "where's ",
            "when ",
            "when's ",
            "how ",
            "how's ",
            "why ",
            "why's ",
        ];

        let trimmed = transcript.trim();
        question_prefixes
            .iter()
            .any(|prefix| trimmed.starts_with(prefix))
    }

    /// Check if transcript contains a text transformation command
    fn is_transformation_command(transcript: &str) -> bool {
        // Check for transformation keywords
        if !Self::contains_any(transcript, TRANSFORMATION_KEYWORDS) {
            return false;
        }

        // Additional heuristics for transformation:
        // - Contains "this" or "it" referring to selected text
        // - Contains comparison words like "into", "to", "from"
        let transformation_indicators = [
            "this text",
            "this passage",
            "this sentence",
            "it into",
            "convert this",
            "change this",
            "fix this",
            "rewrite this",
            "rephrase this",
            "改",
            "修正",
            "转换",
        ];

        Self::contains_any(transcript, &transformation_indicators)
            || transcript.contains("to ")
                && (transcript.contains("into ") || transcript.contains("to be "))
    }

    /// Check if transcript is a text generation request
    fn is_generation_request(transcript: &str) -> bool {
        // Must contain generation keywords
        if !Self::contains_any(transcript, GENERATION_KEYWORDS) {
            return false;
        }

        // Additional context indicators for generation
        let generation_contexts = [
            "write an",
            "write a",
            "create an",
            "create a",
            "draft an",
            "draft a",
            "generate an",
            "generate a",
            "compose an",
            "compose a",
            "给我",
            "写一",
            "创建",
            "生成",
        ];

        Self::contains_any(transcript, &generation_contexts)
            || (transcript.contains("about") && Self::contains_any(transcript, GENERATION_KEYWORDS))
    }

    /// Get a human-readable description of the intent
    pub fn intent_description(intent: Intent) -> &'static str {
        match intent {
            Intent::PlainTranscription => "Sanitized transcription",
            Intent::TextTransformation => "Text transformation",
            Intent::TextGeneration => "Text generation",
            Intent::ImageAnalysis => "Image analysis",
            Intent::WebAssistedAnswering => "Web-assisted answering",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_analysis_intent() {
        let selection = SelectionContext::image("data:image/png;base64,abc".to_string());
        let transcript = "What is this image showing?";
        let intent = IntentRouter::detect(transcript, &selection);
        assert_eq!(intent, Intent::ImageAnalysis);
    }

    #[test]
    fn test_web_search_intent() {
        let selection = SelectionContext::none();
        let transcript = "What is the weather today?";
        let intent = IntentRouter::detect(transcript, &selection);
        assert_eq!(intent, Intent::WebAssistedAnswering);
    }

    #[test]
    fn test_transformation_intent() {
        let selection = SelectionContext::text("Hello world".to_string());
        let transcript = "Fix the grammar in this text";
        let intent = IntentRouter::detect(transcript, &selection);
        assert_eq!(intent, Intent::TextTransformation);
    }

    #[test]
    fn test_generation_intent() {
        let selection = SelectionContext::none();
        let transcript = "Write an email to my boss";
        let intent = IntentRouter::detect(transcript, &selection);
        assert_eq!(intent, Intent::TextGeneration);
    }

    #[test]
    fn test_plain_transcription_default() {
        let selection = SelectionContext::none();
        let transcript = "This is just regular speech";
        let intent = IntentRouter::detect(transcript, &selection);
        assert_eq!(intent, Intent::PlainTranscription);
    }

    #[test]
    fn test_uncertainty_keywords() {
        let selection = SelectionContext::none();
        let transcript = "I'm not sure what the capital of France is";
        let intent = IntentRouter::detect(transcript, &selection);
        assert_eq!(intent, Intent::WebAssistedAnswering);
    }
}
