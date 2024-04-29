#include <iostream>
#include "./lexer/Token.h"
#include "./lexer/Lexer.h"
#include <string>
#include <iostream>
#include "tests.h"

struct TestToken {
    std::string tokenType;
    std::string tokenLiteral;

    TestToken(std::string tt, std::string tl)
        : tokenType(tt), tokenLiteral(tl) {};
};
    
void tokenParsing() {
    const std::string tokenLiterals = "=+(){},;";
    const Lexer testLexer = Lexer(tokenLiterals);

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

        if (t.literal != tt.tokenLiteral) {
            raiseError("Actual literal '" + t.literal + "' does not match expected literal: " + tt.tokenLiteral);
        } else if (t.type != tt.tokenType) {
            raiseError("Actual type '" + t.type + "' does not match expected type: " + tt.tokenType);
        }

        testLexer.readChar(); 

    }
}

void raiseError(std::string msg) {
    std::cerr << "Error in testing: " << msg << std::endl;
}
