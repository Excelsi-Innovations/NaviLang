# NaviLang Core - Development Roadmap & Implementation Guide

## 🎯 Development Overview

This document provides the detailed implementation roadmap for building the NaviLang compiler core in Rust, following the architecture pipeline established in our PRD.

## 📋 Pre-Development Setup

### Environment Setup
```bash
# Install Rust and toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup component add clippy rustfmt

# Create project structure
cargo new navilang-core --name navilang
cd navilang-core

# Install development tools
cargo install cargo-watch
cargo install cargo-tarpaulin  # Code coverage
cargo install cargo-criterion  # Benchmarking
```

### Initial Project Configuration
```toml
# Cargo.toml
[package]
name = "navilang"
version = "0.1.0"
edition = "2021"
authors = ["NaviLang Team"]
description = "A declarative flow modeling language compiler"
license = "MIT OR Apache-2.0"
readme = "README.md"
repository = "https://github.com/Excelsi-Innovations/NaviLang"

[dependencies]
# Core dependencies
logos = "0.13"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"
clap = { version = "4.0", features = ["derive"] }
anyhow = "1.0"
thiserror = "1.0"
petgraph = "0.6"
miette = { version = "5.0", features = ["fancy"] }

# Optional features
regex = { version = "1.0", optional = true }
graphviz-rust = { version = "0.6", optional = true }

[dev-dependencies]
insta = "1.0"
criterion = { version = "0.5", features = ["html_reports"] }
tempfile = "3.0"
pretty_assertions = "1.0"

[features]
default = ["mermaid", "graphviz"]
mermaid = []
graphviz = ["graphviz-rust"]
advanced = ["regex"]

[[bin]]
name = "navilang"
path = "src/main.rs"

[[bench]]
name = "parser_bench"
harness = false
```

## 🏗️ Phase 1: Foundation (Weeks 1-4)

### Week 1: Project Infrastructure

#### Day 1-2: Basic CLI Setup
```rust
// src/main.rs
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "navilang")]
#[command(about = "NaviLang compiler and toolchain")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse NaviLang file and output AST
    Parse {
        #[arg(short, long)]
        file: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Generate diagrams from NaviLang
    Generate {
        #[arg(short, long)]
        file: PathBuf,
        #[arg(short, long)]
        format: String,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Validate NaviLang syntax
    Check {
        #[arg(short, long)]
        file: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Parse { file, output } => {
            // TODO: Implement parsing
            println!("Parsing file: {:?}", file);
        }
        Commands::Generate { file, format, output } => {
            // TODO: Implement generation
            println!("Generating {} from {:?}", format, file);
        }
        Commands::Check { file } => {
            // TODO: Implement validation
            println!("Checking file: {:?}", file);
        }
    }
    
    Ok(())
}
```

#### Day 3-4: File Reader Module
```rust
// src/reader/mod.rs
use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

pub struct SourceFile {
    pub content: String,
    pub path: String,
    pub lines: Vec<String>,
}

impl SourceFile {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read file: {}", path_str))?;
        
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        
        Ok(SourceFile {
            content,
            path: path_str,
            lines,
        })
    }
    
    pub fn from_string(content: String, path: String) -> Self {
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        SourceFile {
            content,
            path,
            lines,
        }
    }
    
    pub fn get_line(&self, line_number: usize) -> Option<&str> {
        self.lines.get(line_number.saturating_sub(1)).map(|s| s.as_str())
    }
}

pub fn read_source<P: AsRef<Path>>(path: P) -> Result<SourceFile> {
    SourceFile::from_file(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_read_source_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "CONTEXT Test {{").unwrap();
        writeln!(temp_file, "  VAR User").unwrap();
        writeln!(temp_file, "}}").unwrap();
        
        let source = read_source(temp_file.path()).unwrap();
        assert_eq!(source.lines.len(), 3);
        assert!(source.content.contains("CONTEXT Test"));
    }
}
```

