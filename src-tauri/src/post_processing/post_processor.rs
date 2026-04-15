//! Main post-processing orchestrator
//!
//! This module provides the main [`PostProcessor`] struct that orchestrates
//! the post-processing pipeline, coordinating intent detection, prompt building,
//! and LLM API calls with MCP tool support.

use crate::llm_client::{self, ToolDefinition, ToolMessage, MessageContent, ContentPart, ImageUrl};
use crate::post_processing::intent_router::{Intent, IntentRouter};
use crate::post_processing::mcp_tools::MiniMaxMcpClient;
use crate::post_processing::prompt_builder::{
    PostProcessingPayload, POST_PROCESSING_SYSTEM_PROMPT,
};
use crate::post_processing::selection_resolver::SelectionContext;
use crate::settings::{AppSettings, PostProcessProvider};
use log::{debug, error, warn};
use serde_json::Value;

/// The MiniMax model that supports MCP tools
pub const MINIMAX_MCP_MODEL: &str = "minimax-m2.7";

/// Default MiniMax model
pub const MINIMAX_DEFAULT_MODEL: &str = "minimax-m2.7";

/// Main post-processor that orchestrates the pipeline
pub struct PostProcessor {
    settings: AppSettings,
    mcp_client: Option<MiniMaxMcpClient>,
}

impl PostProcessor {
    /// Creates a new PostProcessor with the given settings
    pub fn new(settings: AppSettings) -> Self {
        let mcp_client = if settings.post_process_provider_id == "minimax" {
            let api_key = settings
                .post_process_api_keys
                .get("minimax")
                .cloned()
                .unwrap_or_default();
            let base_url = settings
                .post_process_providers
                .iter()
                .find(|p| p.id == "minimax")
                .map(|p| p.base_url.clone())
                .unwrap_or_else(|| "https://api.minimax.chat/v1".to_string());

            Some(MiniMaxMcpClient::with_base_url(api_key, base_url))
        } else {
            None
        };

        Self {
            settings,
            mcp_client,
        }
    }

    /// Process a transcription with the given selection context
    ///
    /// This is the main entry point for post-processing. It:
    /// 1. Detects intent from transcript and selection
    /// 2. Builds appropriate prompt
    /// 3. Calls the LLM with MCP tools if available
    /// 4. Returns the final processed result
    pub async fn process(
        &self,
        transcription: &str,
        selection: SelectionContext,
    ) -> Option<String> {
        // Build payload
        let payload = PostProcessingPayload::new(
            transcription.to_string(),
            &selection,
            self.settings
                .custom_instructions
                .clone()
                .unwrap_or_default(),
        );

        // Detect intent
        let intent = IntentRouter::detect(transcription, &selection);
        debug!(
            "Detected intent: {:?} ({})",
            intent,
            IntentRouter::intent_description(intent)
        );

        // Get provider and model info
        let provider = self.settings.active_post_process_provider()?;
        let model = self
            .settings
            .post_process_models
            .get(&provider.id)
            .cloned()
            .unwrap_or_default();

        if model.trim().is_empty() {
            warn!("No model configured for post-processing");
            return None;
        }

        let api_key = self
            .settings
            .post_process_api_keys
            .get(&provider.id)
            .cloned()
            .unwrap_or_default();

        // Handle based on intent and provider capabilities
        match intent {
            Intent::ImageAnalysis => {
                self.process_image_analysis(&payload, &provider, &model, &api_key)
                    .await
            }
            Intent::WebAssistedAnswering => {
                self.process_web_assisted(&payload, &provider, &model, &api_key)
                    .await
            }
            Intent::TextTransformation | Intent::TextGeneration => {
                self.process_text_generation(&payload, &provider, &model, &api_key)
                    .await
            }
            Intent::PlainTranscription => {
                self.process_plain_transcription(&payload, &provider, &model, &api_key)
                    .await
            }
        }
    }

