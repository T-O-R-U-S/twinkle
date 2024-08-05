pub use centered_prompt::CenteredPrompt;
pub use centered_text::CenteredText;
pub use ruler::Ruler;
pub use sweeper::Sweeper;
pub use text::Text;
pub use uv::Uv;
pub use margin::Margin;
pub use centered::Centered;

/// The code for the [`Sweeper`] widget.
pub mod sweeper;
/// The code for the [`CenteredText`] widget.
pub mod centered_text;
/// The code for the [`CenteredPrompt`] widget.
pub mod centered_prompt;
/// The code for the [`Ruler`] widget.
pub mod ruler;
/// The code for the [`Text`] widget.
pub mod text;
/// The code for the [`Uv`] widget.
pub mod uv;
mod margin;
/// The code the [`dummy::Dummy`] widget.
pub mod dummy;
/// The code for the [`Centered`] widget.
pub mod centered;