#### Day 5-7: Basic Error System
```rust
// src/error/mod.rs
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum NaviLangError {
    #[error("Syntax error: {message}")]
    #[diagnostic(code(syntax::invalid))]
    SyntaxError {
        message: String,
        #[source_code]
        src: String,
        #[label("Error occurred here")]
        span: SourceSpan,
    },
    
    #[error("Semantic error: {message}")]
    #[diagnostic(code(semantic::invalid))]
    SemanticError {
        message: String,
        #[source_code]
        src: String,
        #[label("Error occurred here")]
        span: SourceSpan,
    },
    
    #[error("Unknown identifier: {name}")]
    #[diagnostic(code(semantic::unknown_identifier))]
    UnknownIdentifier {
        name: String,
        #[source_code]
        src: String,
        #[label("Unknown identifier")]
        span: SourceSpan,
    },
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, NaviLangError>;

// Position tracking for errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl Position {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self { line, column, offset }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
    
    pub fn to_miette_span(&self) -> SourceSpan {
        SourceSpan::new(self.start.offset.into(), (self.end.offset - self.start.offset).into())
    }
}
```

### Week 2: Tokenization Engine

#### Day 8-10: Token Definitions
```rust
// src/lexer/tokens.rs
use logos::Logos;
use serde::{Deserialize, Serialize};

#[derive(Logos, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Token {
    // Keywords
    #[token("VAR")]
    Var,
    
    #[token("CONTEXT")]
    Context,
    
    #[token("GOES")]
    Goes,
    
    #[token("TO")]
    To,
    
    #[token("CREATED")]
    Created,
    
    #[token("BY")]
    By,
    
    #[token("IF")]
    If,
    
    #[token("THEN")]
    Then,
    
    #[token("WHEN")]
    When,
    
    #[token("CALLS")]
    Calls,
    
    #[token("RECEIVES")]
    Receives,
    
    #[token("RETURNS")]
    Returns,
    
    #[token("DOES")]
    Does,
    
    #[token("USES")]
    Uses,
    
    #[token("IS")]
    Is,
    
    #[token("AFTER")]
    After,
    
    #[token("BEFORE")]
    Before,
    
    #[token("PARALLEL")]
    Parallel,
    
    #[token("AND")]
    And,
    
    #[token("OR")]
    Or,
    
    // Advanced keywords
    #[token("RETRY")]
    Retry,
    
    #[token("TIMEOUT")]
    Timeout,
    
    #[token("ASYNC")]
    Async,
    
    #[token("BATCH")]
    Batch,
    
    #[token("LOOP")]
    Loop,
    
    #[token("WHILE")]
    While,
    
    #[token("BREAK")]
    Break,
    
    #[token("CONTINUE")]
    Continue,
    
    // Punctuation
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
    
    #[regex(r"[0-9]+s|[0-9]+ms", |lex| lex.slice().to_string())]
    Duration(String),
    
    // Whitespace and comments
    #[regex(r"[ \t\f]+")]
    Whitespace,
    
    #[regex(r"\r?\n")]
    Newline,
    
    #[regex(r"//[^\r\n]*")]
    Comment,
    
    #[regex(r"/\*([^*]|\*[^/])*\*/")]
    BlockComment,
    
    // Error token
    #[error]
    Error,
}

impl Token {
    pub fn is_keyword(&self) -> bool {
        matches!(self, 
            Token::Var | Token::Context | Token::Goes | Token::To |
            Token::Created | Token::By | Token::If | Token::Then |
            Token::When | Token::Calls | Token::Receives | Token::Returns |
            Token::Does | Token::Uses | Token::Is | Token::After |
            Token::Before | Token::Parallel | Token::And | Token::Or |
            Token::Retry | Token::Timeout | Token::Async | Token::Batch |
            Token::Loop | Token::While | Token::Break | Token::Continue
        )
    }
    
    pub fn is_whitespace(&self) -> bool {
        matches!(self, Token::Whitespace | Token::Newline | Token::Comment | Token::BlockComment)
    }
}
```

