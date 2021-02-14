use std::borrow::{Borrow, BorrowMut};

use combine::{Network, WeightsStore, backend::matrix::Matrix, Module};

#[derive(Clone)]
pub struct Gate {
    pub net: Network<f32>,
}

#[derive(Debug, Copy, Clone)]
pub struct GateOutput {
    pub output: f32,
}


impl GateOutput {
    pub fn negate(self) -> GateOutput {
        let mut output = self.output;
        if output > 0.1 {
            output = 0.;
        } else {
            output = 1.;
        }
        GateOutput {
            output
        }
    }
}

impl std::ops::Add for GateOutput {
    type Output = Matrix<f32>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut vec = Vec::new();
        vec.push(self.output);
        vec.push(rhs.output);
        Matrix::from_vector(1, 2, vec)
    }
}





impl Gate {
    pub fn compute(mut self, input: Matrix<f32>) -> GateOutput {
        GateOutput {
            output: self.net.forward(input).data[0],
        }
    }
    pub fn from_and() -> Gate {
        let net = WeightsStore::new().construct("./gates/and/");
        Gate {
            net,
        }
    }
    pub fn from_or() -> Gate {
        let net = WeightsStore::new().construct("./gates/or/");
        Gate {
            net,
        }
    }
    pub fn from_xor() -> Gate {
        let net = WeightsStore::new().construct("./gates/xor/");
        Gate {
            net,
        }
    }
}