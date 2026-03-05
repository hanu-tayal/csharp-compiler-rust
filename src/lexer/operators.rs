//! Operator handling for the lexer

use crate::lexer::token::TokenKind;

/// Information about an operator
#[derive(Debug, Clone)]
pub struct OperatorInfo {
    pub kind: TokenKind,
    pub precedence: u8,
    pub associativity: Associativity,
}

/// Operator associativity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Associativity {
    Left,
    Right,
}

/// Get operator info for a token kind
pub fn get_operator_info(kind: TokenKind) -> Option<OperatorInfo> {
    use TokenKind::*;
    use Associativity::*;
    
    let (precedence, associativity) = match kind {
        // Primary
        Dot | Arrow | OpenBracket | OpenParen => (15, Left),
        
        // Unary
        Plus | Minus | Exclamation | Tilde | PlusPlus | MinusMinus => (14, Right),
        
        // Multiplicative
        Star | Slash | Percent => (13, Left),
        
        // Additive
        // Plus | Minus => (12, Left), // Already handled as unary
        
        // Shift
        LeftShift | RightShift | UnsignedRightShift => (11, Left),
        
        // Relational and type testing
        LessThan | GreaterThan | LessThanEquals | GreaterThanEquals | Is | As => (10, Left),
        
        // Equality
        EqualsEquals | ExclamationEquals => (9, Left),
        
        // Logical AND
        Ampersand => (8, Left),
        
        // Logical XOR
        Caret => (7, Left),
        
        // Logical OR
        Pipe => (6, Left),
        
        // Conditional AND
        AmpersandAmpersand => (5, Left),
        
        // Conditional OR
        PipePipe => (4, Left),
        
        // Null coalescing
        QuestionQuestion => (3, Right),
        
        // Conditional
        Question => (2, Right),
        
        // Assignment and lambda
        Equals | PlusEquals | MinusEquals | StarEquals | SlashEquals | 
        PercentEquals | AmpersandEquals | PipeEquals | CaretEquals | 
        LeftShiftEquals | RightShiftEquals | UnsignedRightShiftEquals | 
        QuestionQuestionEquals | FatArrow => (1, Right),
        
        _ => return None,
    };
    
    Some(OperatorInfo {
        kind,
        precedence,
        associativity,
    })
}