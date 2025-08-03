use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

/// Main error type for NaviLang compiler
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
    
    #[error("Type error: expected {expected}, found {found}")]
    #[diagnostic(code(semantic::type_error))]
    TypeError {
        expected: String,
        found: String,
        #[source_code]
        src: String,
        #[label("Type mismatch")]
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
    
    #[error("Flow validation error: {message}")]
    #[diagnostic(code(flow::invalid))]
    FlowError {
        message: String,
        #[source_code]
        src: String,
        #[label("Flow error")]
        span: SourceSpan,
    },
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Generic error: {0}")]
    GenericError(#[from] anyhow::Error),
    
    #[error("Multiple errors occurred")]
    #[diagnostic(code(compilation::multiple_errors))]
    MultipleErrors {
        errors: Vec<NaviLangError>,
    },
}

/// Convenience type alias for Results with NaviLangError
pub type Result<T> = std::result::Result<T, NaviLangError>;

/// Position tracking for precise error locations
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl Position {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self { line, column, offset }
    }
    
    pub fn advance_line(&mut self) {
        self.line += 1;
        self.column = 1;
    }
    
    pub fn advance_column(&mut self) {
        self.column += 1;
    }
    
    pub fn advance_offset(&mut self, count: usize) {
        self.offset += count;
    }
}

/// Span representing a range in source code
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
    
    pub fn single_char(pos: Position) -> Self {
        let mut end = pos;
        end.advance_column();
        end.advance_offset(1);
        Self::new(pos, end)
    }
    
    /// Convert to miette's SourceSpan for error reporting
    pub fn to_miette_span(&self) -> SourceSpan {
        SourceSpan::new(
            self.start.offset.into(), 
            (self.end.offset - self.start.offset).into()
        )
    }
    
    /// Combine two spans into one that covers both
    pub fn combine(&self, other: &Span) -> Span {
        let start = if self.start.offset <= other.start.offset {
            self.start
        } else {
            other.start
        };
        
        let end = if self.end.offset >= other.end.offset {
            self.end
        } else {
            other.end
        };
        
        Span::new(start, end)
    }
}

/// Error collector for batch processing and error recovery
#[derive(Debug, Default)]
pub struct ErrorCollector {
    errors: Vec<NaviLangError>,
}

impl ErrorCollector {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
        }
    }
    
    pub fn add_error(&mut self, error: NaviLangError) {
        self.errors.push(error);
    }
    
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }
    
    pub fn into_result<T>(self, value: T) -> Result<T> {
        if self.errors.is_empty() {
            Ok(value)
        } else if self.errors.len() == 1 {
            Err(self.errors.into_iter().next().unwrap())
        } else {
            Err(NaviLangError::MultipleErrors { errors: self.errors })
        }
    }
    
    pub fn errors(&self) -> &[NaviLangError] {
        &self.errors
    }
}

/// Helper functions for creating common errors
impl NaviLangError {
    pub fn syntax_error(message: String, src: String, span: Span) -> Self {
        Self::SyntaxError {
            message,
            src,
            span: span.to_miette_span(),
        }
    }
    
    pub fn semantic_error(message: String, src: String, span: Span) -> Self {
        Self::SemanticError {
            message,
            src,
            span: span.to_miette_span(),
        }
    }
    
    pub fn type_error(expected: String, found: String, src: String, span: Span) -> Self {
        Self::TypeError {
            expected,
            found,
            src,
            span: span.to_miette_span(),
        }
    }
    
    pub fn unknown_identifier(name: String, src: String, span: Span) -> Self {
        Self::UnknownIdentifier {
            name,
            src,
            span: span.to_miette_span(),
        }
    }
    
    pub fn flow_error(message: String, src: String, span: Span) -> Self {
        Self::FlowError {
            message,
            src,
            span: span.to_miette_span(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_advancement() {
        let mut pos = Position::new(1, 1, 0);
        
        pos.advance_column();
        assert_eq!(pos.column, 2);
        assert_eq!(pos.line, 1);
        
        pos.advance_line();
        assert_eq!(pos.line, 2);
        assert_eq!(pos.column, 1);
        
        pos.advance_offset(5);
        assert_eq!(pos.offset, 5);
    }
    
    #[test]
    fn test_span_operations() {
        let start = Position::new(1, 1, 0);
        let end = Position::new(1, 5, 4);
        let span = Span::new(start, end);
        
        let miette_span = span.to_miette_span();
        assert_eq!(miette_span.offset(), 0);
        assert_eq!(miette_span.len(), 4);
    }
    
    #[test]
    fn test_error_collector() {
        let mut collector = ErrorCollector::new();
        assert!(!collector.has_errors());
        
        collector.add_error(NaviLangError::IoError(
            std::io::Error::new(std::io::ErrorKind::NotFound, "test")
        ));
        
        assert!(collector.has_errors());
        assert_eq!(collector.error_count(), 1);
        
        let result: Result<()> = collector.into_result(());
        assert!(result.is_err());
    }
}
