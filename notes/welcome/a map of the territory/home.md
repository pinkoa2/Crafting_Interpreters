# A Map of the Territory

[welcome](../home.md)

Even though computers are so much more advanced, the approach to writing a
programming language has not changed

## Scanning

The first step in scanning, also know as _lexing_
Some characters in a source file don’t actually mean anything.
Whitespace is often insignificant, and comments, by definition,
are ignored by the language. The scanner usually discards these,
leaving a clean sequence of meaningful tokens.

## Parsing

After scanning come parsing
This is like grammer for a spoken language

A parser takes the flat sequence of tokens and builds a tree structure
that mirrors the nested nature of the grammar.
These trees have a couple of different names _parse tree_ or _abstract syntax tree_

## Statis Analysis

Scanning and Parsing are pretty similar across all implementations

The first bit of analysis that most languages do is called binding or
resolution. For each identifier, we find out where that name is defined and
wire the two together. This is where scope comes into play—the region of source code
where a certain name can be used to refer to a certain declaration.

In languages that are _statically typed_ this is when we type check

## Intermediate representations

Front end of the pipeline is specific to the source language the program is written
in. The back end is concerned with the final architecture where the program will
run.
In the middle, the code may be stored in some intermediate representation

A shared intermediate representation reduces that dramatically. You write one
front end for each source language that produces the IR. Then one back end for
each target architecture. Now you can mix and match those to get every combination.

## Optimization

Constant folding for example. Do evaluation at compile time instead of run time

## Code Generation

generating code (or code gen), where “code” here usually refers to the kind of
primitive assembly-like instructions a CPU runs and not the kind of
“source code” a human might want to read.

Going down the mountain, we get closer to simple minded machines
If you generate instructions for a real CPU, then you are tied to that type of CPU.
Instead I think we should use virtual machines

Their compilers produce virtual machine code. Instead of instructions for some real
chip, they produced code for a hypothetical, idealized machine. Wirth called
this p-code for portable, but today, we generally call it bytecode because each
instruction is often a single byte long.

## Virtual Machine

Compile VM code to native code or write a VM program that simulates running on chip

## Runtime

If in machine code, you can just run it. If in VM you load it then run it.
We need garbage collection
In, say, Go, each compiled application has its own copy of Go’s runtime directly
embedded in it. If the language is run inside an interpreter or VM, then the runtime
lives there. This is how most implementations of languages
like Java, Python, and JavaScript work.


