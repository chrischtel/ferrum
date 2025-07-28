use crate::lexer::*;

#[test]
fn test_simple_tokens() {
    let mut lexer = Lexer::new("let x = 42;");
    let tokens = lexer.tokenize();

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].token_type, TokenType::Let);
    assert_eq!(tokens[1].token_type, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[2].token_type, TokenType::Assign);
    assert_eq!(tokens[3].token_type, TokenType::Number(42));
    assert_eq!(tokens[4].token_type, TokenType::Semicolon);
    assert_eq!(tokens[5].token_type, TokenType::Eof);
}

#[test]
fn test_string_literal() {
    let mut lexer = Lexer::new(r#"let name = "hello";"#);
    let tokens = lexer.tokenize();

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].token_type, TokenType::Let);
    assert_eq!(
        tokens[1].token_type,
        TokenType::Identifier("name".to_string())
    );
    assert_eq!(tokens[2].token_type, TokenType::Assign);
    assert_eq!(tokens[3].token_type, TokenType::String("hello".to_string()));
    assert_eq!(tokens[4].token_type, TokenType::Semicolon);
    assert_eq!(tokens[5].token_type, TokenType::Eof);
}

#[test]
fn test_mut_keyword() {
    let mut lexer = Lexer::new("let mut x = 10;");
    let tokens = lexer.tokenize();

    assert_eq!(tokens.len(), 7);
    assert_eq!(tokens[0].token_type, TokenType::Let);
    assert_eq!(tokens[1].token_type, TokenType::Mut);
    assert_eq!(tokens[2].token_type, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[3].token_type, TokenType::Assign);
    assert_eq!(tokens[4].token_type, TokenType::Number(10));
    assert_eq!(tokens[5].token_type, TokenType::Semicolon);
    assert_eq!(tokens[6].token_type, TokenType::Eof);
}

#[test]
fn test_arithmetic_expression() {
    let mut lexer = Lexer::new("x + y - 5");
    let tokens = lexer.tokenize();

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].token_type, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[1].token_type, TokenType::Plus);
    assert_eq!(tokens[2].token_type, TokenType::Identifier("y".to_string()));
    assert_eq!(tokens[3].token_type, TokenType::Minus);
    assert_eq!(tokens[4].token_type, TokenType::Number(5));
    assert_eq!(tokens[5].token_type, TokenType::Eof);
}

#[test]
fn test_whitespace_handling() {
    let mut lexer = Lexer::new("let   x\t=\n42;");
    let tokens = lexer.tokenize();

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].token_type, TokenType::Let);
    assert_eq!(tokens[1].token_type, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[2].token_type, TokenType::Assign);
    assert_eq!(tokens[3].token_type, TokenType::Number(42));
    assert_eq!(tokens[4].token_type, TokenType::Semicolon);

    // Check line tracking
    assert_eq!(tokens[0].line, 1);
    assert_eq!(tokens[3].line, 2); // Number should be on line 2
}

#[test]
fn test_invalid_character() {
    let mut lexer = Lexer::new("let x = @;");
    let tokens = lexer.tokenize();

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].token_type, TokenType::Let);
    assert_eq!(tokens[1].token_type, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[2].token_type, TokenType::Assign);
    assert_eq!(tokens[3].token_type, TokenType::Invalid('@'));
    assert_eq!(tokens[4].token_type, TokenType::Semicolon);
    assert_eq!(tokens[5].token_type, TokenType::Eof);
}

#[test]
fn test_empty_string() {
    let mut lexer = Lexer::new(r#"let empty = "";"#);
    let tokens = lexer.tokenize();

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[3].token_type, TokenType::String("".to_string()));
}

#[test]
fn test_large_number() {
    let mut lexer = Lexer::new("let big = 1234567890;");
    let tokens = lexer.tokenize();

    assert_eq!(tokens[3].token_type, TokenType::Number(1234567890));
}
