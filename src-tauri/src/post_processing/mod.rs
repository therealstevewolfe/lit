//! Post-processing architecture for intelligent transcript handling
//!
//! This module provides a full pipeline for:
//! - Text transformation
//! - Text generation
//! - Image analysis
//! - Web-assisted answering
//! - Sanitized transcription
//!
//! The pipeline uses intent routing to determine the appropriate mode based on
//! transcript content and selection context.

pub mod intent_router;
pub mod mcp_tools;
pub mod post_processor;
pub mod prompt_builder;
pub mod selection_resolver;

pub use intent_router::{Intent, IntentRouter};
pub use mcp_tools::{McpTool, McpToolCall, McpToolResult, MiniMaxMcpClient};
pub use post_processor::PostProcessor;
pub use prompt_builder::{PostProcessingPayload, PromptBuilder, POST_PROCESSING_SYSTEM_PROMPT};
pub use selection_resolver::{SelectionContext, SelectionResolver, SelectionType};
