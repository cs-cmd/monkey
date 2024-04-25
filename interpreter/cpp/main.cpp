#include "Token.h"
#include <iostream>

int main() 
{
    Token t = 
    {
        .literal = "Hello",
        .type = "Hello!",
    };

    std::cout << t.literal << " " << t.type << std::endl;
}
