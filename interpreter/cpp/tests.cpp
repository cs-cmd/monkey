#include <iostream>
#include "./lexer/Token.h"
#include "./lexer/Lexer.h"
#include <string>
#include <iostream>
#include "tests.h"

struct TestToken 
{
    std::string tokenType;
    std::string tokenLiteral;

    TestToken(std::string tt, std::string tl)
        : tokenType(tt), tokenLiteral(tl) {};
};
    
void testTokenParsing() 
{
    std::string tokenLiterals = "=+(){},;";
    Lexer testLexer = Lexer(tokenLiterals);

    const TestToken test[] = {
        TestToken(tokens::ASSIGN, "="),
        TestToken(tokens::PLUS, "+"),
        TestToken(tokens::LPAREN, "("),
        TestToken(tokens::RPAREN, ")"),
        TestToken(tokens::LBRACE, "{"),
        TestToken(tokens::RBRACE, "}"),
        TestToken(tokens::COMMA, ","),
        TestToken(tokens::SEMICOLON, ";")
    };

    for(int i = 0; i < tokenLiterals.length(); i += 1) {
        Token t = testLexer.nextToken();

        TestToken tt = test[i];

        if (t.literal != tt.tokenLiteral) 
        {
            raiseError("Actual literal '" + t.literal + "' does not match expected literal: " + tt.tokenLiteral);
        } 
        else if (t.type != tt.tokenType) 
        {
            raiseError("Actual type '" + t.type + "' does not match expected type: " + tt.tokenType);
        }
        else 
        {
            std::cout << "Test for token " + t.type + " passed" << std::endl;
        }
    }
}

void testStatementParsing() 
{
    // adjacent strings are concatenated by compiler (cool!)
    std::string statement = 
        "let five = 5;"
        "let ten = 10"
        ""
        "let add = fn(x, y) {"
        "  x + y; "
        "};"
        ""
        "let result = add(five, ten);";

    TestToken tests[] = {
        TestToken(tokens::LET, "let"),
        TestToken(tokens::IDENT, "five"),
        TestToken(tokens::ASSIGN, "="),
        TestToken(tokens::INT, "5"),
        TestToken(tokens::SEMICOLON, ";"),
        TestToken(tokens::LET, "let"),
        TestToken(tokens::IDENT, "ten"),
        TestToken(tokens::ASSIGN, "="),
        TestToken(tokens::INT, "10"),
        TestToken(tokens::LET, "let"),
        TestToken(tokens::IDENT, "add"),
        TestToken(tokens::ASSIGN, "="),
        TestToken(tokens::FUNCTION, "fn"),
        TestToken(tokens::LPAREN, "("),
        TestToken(tokens::IDENT, "x"),
        TestToken(tokens::COMMA, ","),
        TestToken(tokens::IDENT, "y"),
        TestToken(tokens::RPAREN, ")"),
        TestToken(tokens::LBRACE, "{"),
        TestToken(tokens::IDENT, "x"),
        TestToken(tokens::PLUS, "+"),
        TestToken(tokens::IDENT, "y"),
        TestToken(tokens::SEMICOLON, ";"),
        TestToken(tokens::RBR
} 
        
void raiseError(std::string msg) 
{
    std::cerr << "Error in testing: " << msg << std::endl;
}
