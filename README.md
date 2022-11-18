# ZK Calculator

Using halo2 crate to create basic calculator logic.

## Steps:

- receive user input
- parse user input into two operands, one operator, and one output
- compute output with caclulator circuit
- generate proof such that one (or two) operand(s) map to a given output via an
    operator
- ???

## TODO

- validate circuit
- render circuit layout as svg/png
- parser
- basic cli

## Circuit Design

```
circuit: calculator_circuit:
    chip: arithmetic_chip
        chip: add_chip
        chip: sub_chip
        chip: mul_chip
```

add, sub, and mul chips implement gates and layouts

arithmetic chip implements api to interface with add, sub, and mul chips

caclulator circuit takes inputs, operator, output, and computes the proof with
the arithmetic chip

... probably ...
