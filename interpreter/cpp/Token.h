#include <string>

// Observations:
// `literal` could be a small value, like an `int` or `byte,` to improve 
// performance`
struct Token 
{
    std::string literal;
    std::string type;
};
