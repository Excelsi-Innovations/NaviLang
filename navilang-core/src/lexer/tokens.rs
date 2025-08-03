// Token definitions using logos for the NaviLang lexer (Case-insensitive)
use logos::Logos;
use serde::{Deserialize, Serialize};

#[derive(Logos, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Token {
    // Keywords - Core NaviLang vocabulary (Case-insensitive)
    #[regex(r"(?i)var")]
    Var,
    
    #[regex(r"(?i)context")]
    Context,
    
    #[regex(r"(?i)goes")]
    Goes,
    
    #[regex(r"(?i)to")]
    To,
    
    #[regex(r"(?i)created")]
    Created,
    
    #[regex(r"(?i)by")]
    By,
    
    #[regex(r"(?i)if")]
    If,
    
    #[regex(r"(?i)then")]
    Then,
    
    #[regex(r"(?i)when")]
    When,
    
    #[regex(r"(?i)calls")]
    Calls,
    
    #[regex(r"(?i)receives")]
    Receives,
    
    #[regex(r"(?i)returns")]
    Returns,
    
    #[regex(r"(?i)does")]
    Does,
    
    #[regex(r"(?i)uses")]
    Uses,
    
    #[regex(r"(?i)is")]
    Is,
    
    #[regex(r"(?i)after")]
    After,
    
    #[regex(r"(?i)before")]
    Before,
    
    #[regex(r"(?i)parallel")]
    Parallel,
    
    #[regex(r"(?i)and")]
    And,
    
    #[regex(r"(?i)or")]
    Or,
    
    // Advanced keywords for complex flows (Case-insensitive)
    #[regex(r"(?i)retry")]
    Retry,
    
    #[regex(r"(?i)timeout")]
    Timeout,
    
    #[regex(r"(?i)async")]
    Async,
    
    #[regex(r"(?i)batch")]
    Batch,
    
    #[regex(r"(?i)loop")]
    Loop,
    
    #[regex(r"(?i)while")]
    While,
    
    #[regex(r"(?i)break")]
    Break,
    
    #[regex(r"(?i)continue")]
    Continue,
    
    // Punctuation and operators
    #[token("{")]
    LeftBrace,
    
    #[token("}")]
    RightBrace,
    
    #[token(":")]
    Colon,
    
    #[token("[")]
    LeftBracket,
    
    #[token("]")]
    RightBracket,
    
    #[token(",")]
    Comma,
    
    #[token("(")]
    LeftParen,
    
    #[token(")")]
    RightParen,
    
    #[token("=")]
    Equals,
    
    #[token("!=")]
    NotEquals,
    
    #[token("<")]
    LessThan,
    
    #[token(">")]
    GreaterThan,
    
    #[token("<=")]
    LessEqual,
    
    #[token(">=")]
    GreaterEqual,
    
    // Identifiers and literals
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),
    
    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = lex.slice();
        s[1..s.len()-1].to_string() // Remove quotes
    })]
    QuotedString(String),
    
    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().ok())]
    Number(Option<i64>),
    
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f64>().ok())]
    Float(Option<f64>),
    
    #[regex(r"[0-9]+s|[0-9]+ms|[0-9]+m|[0-9]+h", |lex| lex.slice().to_string())]
    Duration(String),
    
    // Type annotations (Case-insensitive)
    #[regex(r"(?i)entity")]
    Entity,
    
    #[regex(r"(?i)service")]
    Service,
    
    #[regex(r"(?i)endpoint")]
    Endpoint,
    
    #[regex(r"(?i)object")]
    Object,
    
    #[regex(r"(?i)string")]
    StringType,
    
    #[regex(r"(?i)number")]
    NumberType,
    
    #[regex(r"(?i)boolean")]
    BooleanType,
    
    // Boolean literals (Case-insensitive)
    #[regex(r"(?i)true")]
    True,
    
    #[regex(r"(?i)false")]
    False,
    
    // Whitespace and comments (handled by tokenizer)
    #[regex(r"[ \t\f]+")]
    Whitespace,
    
    #[regex(r"\r?\n")]
    Newline,
    
    #[regex(r"//[^\r\n]*")]
    Comment,
    
    #[regex(r"/\*([^*]|\*[^/])*\*/")]
    BlockComment,
}

