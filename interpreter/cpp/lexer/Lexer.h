#ifndef LEXER
#define LEXER
#include <string>

struct Lexer
{
    std::string input;
    int position; // current char in input
    int readPosition; // current reading position (next character)
    char ch; // current character being examined

    Lexer(std::string input);
    void debugPrintInput();
};
#endif // LEXER
