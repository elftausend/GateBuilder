use std::{cell::RefCell, usize};

use combine::{Network, WeightsStore, backend::matrix::Matrix, Module};


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

#[derive(Clone, Copy)]
pub struct Gate<'a> {
    pub index: usize,
    creator: &'a GateCreator,

}

impl <'a>Gate<'a> {
    pub fn compute(&self, input: Matrix<f32>) -> GateOutput {
        self.creator.compute(self.index, input)
    }
}


pub struct GateLoader;

impl GateLoader {
    pub fn load_and() -> Network<f32> {
        WeightsStore::new().construct("./gates/and/")
    }
    pub fn load_or() -> Network<f32> {
        WeightsStore::new().construct("./gates/or/")
    }
    pub fn load_xor() -> Network<f32> {
        WeightsStore::new().construct("./gates/xor/")
    }
}
pub struct GateCreator {
    pub networks: RefCell<Vec<Network<f32>>>,
}

impl GateCreator {
    pub fn new() -> GateCreator {
        GateCreator { networks: RefCell::new(Vec::new())}
    }
    pub fn push(&self, net: Network<f32>) -> usize {
        let len = self.networks.borrow().len();
        self.networks.borrow_mut().push(net);
        len
    }
    pub fn gate<'a>(&'a self, net: Network<f32>) -> Gate<'a> {
        let len = self.push(net.clone());
        Gate {
            index: len,
            creator: self
        }    
    }
    pub fn compute(&self, index: usize, input: Matrix<f32>) -> GateOutput {
        let output = &self.networks.borrow_mut()[index].forward(input).data[0];
        GateOutput {
            output: *output
        }
    }
}