    /// Process image analysis using MiniMax's understand_image MCP tool
    async fn process_image_analysis(
        &self,
        payload: &PostProcessingPayload,
        provider: &PostProcessProvider,
        model: &str,
        api_key: &str,
    ) -> Option<String> {
        debug!("Processing image analysis");

        // Use MCP client if available for MiniMax
        if provider.id == "minimax" {
            if let Some(ref mcp_client) = self.mcp_client {
                // For MiniMax with MCP, build messages with image
                let content = vec![
                    ContentPart {
                        part_type: "text".to_string(),
                        text: Some(format!(
                            "Please analyze this image based on the user's spoken request.\n\nUser's request: {}",
                            payload.output
                        )),
                        image_url: None,
                    },
                    ContentPart {
                        part_type: "image_url".to_string(),
                        text: None,
                        image_url: Some(ImageUrl {
                            url: payload.selected_image.clone(),
                        }),
                    },
                ];

                let messages = vec![ToolMessage {
                    role: "user".to_string(),
                    content: MessageContent::Array(content),
                    tools: None,
                }];

                let tools = Some(vec![ToolDefinition {
                    name: "understand_image".to_string(),
                    description: "Analyze an image and provide detailed description or answer questions about it".to_string(),
                    parameters: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "image": {
                                "type": "string",
                                "description": "The image data URL (base64 encoded)"
                            },
                            "prompt": {
                                "type": "string",
                                "description": "The question or instruction about the image"
                            }
                        },
                        "required": ["image", "prompt"]
                    }),
                }]);