#### Day 11-12: Lexical Analyzer
```rust
// src/lexer/mod.rs
pub mod tokens;

use crate::error::{NaviLangError, Position, Span};
use logos::Logos;
use tokens::Token;

#[derive(Debug, Clone)]
pub struct TokenWithSpan {
    pub token: Token,
    pub span: Span,
}

pub struct Lexer<'a> {
    input: &'a str,
    lexer: logos::Lexer<'a, Token>,
    line: usize,
    column: usize,
    offset: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            lexer: Token::lexer(input),
            line: 1,
            column: 1,
            offset: 0,
        }
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<TokenWithSpan>, NaviLangError> {
        let mut tokens = Vec::new();
        
        while let Some(token) = self.lexer.next() {
            let span = self.current_span();
            
            match token {
                Token::Error => {
                    return Err(NaviLangError::SyntaxError {
                        message: format!("Unexpected character: '{}'", self.lexer.slice()),
                        src: self.input.to_string(),
                        span: span.to_miette_span(),
                    });
                }
                _ => {
                    tokens.push(TokenWithSpan { token, span });
                }
            }
            
            self.update_position();
        }
        
        Ok(tokens)
    }
    
    pub fn tokenize_filtered(&mut self) -> Result<Vec<TokenWithSpan>, NaviLangError> {
        let tokens = self.tokenize()?;
        Ok(tokens.into_iter()
            .filter(|t| !t.token.is_whitespace())
            .collect())
    }
    
    fn current_span(&self) -> Span {
        let start = Position::new(self.line, self.column, self.offset);
        let end_offset = self.offset + self.lexer.slice().len();
        let end = Position::new(self.line, self.column + self.lexer.slice().len(), end_offset);
        Span::new(start, end)
    }
    
    fn update_position(&mut self) {
        let slice = self.lexer.slice();
        for ch in slice.chars() {
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        self.offset += slice.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokenization() {
        let input = "VAR User";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize_filtered().unwrap();
        
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token, Token::Var);
        assert!(matches!(tokens[1].token, Token::Identifier(_)));
    }

    #[test]
    fn test_context_tokenization() {
        let input = r#"CONTEXT "User Management" {
            VAR User
        }"#;
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize_filtered().unwrap();
        
        assert!(tokens.iter().any(|t| t.token == Token::Context));
        assert!(tokens.iter().any(|t| matches!(t.token, Token::QuotedString(_))));
        assert!(tokens.iter().any(|t| t.token == Token::LeftBrace));
    }
}
```

#### Day 13-14: Integration and Testing
```rust
// tests/integration/lexer_tests.rs
use navilang::lexer::{Lexer, tokens::Token};

#[test]
fn test_complete_navilang_program() {
    let input = r#"
CONTEXT Authentication {
    VAR User:Entity
    VAR Session:Object
    
    User DOES Login
    User CALLS AuthService
    AuthService RETURNS Session
    Session GOES TO UserDashboard
}
"#;
    
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize_filtered().unwrap();
    
    // Verify we have all expected tokens
    let token_types: Vec<_> = tokens.iter().map(|t| &t.token).collect();
    
    assert!(token_types.contains(&&Token::Context));
    assert!(token_types.contains(&&Token::Var));
    assert!(token_types.contains(&&Token::Does));
    assert!(token_types.contains(&&Token::Calls));
    assert!(token_types.contains(&&Token::Returns));
    assert!(token_types.contains(&&Token::Goes));
    assert!(token_types.contains(&&Token::To));
}

#[test]
fn test_error_reporting() {
    let input = "VAR User @invalid";
    let mut lexer = Lexer::new(input);
    let result = lexer.tokenize_filtered();
    
    assert!(result.is_err());
}
```

### Week 3: Basic Parser Foundation

