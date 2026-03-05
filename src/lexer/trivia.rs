//! Trivia (whitespace, comments, etc.) definitions

/// Kinds of trivia that can appear in C# source
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TriviaKind {
    /// Whitespace (spaces and tabs)
    Whitespace,
    /// End of line (newline characters)
    Newline,
    /// Single-line comment (// ...)
    SingleLineComment,
    /// Multi-line comment (/* ... */)
    MultiLineComment,
    /// Documentation comment (/// or /** ... */)
    DocumentationComment,
    /// Preprocessor directive (#if, #region, etc.)
    PreprocessorDirective,
    /// Disabled text (code that's disabled by preprocessor)
    DisabledText,
    /// Skipped tokens (malformed code)
    SkippedTokens,
}