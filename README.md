# Machina

A bytecode assembler and interpreter, focused on efficiency.

## About

Machina is a bytecode language, which is very simple and its syntax is close to Assembly.

This program contains an assembler and an interpreter, to run assembled Machina code.

Although it is close to Assembly, it is also strongly typed.

## How to Use

```
Usage: machina assemble/run <file>
```

The program accepts two CLI options: `assemble` and `run`.

- `assemble`:

Reads the provided source file, and assembles it into Machina bytecode.

- `run`:

Reads the provided bytecode file, and interprets it.

## Internals

The Machina interpreter works with:

- An operation stack, where the instructions push and pop values from;
- A variable map, where the names are stored as strings, mapped to their values.

## Syntax

There are 23 instructions and 3 data types in Machina. Although the number of instructions is low, the language is [Turing-complete](https://en.wikipedia.org/wiki/Turing_completeness) and very fast.

### Instructions

Instruction|Description
---|---
`pushc <value>`|Pushes a constant value onto the stack.
`pushv <name>`|Pushes the value of a variable onto the stack.
`setc <name> <value>`|Sets the value of a variable to a constant value.
`popv <name>`|Pops the last item from the stack into the specified variable.
`pop`|Pops the last item from the stack and discards it.
`add`|Pops two values from the stack, adds them and pushes the result.
`sub`|Pops two values from the stack, subtracts them and pushes the result.
`mul`|Pops two values from the stack, multiplies them and pushes the result.
`div`|Pops two values from the stack, divides them and pushes the result.
`inc`|Pops a value from the stack, increments it (add 1) and pushes the result.
`dec`|Pops a value from the stack, decrements it (sub 1) and pushes the result.
`inputn`|Prompts the user for a number and pushes the result.
`inputb`|Prompts the user for a boolean and pushes the result.
`inputs`|Prompts the user for a string and pushes the result.
`print`|Pops the last item from the stack and prints it.
`println`|Pops the last item from the stack and prints it, and then a newline.
`cmpg`|Pops two values from the stack, compares them, and pushes `true` if the first is greater than the second.
`cmpge`|Pops two values from the stack, compares them, and pushes `true` if the first is greater or equal to the second.
`cmpl`|Pops two values from the stack, compares them, and pushes `true` if the first is lesser than the second.
`cmple`|Pops two values from the stack, compares them, and pushes `true` if the first is lesser or equal to the second.
`cmpe`|Pops two values from the stack, compares them, and pushes `true` if both are equal.
`cmpne`|Pops two values from the stack, compares them, and pushes `true` if both are not equal.
`jmp <label>`|Pops a label from the stack and jumps to it.
`jt <label>`|Pops a label from the stack, then pops one more item from the stack and if it is equal to `true`, jumps to the previously popped label.
`jf <label>`|Pops a label from the stack, then pops one more item from the stack and if it is equal to `false`, jumps to the previously popped label.

### Types

Type|Description
---|---
`num`|Number (float of 64 bits)
`str`|String
`bool`|Boolean
`label`|Label

### Labels

Labels are declared using `#` as prefix, such as: `#label`.

They are treated as values in order to allow for compilers to implement first-class functions and dynamic dispatch.

## Examples

Adding two numbers:
```
pushc 10
pushc 20

add
println
```

Simple counter:
```
setc 0 counter

#loop
pushv counter
println

pushv counter
inc
popv counter

pushv counter
pushc 10

cmpg
pushc #loop
jt
```