#### Day 15-17: AST Definitions
```rust
// src/parser/ast.rs
use crate::error::Span;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub contexts: Vec<Context>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Context {
    pub name: String,
    pub statements: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statement {
    pub kind: StatementKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StatementKind {
    VarDeclaration {
        name: String,
        type_info: Option<TypeInfo>,
    },
    Transition {
        from: String,
        to: String,
    },
    Creation {
        entity: String,
        creator: String,
    },
    Action {
        subject: String,
        action: String,
        object: Option<String>,
    },
    Invocation {
        caller: String,
        callee: String,
        args: Option<String>,
    },
    Reception {
        receiver: String,
        data: String,
    },
    Return {
        returner: String,
        value: String,
    },
    Usage {
        user: String,
        used: String,
    },
    Conditional {
        condition: Condition,
        then_stmt: Box<Statement>,
        else_stmt: Option<Box<Statement>>,
    },
    Event {
        trigger: Condition,
        action: Box<Statement>,
    },
    Parallel {
        statements: Vec<Statement>,
    },
    Sequential {
        first: Box<Statement>,
        second: Box<Statement>,
        relation: SequentialRelation,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SequentialRelation {
    After,
    Before,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Condition {
    pub subject: String,
    pub operator: String,
    pub value: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeInfo {
    Simple(String),
    Entity,
    Service,
    Endpoint,
    Object,
    Enum { variants: Vec<String> },
    Constrained {
        base_type: Box<TypeInfo>,
        constraints: Vec<Constraint>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Constraint {
    Required,
    Optional,
    Auto,
    Positive,
    Range { min: i64, max: i64 },
    Length { min: usize, max: usize },
    Pattern(String),
}

// Helper methods for AST navigation
impl Program {
    pub fn find_context(&self, name: &str) -> Option<&Context> {
        self.contexts.iter().find(|c| c.name == name)
    }
    
    pub fn all_variables(&self) -> HashMap<String, Vec<&Statement>> {
        let mut vars = HashMap::new();
        
        for context in &self.contexts {
            for stmt in &context.statements {
                if let StatementKind::VarDeclaration { name, .. } = &stmt.kind {
                    vars.entry(context.name.clone())
                        .or_insert_with(Vec::new)
                        .push(stmt);
                }
            }
        }
        
        vars
    }
}

impl Context {
    pub fn find_variable(&self, name: &str) -> Option<&Statement> {
        self.statements.iter().find(|stmt| {
            if let StatementKind::VarDeclaration { name: var_name, .. } = &stmt.kind {
                var_name == name
            } else {
                false
            }
        })
    }
}
```

