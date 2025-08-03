//! # NaviLang Core
//! 
//! A declarative flow modeling language compiler following a traditional
//! multi-stage compilation pipeline.
//! 
//! ## Architecture Pipeline
//! 
//! The compiler transforms human-readable declarative flow descriptions into
//! multiple output formats including diagrams, documentation, and executable code.
//! 
//! ## Usage
//! 
//! ```rust,no_run
//! use navilang::reader::read_source;
//! use navilang::lexer::Lexer;
//! use navilang::parser::Parser;
//! 
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Read source file
//!     let source = read_source("example.navi")?;
//!     
//!     // Tokenize
//!     let mut lexer = Lexer::new(&source.content);
//!     let tokens = lexer.tokenize_filtered()?;
//!     
//!     // Parse
//!     let mut parser = Parser::new(tokens);
//!     let ast = parser.parse()?;
//!     
//!     Ok(())
//! }
//! ```

pub mod reader;
pub mod lexer;
pub mod parser;
pub mod analyzer;
pub mod generator;
pub mod error;
pub mod utils;

// Re-export commonly used types
pub use error::{NaviLangError, Result};
pub use reader::{SourceFile, read_source};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Compile a NaviLang source file through the complete pipeline
pub fn compile_file<P: AsRef<std::path::Path>>(
    path: P,
) -> Result<CompilationResult> {
    // TODO: Implement full compilation pipeline
    let source = read_source(path)?;
    
    // Stage 1: Lexical Analysis
    let mut lexer = lexer::Lexer::new(&source.content);
    let tokens = lexer.tokenize_filtered()?;
    
    // Stage 2: Syntax Analysis
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse()?;
    
    Ok(CompilationResult {
        ast,
        source,
    })
}

/// Result of compilation process
#[derive(Debug)]
pub struct CompilationResult {
    pub ast: parser::ast::Program,
    pub source: SourceFile,
}
