mod chips;
mod calculator_circuit;
mod zk_calculator;
mod errors;

use zk_calculator::ZkCalculator;

fn main() {
    ZkCalculator::new().run();
}
