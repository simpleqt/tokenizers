use crate::{PaddingParams, TruncationParams};

#[derive(Debug, Default, Clone)]
pub enum Truncation {
    /// Inherit the tokenizer's config
    #[default]
    Inherit,
    Off,
    With(TruncationParams),
}

#[derive(Debug, Default, Clone)]
pub enum Padding {
    /// Inherit the tokenizer's config
    #[default]
    Inherit,
    Off,
    With(PaddingParams),
}

#[derive(Debug, Clone)]
pub struct EncodeOptions {
    pub add_special_tokens: bool,
    pub truncation: Truncation,
    pub padding: Padding,
}

impl Default for EncodeOptions {
    fn default() -> Self {
        Self {
            add_special_tokens: true,
            truncation: Truncation::default(),
            padding: Padding::default(),
        }
    }
}
