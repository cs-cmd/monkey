#include <iostream>
#include "lexer/Lexer.h"
#include "tests.h"




int main() 
{
    // Token t = 
    // {
    //     .literal = "Hello",
    //     .type = "Hello!",
    // };
    //
    // std::cout << t.literal << " " << t.type << std::endl;

    // make an array of structures.
    // Use the constants defined as the type, and the icon as the literal 

    Lexer l("hello");

    l.debugPrintInput();

    testTokenParsing();

    testStatementParsing();
}
