//! Diagnostic reporting for the C# compiler
//! 
//! This module handles error and warning reporting throughout compilation.

use std::fmt;
use text_size::TextRange;
use miette::SourceCode;
use thiserror::Error;

/// A collection of diagnostics
#[derive(Debug, Default)]
pub struct DiagnosticBag {
    diagnostics: Vec<Diagnostic>,
}

/// A diagnostic message (error, warning, or info)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    /// The severity of the diagnostic
    pub severity: Severity,
    /// The diagnostic code (e.g., CS0001)
    pub code: DiagnosticCode,
    /// The message text
    pub message: String,
    /// The location in source
    pub location: Option<Location>,
    /// Additional notes or suggestions
    pub notes: Vec<String>,
}

/// Diagnostic severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    /// Informational message
    Info,
    /// Warning that doesn't prevent compilation
    Warning,
    /// Error that prevents compilation
    Error,
}

/// Diagnostic codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticCode {
    // Lexer errors
    UnexpectedToken,
    UnexpectedEndOfFile,
    InvalidCharacter,
    InvalidNumber,
    UnterminatedString,
    UnterminatedComment,
    InvalidEscapeSequence,
    
    // Parser errors
    ExpectedToken,
    InvalidSyntax,
    
    // Semantic errors
    UndefinedName,
    TypeMismatch,
    DuplicateDefinition,
    InvalidOperation,
    UseOfUnassignedVariable,
    UnreachableCode,
    PossibleNullReference,
    
    // Other errors
    InternalError,
    UnexpectedSyntax,
    NotImplemented,
    
    // Roslyn-compatible codes
    CS0001, // Internal compiler error
    CS0103, // Name does not exist in the current context
    CS1002, // ; expected
    CS1003, // Syntax error
}

/// Location in source code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    /// File path
    pub file_path: Option<String>,
    /// Text range
    pub range: TextRange,
}

impl Clone for DiagnosticBag {
    fn clone(&self) -> Self {
        Self {
            diagnostics: self.diagnostics.clone(),
        }
    }
}

impl DiagnosticBag {
    /// Create a new empty diagnostic bag
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }
    
    /// Add an error diagnostic
    pub fn add_error(&mut self, message: impl Into<String>, range: TextRange) {
        self.diagnostics.push(Diagnostic {
            severity: Severity::Error,
            code: DiagnosticCode::CS1003, // Generic syntax error
            message: message.into(),
            location: Some(Location {
                file_path: None,
                range,
            }),
            notes: Vec::new(),
        });
    }
    
    /// Get an iterator over diagnostics
    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic> {
        self.diagnostics.iter()
    }
    
    /// Add a warning diagnostic
    pub fn add_warning(&mut self, code: DiagnosticCode, message: impl Into<String>, range: TextRange) {
        self.diagnostics.push(Diagnostic {
            severity: Severity::Warning,
            code,
            message: message.into(),
            location: Some(Location {
                file_path: None,
                range,
            }),
            notes: Vec::new(),
        });
    }
    
    /// Add a diagnostic
    pub fn add(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }
    
    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.severity == Severity::Error)
    }
    
    /// Get all diagnostics
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
    
    /// Take all diagnostics, leaving the bag empty
    pub fn take(&mut self) -> Vec<Diagnostic> {
        std::mem::take(&mut self.diagnostics)
    }
    
    /// Merge another diagnostic bag into this one
    pub fn merge(&mut self, other: &DiagnosticBag) {
        self.diagnostics.extend(other.diagnostics.clone());
    }
    
    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        self.diagnostics.iter().any(|d| d.severity == Severity::Warning)
    }
    
    /// Get the number of diagnostics
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }
    
    /// Check if the diagnostic bag is empty
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Lexer errors
            DiagnosticCode::UnexpectedToken => write!(f, "E0001"),
            DiagnosticCode::UnexpectedEndOfFile => write!(f, "E0002"),
            DiagnosticCode::InvalidCharacter => write!(f, "E0003"),
            DiagnosticCode::InvalidNumber => write!(f, "E0004"),
            DiagnosticCode::UnterminatedString => write!(f, "E0005"),
            DiagnosticCode::UnterminatedComment => write!(f, "E0006"),
            DiagnosticCode::InvalidEscapeSequence => write!(f, "E0007"),
            
            // Parser errors
            DiagnosticCode::ExpectedToken => write!(f, "E0101"),
            DiagnosticCode::InvalidSyntax => write!(f, "E0102"),
            
            // Semantic errors
            DiagnosticCode::UndefinedName => write!(f, "E0201"),
            DiagnosticCode::TypeMismatch => write!(f, "E0202"),
            DiagnosticCode::DuplicateDefinition => write!(f, "E0203"),
            DiagnosticCode::InvalidOperation => write!(f, "E0204"),
            DiagnosticCode::UseOfUnassignedVariable => write!(f, "E0205"),
            DiagnosticCode::UnreachableCode => write!(f, "E0206"),
            DiagnosticCode::PossibleNullReference => write!(f, "E0207"),
            
            // Other errors
            DiagnosticCode::InternalError => write!(f, "E0901"),
            DiagnosticCode::UnexpectedSyntax => write!(f, "E0902"),
            DiagnosticCode::NotImplemented => write!(f, "E0999"),
            
            // Roslyn-compatible codes
            DiagnosticCode::CS0001 => write!(f, "CS0001"),
            DiagnosticCode::CS0103 => write!(f, "CS0103"),
            DiagnosticCode::CS1002 => write!(f, "CS1002"),
            DiagnosticCode::CS1003 => write!(f, "CS1003"),
        }
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let severity = match self.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
        };
        
        write!(f, "{} {}: {}", severity, self.code, self.message)?;
        
        if let Some(location) = &self.location {
            if let Some(file) = &location.file_path {
                write!(f, " at {}:{:?}", file, location.range.start())?;
            }
        }
        
        Ok(())
    }
}

/// Convert our diagnostic to miette for pretty printing
#[derive(Error, Debug)]
#[error("{}", diagnostic)]
pub struct DiagnosticDisplay {
    diagnostic: Diagnostic,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl Diagnostic {
    /// Get the diagnostic code
    pub fn code(&self) -> DiagnosticCode {
        self.code
    }
    
    /// Get the diagnostic message
    pub fn message(&self) -> &str {
        &self.message
    }
    
    /// Create an error diagnostic
    pub fn error(code: DiagnosticCode, message: String) -> Self {
        Self {
            severity: Severity::Error,
            code,
            message,
            location: None,
            notes: Vec::new(),
        }
    }
    
    /// Create a warning diagnostic
    pub fn warning(code: DiagnosticCode, message: String) -> Self {
        Self {
            severity: Severity::Warning,
            code,
            message,
            location: None,
            notes: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_diagnostic_bag() {
        let mut bag = DiagnosticBag::new();
        assert!(!bag.has_errors());
        
        bag.add_error("Test error", TextRange::default());
        assert!(bag.has_errors());
        assert_eq!(bag.diagnostics().len(), 1);
    }
}