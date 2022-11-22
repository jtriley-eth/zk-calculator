use std::io;

use halo2_proofs::{circuit::Value, dev::MockProver, pasta::Fp};

use crate::{
    calculator_circuit::CalculatorCircuit,
    errors::{CircuitError, ParserError},
};

/// Valid operators for the ZkCalculator.
/// Note that other operators are not implemented due to complexity.
#[derive(Clone, Copy)]
pub enum Operator {
    /// Addition operator.
    Add,
    /// Subtraction operator.
    Sub,
    /// Multiplication operator.
    Mul,
}

/// Trait to facilitate parsing from a string slice to the desired Type.
trait FromToken<T, E> {
    /// Parses a string slice into a given type.
    fn from_token(token: &str) -> Result<T, E>;
}

/// FromToken implementation for Operator.
impl FromToken<Operator, ParserError> for Operator {
    /// Parses string slice and returns either the Operator or a ParserError.
    fn from_token(token: &str) -> Result<Operator, ParserError> {
        match token {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Sub),
            "*" => Ok(Operator::Mul),
            _ => Err(ParserError::InvalidOperator),
        }
    }
}

/// Type alias for u64 because i wanna.
type Operand = u64;

/// FromToken implementation for Operand.
impl FromToken<Operand, ParserError> for Operand {
    /// Parses a string slice and returns either an Operand(u64) or a
    /// ParserError.
    fn from_token(token: &str) -> Result<Operand, ParserError> {
        match token.parse::<Operand>() {
            Ok(operand) => Ok(operand),
            Err(_) => Err(ParserError::InvalidOperand),
        }
    }
}

/// Complete Operation.
struct Operation {
    /// Input a (lhs).
    pub a: Operand,
    /// Input b (rhs).
    pub b: Operand,
    /// Operator.
    pub operator: Operator,
}

/// ZkCalculator definition.
pub struct ZkCalculator {
    /// Optionally stores the Operation to execute.
    operation: Option<Operation>,
}

/// ZkCalculator ipmlementation.
impl ZkCalculator {
    /// Creates a new ZkCalculator with no operation defined.
    pub fn new() -> Self {
        Self { operation: None }
    }

    /// Runs the ZkCalculator Program.
    /// NOTE: All error code paths should panic here.
    pub fn run(&mut self) {
        // get user input.
        let mut input = String::new();
        println!("\n\n/- ---------------------------------------------- -/");
        println!("/- enter calculation to perforn (format: `a + b`) -/");
        // panics if io fails
        io::stdin().read_line(&mut input).expect("io failed");

        // parse input, panics if parsing fails
        self.parse(input).expect("parse failed");

        // run the circuit, panics if circuit fails
        let output = self.run_circuit().expect("circuit failed");

        // print the output, if the program hasn't panicked by now, the proof
        // generation and verification is successful
        println!("proof generation successful!\nresult: {:#?}", output);
    }

    /// Parses user input into an Operation and mutates the ZkCalculator.
    fn parse(&mut self, input: String) -> Result<(), ParserError> {
        // split input by whitespace
        let mut tokens = input.split_whitespace();

        // parse into operand a or bubble up error
        let a = match tokens.next() {
            Some(a) => Operand::from_token(a),
            None => Err(ParserError::NotEnoughInputs),
        }?;

        // parse into operator or bubble up error
        let operator = match tokens.next() {
            Some(op) => Operator::from_token(op),
            None => Err(ParserError::NotEnoughInputs),
        }?;

        // parse into operand or bubble up error
        let b = match tokens.next() {
            Some(b) => Operand::from_token(b),
            None => Err(ParserError::NotEnoughInputs),
        }?;

        // if there are more tokens remaining, something went wrong, so we
        // bubble up an error about it
        if tokens.next().is_some() {
            return Err(ParserError::TooManyInputs);
        }

        // mutate the ZkCalculator
        self.operation = Some(Operation { a, operator, b });

        // return ok
        Ok(())
    }

    /// Runs the circuit against a mock prover.
    fn run_circuit(&self) -> Result<Fp, CircuitError> {
        // `2**k` must be greater than the number of rows in the circuit,
        // this circuit only has two rows, so `4` is sufficient
        let k = 4;

        // get operation
        let operation = self.operation.as_ref().ok_or(CircuitError::NoOperation)?;

        // get operator
        let operator = operation.operator;

        // get a and b
        let a = Fp::from(operation.a);
        let b = Fp::from(operation.b);

        // compute c with a and b based on the operator
        let c = match operator {
            Operator::Add => a + b,
            Operator::Sub => a - b,
            Operator::Mul => a * b,
        };

        // create the top-level circuit
        let circuit = CalculatorCircuit {
            a: Value::known(a),
            b: Value::known(b),
            operator,
        };

        // public input is c
        let public_inputs = vec![c];

        // run the mock prover and bubble up any errors
        let prover = match MockProver::run(k, &circuit, vec![public_inputs.clone()]) {
            Ok(prover_run) => prover_run,
            Err(prover_error) => return Err(CircuitError::ProverError(prover_error)),
        };

        // verify the proof and bubble up any errors
        match prover.verify() {
            Ok(_) => (),
            Err(verifier_error) => return Err(CircuitError::VerifierError(verifier_error)),
        };

        // return c
        Ok(c)
    }
}
