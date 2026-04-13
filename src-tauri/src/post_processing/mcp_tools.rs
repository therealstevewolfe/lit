//! MCP tool wrapper for MiniMax
//!
//! This module provides MCP (Model Context Protocol) tool support for MiniMax models,
//! specifically supporting:
//! - web_search: Search the web for current/factual information
//! - understand_image: Analyze images using MiniMax's vision capabilities

use crate::llm_client::{send_chat_completion_with_mcp_tools, ToolDefinition, ToolMessage};
use crate::settings::PostProcessProvider;
use log::{debug, error};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents an MCP tool call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolCall {
    /// The name of the tool to call
    pub name: String,
    /// Arguments to pass to the tool
    pub arguments: Value,
}

/// Represents the result of an MCP tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolResult {
    /// The name of the tool that was executed
    pub name: String,
    /// Whether the tool execution was successful
    pub success: bool,
    /// The result content from the tool
    pub content: String,
    /// Error message if execution failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl McpToolResult {
    /// Creates a successful tool result
    pub fn success(name: &str, content: String) -> Self {
        Self {
            name: name.to_string(),
            success: true,
            content,
            error: None,
        }
    }

    /// Creates a failed tool result
    pub fn failure(name: &str, error: String) -> Self {
        Self {
            name: name.to_string(),
            success: false,
            content: String::new(),
            error: Some(error),
        }
    }
}

/// Represents the available MCP tools
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum McpTool {
    WebSearch,
    UnderstandImage,
}

impl McpTool {
    /// Returns the tool name as a string
    pub fn name(&self) -> &'static str {
        match self {
            McpTool::WebSearch => "web_search",
            McpTool::UnderstandImage => "understand_image",
        }
    }
}

/// Client for executing MCP tools on MiniMax API
pub struct MiniMaxMcpClient {
    api_key: String,
    base_url: String,
}

