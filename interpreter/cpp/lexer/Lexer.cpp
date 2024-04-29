#include <string>
#include <iostream>
#include "Lexer.h"
#include "Token.h"

void Lexer::readChar() const
{
    // if the next character index is oob, set to null terminated val (default)
    if (this->readPosition >= this->input.length()) 
    {
        this->ch = "\n";
    } 
    else 
    {
        this->ch = this->input[this->readPosition];
    }

    // move cursor forward
    this->position = this->readPosition;
    this->readPosition += 1;
}

Token Lexer::nextToken() const
{
    TokenType tt;
    std::string tl;

    // C++ doesn't allow strings in switch statements 
    if (this->ch == "=") 
    {
        tt = tokens::ASSIGN;
    } 
    else if(this->ch == ";")
    {
        tt = tokens::SEMICOLON;
    }
    else if(this->ch == "(")
    {
        tt = tokens::LPAREN;
    }
    else if(this->ch == ")")
    {
        tt = tokens::RPAREN;
    }
    else if(this->ch == ",") 
    {
        tt = tokens::COMMA;
    }
    else if(this->ch == "+") 
    {
        tt = tokens::PLUS;
    }
    else if(this->ch == "{")
    {
        tt = tokens::LBRACE;
    }
    else if(this->ch == "}")
    {
        tt = tokens::RBRACE;
    }
    else if(this->ch == "\0") 
    {
        tt = tokens::END_OF_FILE;
    }

    // format token literal
    if (tt == tokens::END_OF_FILE) 
    {
        tl = "";
    }
    else
    {
        tl = this->ch;
    }

    return Token(tt, tl);
}

void Lexer::debugPrintInput()
{
    std::cout << this->input << std::endl;
}
    
