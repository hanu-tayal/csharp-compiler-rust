//! Lexical analysis for C# source code
//! 
//! This module implements the lexer (tokenizer) that converts raw C# source
//! text into a stream of tokens.

use std::str::Chars;
use std::iter::Peekable;
use crate::diagnostics::DiagnosticBag;
use crate::syntax::{SyntaxKind, SyntaxToken};
use smol_str::SmolStr;
use text_size::{TextRange, TextSize};

pub mod token;
pub mod trivia;
pub mod keywords;
pub mod operators;

use self::token::TokenKind;
use self::trivia::TriviaKind;

/// Lexer modes (equivalent to Roslyn's LexerMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexerMode {
    /// Normal syntax mode
    Syntax,
    /// Debugger syntax mode
    DebuggerSyntax,
    /// Preprocessing directive mode
    Directive,
    /// XML documentation comment mode
    XmlDocComment,
    /// XML element tag mode
    XmlElementTag,
    /// XML attribute text with single quotes
    XmlAttributeTextQuote,
    /// XML attribute text with double quotes
    XmlAttributeTextDoubleQuote,
    /// XML CDATA section
    XmlCDataSectionText,
    /// XML comment text
    XmlCommentText,
    /// XML processing instruction
    XmlProcessingInstructionText,
}

/// The C# lexer
pub struct Lexer<'a> {
    /// Source text
    source: &'a str,
    /// Character iterator
    chars: Peekable<Chars<'a>>,
    /// Current position in the source
    position: TextSize,
    /// Current lexer mode
    mode: LexerMode,
    /// Diagnostics
    diagnostics: DiagnosticBag,
    /// Preprocessor directives state
    directives: DirectiveStack,
    /// Leading trivia accumulator
    leading_trivia: Vec<Trivia>,
    /// Trailing trivia accumulator
    trailing_trivia: Vec<Trivia>,
}

/// A trivia (whitespace, comments, etc.)
#[derive(Debug, Clone)]
pub struct Trivia {
    pub kind: TriviaKind,
    pub text: SmolStr,
    pub range: TextRange,
}

