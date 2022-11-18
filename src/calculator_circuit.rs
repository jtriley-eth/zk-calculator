use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{Circuit, ConstraintSystem, Error},
};

use crate::{
    chips::{
        arithmetic::{ArithmeticChip, ArithmeticConfig, ArithmeticInstructions},
        add::AddInstructions,
        mul::MulInstructions,
        sub::SubInstructions,
    },
    parser::Operator
};

/// Calculator circuit definition.
struct CalculatorCircuit<F: FieldExt> {
    a: Value<F>,
    b: Value<F>,
    operator: Operator
}

impl<F: FieldExt> CalculatorCircuit<F> {
    fn new(a: Value<F>, b: Value<F>, operator: Operator) -> Self {
        Self { a, b, operator }
    }
}

/// Calculator circuit implementation.
impl<F: FieldExt> Circuit<F> for CalculatorCircuit<F> {
    // reuse the top-level config
    type Config = ArithmeticConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self {
            a: Value::default(),
            b: Value::default(),
            operator: self.operator.clone(),
        }
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        // get advice columns
        let a = meta.advice_column();
        let b = meta.advice_column();
        // get instance column
        let instance = meta.instance_column();

        // reuse the ArithmeticChip configuration and return
        ArithmeticChip::configure(meta, a, b, instance)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        // construct the arithmetic chip
        let arithmetic_chip = ArithmeticChip::<F>::construct(config, ());

        // load private values into the circuit
        let a = arithmetic_chip.load_private(layouter.namespace(|| "load a"), self.a)?;
        let b = arithmetic_chip.load_private(layouter.namespace(|| "load b"), self.b)?;

        let c = match &self.operator {
            Operator::Add => arithmetic_chip.add(&mut layouter, a, b),
            Operator::Sub => arithmetic_chip.sub(&mut layouter, a, b),
            Operator::Mul => arithmetic_chip.mul(&mut layouter, a, b),
        }?;

        arithmetic_chip.expose_public(layouter.namespace(|| "expose c"), c, 0)
    }
}