                match mcp_client.execute_mcp_completion(api_key, &provider.base_url, model, messages, tools).await {
                    Ok(result) => {
                        // If result contains tool call, execute it
                        if result.starts_with("[TOOL_CALL:") {
                            return self.handle_tool_call(&result[11..], api_key, provider).await;
                        }
                        return Some(result);
                    }
                    Err(e) => {
                        error!("Image analysis failed: {}", e);
                        return None;
                    }
                }
            }
        }

        // Fallback: Use standard chat completion with image in content
        self.process_with_llm(payload, provider, model, api_key).await
    }

    /// Process web-assisted answering using MiniMax's web_search MCP tool
    async fn process_web_assisted(
        &self,
        payload: &PostProcessingPayload,
        provider: &PostProcessProvider,
        model: &str,
        api_key: &str,
    ) -> Option<String> {
        debug!("Processing web-assisted answering");

        // Use MCP client if available for MiniMax
        if provider.id == "minimax" {
            if let Some(ref mcp_client) = self.mcp_client {
                let messages = vec![ToolMessage {
                    role: "user".to_string(),
                    content: MessageContent::Text(format!(
                        "Search the web for information related to this request and provide an answer:\n\n{}",
                        payload.output
                    )),
                    tools: None,
                }];

                let tools = Some(vec![ToolDefinition {
                    name: "web_search".to_string(),
                    description: "Search the web for current information, facts, news, or answers to questions".to_string(),
                    parameters: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "query": {
                                "type": "string",
                                "description": "The search query"
                            }
                        },
                        "required": ["query"]
                    }),
                }]);

                match mcp_client.execute_mcp_completion(api_key, &provider.base_url, model, messages, tools).await {
                    Ok(result) => {
                        if result.starts_with("[TOOL_CALL:") {
                            return self.handle_tool_call(&result[11..], api_key, provider).await;
                        }
                        return Some(result);
                    }
                    Err(e) => {
                        error!("Web search failed: {}", e);
                        // Fallback to regular processing
                        return self.process_with_llm(payload, provider, model, api_key).await;
                    }
                }
            }
        }

        // Fallback for non-MiniMax providers
        self.process_with_llm(payload, provider, model, api_key).await
    }

    /// Handle a tool call from the LLM response
    async fn handle_tool_call(
        &self,
        tool_info: &str,
        api_key: &str,
        provider: &PostProcessProvider,
    ) -> Option<String> {
        // Parse tool_info: "tool_name:{...}}]"
        if let Some(colon_pos) = tool_info.find(':') {
            let tool_name = &tool_info[..colon_pos];
            let args_str = &tool_info[colon_pos + 1..].trim_end_matches(']');
            let args: Value = serde_json::from_str(args_str).ok()?;

            if let Some(ref mcp_client) = self.mcp_client {
                let result = mcp_client.execute_tool(tool_name, args).await;
                if result.success {
                    // Continue conversation with tool result
                    let messages = vec![
                        ToolMessage {
                            role: "user".to_string(),
                            content: MessageContent::Text(format!("Tool result: {}", result.content)),
                            tools: None,
                        },
                    ];

                    let tools = Some(vec![
                        ToolDefinition {
                            name: "web_search".to_string(),
                            description: "Search the web for current information".to_string(),
                            parameters: serde_json::json!({
                                "type": "object",
                                "properties": {
                                    "query": {"type": "string"}
                                }
                            }),
                        },
                        ToolDefinition {
                            name: "understand_image".to_string(),
                            description: "Analyze an image".to_string(),
                            parameters: serde_json::json!({
                                "type": "object",
                                "properties": {
                                    "image": {"type": "string"},
                                    "prompt": {"type": "string"}
                                }
                            }),
                        },
                    ]);

                    let model = self
                        .settings
                        .post_process_models
                        .get(&provider.id)
                        .cloned()
                        .unwrap_or_default();

                    match mcp_client.execute_mcp_completion(api_key, &provider.base_url, &model, messages, tools).await {
                        Ok(final_result) => Some(final_result),
                        Err(e) => {
                            error!("Tool call continuation failed: {}", e);
                            Some(result.content)
                        }
                    }
                } else {
                    error!("Tool execution failed: {:?}", result.error);
                    None
                }
            } else {
                error!("MCP client not available for tool execution");
                None
            }
        } else {
            error!("Failed to parse tool call info: {}", tool_info);
            None
        }
    }

    /// Process text transformation and generation
    async fn process_text_generation(
        &self,
        payload: &PostProcessingPayload,
        provider: &PostProcessProvider,
        model: &str,
        api_key: &str,
    ) -> Option<String> {
        debug!("Processing text generation/transformation");
        self.process_with_llm(payload, provider, model, api_key)
            .await
    }

    /// Process plain transcription with sanitization
    async fn process_plain_transcription(
        &self,
        payload: &PostProcessingPayload,
        provider: &PostProcessProvider,
        model: &str,
        api_key: &str,
    ) -> Option<String> {
        debug!("Processing plain transcription");
        self.process_with_llm(payload, provider, model, api_key)
            .await
    }

    /// Generic LLM processing with prompt
    async fn process_with_llm(
        &self,
        payload: &PostProcessingPayload,
        provider: &PostProcessProvider,
        model: &str,
        api_key: &str,
    ) -> Option<String> {
        // Build prompt from template
        let prompt_template = self
            .settings
            .post_process_prompts
            .iter()
            .find(|p| {
                self.settings
                    .post_process_selected_prompt_id
                    .as_ref()
                    .map(|id| &p.id == id)
                    .unwrap_or(false)
            })
            .map(|p| p.prompt.clone())
            .unwrap_or_else(|| POST_PROCESSING_SYSTEM_PROMPT.to_string());

        // Build user content with variable injection
        let user_content = crate::post_processing::PromptBuilder::build_user_content(
            &prompt_template,
            payload,
        );
        debug!(
            "Prompt/context built: user_content length={} chars",
            user_content.len()
        );

        // Build system prompt
        let _system_prompt = crate::post_processing::PromptBuilder::build_system_prompt(
            &self.settings
                .custom_instructions
                .clone()
                .unwrap_or_default(),
        );

        debug!("Sending to LLM: provider={}, model={}", provider.id, model);
        debug!("Post-process model/provider invoked: provider={}, model={}", provider.id, model);

        // Use existing LLM client
        match llm_client::send_chat_completion(
            provider,
            api_key.to_string(),
            model,
            user_content,
            None,
            None,
        )
        .await
        {
            Ok(Some(result)) => {
                debug!("LLM response length: {} chars", result.len());
                debug!(
                    "Post-process output received: '{}' ({} chars)",
                    result,
                    result.len()
                );
                Some(result)
            }
            Ok(None) => {
                warn!("LLM returned empty content");
                None
            }
            Err(e) => {
                error!("LLM request failed: {}", e);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimax_model_constant() {
        assert_eq!(MINIMAX_MCP_MODEL, "minimax-m2.7");
    }
}
