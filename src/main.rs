mod calculator_circuit;
mod chips;
mod errors;
mod zk_calculator;

use zk_calculator::ZkCalculator;

fn main() {
    ZkCalculator::new().run();
}
