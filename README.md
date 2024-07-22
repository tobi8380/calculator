# Evaluating expressions

Rational numbers are represented in Python as floating points. However, in some applications it is important to work with exact fractions, without introducing rounding errors. One possible solution is to represent each fraction as a pair of [tuple of two] integer numbers (numerator and denominator), with the restriction that the denominator be positive.

The goal of this exercise is to develop a calculator that receives an expression typed in by the user and evaluates it, printing the result as an (exact) fraction. The input is a valid arithmetic expression consisting of numbers, arithmetic operators and parentheses. Formally, we allow expressions generated by the following grammar.

```
e ::= e + m | e - m | - m | m
m ::= m * t | m / t | t
t ::= ( e ) | int
```

For simplicity, we assume that every token in the input is separated by spaces. That is, 2 + 3 is correct input, but 2+3 is not.

Your program should parse the input and compute the expression while parsing. The parser is developed in a straightforward way from the grammar of expressions, by writing one method for each case.

For example, an expression e always start with a multiplication m, possibly followed by a + or - sign and another expression. This means that we first need to call the function for parsing multiplications (which returns a value); after this function returns, if there are still unprocessed tokens, we check whether the next token is a + or a -, parse the remaining tokens as a (smaller) expression, and add or subtract the results.

The remaining functions are developed in a similar way.

Assume that only syntactically correct input is provided.

