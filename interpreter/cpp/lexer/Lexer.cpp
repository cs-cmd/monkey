#include <string>
#include <iostream>
#include "Lexer.h"
#include "Token.h"

Lexer::Lexer(std::string input)
{
    this->input = input;
    position = -1;
    readPosition = 0;
    ch = '_';
}

void Lexer::debugPrintInput()
{
    std::cout << this->input << std::endl;
}
    