impl Token {
    /// Check if this token is a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(self, 
            Token::Var | Token::Context | Token::Goes | Token::To |
            Token::Created | Token::By | Token::If | Token::Then |
            Token::When | Token::Calls | Token::Receives | Token::Returns |
            Token::Does | Token::Uses | Token::Is | Token::After |
            Token::Before | Token::Parallel | Token::And | Token::Or |
            Token::Retry | Token::Timeout | Token::Async | Token::Batch |
            Token::Loop | Token::While | Token::Break | Token::Continue |
            Token::Entity | Token::Service | Token::Endpoint | Token::Object |
            Token::StringType | Token::NumberType | Token::BooleanType |
            Token::True | Token::False
        )
    }
    
    /// Check if this token should be filtered out (whitespace/comments)
    pub fn is_whitespace(&self) -> bool {
        matches!(self, 
            Token::Whitespace | Token::Newline | 
            Token::Comment | Token::BlockComment
        )
    }
    
    /// Check if this token is a type annotation
    pub fn is_type(&self) -> bool {
        matches!(self,
            Token::Entity | Token::Service | Token::Endpoint | Token::Object |
            Token::StringType | Token::NumberType | Token::BooleanType
        )
    }
    
    /// Check if this token is a literal value
    pub fn is_literal(&self) -> bool {
        matches!(self,
            Token::QuotedString(_) | Token::Number(_) | Token::Float(_) |
            Token::Duration(_) | Token::True | Token::False
        )
    }
    
    /// Get the string representation of the token (for error messages)
    pub fn to_string(&self) -> String {
        match self {
            Token::Var => "VAR".to_string(),
            Token::Context => "CONTEXT".to_string(),
            Token::Goes => "GOES".to_string(),
            Token::To => "TO".to_string(),
            Token::Created => "CREATED".to_string(),
            Token::By => "BY".to_string(),
            Token::If => "IF".to_string(),
            Token::Then => "THEN".to_string(),
            Token::When => "WHEN".to_string(),
            Token::Calls => "CALLS".to_string(),
            Token::Receives => "RECEIVES".to_string(),
            Token::Returns => "RETURNS".to_string(),
            Token::Does => "DOES".to_string(),
            Token::Uses => "USES".to_string(),
            Token::Is => "IS".to_string(),
            Token::After => "AFTER".to_string(),
            Token::Before => "BEFORE".to_string(),
            Token::Parallel => "PARALLEL".to_string(),
            Token::And => "AND".to_string(),
            Token::Or => "OR".to_string(),
            Token::Retry => "RETRY".to_string(),
            Token::Timeout => "TIMEOUT".to_string(),
            Token::Async => "ASYNC".to_string(),
            Token::Batch => "BATCH".to_string(),
            Token::Loop => "LOOP".to_string(),
            Token::While => "WHILE".to_string(),
            Token::Break => "BREAK".to_string(),
            Token::Continue => "CONTINUE".to_string(),
            Token::LeftBrace => "{".to_string(),
            Token::RightBrace => "}".to_string(),
            Token::Colon => ":".to_string(),
            Token::LeftBracket => "[".to_string(),
            Token::RightBracket => "]".to_string(),
            Token::Comma => ",".to_string(),
            Token::LeftParen => "(".to_string(),
            Token::RightParen => ")".to_string(),
            Token::Equals => "=".to_string(),
            Token::NotEquals => "!=".to_string(),
            Token::LessThan => "<".to_string(),
            Token::GreaterThan => ">".to_string(),
            Token::LessEqual => "<=".to_string(),
            Token::GreaterEqual => ">=".to_string(),
            Token::Identifier(name) => name.clone(),
            Token::QuotedString(s) => format!("\"{}\"", s),
            Token::Number(Some(n)) => n.to_string(),
            Token::Number(None) => "invalid number".to_string(),
            Token::Float(Some(f)) => f.to_string(),
            Token::Float(None) => "invalid float".to_string(),
            Token::Duration(d) => d.clone(),
            Token::Entity => "Entity".to_string(),
            Token::Service => "Service".to_string(),
            Token::Endpoint => "Endpoint".to_string(),
            Token::Object => "Object".to_string(),
            Token::StringType => "String".to_string(),
            Token::NumberType => "Number".to_string(),
            Token::BooleanType => "Boolean".to_string(),
            Token::True => "true".to_string(),
            Token::False => "false".to_string(),
            Token::Whitespace => "whitespace".to_string(),
            Token::Newline => "newline".to_string(),
            Token::Comment => "comment".to_string(),
            Token::BlockComment => "block comment".to_string(),
        }
    }
}
