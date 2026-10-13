# Expression parser design notes

A future expression parser should tokenize input first, then parse tokens according to explicit operator precedence. A Pratt parser or shunting-yard algorithm would both be reasonable for this project.

Suggested precedence:

1. Parenthesized expressions and function calls.
2. Exponentiation.
3. Unary signs.
4. Multiplication and division.
5. Addition and subtraction.

The final precedence and associativity rules must be chosen deliberately and tested. Never evaluate user input as C++ code or pass it to a shell. Return structured parse errors with useful positions where possible.
