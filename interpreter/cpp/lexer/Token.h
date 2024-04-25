#ifndef TOKEN
#define TOKEN
#include <string>

// Observations:
// `literal` could be a small value, like an `int` or `byte,` to improve 
// performance`
struct Token 
{
    std::string literal;
    std::string type;
};

// These macros are used as the `type` of the tokens
// Illegal tokens not recognized in languages
namespace tokens 
{
    static const std::string ILLEGAL = "ILLEGAL";
    static const std::string END_OF_FILE = "EOF";
    static const std::string IDENT = "IDENT";
    static const std::string INT = "INT";
    static const std::string ASSIGN = "=";
    static const std::string PLUS = "+";
    static const std::string COMMA = ",";
    static const std::string SEMICOLON = ",";
    static const std::string LPAREN = "(";
    static const std::string RPAREN = ")";
    static const std::string LBRACE = "{";
    static const std::string RBRACE = "}";
    static const std::string FUNCTION = "FUNCTION";
    static const std::string LET = "LET";
}
#endif // TOKEN
