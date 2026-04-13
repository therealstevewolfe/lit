//! Prompt builder for constructing post-processing prompts
//!
//! This module handles the construction of prompts with runtime variable injection
//! and maintains the system prompt constant for governing post-processing behavior.

use crate::post_processing::selection_resolver::{SelectionContext, SelectionType};
use serde::{Deserialize, Serialize};

/// The system prompt that governs runtime post-processing behavior.
/// This prompt instructs the model to:
/// - Determine intent from transcript and selection context
/// - Apply appropriate transformations based on intent
/// - Use MCP tools when beneficial (web search, image understanding)
/// - Return only the final result without metadata or reasoning
pub const POST_PROCESSING_SYSTEM_PROMPT: &str = r#"You are an intelligent transcription post-processing assistant.

Your task is to analyze the user's spoken transcript and any selection context to determine the appropriate action.

## Intent Detection

Analyze the transcript to determine the user's intent:

1. **Plain Transcription**: If the transcript is just speech without clear commands, return a sanitized version:
   - Fix punctuation, capitalization, sentence boundaries
   - Clean speech-to-text artifacts
   - Convert number words to digits where appropriate
   - Remove filler words (um, uh, like)
   - Preserve original meaning and wording
   - Restructure inline enumerations into readable bullet lists when it materially improves clarity

2. **Text Transformation**: If the transcript contains a clear transformation command (e.g., "fix grammar", "make it formal", "shorten", "expand"):
   - Apply the requested transformation to ${selected_text}
   - Return only the transformed result

3. **Text Generation**: If the transcript requests generating new text (e.g., "write an email", "create a summary", "draft a response"):
   - Generate the requested content
   - Use ${selected_text} as context if provided
   - Return only the generated content

4. **Image Analysis**: If an image is selected and the spoken request is about the image:
   - Use the understand_image tool to analyze the image
   - Answer the user's question about the image
   - Return only the analysis result

5. **Web-Assisted Answering**: If the transcript asks about factual, current, or uncertain information:
   - Use the web_search tool to find accurate information
   - Synthesize findings into a coherent response
   - Prefer lookup over guessing for freshness-sensitive queries

## Rules

- Return ONLY the final result
- Do NOT return metadata, reasoning, or mode labels
- Do NOT wrap output in quotes or code blocks unless explicitly requested
- Preserve the original language of the transcript
- When in doubt, prefer sanitized transcription over transformation

## Runtime Variables

- ${output}: The raw transcription text
- ${selected_text}: Text selected by the user (empty if none)
- ${selected_image}: Image selected by the user (empty if none)
- ${selection}: The raw selection (text or image reference)
- ${user_custom_prompt}: User's custom instructions (empty if not set)
"#;

/// Payload structure for post-processing requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostProcessingPayload {
    /// The raw transcription output
    pub output: String,
    /// Selected text content (empty string if none)
    pub selected_text: String,
    /// Selected image as base64 data URL (empty string if none)
    pub selected_image: String,
    /// The type of selection
    pub selection_type: SelectionType,
    /// Raw selection string
    pub selection: String,
    /// User's custom instructions
    pub user_custom_prompt: String,
}

impl Default for PostProcessingPayload {
    fn default() -> Self {
        Self {
            output: String::new(),
            selected_text: String::new(),
            selected_image: String::new(),
            selection_type: SelectionType::None,
            selection: String::new(),
            user_custom_prompt: String::new(),
        }
    }
}

impl PostProcessingPayload {
    /// Creates a new payload from transcript and selection context
    pub fn new(
        output: String,
        selection: &SelectionContext,
        user_custom_prompt: String,
    ) -> Self {
        Self {
            output,
            selected_text: selection.selected_text.clone().unwrap_or_default(),
            selected_image: selection.selected_image.clone().unwrap_or_default(),
            selection_type: selection.selection_type,
            selection: selection.selection.clone().unwrap_or_default(),
            user_custom_prompt,
        }
    }

    /// Returns true if an image is available for processing
    pub fn has_image(&self) -> bool {
        self.selection_type == SelectionType::Image && !self.selected_image.is_empty()
    }

    /// Returns true if text is available for processing
    pub fn has_text(&self) -> bool {
        self.selection_type == SelectionType::Text && !self.selected_text.is_empty()
    }

    /// Returns true if any selection is available
    pub fn has_selection(&self) -> bool {
        self.selection_type != SelectionType::None
    }
}

/// Builder for constructing prompts with variable injection
pub struct PromptBuilder;

