//! Keyword recognition for C#

use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::lexer::token::TokenKind;

/// Map of keyword strings to token kinds
pub static KEYWORDS: Lazy<HashMap<&'static str, TokenKind>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    // Keywords
    map.insert("abstract", TokenKind::Abstract);
    map.insert("as", TokenKind::As);
    map.insert("base", TokenKind::Base);
    map.insert("bool", TokenKind::Bool);
    map.insert("break", TokenKind::Break);
    map.insert("byte", TokenKind::Byte);
    map.insert("case", TokenKind::Case);
    map.insert("catch", TokenKind::Catch);
    map.insert("char", TokenKind::Char);
    map.insert("checked", TokenKind::Checked);
    map.insert("class", TokenKind::Class);
    map.insert("const", TokenKind::Const);
    map.insert("continue", TokenKind::Continue);
    map.insert("decimal", TokenKind::Decimal);
    map.insert("default", TokenKind::Default);
    map.insert("delegate", TokenKind::Delegate);
    map.insert("do", TokenKind::Do);
    map.insert("double", TokenKind::Double);
    map.insert("else", TokenKind::Else);
    map.insert("enum", TokenKind::Enum);
    map.insert("event", TokenKind::Event);
    map.insert("explicit", TokenKind::Explicit);
    map.insert("extern", TokenKind::Extern);
    map.insert("false", TokenKind::False);
    map.insert("finally", TokenKind::Finally);
    map.insert("fixed", TokenKind::Fixed);
    map.insert("float", TokenKind::Float);
    map.insert("for", TokenKind::For);
    map.insert("foreach", TokenKind::Foreach);
    map.insert("goto", TokenKind::Goto);
    map.insert("if", TokenKind::If);
    map.insert("implicit", TokenKind::Implicit);
    map.insert("in", TokenKind::In);
    map.insert("int", TokenKind::Int);
    map.insert("interface", TokenKind::Interface);
    map.insert("internal", TokenKind::Internal);
    map.insert("is", TokenKind::Is);
    map.insert("lock", TokenKind::Lock);
    map.insert("long", TokenKind::Long);
    map.insert("namespace", TokenKind::Namespace);
    map.insert("new", TokenKind::New);
    map.insert("null", TokenKind::Null);
    map.insert("object", TokenKind::Object);
    map.insert("operator", TokenKind::Operator);
    map.insert("out", TokenKind::Out);
    map.insert("override", TokenKind::Override);
    map.insert("params", TokenKind::Params);
    map.insert("private", TokenKind::Private);
    map.insert("protected", TokenKind::Protected);
    map.insert("public", TokenKind::Public);
    map.insert("readonly", TokenKind::Readonly);
    map.insert("ref", TokenKind::Ref);
    map.insert("return", TokenKind::Return);
    map.insert("sbyte", TokenKind::Sbyte);
    map.insert("sealed", TokenKind::Sealed);
    map.insert("short", TokenKind::Short);
    map.insert("sizeof", TokenKind::Sizeof);
    map.insert("stackalloc", TokenKind::Stackalloc);
    map.insert("static", TokenKind::Static);
    map.insert("string", TokenKind::String);
    map.insert("struct", TokenKind::Struct);
    map.insert("switch", TokenKind::Switch);
    map.insert("this", TokenKind::This);
    map.insert("throw", TokenKind::Throw);
    map.insert("true", TokenKind::True);
    map.insert("try", TokenKind::Try);
    map.insert("typeof", TokenKind::Typeof);
    map.insert("uint", TokenKind::Uint);
    map.insert("ulong", TokenKind::Ulong);
    map.insert("unchecked", TokenKind::Unchecked);
    map.insert("unsafe", TokenKind::Unsafe);
    map.insert("ushort", TokenKind::Ushort);
    map.insert("using", TokenKind::Using);
    map.insert("virtual", TokenKind::Virtual);
    map.insert("void", TokenKind::Void);
    map.insert("volatile", TokenKind::Volatile);
    map.insert("while", TokenKind::While);
    
    // Contextual keywords
    map.insert("add", TokenKind::Add);
    map.insert("alias", TokenKind::Alias);
    map.insert("ascending", TokenKind::Ascending);
    map.insert("async", TokenKind::Async);
    map.insert("await", TokenKind::Await);
    map.insert("by", TokenKind::By);
    map.insert("descending", TokenKind::Descending);
    map.insert("dynamic", TokenKind::Dynamic);
    map.insert("equals", TokenKind::Equals);
    map.insert("from", TokenKind::From);
    map.insert("get", TokenKind::Get);
    map.insert("global", TokenKind::Global);
    map.insert("group", TokenKind::Group);
    map.insert("init", TokenKind::Init);
    map.insert("into", TokenKind::Into);
    map.insert("join", TokenKind::Join);
    map.insert("let", TokenKind::Let);
    map.insert("managed", TokenKind::Managed);
    map.insert("nameof", TokenKind::Nameof);
    map.insert("nint", TokenKind::Nint);
    map.insert("not", TokenKind::Not);
    map.insert("notnull", TokenKind::Notnull);
    map.insert("nuint", TokenKind::Nuint);
    map.insert("on", TokenKind::On);
    map.insert("or", TokenKind::Or);
    map.insert("orderby", TokenKind::Orderby);
    map.insert("partial", TokenKind::Partial);
    map.insert("record", TokenKind::Record);
    map.insert("remove", TokenKind::Remove);
    map.insert("required", TokenKind::Required);
    map.insert("scoped", TokenKind::Scoped);
    map.insert("select", TokenKind::Select);
    map.insert("set", TokenKind::Set);
    map.insert("unmanaged", TokenKind::Unmanaged);
    map.insert("value", TokenKind::Value);
    map.insert("var", TokenKind::Var);
    map.insert("when", TokenKind::When);
    map.insert("where", TokenKind::Where);
    map.insert("with", TokenKind::With);
    map.insert("yield", TokenKind::Yield);
    
    map
});

/// Check if a string is a keyword and return its token kind
pub fn get_keyword_kind(text: &str) -> Option<TokenKind> {
    KEYWORDS.get(text).copied()
}