# ZK Calculator

Using halo2 crate to create basic calculator logic.

## Steps:

- receive user input
- parse user input into two operands, one operator, and one output
- compute output with caclulator circuit
- generate proof such that one (or two) operand(s) map to a given output via an
    operator
- verify proof

> NOTE: this only uses the Halo2 MockProver. It asserts circuit correctness, but
> keygen and such is synthetic.

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

caclulator circuit takes inputs, operator, output, and computes the proof via
the arithmetic chip
