#ifndef TOKEN
#define TOKEN
#include <string>

using TokenType = std::string;
// Observations:
// `literal` could be a small value, like an `int` or `byte,` to improve 
// performance`
struct Token 
{
    std::string literal;
    TokenType type;

    Token(TokenType type, std::string literal) 
        : literal(literal), type(type) {};
};

// These macros are used as the `type` of the tokens
// Illegal tokens not recognized in languages
namespace tokens 
{
    static const TokenType ILLEGAL = "ILLEGAL";
    static const TokenType END_OF_FILE = "EOF";
    static const TokenType IDENT = "IDENT";
    static const TokenType INT = "INT";
    static const TokenType ASSIGN = "=";
    static const TokenType PLUS = "+";
    static const TokenType COMMA = ",";
    static const TokenType SEMICOLON = ",";
    static const TokenType LPAREN = "(";
    static const TokenType RPAREN = ")";
    static const TokenType LBRACE = "{";
    static const TokenType RBRACE = "}";
    static const TokenType FUNCTION = "FUNCTION";
    static const TokenType LET = "LET";
}
#endif // TOKEN
