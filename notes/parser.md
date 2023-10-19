# Parsing

use of | to seperate single choices
  ex: bread → "toast" | "biscuits" | "English muffin" ;
use of () to combine and can have | within
  ex: protein → ( "scrambled" | "poached" | "fried" ) "eggs" ;
use of * for 0 or more times
  ex: crispiness → "really" "really"* ;
use of + for at least 1
  ex: crispiness → "really"+ ;
use of ? for 0 or 1
  ex: breakfast → protein ( "with" breakfast "on the side" )? ;


The Rule for Lox

expression     → literal
               | unary
               | binary
               | grouping ;

literal        → NUMBER | STRING | "true" | "false" | "nil" ;
grouping       → "(" expression ")" ;
unary          → ( "-" | "!" ) expression ;
binary         → expression operator expression ;
operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
               | "+"  | "-"  | "*" | "/" ;

New Rules:

expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;



Terminal	    Code to match and consume a token
Nonterminal	  Call to that rule’s function
|	            if or switch statement
* or +	      while or for loop
?	            if statement
