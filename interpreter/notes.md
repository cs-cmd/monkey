# Introduction
- Tree-walking interpreters parse the source code into an Abstract Syntax Tree
  (AST), and then 'walk' over that tree and interprets the code.
- `Monkey` is a language designed for teaching interpreter design
- `Monkey` is dynamically typed
- `Monkey` doesn't appear to have an `else if` structure:
    if (x == 0) {
        0 <-- Rust-like omittance of semi-colons to return values
    } else {
        if (x == 1) {
            1
        } else {
            fibonacci(x - 1) + fibonacci(x - 2)
        }
    }
- REPL: Read-Eval-Print Loop
- Major parts:
    - the lexer,
    - the parser,
    - the Abstract Syntax Tree (AST),
    - the internal object system, and
    - the evaluator

# Chapter 1 - Lexing
## Lexical Analysis
- Source codes needs to be turned into something 'more accessible'
- Source code will be transformed twice before being evaluated:
    - First into 'tokens'
    - Second into the Abstract Syntax Tree
- Source-code -> tokens is called 'lexical analysis'/'lexing'
- These tokens are fed into the parser that then produces the Abstract Syntax
  Tree
- Source Code -> tokenizer/lexer/scanner (tokens) -> parser (AST)
- Examples of tokens: 
    - let x = 5 + 5;
    - [
        LET,
        IDENTIFIER("x"),
        EQUAL_SIGN,
        INTEGER(5),
        PLUS_SIGN,
        INTEGER(5),
        SEMICOLON
      ]
    - Depending on implementation of a lexer, the conversion of '5' to an
      integer may come later in the interpretation process (such as during
      parsing or even later)
    - Whitespace will not be interpreted as a token in this implemetation of
      the 'Monkey' language specification

## Defining Tokens
- First example:
    let five = 5;
    let ten = 10;

    let add = fn(x, y) {
        return x + y;
    }

    let result = add(five, ten);

  - Tokens: LET, IDENTIFIER(_), EQUAL_SIGN, INTEGER(_), SEMICOLON, FN(ARGS...),
            R_/L_PAREN, R_/L_CURLY_BRACE, RETURN, PLUS_SIGN, FUNCTION_CALL(_)
- Tokens are defined as:
    - the token type (a predefined variable),
    - the token literal (the actual string value)

## The Lexer
- The lexer receives a string of input (i.e. the line of source code)
- `nextToken()` will read the next character of the input string
- `readChar()` determines the actual character used and moves the cursor 
  forward
- There are two pointers: one to the current character, and another to the next
    - The `next` pointer is used to peek ahead
- The `ch` value keeps track of the current value. Not all keywords in Monkey
  are single character, but using the `next` pointer will circumvent any issues
- This interpreter can be optimized by using bytes or integers as the TokenType
  datatype. Having to evaluate strings like this can be computationally
  expensive
- This current interpreter only supports basic ASCII to keep things simple.
- `readChar()` is used in the constructor to initialize the positions and 
  first value of the line
- The `nextToken()` method determines what the next token will be, then calls 
  `readChar()` to move the pointer forward
-
