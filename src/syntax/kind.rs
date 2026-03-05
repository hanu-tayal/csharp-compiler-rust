//! Syntax kinds for all C# language constructs

/// All possible syntax kinds in C#
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum SyntaxKind {
    // Special
    Error = 0,
    EndOfFile,
    
    // Tokens
    IdentifierToken,
    NumericLiteralToken,
    StringLiteralToken,
    CharacterLiteralToken,
    
    // Keywords
    AbstractKeyword,
    AsKeyword,
    AsyncKeyword,
    AwaitKeyword,
    BaseKeyword,
    BoolKeyword,
    BreakKeyword,
    ByteKeyword,
    CaseKeyword,
    CatchKeyword,
    CharKeyword,
    CheckedKeyword,
    ClassKeyword,
    ConstKeyword,
    ContinueKeyword,
    DecimalKeyword,
    DefaultKeyword,
    DelegateKeyword,
    DoKeyword,
    DoubleKeyword,
    DynamicKeyword,
    ElseKeyword,
    EnumKeyword,
    EventKeyword,
    ExplicitKeyword,
    ExternKeyword,
    FalseKeyword,
    FinallyKeyword,
    FixedKeyword,
    FloatKeyword,
    ForKeyword,
    ForeachKeyword,
    GotoKeyword,
    IfKeyword,
    ImplicitKeyword,
    InKeyword,
    IntKeyword,
    InterfaceKeyword,
    InternalKeyword,
    IsKeyword,
    LockKeyword,
    LongKeyword,
    NameofKeyword,
    NamespaceKeyword,
    NewKeyword,
    NullKeyword,
    ObjectKeyword,
    OperatorKeyword,
    OutKeyword,
    OverrideKeyword,
    ParamsKeyword,
    PartialKeyword,
    PrivateKeyword,
    ProtectedKeyword,
    PublicKeyword,
    ReadonlyKeyword,
    RecordKeyword,
    RefKeyword,
    ReturnKeyword,
    SByteKeyword,
    SealedKeyword,
    ShortKeyword,
    SizeofKeyword,
    StackallocKeyword,
    StaticKeyword,
    StringKeyword,
    StructKeyword,
    SwitchKeyword,
    ThisKeyword,
    ThrowKeyword,
    TrueKeyword,
    TryKeyword,
    TypeofKeyword,
    UIntKeyword,
    ULongKeyword,
    UncheckedKeyword,
    UnsafeKeyword,
    UShortKeyword,
    UsingKeyword,
    VarKeyword,
    VirtualKeyword,
    VolatileKeyword,
    VoidKeyword,
    WhenKeyword,
    WhereKeyword,
    WhileKeyword,
    WithKeyword,
    YieldKeyword,
    
    // Operators
    PlusToken,
    MinusToken,
    StarToken,
    SlashToken,
    PercentToken,
    AmpersandToken,
    PipeToken,
    CaretToken,
    ExclamationToken,
    TildeToken,
    QuestionToken,
    ColonToken,
    SemicolonToken,
    CommaToken,
    DotToken,
    EqualsToken,
    LessThanToken,
    GreaterThanToken,
    
    // Compound operators
    PlusEqualsToken,
    MinusEqualsToken,
    StarEqualsToken,
    SlashEqualsToken,
    PercentEqualsToken,
    AmpersandEqualsToken,
    PipeEqualsToken,
    CaretEqualsToken,
    LeftShiftEqualsToken,
    RightShiftEqualsToken,
    EqualsEqualsToken,
    ExclamationEqualsToken,
    LessThanEqualsToken,
    GreaterThanEqualsToken,
    PlusPlusToken,
    MinusMinusToken,
    AmpersandAmpersandToken,
    PipePipeToken,
    LeftShiftToken,
    RightShiftToken,
    QuestionQuestionToken,
    QuestionQuestionEqualsToken,
    ColonColonToken,
    ArrowToken,
    FatArrowToken,
    DotDotToken,
    UnsignedRightShiftToken,
    UnsignedRightShiftEqualsToken,
    
    // Special tokens
    AliasKeyword,
    GlobalKeyword,
    
    // Preprocessor
    HashToken,
    HashIfToken,
    HashElseToken,
    HashElifToken,
    HashEndIfToken,
    HashDefineToken,
    HashUndefToken,
    HashWarningToken,
    HashErrorToken,
    HashLineToken,
    HashRegionToken,
    HashEndRegionToken,
    HashPragmaToken,
    HashNullableToken,
    HashReferenceToken,
    HashLoadToken,
    
    // Delimiters
    OpenParenToken,
    CloseParenToken,
    OpenBracketToken,
    CloseBracketToken,
    OpenBraceToken,
    CloseBraceToken,
    
    // Trivia
    WhitespaceTrivia,
    EndOfLineTrivia,
    SingleLineCommentTrivia,
    MultiLineCommentTrivia,
    
    // Expressions
    IdentifierName,
    QualifiedName,
    AliasQualifiedName,
    GenericName,
    TypeArgumentList,
    PredefinedType,
    ArrayType,
    PointerType,
    NullableType,
    TupleType,
    TupleElement,
    
    // Literal expressions
    NumericLiteralExpression,
    StringLiteralExpression,
    CharacterLiteralExpression,
    TrueLiteralExpression,
    FalseLiteralExpression,
    NullLiteralExpression,
    DefaultLiteralExpression,
    
    // Unary expressions
    UnaryPlusExpression,
    UnaryMinusExpression,
    BitwiseNotExpression,
    LogicalNotExpression,
    PreIncrementExpression,
    PreDecrementExpression,
    PostIncrementExpression,
    PostDecrementExpression,
    CastExpression,
    
    // Binary expressions
    AddExpression,
    SubtractExpression,
    MultiplyExpression,
    DivideExpression,
    ModuloExpression,
    LeftShiftExpression,
    RightShiftExpression,
    UnsignedRightShiftExpression,
    LogicalOrExpression,
    LogicalAndExpression,
    BitwiseOrExpression,
    BitwiseAndExpression,
    ExclusiveOrExpression,
    EqualsExpression,
    NotEqualsExpression,
    LessThanExpression,
    LessThanOrEqualExpression,
    GreaterThanExpression,
    GreaterThanOrEqualExpression,
    IsExpression,
    AsExpression,
    CoalesceExpression,
    
    // Assignment expressions
    SimpleAssignmentExpression,
    AddAssignmentExpression,
    SubtractAssignmentExpression,
    MultiplyAssignmentExpression,
    DivideAssignmentExpression,
    ModuloAssignmentExpression,
    AndAssignmentExpression,
    OrAssignmentExpression,
    ExclusiveOrAssignmentExpression,
    LeftShiftAssignmentExpression,
    RightShiftAssignmentExpression,
    
    // Member access
    SimpleMemberAccessExpression,
    PointerMemberAccessExpression,
    ConditionalAccessExpression,
    
    // Invocation
    InvocationExpression,
    ArgumentList,
    Argument,
    
    // Object creation
    ObjectCreationExpression,
    ArrayCreationExpression,
    ImplicitArrayCreationExpression,
    
    // Lambda and anonymous functions
    SimpleLambdaExpression,
    ParenthesizedLambdaExpression,
    AnonymousMethodExpression,
    
    // Other expressions
    ThisExpression,
    BaseExpression,
    SizeofExpression,
    TypeofExpression,
    CheckedExpression,
    UncheckedExpression,
    DefaultExpression,
    NameofExpression,
    InterpolatedStringExpression,
    IsPatternExpression,
    SwitchExpression,
    WithExpression,
    ThrowExpression,
    AwaitExpression,
    TupleExpression,
    ConditionalExpression,
    ElementAccessExpression,
    MemberBindingExpression,
    RefExpression,
    RefTypeExpression,
    RefValueExpression,
    MakeRefExpression,
    RangeExpression,
    IndexExpression,
    ParenthesizedExpression,
    InitializerExpression,
    StackAllocArrayCreationExpression,
    TypeOfExpression,
    SizeOfExpression,
    
    // Query expressions
    QueryExpression,
    FromClause,
    WhereClause,
    SelectClause,
    OrderByClause,
    
    // Statements
    Block,
    LocalDeclarationStatement,
    ExpressionStatement,
    EmptyStatement,
    IfStatement,
    ElseClause,
    SwitchStatement,
    SwitchSection,
    CaseLabel,
    DefaultLabel,
    WhileStatement,
    DoStatement,
    ForStatement,
    ForEachStatement,
    BreakStatement,
    ContinueStatement,
    GotoStatement,
    ReturnStatement,
    ThrowStatement,
    TryStatement,
    CatchClause,
    FinallyClause,
    LockStatement,
    UsingStatement,
    YieldReturnStatement,
    YieldBreakStatement,
    
    // Declarations
    CompilationUnit,
    NamespaceDeclaration,
    UsingDirective,
    ClassDeclaration,
    StructDeclaration,
    InterfaceDeclaration,
    EnumDeclaration,
    EnumMemberDeclaration,
    DelegateDeclaration,
    FieldDeclaration,
    PropertyDeclaration,
    MethodDeclaration,
    ConstructorDeclaration,
    DestructorDeclaration,
    EventDeclaration,
    IndexerDeclaration,
    OperatorDeclaration,
    ConversionOperatorDeclaration,
    
    // Declaration parts
    ParameterList,
    Parameter,
    TypeParameterList,
    TypeParameter,
    BaseList,
    TypeConstraintClause,
    AttributeList,
    Attribute,
    VariableDeclaration,
    VariableDeclarator,
    AttributeArgumentList,
    AttributeArgument,
    EqualsValueClause,
    LocalFunctionStatement,
    AccessorList,
    AccessorDeclaration,
    ArrayRankSpecifier,
    GetAccessor,
    SetAccessor,
    PropertyGetAccessor,
    PropertySetAccessor,
    
    // Modifiers
    PublicModifier,
    PrivateModifier,
    ProtectedModifier,
    InternalModifier,
    StaticModifier,
    VirtualModifier,
    OverrideModifier,
    AbstractModifier,
    SealedModifier,
    ConstModifier,
    ReadOnlyModifier,
    ExternModifier,
    PartialModifier,
    AsyncModifier,
}

