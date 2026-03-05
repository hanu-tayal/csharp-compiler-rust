//! Token definitions for the C# lexer

use crate::syntax::SyntaxKind;

/// Token kinds produced by the lexer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum TokenKind {
    // Special
    Error = 0,
    EndOfFile,
    
    // Identifiers and literals
    Identifier,
    NumericLiteral,
    StringLiteral,
    CharacterLiteral,
    InterpolatedStringStart,
    InterpolatedStringMid,
    InterpolatedStringEnd,
    
    // Keywords
    Abstract,
    As,
    Base,
    Bool,
    Break,
    Byte,
    Case,
    Catch,
    Char,
    Checked,
    Class,
    Const,
    Continue,
    Decimal,
    Default,
    Delegate,
    Do,
    Double,
    Else,
    Enum,
    Event,
    Explicit,
    Extern,
    False,
    Finally,
    Fixed,
    Float,
    For,
    Foreach,
    Goto,
    If,
    Implicit,
    In,
    Int,
    Interface,
    Internal,
    Is,
    Lock,
    Long,
    Namespace,
    New,
    Null,
    Object,
    Operator,
    Out,
    Override,
    Params,
    Private,
    Protected,
    Public,
    Readonly,
    Ref,
    Return,
    Sbyte,
    Sealed,
    Short,
    Sizeof,
    Stackalloc,
    Static,
    String,
    Struct,
    Switch,
    This,
    Throw,
    True,
    Try,
    Typeof,
    Uint,
    Ulong,
    Unchecked,
    Unsafe,
    Ushort,
    Using,
    Virtual,
    Void,
    Volatile,
    While,
    
    // Contextual keywords
    Add,
    Alias,
    Ascending,
    Async,
    Await,
    By,
    Descending,
    Dynamic,
    Equals,
    From,
    Get,
    Global,
    Group,
    Init,
    Into,
    Join,
    Let,
    Managed,
    Nameof,
    Nint,
    Not,
    Notnull,
    Nuint,
    On,
    Or,
    Orderby,
    Partial,
    Record,
    Remove,
    Required,
    Scoped,
    Select,
    Set,
    Unmanaged,
    Value,
    Var,
    When,
    Where,
    With,
    Yield,
    
    // Operators and punctuation
    Plus,                  // +
    Minus,                 // -
    Star,                  // *
    Slash,                 // /
    Percent,               // %
    Ampersand,             // &
    Pipe,                  // |
    Caret,                 // ^
    Exclamation,           // !
    Tilde,                 // ~
    PlusPlus,              // ++
    MinusMinus,            // --
    LeftShift,             // <<
    RightShift,            // >>
    UnsignedRightShift,    // >>>
    EqualsEquals,          // ==
    ExclamationEquals,     // !=
    LessThan,              // <
    GreaterThan,           // >
    LessThanEquals,        // <=
    GreaterThanEquals,     // >=
    AmpersandAmpersand,    // &&
    PipePipe,              // ||
    Question,              // ?
    QuestionQuestion,      // ??
    Colon,                 // :
    ColonColon,            // ::
    Semicolon,             // ;
    Comma,                 // ,
    Dot,                   // .
    DotDot,                // ..
    Arrow,                 // ->
    FatArrow,              // =>
    
    // Assignment operators
    // Equals is already defined in contextual keywords
    PlusEquals,            // +=
    MinusEquals,           // -=
    StarEquals,            // *=
    SlashEquals,           // /=
    PercentEquals,         // %=
    AmpersandEquals,       // &=
    PipeEquals,            // |=
    CaretEquals,           // ^=
    LeftShiftEquals,       // <<=
    RightShiftEquals,      // >>=
    UnsignedRightShiftEquals, // >>>=
    QuestionQuestionEquals,   // ??=
    
    // Delimiters
    OpenParen,             // (
    CloseParen,            // )
    OpenBracket,           // [
    CloseBracket,          // ]
    OpenBrace,             // {
    CloseBrace,            // }
    
    // Preprocessor
    HashToken,             // #
    HashIf,
    HashElse,
    HashElif,
    HashEndIf,
    HashDefine,
    HashUndef,
    HashWarning,
    HashError,
    HashLine,
    HashRegion,
    HashEndRegion,
    HashPragma,
    HashNullable,
    HashReference,
    HashLoad,
}

