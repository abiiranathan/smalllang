# smalllang

A small arithmetic language written in rust to demonstrate language design.

## Grammer

```txt
a=2
b=3
c=a *2 + b-7
print(c)

Grammar
=======
expression: assignment
assignment: (Variable "=")?expression | term
term: factor ( ("+" | "-") factor )*
factor: primary( ("*" | "/") primary )*
primary: Variable | Number | Funcall | "(" expression ")"
Funcall: Variable "(" expression ")"

```