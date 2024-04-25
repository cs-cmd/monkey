#!/bin/bash

# Build step for the interpreter project in C++
# Did not realize how manual this would be (compiling manually from the 
# command-line.


# the `-g` flag adds additional debugging information to the GDB ([G]NU 
# Project [D]e[b]ugger
# The `find` command finds all [F]iles that match the regex for a `.cpp`
# extension
g++ -g $(find . -type f -iregex ".*\.cpp") -o test
