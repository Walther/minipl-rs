# minipl-rs

Interpreter for the pedagogic language Mini-PL, for the Compilers course at Univ. Helsinki

> **Mini-PL** is a simple programming language designed for pedagogic purposes. The language is purposely small and is not actually meant for any real programming. Mini-PL contains few statements, arithmetic expressions, and some IO primitives. The language uses static typing and has three built-in types representing primitive values: int, string, and bool.

## Task

> Implement an **interpreter** for the Mini-PL programming language. The language analyzer must correctly recognize and process all valid (and invalid) Mini-PL programs. It should report syntactic errors, and then continue analyzing the rest of the source program. It must also construct an AST and make necessary passes over this program representation. The semantic analysis part binds names to their declarations, and checks semantic constraints, e.g., expressions types and their correct use. If the given program was found free from errors, the interpreter part will immediately execute it. 

[Task PDF](https://www.cs.helsinki.fi/u/vihavain/k20/Compilers/Project/Compilers%20Project%202020.pdf)

## Lanugage

[Syntax PDF](https://www.cs.helsinki.fi/u/vihavain/k20/Compilers/Project/Mini%20pl%20syntax%202020.pdf)

Some brief details outlined below.

**EBNF**:

```BNF
<prog>   ::=  <stmts>

<stmts>  ::=  <stmt> ";" ( <stmt> ";" )*

<stmt>   ::=  "var" <var_ident> ":" <type> [ ":=" <expr> ]
          |   <var_ident> ":=" <expr>
          |   "for" <var_ident> "in" <expr> ".." <expr> "do"
                <stmts> "end" "for"
          |   "read" <var_ident>
          |   "print" <expr>
          |   "assert" "(" <expr> ")"

<expr>   ::=  <opnd> <op> <opnd>
          |   [ <unary_op> ] <opnd>

<opnd>   ::=  <int>
          |   <string>
          |   <var_ident>
          |   "(" expr ")"

<type>   ::=  "int" | "string" | "bool"

<var_ident> ::= <ident>

<reserved keyword> ::=
          "var" | "for" | "end" | "in" | "do" | "read" |
          "print" | "int" | "string" | "bool" | "assert"
 ```

**Operators**:

```
+ - * / < = & !
```

> There is one unary operator symbol (`<unary_op>`): `!`, meaning the logical not operation. The operator symbol '&' stands for the logical and operation. Note that in Mini-PL, `=` is the equal operator - not assignment. 
