// Lexer module - Lexical Analysis Stage
// This module implements the tokenization engine using logos

pub mod tokens;

use crate::error::{NaviLangError, Position, Span};
use logos::Logos;
use tokens::Token;

/// Token with associated span information for error reporting
#[derive(Debug, Clone)]
pub struct TokenWithSpan {
    pub token: Token,
    pub span: Span,
}

/// The main lexer struct that converts source text into tokens
pub struct Lexer<'a> {
    input: &'a str,
    lexer: logos::Lexer<'a, Token>,
    line: usize,
    column: usize,
    offset: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer for the given input
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            lexer: Token::lexer(input),
            line: 1,
            column: 1,
            offset: 0,
        }
    }
    
    /// Tokenize the entire input and return all tokens (including whitespace)
    pub fn tokenize(&mut self) -> Result<Vec<TokenWithSpan>, NaviLangError> {
        let mut tokens = Vec::new();
        
        while let Some(result) = self.lexer.next() {
            let span = self.current_span();
            
            match result {
                Ok(token) => {
                    tokens.push(TokenWithSpan { token, span });
                }
                Err(_) => {
                    return Err(NaviLangError::syntax_error(
                        format!("Unexpected character: '{}'", self.lexer.slice()),
                        self.input.to_string(),
                        span,
                    ));
                }
            }
            
            self.update_position();
        }
        
        Ok(tokens)
    }
    
    /// Tokenize input and filter out whitespace/comments
    pub fn tokenize_filtered(&mut self) -> Result<Vec<TokenWithSpan>, NaviLangError> {
        let tokens = self.tokenize()?;
        Ok(tokens.into_iter()
            .filter(|t| !t.token.is_whitespace())
            .collect())
    }
    
    /// Get the current span for the token being processed
    fn current_span(&self) -> Span {
        let start = Position::new(self.line, self.column, self.offset);
        let slice_len = self.lexer.slice().len();
        let end_offset = self.offset + slice_len;
        
        // Calculate end position considering newlines
        let mut end_line = self.line;
        let mut end_column = self.column;
        
        for ch in self.lexer.slice().chars() {
            if ch == '\n' {
                end_line += 1;
                end_column = 1;
            } else {
                end_column += 1;
            }
        }
        
        let end = Position::new(end_line, end_column, end_offset);
        Span::new(start, end)
    }
    
    /// Update internal position tracking
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
    
    /// Get the current lexer slice (for debugging)
    pub fn current_slice(&self) -> &str {
        self.lexer.slice()
    }
    
    /// Get the current position
    pub fn current_position(&self) -> Position {
        Position::new(self.line, self.column, self.offset)
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
        
        if let Token::Identifier(name) = &tokens[1].token {
            assert_eq!(name, "User");
        }
    }

    #[test]
    fn test_context_tokenization() {
        let input = r#"CONTEXT "User Management" {
            VAR User
        }"#;
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize_filtered().unwrap();
        
        // Verify we have the expected tokens
        assert!(tokens.iter().any(|t| t.token == Token::Context));
        assert!(tokens.iter().any(|t| matches!(t.token, Token::QuotedString(_))));
        assert!(tokens.iter().any(|t| t.token == Token::LeftBrace));
        assert!(tokens.iter().any(|t| t.token == Token::Var));
        assert!(tokens.iter().any(|t| t.token == Token::RightBrace));
    }
    
    #[test]
    fn test_flow_statements() {
        let input = "User GOES TO Dashboard";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize_filtered().unwrap();
        
        assert_eq!(tokens.len(), 4);
        assert!(matches!(tokens[0].token, Token::Identifier(_)));
        assert_eq!(tokens[1].token, Token::Goes);
        assert_eq!(tokens[2].token, Token::To);
        assert!(matches!(tokens[3].token, Token::Identifier(_)));
    }
    
    #[test]
    fn test_type_annotations() {
        let input = "VAR User:Entity";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize_filtered().unwrap();
        
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token, Token::Var);
        assert!(matches!(tokens[1].token, Token::Identifier(_)));
        assert_eq!(tokens[2].token, Token::Colon);
        assert_eq!(tokens[3].token, Token::Entity);
    }
    
    #[test]
    fn test_numbers_and_durations() {
        let input = "TIMEOUT 30s";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize_filtered().unwrap();
        
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token, Token::Timeout);
        assert!(matches!(tokens[1].token, Token::Duration(_)));
    }
    
    #[test]
    fn test_quoted_strings() {
        let input = r#"CONTEXT "My Context""#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize_filtered().unwrap();
        
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token, Token::Context);
        
        if let Token::QuotedString(content) = &tokens[1].token {
            assert_eq!(content, "My Context");
        } else {
            panic!("Expected QuotedString token");
        }
    }
    
    #[test]
    fn test_error_handling() {
        let input = "VAR User @invalid";
        let mut lexer = Lexer::new(input);
        let result = lexer.tokenize_filtered();
        
        assert!(result.is_err());
        if let Err(NaviLangError::SyntaxError { message, .. }) = result {
            assert!(message.contains("Unexpected character"));
        }
    }
    
    #[test]
    fn test_position_tracking() {
        let input = "CONTEXT Test {\n  VAR User\n}";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize_filtered().unwrap();
        
        // Check that spans are properly tracked
        assert!(tokens[0].span.start.line == 1);
        assert!(tokens[0].span.start.column == 1);
        
        // VAR should be on line 2
        let var_token = tokens.iter().find(|t| t.token == Token::Var).unwrap();
        assert_eq!(var_token.span.start.line, 2);
    }
}
