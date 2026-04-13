//! Selection resolver for determining selection type and context
//!
//! This module handles the capture and classification of user selections,
//! distinguishing between text and image selections.

use serde::{Deserialize, Serialize};

/// Represents the type of selection made by the user
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SelectionType {
    /// No selection was made
    None,
    /// Text was selected
    Text,
    /// An image was selected
    Image,
}

impl Default for SelectionType {
    fn default() -> Self {
        SelectionType::None
    }
}

/// Holds the selection context including type and content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionContext {
    /// The type of selection
    pub selection_type: SelectionType,
    /// Selected text content (when selection_type is Text)
    pub selected_text: Option<String>,
    /// Selected image as base64-encoded data URL (when selection_type is Image)
    pub selected_image: Option<String>,
    /// Raw selection string that can represent either text or image reference
    pub selection: Option<String>,
}

impl Default for SelectionContext {
    fn default() -> Self {
        Self {
            selection_type: SelectionType::None,
            selected_text: None,
            selected_image: None,
            selection: None,
        }
    }
}

impl SelectionContext {
    /// Creates a text selection context
    pub fn text(text: String) -> Self {
        Self {
            selection_type: SelectionType::Text,
            selected_text: Some(text.clone()),
            selected_image: None,
            selection: Some(text),
        }
    }

    /// Creates an image selection context
    pub fn image(image_data: String) -> Self {
        Self {
            selection_type: SelectionType::Image,
            selected_text: None,
            selected_image: Some(image_data.clone()),
            selection: Some(image_data),
        }
    }

    /// Creates an empty selection context
    pub fn none() -> Self {
        Self::default()
    }

    /// Returns true if an image is selected
    pub fn has_image(&self) -> bool {
        self.selection_type == SelectionType::Image && self.selected_image.is_some()
    }

    /// Returns true if text is selected
    pub fn has_text(&self) -> bool {
        self.selection_type == SelectionType::Text && self.selected_text.is_some()
    }

    /// Returns true if anything is selected
    pub fn has_selection(&self) -> bool {
        self.selection_type != SelectionType::None
    }
}

/// Resolves selection from various input sources
pub struct SelectionResolver;

impl SelectionResolver {
    /// Capture text selection from clipboard
    ///
    /// Returns SelectionContext with selection_type = Text if text was captured,
    /// or SelectionContext with selection_type = None if no text was selected.
    pub fn resolve_text_selection(text: Option<String>) -> SelectionContext {
        match text {
            Some(t) if !t.trim().is_empty() => SelectionContext::text(t),
            _ => SelectionContext::none(),
        }
    }

    /// Capture image selection
    ///
    /// Returns SelectionContext with selection_type = Image if an image was captured,
    /// or SelectionContext with selection_type = None if no image was selected.
    pub fn resolve_image_selection(image_data: Option<String>) -> SelectionContext {
        match image_data {
            Some(data) if !data.is_empty() => SelectionContext::image(data),
            _ => SelectionContext::none(),
        }
    }

    /// Resolve selection based on captured content
    ///
    /// This method determines the selection type by examining the captured content.
    /// Image data is expected to be a base64-encoded data URL.
    pub fn resolve(captured: Option<String>, is_image: bool) -> SelectionContext {
        if is_image {
            Self::resolve_image_selection(captured)
        } else {
            Self::resolve_text_selection(captured)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_selection() {
        let ctx = SelectionContext::text("Hello, world!".to_string());
        assert_eq!(ctx.selection_type, SelectionType::Text);
        assert!(ctx.has_text());
        assert!(!ctx.has_image());
        assert!(ctx.has_selection());
    }

    #[test]
    fn test_image_selection() {
        let ctx = SelectionContext::image("data:image/png;base64,abc123".to_string());
        assert_eq!(ctx.selection_type, SelectionType::Image);
        assert!(!ctx.has_text());
        assert!(ctx.has_image());
        assert!(ctx.has_selection());
    }

    #[test]
    fn test_none_selection() {
        let ctx = SelectionContext::none();
        assert_eq!(ctx.selection_type, SelectionType::None);
        assert!(!ctx.has_text());
        assert!(!ctx.has_image());
        assert!(!ctx.has_selection());
    }

    #[test]
    fn test_empty_text_not_selected() {
        let ctx = SelectionResolver::resolve_text_selection(Some("   ".to_string()));
        assert_eq!(ctx.selection_type, SelectionType::None);
    }
}
