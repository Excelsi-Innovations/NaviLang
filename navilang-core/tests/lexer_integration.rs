use navilang::lexer::{Lexer, tokens::Token};
use navilang::reader::SourceFile;

#[test]
fn test_complete_navilang_program() {
    let input = r#"
CONTEXT "Authentication" {
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
    assert!(token_types.contains(&&Token::Entity));
    assert!(token_types.contains(&&Token::Object));
    assert!(token_types.contains(&&Token::Colon));
    
    // Count specific tokens
    let context_count = token_types.iter().filter(|&&t| *t == Token::Context).count();
    let var_count = token_types.iter().filter(|&&t| *t == Token::Var).count();
    
    assert_eq!(context_count, 1);
    assert_eq!(var_count, 2);
}

#[test]
fn test_complex_flow_with_conditionals() {
    let input = r#"
IF User IS "valid" THEN
    User GOES TO Dashboard
PARALLEL {
    Service1 DOES ProcessA
    Service2 DOES ProcessB
}
"#;
    
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize_filtered().unwrap();
    
    // Verify conditional and parallel constructs
    let token_types: Vec<_> = tokens.iter().map(|t| &t.token).collect();
    
    assert!(token_types.contains(&&Token::If));
    assert!(token_types.contains(&&Token::Is));
    assert!(token_types.contains(&&Token::Then));
    assert!(token_types.contains(&&Token::Parallel));
    
    // Check for quoted string
    let has_quoted_string = tokens.iter().any(|t| matches!(t.token, Token::QuotedString(ref s) if s == "valid"));
    assert!(has_quoted_string);
}

#[test]
fn test_error_reporting_with_source_file() {
    use std::io::Write;
    use tempfile::NamedTempFile;
    
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "CONTEXT Test {{").unwrap();
    writeln!(temp_file, "  VAR User").unwrap();
    writeln!(temp_file, "  User @invalid_token").unwrap();
    writeln!(temp_file, "}}").unwrap();
    
    let source = SourceFile::from_file(temp_file.path()).unwrap();
    let mut lexer = Lexer::new(&source.content);
    let result = lexer.tokenize_filtered();
    
    assert!(result.is_err());
    
    if let Err(error) = result {
        println!("Error message: {:?}", error);
        // The error should contain information about the invalid token
    }
}
