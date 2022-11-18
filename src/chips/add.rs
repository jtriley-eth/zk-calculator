use std::marker::PhantomData;

use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{Chip, Layouter, Region},
    plonk::{Advice, ConstraintSystem, Column, Error, Selector},
    poly::Rotation,
};

// we import `Number` from the top level chip to avoid redeclaring the same
// `Number` type for each operator chip.
use crate::chips::arithmetic::Number;

/// Addition instruction set.
pub trait AddInstructions<F: FieldExt>: Chip<F> {
    /// Numeric variable.
    type Num;

    /// Addition instruction.
    /// Takes two inputs and returns the sum.
    fn add(
        &self,
        layouter: &mut impl Layouter<F>,
        a: Self::Num,
        b: Self::Num,
    ) -> Result<Self::Num, Error>;
}

/// Addition chip configuration.
/// Derived during `Chip::configure`.
#[derive(Clone, Debug)]
pub struct AddConfig {
    /// Advice column for `input_a` and `output`.
    a: Column<Advice>,
    /// Advice column for `input_b`.
    b: Column<Advice>,
    /// Addition Selector.
    sel_add: Selector,
}

/// Addition chip definition.
pub struct AddChip<F: FieldExt> {
    /// Addition configuration.
    config: AddConfig,
    /// Placeholder data.
    _marker: PhantomData<F>,
}

/// Addition chip implementation.
impl<F: FieldExt> AddChip<F> {
    /// Construct AddChip and return.
    pub fn construct(
        config: <Self as Chip<F>>::Config,
        _loaded: <Self as Chip<F>>::Loaded
    ) -> Self {
        Self {
            config,
            _marker: PhantomData,
        }
    }

    /// Configure AddChip and return the Config.
    pub fn configure(
        meta: &mut ConstraintSystem<F>,
        a: Column<Advice>,
        b: Column<Advice>,
    ) -> <Self as Chip<F>>::Config {
        // get selector
        let sel_add = meta.selector();

        // define the addition gate
        meta.create_gate(
            // gate name
            "add",
            // gate logic
            |meta| {
                // query advice value from a on the current rotation
                let lhs = meta.query_advice(a, Rotation::cur());
                // query advice value from b on the current rotation
                let rhs = meta.query_advice(b, Rotation::cur());
                // query advice value from c on the next rotation
                let out = meta.query_advice(a, Rotation::next());
                // query selector
                let sel_add = meta.query_selector(sel_add);

                // return an iterable of `selector * (a + b - c)`
                // if `sel_add == 0`, then lhs, rhs and out are not constrained.
                // if `sel_add != 0`, then `lhs + rhs = out` is contrained.
                vec![sel_add * (lhs + rhs - out)]
            }
        );

        // return config
        AddConfig { a, b, sel_add }
    }
}

/// Halo2 Chip implementation for AddChip.
impl<F: FieldExt> Chip<F> for AddChip<F> {
    /// Addition configuration.
    type Config = AddConfig;
    /// Loaded data.
    type Loaded = ();

    /// Returns a configuration reference.
    fn config(&self) -> &Self::Config {
        &self.config
    }

    /// Returns the loaded data reference.
    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

/// Addition instruction set implementation for AddChip.
impl<F: FieldExt> AddInstructions<F> for AddChip<F> {
    /// Num type definition.
    type Num = Number<F>;

    /// Addition instruction implementation.
    fn add(
        &self,
        layouter: &mut impl Layouter<F>,
        a: Self::Num,
        b: Self::Num,
    ) -> Result<Self::Num, Error> {
        // get config
        let config = self.config();

        // assign a region of gates and return
        layouter.assign_region(
            // region name
            || "add",
            // assignment
            |mut region: Region<'_, F>| {
                // enable addition gate, set at region offset zero,
                // it will constrain cells zero and one
                config.sel_add.enable(&mut region, 0)?;

                // copy advice value a to offset zero, column a of the region
                a.0.copy_advice(|| "lhs", &mut region, config.a, 0)?;

                // copy advice value b to offset zero, column b of the region
                b.0.copy_advice(|| "rhs", &mut region, config.b, 0)?;

                // sum the values in columns a and b at offset zero
                let c = a.0.value().copied() + b.0.value();

                // mutate the region and return
                region
                    // assign the sum c as an advice into column a, offset one
                    .assign_advice(|| "lhs + rhs", config.a, 1, || c)
                    // map the result to `Number`
                    .map(Number)
            }
        )
    }
}