impl MiniMaxMcpClient {
    /// Creates a new MiniMax MCP client
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.minimax.chat/v1".to_string(),
        }
    }

    /// Creates a new MiniMax MCP client with custom base URL
    pub fn with_base_url(api_key: String, base_url: String) -> Self {
        Self { api_key, base_url }
    }

    /// Execute an MCP chat completion with tools
    pub async fn execute_mcp_completion(
        &self,
        api_key: &str,
        base_url: &str,
        model: &str,
        messages: Vec<ToolMessage>,
        _tools: Option<Vec<ToolDefinition>>,
    ) -> Result<String, String> {
        let provider = PostProcessProvider {
            id: "minimax".to_string(),
            label: "MiniMax".to_string(),
            base_url: base_url.to_string(),
            allow_base_url_edit: false,
            models_endpoint: None,
            supports_structured_output: true,
        };

        match send_chat_completion_with_mcp_tools(&provider, api_key.to_string(), model, messages).await {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err("No content in response".to_string()),
            Err(e) => Err(e),
        }
    }

    /// Build headers for MiniMax API requests
    fn build_headers(&self) -> Result<HeaderMap, String> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(USER_AGENT, HeaderValue::from_static("Lit/1.0"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.api_key))
                .map_err(|e| format!("Invalid API key header value: {}", e))?,
        );
        Ok(headers)
    }

    /// Execute a web search using MiniMax MCP tools
    ///
    /// This searches the web for information related to the query.
    pub async fn web_search(&self, query: &str) -> McpToolResult {
        debug!("Executing web_search for query: {}", query);

        let url = format!("{}/mcp/tools/call", self.base_url.trim_end_matches('/'));

        let request_body = serde_json::json!({
            "name": "web_search",
            "arguments": {
                "query": query
            }
        });

        let headers = match self.build_headers() {
            Ok(h) => h,
            Err(e) => return McpToolResult::failure("web_search", e),
        };

        let client = match reqwest::Client::builder().default_headers(headers).build() {
            Ok(c) => c,
            Err(e) => return McpToolResult::failure("web_search", format!("Failed to build client: {}", e)),
        };

        match client.post(&url).json(&request_body).send().await {
            Ok(response) => {
                let status = response.status();
                if !status.is_success() {
                    let error_text = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "Unknown error".to_string());
                    error!("Web search API failed ({}): {}", status, error_text);
                    return McpToolResult::failure(
                        "web_search",
                        format!("API request failed with status {}", status),
                    );
                }

                match response.json::<Value>().await {
                    Ok(json) => {
                        let content = json
                            .get("content")
                            .and_then(|c| c.as_str())
                            .unwrap_or("")
                            .to_string();
                        debug!("Web search returned: {}", content);
                        McpToolResult::success("web_search", content)
                    }
                    Err(e) => McpToolResult::failure(
                        "web_search",
                        format!("Failed to parse response: {}", e),
                    ),
                }
            }
            Err(e) => {
                error!("Web search request failed: {}", e);
                McpToolResult::failure("web_search", format!("Request failed: {}", e))
            }
        }
    }

    /// Execute image understanding using MiniMax MCP tools
    ///
    /// This analyzes an image and returns a description/analysis.
    pub async fn understand_image(
        &self,
        image_data: &str,
        instruction: &str,
    ) -> McpToolResult {
        debug!(
            "Executing understand_image with instruction: {}",
            instruction
        );

        let url = format!("{}/mcp/tools/call", self.base_url.trim_end_matches('/'));

        let request_body = serde_json::json!({
            "name": "understand_image",
            "arguments": {
                "image": image_data,
                "prompt": instruction
            }
        });

        let headers = match self.build_headers() {
            Ok(h) => h,
            Err(e) => return McpToolResult::failure("understand_image", e),
        };

        let client = match reqwest::Client::builder().default_headers(headers).build() {
            Ok(c) => c,
            Err(e) => {
                return McpToolResult::failure(
                    "understand_image",
                    format!("Failed to build client: {}", e),
                )
            }
        };

        match client.post(&url).json(&request_body).send().await {
            Ok(response) => {
                let status = response.status();
                if !status.is_success() {
                    let error_text = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "Unknown error".to_string());
                    error!("Image understanding API failed ({}): {}", status, error_text);
                    return McpToolResult::failure(
                        "understand_image",
                        format!("API request failed with status {}", status),
                    );
                }

                match response.json::<Value>().await {
                    Ok(json) => {
                        let content = json
                            .get("content")
                            .and_then(|c| c.as_str())
                            .unwrap_or("")
                            .to_string();
                        debug!("Image understanding returned: {}", content);
                        McpToolResult::success("understand_image", content)
                    }
                    Err(e) => McpToolResult::failure(
                        "understand_image",
                        format!("Failed to parse response: {}", e),
                    ),
                }
            }
            Err(e) => {
                error!("Image understanding request failed: {}", e);
                McpToolResult::failure(
                    "understand_image",
                    format!("Request failed: {}", e),
                )
            }
        }
    }

    /// Execute an MCP tool by name
    ///
    /// Returns None if the tool name is not recognized.
    pub async fn execute_tool(&self, tool_name: &str, arguments: Value) -> McpToolResult {
        match tool_name {
            "web_search" => {
                let query = arguments
                    .get("query")
                    .and_then(|q| q.as_str())
                    .unwrap_or("");
                self.web_search(query).await
            }
            "understand_image" => {
                let image = arguments
                    .get("image")
                    .and_then(|i| i.as_str())
                    .unwrap_or("");
                let instruction = arguments
                    .get("prompt")
                    .and_then(|p| p.as_str())
                    .unwrap_or("");
                self.understand_image(image, instruction).await
            }
            _ => McpToolResult::failure(tool_name, format!("Unknown tool: {}", tool_name)),
        }
    }
}

/// Send an MCP-enabled chat completion request
pub async fn send_mcp_chat_completion(
    api_key: &str,
    base_url: &str,
    model: &str,
    messages: Vec<ToolMessage>,
    _tools: Option<Vec<ToolDefinition>>,
) -> Result<String, String> {
    // Use the LLM client's function with MCP tools
    let provider = crate::settings::PostProcessProvider {
        id: "minimax".to_string(),
        label: "MiniMax".to_string(),
        base_url: base_url.to_string(),
        allow_base_url_edit: false,
        models_endpoint: None,
        supports_structured_output: true,
    };

    match send_chat_completion_with_mcp_tools(&provider, api_key.to_string(), model, messages).await {
        Ok(Some(result)) => Ok(result),
        Ok(None) => Err("No content in response".to_string()),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_names() {
        assert_eq!(McpTool::WebSearch.name(), "web_search");
        assert_eq!(McpTool::UnderstandImage.name(), "understand_image");
    }

    #[test]
    fn test_mcp_tool_result_success() {
        let result = McpToolResult::success("test", "content".to_string());
        assert!(result.success);
        assert_eq!(result.content, "content");
        assert!(result.error.is_none());
    }

    #[test]
    fn test_mcp_tool_result_failure() {
        let result = McpToolResult::failure("test", "error".to_string());
        assert!(!result.success);
        assert!(result.content.is_empty());
        assert_eq!(result.error, Some("error".to_string()));
    }
}