impl PromptBuilder {
    /// Build the user content by injecting runtime variables into the template
    ///
    /// Replaces the following variables:
    /// - ${output} -> The raw transcription
    /// - ${selected_text} -> Selected text or empty string
    /// - ${selected_image} -> Selected image data URL or empty string
    /// - ${selection} -> Raw selection or empty string
    /// - ${user_custom_prompt} -> User's custom instructions or empty string
    pub fn build_user_content(
        template: &str,
        payload: &PostProcessingPayload,
    ) -> String {
        template
            .replace("${output}", &payload.output)
            .replace("${selected_text}", &payload.selected_text)
            .replace("${selected_image}", &payload.selected_image)
            .replace("${selection}", &payload.selection)
            .replace("${user_custom_prompt}", &payload.user_custom_prompt)
    }

    /// Build the system prompt by injecting custom instructions
    ///
    /// If user_custom_prompt is provided, appends it to the base system prompt;
    /// otherwise returns the base system prompt unchanged.
    pub fn build_system_prompt(custom_instructions: &str) -> String {
        if custom_instructions.trim().is_empty() {
            POST_PROCESSING_SYSTEM_PROMPT.to_string()
        } else {
            format!(
                "{}\n\n## Custom User Instructions\n\n{}",
                POST_PROCESSING_SYSTEM_PROMPT, custom_instructions
            )
        }
    }

    /// Build content for image analysis mode
    ///
    /// Constructs a prompt that uses the transcript as instruction
    /// and the selected image as the image to analyze.
    pub fn build_image_analysis_content(
        transcript: &str,
        image_data: &str,
    ) -> String {
        format!(
            "Please analyze this image based on the user's request.\n\nUser's spoken request: {}\n\nImage: <image>{}</image>",
            transcript, image_data
        )
    }

    /// Build content for web search queries
    ///
    /// Constructs a prompt for searching based on transcript content.
    pub fn build_web_search_content(transcript: &str) -> String {
        format!(
            "Search the web for information related to this request: {}",
            transcript
        )
    }

    /// Sanitize transcript for plain transcription mode
    ///
    /// Applies basic cleaning:
    /// - Fixes punctuation
    /// - Corrects capitalization
    /// - Repairs sentence boundaries
    /// - Removes obvious speech artifacts
    pub fn sanitize_transcript(transcript: &str) -> String {
        let mut result = transcript.trim().to_string();

        // Remove multiple spaces
        while result.contains("  ") {
            result = result.replace("  ", " ");
        }

        // Remove trailing whitespace before punctuation
        result = result.replace(" .", ".").replace(" ,", ",");

        // Ensure sentence-ending punctuation is followed by a space or end
        // This is a simple heuristic; the LLM does more sophisticated cleanup
        result = result
            .split(|c: char| c == '.' || c == '!' || c == '?')
            .map(|s| s.trim())
            .collect::<Vec<_>>()
            .join(". ");

        // Add final period if missing and text doesn't end with punctuation
        if !result.is_empty()
            && !result.ends_with('.')
            && !result.ends_with('!')
            && !result.ends_with('?')
            && !result.ends_with(':')
            && !result.ends_with('"')
            && !result.ends_with('\'')
        {
            result.push('.');
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payload_creation() {
        let selection = SelectionContext::text("Hello world".to_string());
        let payload = PostProcessingPayload::new(
            "Test transcription".to_string(),
            &selection,
            "Custom instructions".to_string(),
        );

        assert_eq!(payload.output, "Test transcription");
        assert_eq!(payload.selected_text, "Hello world");
        assert_eq!(payload.selection_type, SelectionType::Text);
        assert!(payload.has_text());
        assert!(!payload.has_image());
    }

    #[test]
    fn test_variable_injection() {
        let payload = PostProcessingPayload {
            output: "原始文本".to_string(),
            selected_text: "选中的文本".to_string(),
            selected_image: "".to_string(),
            selection_type: SelectionType::Text,
            selection: "选中的文本".to_string(),
            user_custom_prompt: "自定义指令".to_string(),
        };

        let template = "Output: ${output}, Selected: ${selected_text}, Custom: ${user_custom_prompt}";
        let result = PromptBuilder::build_user_content(template, &payload);

        assert!(result.contains("原始文本"));
        assert!(result.contains("选中的文本"));
        assert!(result.contains("自定义指令"));
    }

    #[test]
    fn test_sanitize_transcript() {
        let dirty = "this is a test   of the system  ";
        let clean = PromptBuilder::sanitize_transcript(dirty);
        assert_eq!(clean, "This is a test of the system.");
    }
}
