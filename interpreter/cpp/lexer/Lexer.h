#ifndef LEXER
#define LEXER
#include <string>
#include "Token.h"

struct Lexer
{
    std::string input;
    int position; // current char in input
    int readPosition; // current reading position (next character)
    std::string ch; // current character being examined

    Lexer(std::string input) 
        : input(input), position(-1), readPosition(0), ch("\0") 
    {
        this->readChar();
    }

    Token nextToken() const;
    void readChar() const;

    void debugPrintInput();
};
#endif // LEXER