#### Day 18-21: Parser Implementation
```rust
// src/parser/mod.rs
pub mod ast;

use crate::error::{NaviLangError, Span};
use crate::lexer::{TokenWithSpan, tokens::Token};
use ast::*;

pub struct Parser {
    tokens: Vec<TokenWithSpan>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenWithSpan>) -> Self {
        Self { tokens, current: 0 }
    }
    
    pub fn parse(&mut self) -> Result<Program, NaviLangError> {
        let mut contexts = Vec::new();
        let start_span = self.current_span();
        
        while !self.is_at_end() {
            contexts.push(self.parse_context()?);
        }
        
        let end_span = self.previous_span();
        let span = Span::new(start_span.start, end_span.end);
        
        Ok(Program { contexts, span })
    }
    
    fn parse_context(&mut self) -> Result<Context, NaviLangError> {
        self.consume(Token::Context, "Expected 'CONTEXT'")?;
        
        let name = self.consume_identifier("Expected context name")?;
        self.consume(Token::LeftBrace, "Expected '{'")?;
        
        let mut statements = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        
        self.consume(Token::RightBrace, "Expected '}'")?;
        
        let span = self.span_from_previous();
        
        Ok(Context { name, statements, span })
    }
    
    fn parse_statement(&mut self) -> Result<Statement, NaviLangError> {
        let start_span = self.current_span();
        
        let kind = match &self.peek().token {
            Token::Var => self.parse_var_declaration()?,
            Token::If => self.parse_conditional()?,
            Token::When => self.parse_event()?,
            Token::Parallel => self.parse_parallel()?,
            Token::Identifier(_) => self.parse_entity_statement()?,
            _ => {
                return Err(NaviLangError::SyntaxError {
                    message: "Expected statement".to_string(),
                    src: "".to_string(), // TODO: Add source context
                    span: start_span.to_miette_span(),
                });
            }
        };
        
        let end_span = self.previous_span();
        let span = Span::new(start_span.start, end_span.end);
        
        Ok(Statement { kind, span })
    }
    
    fn parse_var_declaration(&mut self) -> Result<StatementKind, NaviLangError> {
        self.consume(Token::Var, "Expected 'VAR'")?;
        let name = self.consume_identifier("Expected variable name")?;
        
        let type_info = if self.match_token(&Token::Colon) {
            Some(self.parse_type_info()?)
        } else {
            None
        };
        
        Ok(StatementKind::VarDeclaration { name, type_info })
    }
    
    fn parse_type_info(&mut self) -> Result<TypeInfo, NaviLangError> {
        let type_name = self.consume_identifier("Expected type name")?;
        
        match type_name.as_str() {
            "Entity" => Ok(TypeInfo::Entity),
            "Service" => Ok(TypeInfo::Service),
            "Endpoint" => Ok(TypeInfo::Endpoint),
            "Object" => Ok(TypeInfo::Object),
            _ => {
                if self.match_token(&Token::LeftBracket) {
                    // Parse enum or constraints
                    let mut variants = Vec::new();
                    
                    loop {
                        variants.push(self.consume_identifier("Expected enum variant")?);
                        
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }
                    
                    self.consume(Token::RightBracket, "Expected ']'")?;
                    
                    if type_name == "Enum" {
                        Ok(TypeInfo::Enum { variants })
                    } else {
                        // TODO: Parse constraints
                        Ok(TypeInfo::Simple(type_name))
                    }
                } else {
                    Ok(TypeInfo::Simple(type_name))
                }
            }
        }
    }
    
    fn parse_entity_statement(&mut self) -> Result<StatementKind, NaviLangError> {
        let subject = self.consume_identifier("Expected identifier")?;
        
        match &self.peek().token {
            Token::Goes => {
                self.advance(); // consume GOES
                self.consume(Token::To, "Expected 'TO' after 'GOES'")?;
                let to = self.consume_identifier("Expected destination")?;
                Ok(StatementKind::Transition { from: subject, to })
            }
            Token::Created => {
                self.advance(); // consume CREATED
                self.consume(Token::By, "Expected 'BY' after 'CREATED'")?;
                let creator = self.consume_identifier("Expected creator")?;
                Ok(StatementKind::Creation { entity: subject, creator })
            }
            Token::Does => {
                self.advance(); // consume DOES
                let action = self.consume_identifier("Expected action")?;
                Ok(StatementKind::Action { subject, action, object: None })
            }
            Token::Calls => {
                self.advance(); // consume CALLS
                let callee = self.consume_identifier("Expected callee")?;
                Ok(StatementKind::Invocation { caller: subject, callee, args: None })
            }
            Token::Receives => {
                self.advance(); // consume RECEIVES
                let data = self.consume_identifier("Expected data")?;
                Ok(StatementKind::Reception { receiver: subject, data })
            }
            Token::Returns => {
                self.advance(); // consume RETURNS
                let value = self.consume_identifier("Expected return value")?;
                Ok(StatementKind::Return { returner: subject, value })
            }
            Token::Uses => {
                self.advance(); // consume USES
                let used = self.consume_identifier("Expected used entity")?;
                Ok(StatementKind::Usage { user: subject, used })
            }
            _ => Err(NaviLangError::SyntaxError {
                message: format!("Unexpected token after '{}'", subject),
                src: "".to_string(),
                span: self.current_span().to_miette_span(),
            })
        }
    }
    
    // Helper methods
    fn consume(&mut self, token_type: Token, message: &str) -> Result<(), NaviLangError> {
        if self.check(&token_type) {
            self.advance();
            Ok(())
        } else {
            Err(NaviLangError::SyntaxError {
                message: message.to_string(),
                src: "".to_string(),
                span: self.current_span().to_miette_span(),
            })
        }
    }
    
    fn consume_identifier(&mut self, message: &str) -> Result<String, NaviLangError> {
        match &self.peek().token {
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            Token::QuotedString(name) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            _ => Err(NaviLangError::SyntaxError {
                message: message.to_string(),
                src: "".to_string(),
                span: self.current_span().to_miette_span(),
            })
        }
    }
    
    fn match_token(&mut self, token_type: &Token) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    fn check(&self, token_type: &Token) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.peek().token) == std::mem::discriminant(token_type)
        }
    }
    
    fn advance(&mut self) -> &TokenWithSpan {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
    
    fn peek(&self) -> &TokenWithSpan {
        &self.tokens[self.current]
    }
    
    fn previous(&self) -> &TokenWithSpan {
        &self.tokens[self.current - 1]
    }
    
    fn current_span(&self) -> Span {
        if self.is_at_end() {
            self.previous().span.clone()
        } else {
            self.peek().span.clone()
        }
    }
    
    fn previous_span(&self) -> Span {
        self.previous().span.clone()
    }
    
    fn span_from_previous(&self) -> Span {
        // TODO: Implement proper span creation
        self.previous_span()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parse_simple_context() {
        let input = r#"
CONTEXT Test {
    VAR User
}
"#;
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize_filtered().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        
        assert_eq!(program.contexts.len(), 1);
        assert_eq!(program.contexts[0].name, "Test");
        assert_eq!(program.contexts[0].statements.len(), 1);
    }
}
```