/// Preprocessor directive stack
#[derive(Debug, Default)]
struct DirectiveStack {
    // Simplified for now
    active_regions: Vec<bool>,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer for the given source text
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars().peekable(),
            position: TextSize::from(0),
            mode: LexerMode::Syntax,
            diagnostics: DiagnosticBag::new(),
            directives: DirectiveStack::default(),
            leading_trivia: Vec::new(),
            trailing_trivia: Vec::new(),
        }
    }
    
    /// Get the diagnostics
    pub fn diagnostics(&self) -> &DiagnosticBag {
        &self.diagnostics
    }

    /// Tokenize the entire source
    pub fn tokenize(&mut self) -> Vec<SyntaxToken> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens
    }

    /// Lex the next token
    pub fn next_token(&mut self) -> Option<SyntaxToken> {
        // Skip leading trivia
        self.scan_leading_trivia();

        let start = self.position;
        
        // Check for EOF
        let ch = self.peek_char()?;

        let kind = match ch {
            // Whitespace should have been consumed as trivia
            ' ' | '\t' | '\r' | '\n' => unreachable!("Whitespace should be trivia"),
            
            // Identifiers and keywords
            'a'..='z' | 'A'..='Z' | '_' | '@' => self.scan_identifier_or_keyword(),
            
            // Numbers
            '0'..='9' => self.scan_numeric_literal(),
            
            // Strings and characters
            '"' => self.scan_string_literal(),
            '\'' => self.scan_character_literal(),
            
            // Operators and punctuation
            '+' => self.scan_plus(),
            '-' => self.scan_minus(),
            '*' => self.scan_star(),
            '/' => self.scan_slash(),
            '%' => self.scan_percent(),
            '&' => self.scan_ampersand(),
            '|' => self.scan_pipe(),
            '^' => self.scan_caret(),
            '!' => self.scan_exclamation(),
            '~' => self.simple_token(TokenKind::Tilde),
            '=' => self.scan_equals(),
            '<' => self.scan_less_than(),
            '>' => self.scan_greater_than(),
            '?' => self.scan_question(),
            ':' => self.scan_colon(),
            ';' => self.simple_token(TokenKind::Semicolon),
            ',' => self.simple_token(TokenKind::Comma),
            '.' => self.scan_dot(),
            '(' => self.simple_token(TokenKind::OpenParen),
            ')' => self.simple_token(TokenKind::CloseParen),
            '[' => self.simple_token(TokenKind::OpenBracket),
            ']' => self.simple_token(TokenKind::CloseBracket),
            '{' => self.simple_token(TokenKind::OpenBrace),
            '}' => self.simple_token(TokenKind::CloseBrace),
            
            // Preprocessor
            '#' if self.at_start_of_line() => self.scan_preprocessor_directive(),
            
            // Unknown character
            _ => {
                self.advance();
                self.diagnostics.add_error("Unexpected character", TextRange::new(start, self.position));
                TokenKind::Error
            }
        };

        // Scan trailing trivia
        self.scan_trailing_trivia();

        let end = self.position;
        let text = &self.source[start.into()..end.into()];

        Some(SyntaxToken {
            kind: kind.into(),
            text: SmolStr::new(text),
            range: TextRange::new(start, end),
            leading_trivia: std::mem::take(&mut self.leading_trivia),
            trailing_trivia: std::mem::take(&mut self.trailing_trivia),
        })
    }

    // Helper methods

    fn peek_char(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.chars.next()?;
        self.position += TextSize::of(ch);
        Some(ch)
    }

    fn simple_token(&mut self, kind: TokenKind) -> TokenKind {
        self.advance();
        kind
    }

    fn at_start_of_line(&self) -> bool {
        // Check if we're at the start of a line (after newline or at start of file)
        if self.position == TextSize::from(0) {
            return true;
        }
        
        // Look back to see if previous character was newline
        let prev_pos = (self.position - TextSize::from(1)).into();
        self.source.chars().nth(prev_pos).map_or(false, |ch| ch == '\n')
    }

    // Scanning methods (to be implemented)

    fn scan_leading_trivia(&mut self) {
        while let Some(ch) = self.peek_char() {
            match ch {
                ' ' | '\t' => self.scan_whitespace_trivia(),
                '\r' | '\n' => self.scan_newline_trivia(),
                '/' => {
                    if self.peek_char_at(1) == Some('/') {
                        self.scan_single_line_comment();
                    } else if self.peek_char_at(1) == Some('*') {
                        self.scan_multi_line_comment();
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    fn scan_trailing_trivia(&mut self) {
        while let Some(ch) = self.peek_char() {
            match ch {
                ' ' | '\t' => self.scan_whitespace_trivia(),
                '\r' | '\n' => {
                    self.scan_newline_trivia();
                    break; // Newline ends trailing trivia
                }
                '/' => {
                    if self.peek_char_at(1) == Some('/') {
                        self.scan_single_line_comment();
                        break; // Comment includes the newline
                    } else if self.peek_char_at(1) == Some('*') {
                        self.scan_multi_line_comment();
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    fn peek_char_at(&mut self, offset: usize) -> Option<char> {
        self.source[self.position.into()..].chars().nth(offset)
    }

    // Placeholder implementations
    fn scan_whitespace_trivia(&mut self) {
        let start = self.position;
        while matches!(self.peek_char(), Some(' ') | Some('\t')) {
            self.advance();
        }
        let trivia = Trivia {
            kind: TriviaKind::Whitespace,
            text: SmolStr::new(&self.source[start.into()..self.position.into()]),
            range: TextRange::new(start, self.position),
        };
        self.leading_trivia.push(trivia);
    }

    fn scan_newline_trivia(&mut self) {
        let start = self.position;
        if self.peek_char() == Some('\r') {
            self.advance();
            if self.peek_char() == Some('\n') {
                self.advance();
            }
        } else if self.peek_char() == Some('\n') {
            self.advance();
        }
        let trivia = Trivia {
            kind: TriviaKind::Newline,
            text: SmolStr::new(&self.source[start.into()..self.position.into()]),
            range: TextRange::new(start, self.position),
        };
        self.leading_trivia.push(trivia);
    }

    fn scan_single_line_comment(&mut self) {
        let start = self.position;
        self.advance(); // /
        self.advance(); // /
        
        // Scan until newline
        while let Some(ch) = self.peek_char() {
            if ch == '\n' || ch == '\r' {
                break;
            }
            self.advance();
        }
        
        let trivia = Trivia {
            kind: TriviaKind::SingleLineComment,
            text: SmolStr::new(&self.source[start.into()..self.position.into()]),
            range: TextRange::new(start, self.position),
        };
        self.leading_trivia.push(trivia);
    }

    fn scan_multi_line_comment(&mut self) {
        let start = self.position;
        self.advance(); // /
        self.advance(); // *
        
        // Scan until */
        while let Some(ch) = self.peek_char() {
            if ch == '*' && self.peek_char_at(1) == Some('/') {
                self.advance(); // *
                self.advance(); // /
                break;
            }
            self.advance();
        }
        
        let trivia = Trivia {
            kind: TriviaKind::MultiLineComment,
            text: SmolStr::new(&self.source[start.into()..self.position.into()]),
            range: TextRange::new(start, self.position),
        };
        self.leading_trivia.push(trivia);
    }

    // Token scanning methods
    
    fn scan_identifier_or_keyword(&mut self) -> TokenKind {
        let start = self.position;
        
        // Handle verbatim identifier (@identifier)
        let is_verbatim = self.peek_char() == Some('@');
        if is_verbatim {
            self.advance();
        }
        
        // First character must be letter or underscore
        if let Some(ch) = self.peek_char() {
            if ch.is_alphabetic() || ch == '_' {
                self.advance();
            } else {
                return TokenKind::Error;
            }
        }
        
        // Continue with letters, digits, or underscores
        while let Some(ch) = self.peek_char() {
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }
        
        // Check if it's a keyword (unless it's verbatim)
        if !is_verbatim {
            let text = &self.source[start.into()..self.position.into()];
            if let Some(keyword) = keywords::get_keyword_kind(text) {
                return keyword;
            }
        }
        
        TokenKind::Identifier
    }
    
    fn scan_numeric_literal(&mut self) -> TokenKind {
        // Check for hex literal (0x or 0X)
        if self.peek_char() == Some('0') && matches!(self.peek_char_at(1), Some('x') | Some('X')) {
            self.advance(); // 0
            self.advance(); // x
            self.scan_hex_digits();
            self.scan_integer_suffix();
            return TokenKind::NumericLiteral;
        }
        
        // Check for binary literal (0b or 0B)
        if self.peek_char() == Some('0') && matches!(self.peek_char_at(1), Some('b') | Some('B')) {
            self.advance(); // 0
            self.advance(); // b
            self.scan_binary_digits();
            self.scan_integer_suffix();
            return TokenKind::NumericLiteral;
        }
        
        // Scan decimal digits
        self.scan_decimal_digits();
        
        // Check for decimal point
        if self.peek_char() == Some('.') && self.peek_char_at(1).map_or(false, |c| c.is_ascii_digit()) {
            self.advance(); // .
            self.scan_decimal_digits();
            self.scan_float_suffix();
            return TokenKind::NumericLiteral;
        }
        
        // Check for exponent
        if matches!(self.peek_char(), Some('e') | Some('E')) {
            self.scan_exponent();
            self.scan_float_suffix();
            return TokenKind::NumericLiteral;
        }
        
        // Integer or float suffix
        self.scan_numeric_suffix();
        TokenKind::NumericLiteral
    }
    
    fn scan_string_literal(&mut self) -> TokenKind {
        self.advance(); // Opening "
        
        // Check for verbatim string (@")
        let is_verbatim = self.source[..self.position.into()].ends_with("@\"");
        
        while let Some(ch) = self.peek_char() {
            match ch {
                '"' => {
                    self.advance();
                    if is_verbatim && self.peek_char() == Some('"') {
                        // Escaped quote in verbatim string
                        self.advance();
                        continue;
                    }
                    return TokenKind::StringLiteral;
                }
                '\\' if !is_verbatim => {
                    self.advance();
                    // Skip escaped character
                    if self.peek_char().is_some() {
                        self.advance();
                    }
                }
                '\r' | '\n' if !is_verbatim => {
                    // Unterminated string
                    self.diagnostics.add_error("Unterminated string literal", TextRange::new(self.position, self.position));
                    return TokenKind::Error;
                }
                _ => { self.advance(); }
            }
        }
        
        // EOF reached
        self.diagnostics.add_error("Unterminated string literal", TextRange::new(self.position, self.position));
        TokenKind::Error
    }
    
    fn scan_character_literal(&mut self) -> TokenKind {
        self.advance(); // Opening '
        
        // Handle escape sequences
        if self.peek_char() == Some('\\') {
            self.advance();
            self.scan_escape_sequence();
        } else if let Some(ch) = self.peek_char() {
            if ch != '\'' {
                self.advance();
            }
        }
        
        // Expect closing '
        if self.peek_char() == Some('\'') {
            self.advance();
            TokenKind::CharacterLiteral
        } else {
            self.diagnostics.add_error("Unterminated character literal", TextRange::new(self.position, self.position));
            TokenKind::Error
        }
    }
    
    fn scan_plus(&mut self) -> TokenKind {
        self.advance();
        match self.peek_char() {
            Some('+') => {
                self.advance();
                TokenKind::PlusPlus
            }
            Some('=') => {
                self.advance();
                TokenKind::PlusEquals
            }
            _ => TokenKind::Plus,
        }
    }
    
    fn scan_minus(&mut self) -> TokenKind {
        self.advance();
        match self.peek_char() {
            Some('-') => {
                self.advance();
                TokenKind::MinusMinus
            }
            Some('=') => {
                self.advance();
                TokenKind::MinusEquals
            }
            Some('>') => {
                self.advance();
                TokenKind::Arrow
            }
            _ => TokenKind::Minus,
        }
    }
    
    fn scan_star(&mut self) -> TokenKind {
        self.advance();
        if self.peek_char() == Some('=') {
            self.advance();
            TokenKind::StarEquals
        } else {
            TokenKind::Star
        }
    }
    
    fn scan_slash(&mut self) -> TokenKind {
        self.advance();
        if self.peek_char() == Some('=') {
            self.advance();
            TokenKind::SlashEquals
        } else {
            TokenKind::Slash
        }
    }
    
    fn scan_percent(&mut self) -> TokenKind {
        self.advance();
        if self.peek_char() == Some('=') {
            self.advance();
            TokenKind::PercentEquals
        } else {
            TokenKind::Percent
        }
    }
    
    fn scan_ampersand(&mut self) -> TokenKind {
        self.advance();
        match self.peek_char() {
            Some('&') => {
                self.advance();
                TokenKind::AmpersandAmpersand
            }
            Some('=') => {
                self.advance();
                TokenKind::AmpersandEquals
            }
            _ => TokenKind::Ampersand,
        }
    }
    
    fn scan_pipe(&mut self) -> TokenKind {
        self.advance();
        match self.peek_char() {
            Some('|') => {
                self.advance();
                TokenKind::PipePipe
            }
            Some('=') => {
                self.advance();
                TokenKind::PipeEquals
            }
            _ => TokenKind::Pipe,
        }
    }
    
    fn scan_caret(&mut self) -> TokenKind {
        self.advance();
        if self.peek_char() == Some('=') {
            self.advance();
            TokenKind::CaretEquals
        } else {
            TokenKind::Caret
        }
    }
    
    fn scan_exclamation(&mut self) -> TokenKind {
        self.advance();
        if self.peek_char() == Some('=') {
            self.advance();
            TokenKind::ExclamationEquals
        } else {
            TokenKind::Exclamation
        }
    }
    
    fn scan_equals(&mut self) -> TokenKind {
        self.advance();
        match self.peek_char() {
            Some('=') => {
                self.advance();
                TokenKind::EqualsEquals
            }
            Some('>') => {
                self.advance();
                TokenKind::FatArrow
            }
            _ => TokenKind::Equals,
        }
    }
    
    fn scan_less_than(&mut self) -> TokenKind {
        self.advance();
        match self.peek_char() {
            Some('<') => {
                self.advance();
                if self.peek_char() == Some('=') {
                    self.advance();
                    TokenKind::LeftShiftEquals
                } else {
                    TokenKind::LeftShift
                }
            }
            Some('=') => {
                self.advance();
                TokenKind::LessThanEquals
            }
            _ => TokenKind::LessThan,
        }
    }
    
    fn scan_greater_than(&mut self) -> TokenKind {
        self.advance();
        match self.peek_char() {
            Some('>') => {
                self.advance();
                match self.peek_char() {
                    Some('>') => {
                        self.advance();
                        if self.peek_char() == Some('=') {
                            self.advance();
                            TokenKind::UnsignedRightShiftEquals
                        } else {
                            TokenKind::UnsignedRightShift
                        }
                    }
                    Some('=') => {
                        self.advance();
                        TokenKind::RightShiftEquals
                    }
                    _ => TokenKind::RightShift,
                }
            }
            Some('=') => {
                self.advance();
                TokenKind::GreaterThanEquals
            }
            _ => TokenKind::GreaterThan,
        }
    }
    
    fn scan_question(&mut self) -> TokenKind {
        self.advance();
        match self.peek_char() {
            Some('?') => {
                self.advance();
                if self.peek_char() == Some('=') {
                    self.advance();
                    TokenKind::QuestionQuestionEquals
                } else {
                    TokenKind::QuestionQuestion
                }
            }
            _ => TokenKind::Question,
        }
    }
    
    fn scan_colon(&mut self) -> TokenKind {
        self.advance();
        if self.peek_char() == Some(':') {
            self.advance();
            TokenKind::ColonColon
        } else {
            TokenKind::Colon
        }
    }
    
    fn scan_dot(&mut self) -> TokenKind {
        self.advance();
        if self.peek_char() == Some('.') {
            self.advance();
            TokenKind::DotDot
        } else {
            TokenKind::Dot
        }
    }
    
    fn scan_preprocessor_directive(&mut self) -> TokenKind {
        self.advance(); // #
        
        // Skip whitespace
        while matches!(self.peek_char(), Some(' ') | Some('\t')) {
            self.advance();
        }
        
        // Read directive name
        let start = self.position;
        while let Some(ch) = self.peek_char() {
            if ch.is_alphabetic() {
                self.advance();
            } else {
                break;
            }
        }
        
        let directive = &self.source[start.into()..self.position.into()];
        match directive {
            "if" => TokenKind::HashIf,
            "else" => TokenKind::HashElse,
            "elif" => TokenKind::HashElif,
            "endif" => TokenKind::HashEndIf,
            "define" => TokenKind::HashDefine,
            "undef" => TokenKind::HashUndef,
            "warning" => TokenKind::HashWarning,
            "error" => TokenKind::HashError,
            "line" => TokenKind::HashLine,
            "region" => TokenKind::HashRegion,
            "endregion" => TokenKind::HashEndRegion,
            "pragma" => TokenKind::HashPragma,
            "nullable" => TokenKind::HashNullable,
            "reference" => TokenKind::HashReference,
            "load" => TokenKind::HashLoad,
            _ => TokenKind::HashToken,
        }
    }
    
    // Helper methods for numeric literal scanning
    
    fn scan_decimal_digits(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch.is_ascii_digit() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    fn scan_hex_digits(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch.is_ascii_hexdigit() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    fn scan_binary_digits(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch == '0' || ch == '1' || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    fn scan_exponent(&mut self) {
        if matches!(self.peek_char(), Some('e') | Some('E')) {
            self.advance();
            if matches!(self.peek_char(), Some('+') | Some('-')) {
                self.advance();
            }
            self.scan_decimal_digits();
        }
    }
    
    fn scan_numeric_suffix(&mut self) {
        match self.peek_char() {
            Some('u') | Some('U') => {
                self.advance();
                if matches!(self.peek_char(), Some('l') | Some('L')) {
                    self.advance();
                }
            }
            Some('l') | Some('L') => {
                self.advance();
                if matches!(self.peek_char(), Some('u') | Some('U')) {
                    self.advance();
                }
            }
            Some('f') | Some('F') | Some('d') | Some('D') | Some('m') | Some('M') => {
                self.advance();
            }
            _ => {}
        }
    }
    
    fn scan_integer_suffix(&mut self) {
        self.scan_numeric_suffix();
    }
    
    fn scan_float_suffix(&mut self) {
        if matches!(self.peek_char(), Some('f') | Some('F') | Some('d') | Some('D') | Some('m') | Some('M')) {
            self.advance();
        }
    }
    
    fn scan_escape_sequence(&mut self) {
        match self.peek_char() {
            Some('\'') | Some('"') | Some('\\') | Some('0') | Some('a') | Some('b') | 
            Some('f') | Some('n') | Some('r') | Some('t') | Some('v') => {
                self.advance();
            }
            Some('x') => {
                self.advance();
                // Hex escape sequence
                for _ in 0..2 {
                    if self.peek_char().map_or(false, |c| c.is_ascii_hexdigit()) {
                        self.advance();
                    }
                }
            }
            Some('u') => {
                self.advance();
                // Unicode escape sequence
                for _ in 0..4 {
                    if self.peek_char().map_or(false, |c| c.is_ascii_hexdigit()) {
                        self.advance();
                    }
                }
            }
            Some('U') => {
                self.advance();
                // Long unicode escape sequence
                for _ in 0..8 {
                    if self.peek_char().map_or(false, |c| c.is_ascii_hexdigit()) {
                        self.advance();
                    }
                }
            }
            _ => {
                // Invalid escape sequence
                if self.peek_char().is_some() {
                    self.advance();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokens() {
        let mut lexer = Lexer::new("+ - * / ;");
        
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::Plus));
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::Minus));
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::Star));
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::Slash));
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::Semicolon));
    }
    
    #[test]
    fn test_identifiers_and_keywords() {
        let mut lexer = Lexer::new("foo class Bar public _test @class");
        
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::Identifier));
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::Class));
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::Identifier));
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::Public));
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::Identifier));
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::Identifier)); // @class is identifier
    }
    
    #[test]
    fn test_numeric_literals() {
        let mut lexer = Lexer::new("123 456.789 1.23e4 0xFF 0b1010 100_000");
        
        for _ in 0..6 {
            assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::NumericLiteral));
        }
    }
    
    #[test]
    fn test_string_literals() {
        let mut lexer = Lexer::new(r#""hello" "world\n" @"verbatim""#);
        
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::StringLiteral));
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::StringLiteral));
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::StringLiteral));
    }
    
    #[test]
    fn test_compound_operators() {
        let mut lexer = Lexer::new("++ -- += -= == != <= >= && || ?? ??= => ->");
        
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::PlusPlus));
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::MinusMinus));
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::PlusEquals));
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::MinusEquals));
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::EqualsEquals));
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::ExclamationEquals));
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::LessThanEquals));
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::GreaterThanEquals));
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::AmpersandAmpersand));
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::PipePipe));
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::QuestionQuestion));
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::QuestionQuestionEquals));
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::FatArrow));
        assert_eq!(lexer.next_token().unwrap().kind, SyntaxKind::from(TokenKind::Arrow));
    }
}