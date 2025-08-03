use navilang::lexer::{Lexer, tokens::Token};

#[test]
fn test_case_insensitive_keywords() {
    // Test various cases for keywords
    let test_cases = vec![
        ("VAR", Token::Var),
        ("var", Token::Var),
        ("Var", Token::Var),
        ("vAr", Token::Var),
        ("CONTEXT", Token::Context),
        ("context", Token::Context),
        ("Context", Token::Context),
        ("CoNtExT", Token::Context),
        ("GOES", Token::Goes),
        ("goes", Token::Goes),
        ("Goes", Token::Goes),
        ("gOeS", Token::Goes),
        ("TO", Token::To),
        ("to", Token::To),
        ("To", Token::To),
        ("tO", Token::To),
        ("ENTITY", Token::Entity),
        ("entity", Token::Entity),
        ("Entity", Token::Entity),
        ("eNtItY", Token::Entity),
        ("TRUE", Token::True),
        ("true", Token::True),
        ("True", Token::True),
        ("tRuE", Token::True),
        ("FALSE", Token::False),
        ("false", Token::False),
        ("False", Token::False),
        ("fAlSe", Token::False),
    ];
    
    for (input, expected_token) in test_cases {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize_filtered().unwrap();
        
        assert_eq!(tokens.len(), 1, "Expected exactly one token for input: {}", input);
        assert_eq!(tokens[0].token, expected_token, "Token mismatch for input: {}", input);
    }
}

#[test]
fn test_case_insensitive_complete_program() {
    let input = r#"
context "user authentication" {
    var user:entity
    var session:object
    
    user does login
    user calls authservice
    authservice returns session
    session goes to userdashboard
    
    if user is "invalid" then
        user goes to loginpage
}
"#;
    
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize_filtered().unwrap();
    
    // Count specific tokens to verify they were recognized
    let context_count = tokens.iter().filter(|t| t.token == Token::Context).count();
    let var_count = tokens.iter().filter(|t| t.token == Token::Var).count();
    let entity_count = tokens.iter().filter(|t| t.token == Token::Entity).count();
    let object_count = tokens.iter().filter(|t| t.token == Token::Object).count();
    let does_count = tokens.iter().filter(|t| t.token == Token::Does).count();
    let calls_count = tokens.iter().filter(|t| t.token == Token::Calls).count();
    let returns_count = tokens.iter().filter(|t| t.token == Token::Returns).count();
    let goes_count = tokens.iter().filter(|t| t.token == Token::Goes).count();
    let to_count = tokens.iter().filter(|t| t.token == Token::To).count();
    let if_count = tokens.iter().filter(|t| t.token == Token::If).count();
    let is_count = tokens.iter().filter(|t| t.token == Token::Is).count();
    let then_count = tokens.iter().filter(|t| t.token == Token::Then).count();
    
    assert_eq!(context_count, 1);
    assert_eq!(var_count, 2);
    assert_eq!(entity_count, 1);
    assert_eq!(object_count, 1);
    assert_eq!(does_count, 1);
    assert_eq!(calls_count, 1);
    assert_eq!(returns_count, 1);
    assert_eq!(goes_count, 2);
    assert_eq!(to_count, 2);
    assert_eq!(if_count, 1);
    assert_eq!(is_count, 1);
    assert_eq!(then_count, 1);
}

#[test]
fn test_mixed_case_with_identifiers() {
    let input = "var UserName:entity";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize_filtered().unwrap();
    
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].token, Token::Var);
    
    // Identifier should remain case-sensitive
    if let Token::Identifier(name) = &tokens[1].token {
        assert_eq!(name, "UserName");
    } else {
        panic!("Expected identifier token");
    }
    
    assert_eq!(tokens[2].token, Token::Colon);
    assert_eq!(tokens[3].token, Token::Entity);
}