impl From<TokenKind> for SyntaxKind {
    fn from(token: TokenKind) -> Self {
        // Convert TokenKind to SyntaxKind
        // This is a simplified mapping
        match token {
            TokenKind::Error => SyntaxKind::Error,
            TokenKind::EndOfFile => SyntaxKind::EndOfFile,
            TokenKind::Identifier => SyntaxKind::IdentifierToken,
            TokenKind::NumericLiteral => SyntaxKind::NumericLiteralToken,
            TokenKind::StringLiteral => SyntaxKind::StringLiteralToken,
            TokenKind::CharacterLiteral => SyntaxKind::CharacterLiteralToken,
            
            // Keywords
            TokenKind::Abstract => SyntaxKind::AbstractKeyword,
            TokenKind::As => SyntaxKind::AsKeyword,
            TokenKind::Base => SyntaxKind::BaseKeyword,
            TokenKind::Bool => SyntaxKind::BoolKeyword,
            TokenKind::Break => SyntaxKind::BreakKeyword,
            TokenKind::Byte => SyntaxKind::ByteKeyword,
            TokenKind::Case => SyntaxKind::CaseKeyword,
            TokenKind::Catch => SyntaxKind::CatchKeyword,
            TokenKind::Char => SyntaxKind::CharKeyword,
            TokenKind::Class => SyntaxKind::ClassKeyword,
            TokenKind::Const => SyntaxKind::ConstKeyword,
            TokenKind::Continue => SyntaxKind::ContinueKeyword,
            TokenKind::Default => SyntaxKind::DefaultKeyword,
            TokenKind::Do => SyntaxKind::DoKeyword,
            TokenKind::Double => SyntaxKind::DoubleKeyword,
            TokenKind::Else => SyntaxKind::ElseKeyword,
            TokenKind::Enum => SyntaxKind::EnumKeyword,
            TokenKind::False => SyntaxKind::FalseKeyword,
            TokenKind::Finally => SyntaxKind::FinallyKeyword,
            TokenKind::Float => SyntaxKind::FloatKeyword,
            TokenKind::For => SyntaxKind::ForKeyword,
            TokenKind::If => SyntaxKind::IfKeyword,
            TokenKind::In => SyntaxKind::InKeyword,
            TokenKind::Int => SyntaxKind::IntKeyword,
            TokenKind::Interface => SyntaxKind::InterfaceKeyword,
            TokenKind::Internal => SyntaxKind::InternalKeyword,
            TokenKind::Is => SyntaxKind::IsKeyword,
            TokenKind::Long => SyntaxKind::LongKeyword,
            TokenKind::Namespace => SyntaxKind::NamespaceKeyword,
            TokenKind::New => SyntaxKind::NewKeyword,
            TokenKind::Null => SyntaxKind::NullKeyword,
            TokenKind::Object => SyntaxKind::ObjectKeyword,
            TokenKind::Private => SyntaxKind::PrivateKeyword,
            TokenKind::Protected => SyntaxKind::ProtectedKeyword,
            TokenKind::Public => SyntaxKind::PublicKeyword,
            TokenKind::Return => SyntaxKind::ReturnKeyword,
            TokenKind::Sealed => SyntaxKind::SealedKeyword,
            TokenKind::Short => SyntaxKind::ShortKeyword,
            TokenKind::Static => SyntaxKind::StaticKeyword,
            TokenKind::String => SyntaxKind::StringKeyword,
            TokenKind::Struct => SyntaxKind::StructKeyword,
            TokenKind::This => SyntaxKind::ThisKeyword,
            TokenKind::Throw => SyntaxKind::ThrowKeyword,
            TokenKind::True => SyntaxKind::TrueKeyword,
            TokenKind::Try => SyntaxKind::TryKeyword,
            TokenKind::Typeof => SyntaxKind::TypeofKeyword,
            TokenKind::Using => SyntaxKind::UsingKeyword,
            TokenKind::Void => SyntaxKind::VoidKeyword,
            TokenKind::While => SyntaxKind::WhileKeyword,
            
            // Operators
            TokenKind::Plus => SyntaxKind::PlusToken,
            TokenKind::Minus => SyntaxKind::MinusToken,
            TokenKind::Star => SyntaxKind::StarToken,
            TokenKind::Slash => SyntaxKind::SlashToken,
            TokenKind::Percent => SyntaxKind::PercentToken,
            TokenKind::Ampersand => SyntaxKind::AmpersandToken,
            TokenKind::Pipe => SyntaxKind::PipeToken,
            TokenKind::Caret => SyntaxKind::CaretToken,
            TokenKind::Exclamation => SyntaxKind::ExclamationToken,
            TokenKind::Tilde => SyntaxKind::TildeToken,
            TokenKind::Question => SyntaxKind::QuestionToken,
            TokenKind::Colon => SyntaxKind::ColonToken,
            TokenKind::Semicolon => SyntaxKind::SemicolonToken,
            TokenKind::Comma => SyntaxKind::CommaToken,
            TokenKind::Dot => SyntaxKind::DotToken,
            TokenKind::Equals => SyntaxKind::EqualsToken,
            TokenKind::LessThan => SyntaxKind::LessThanToken,
            TokenKind::GreaterThan => SyntaxKind::GreaterThanToken,
            
            // Delimiters
            TokenKind::OpenParen => SyntaxKind::OpenParenToken,
            TokenKind::CloseParen => SyntaxKind::CloseParenToken,
            TokenKind::OpenBracket => SyntaxKind::OpenBracketToken,
            TokenKind::CloseBracket => SyntaxKind::CloseBracketToken,
            TokenKind::OpenBrace => SyntaxKind::OpenBraceToken,
            TokenKind::CloseBrace => SyntaxKind::CloseBraceToken,
            
            // Add more mappings as needed
            _ => SyntaxKind::Error,
        }
    }
}