### Week 4: Error Handling and Testing

#### Day 22-24: Enhanced Error Reporting
```rust
// src/error/reporting.rs
use crate::error::{NaviLangError, Span};
use crate::reader::SourceFile;
use miette::{GraphicalReportHandler, GraphicalTheme};

pub struct ErrorReporter {
    handler: GraphicalReportHandler,
}

impl ErrorReporter {
    pub fn new() -> Self {
        Self {
            handler: GraphicalReportHandler::new_themed(GraphicalTheme::unicode_nocolor()),
        }
    }
    
    pub fn report(&self, error: &NaviLangError, source: &SourceFile) -> String {
        // Enhanced error reporting with context
        match error {
            NaviLangError::SyntaxError { message, span, .. } => {
                format!(
                    "Syntax Error in {}:{}:{}\n{}\n{}",
                    source.path,
                    span.offset(),
                    span.len(),
                    message,
                    self.format_source_context(source, span)
                )
            }
            _ => format!("{}", error),
        }
    }
    
    fn format_source_context(&self, source: &SourceFile, span: &miette::SourceSpan) -> String {
        // TODO: Implement source context formatting
        String::new()
    }
}

// Error recovery for batch processing
pub struct ErrorCollector {
    errors: Vec<NaviLangError>,
}

impl ErrorCollector {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }
    
    pub fn add_error(&mut self, error: NaviLangError) {
        self.errors.push(error);
    }
    
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    
    pub fn into_errors(self) -> Vec<NaviLangError> {
        self.errors
    }
}
```

#### Day 25-28: Comprehensive Testing
```rust
// tests/integration/parser_integration.rs
use navilang::{lexer::Lexer, parser::Parser, reader::read_source};
use std::fs;

#[test]
fn test_ecommerce_example() {
    let source = read_source("tests/fixtures/ecommerce.navi").unwrap();
    let mut lexer = Lexer::new(&source.content);
    let tokens = lexer.tokenize_filtered().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    // Verify structure
    assert!(program.find_context("UserManagement").is_some());
    assert!(program.find_context("OrderProcessing").is_some());
    assert!(program.find_context("Notifications").is_some());
}

// Benchmark tests
// benches/parser_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use navilang::{lexer::Lexer, parser::Parser};

fn bench_parsing(c: &mut Criterion) {
    let large_navilang = fs::read_to_string("benches/fixtures/large_example.navi").unwrap();
    
    c.bench_function("parse_large_file", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new(black_box(&large_navilang));
            let tokens = lexer.tokenize_filtered().unwrap();
            let mut parser = Parser::new(tokens);
            parser.parse().unwrap()
        })
    });
}

criterion_group!(benches, bench_parsing);
criterion_main!(benches);
```

## 📈 Success Criteria for Phase 1

By the end of Phase 1, we should have:

✅ **Working CLI Tool**
- Parse simple NaviLang files
- Display AST representation
- Show meaningful error messages

✅ **Complete Tokenizer**
- Handle all NaviLang keywords
- Support quoted identifiers
- Proper error reporting with positions

✅ **Basic Parser**
- Parse contexts and variable declarations
- Handle simple transitions and actions
- Build correct AST structure

✅ **Error System**
- Precise error locations
- Helpful error messages
- Graceful error recovery

✅ **Test Coverage**
- Unit tests for all components
- Integration tests with real examples
- Performance benchmarks

## 🔄 Phase 1 Deliverables Checklist

- [ ] CLI tool with `parse`, `check`, and `generate` commands
- [ ] Complete lexical analyzer with error handling
- [ ] Parser supporting basic NaviLang syntax
- [ ] AST representation for all parsed constructs
- [ ] Comprehensive test suite with >80% coverage
- [ ] Documentation with usage examples
- [ ] Performance benchmarks baseline
- [ ] Error reporting with source context

---

**Next**: [Phase 2 Development Guide](./Phase2_Advanced_Parsing.md)