impl SyntaxKind {
    /// Check if this is a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(self,
            SyntaxKind::AbstractKeyword |
            SyntaxKind::AsKeyword |
            SyntaxKind::BaseKeyword |
            SyntaxKind::BoolKeyword |
            SyntaxKind::BreakKeyword |
            SyntaxKind::ByteKeyword |
            SyntaxKind::CaseKeyword |
            SyntaxKind::CatchKeyword |
            SyntaxKind::CharKeyword |
            SyntaxKind::ClassKeyword |
            SyntaxKind::ConstKeyword |
            SyntaxKind::ContinueKeyword |
            SyntaxKind::DefaultKeyword |
            SyntaxKind::DoKeyword |
            SyntaxKind::DoubleKeyword |
            SyntaxKind::ElseKeyword |
            SyntaxKind::EnumKeyword |
            SyntaxKind::FalseKeyword |
            SyntaxKind::FinallyKeyword |
            SyntaxKind::FloatKeyword |
            SyntaxKind::ForKeyword |
            SyntaxKind::IfKeyword |
            SyntaxKind::InKeyword |
            SyntaxKind::IntKeyword |
            SyntaxKind::InterfaceKeyword |
            SyntaxKind::InternalKeyword |
            SyntaxKind::IsKeyword |
            SyntaxKind::LongKeyword |
            SyntaxKind::NamespaceKeyword |
            SyntaxKind::NewKeyword |
            SyntaxKind::NullKeyword |
            SyntaxKind::ObjectKeyword |
            SyntaxKind::PrivateKeyword |
            SyntaxKind::ProtectedKeyword |
            SyntaxKind::PublicKeyword |
            SyntaxKind::ReturnKeyword |
            SyntaxKind::SealedKeyword |
            SyntaxKind::ShortKeyword |
            SyntaxKind::StaticKeyword |
            SyntaxKind::StringKeyword |
            SyntaxKind::StructKeyword |
            SyntaxKind::ThisKeyword |
            SyntaxKind::ThrowKeyword |
            SyntaxKind::TrueKeyword |
            SyntaxKind::TryKeyword |
            SyntaxKind::TypeofKeyword |
            SyntaxKind::UsingKeyword |
            SyntaxKind::VoidKeyword |
            SyntaxKind::WhileKeyword
        )
    }
    
    /// Check if this is an operator
    pub fn is_operator(&self) -> bool {
        matches!(self,
            SyntaxKind::PlusToken |
            SyntaxKind::MinusToken |
            SyntaxKind::StarToken |
            SyntaxKind::SlashToken |
            SyntaxKind::PercentToken |
            SyntaxKind::AmpersandToken |
            SyntaxKind::PipeToken |
            SyntaxKind::CaretToken |
            SyntaxKind::ExclamationToken |
            SyntaxKind::TildeToken |
            SyntaxKind::EqualsToken |
            SyntaxKind::LessThanToken |
            SyntaxKind::GreaterThanToken
        )
    }
    
    /// Check if this is a literal
    pub fn is_literal(&self) -> bool {
        matches!(self,
            SyntaxKind::NumericLiteralExpression |
            SyntaxKind::StringLiteralExpression |
            SyntaxKind::CharacterLiteralExpression |
            SyntaxKind::TrueLiteralExpression |
            SyntaxKind::FalseLiteralExpression |
            SyntaxKind::NullLiteralExpression |
            SyntaxKind::DefaultLiteralExpression
        )
    }
}