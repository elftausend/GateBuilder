
use std::{thread, vec};

use combine::{Matrix, Network, ModuleType, WeightsStore, ActivationType::{Relu, Sigmoid}, Optimizer};
use rand::prelude::*;

const EPOCHS: usize = 1691*2;
const INPUT_DUR: usize = 170*2;

fn get_rand_idx(max: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..max)
}

fn get_index(input: Matrix<f32>, values: [f32; 4]) -> f32 {
    if input.data[0] == 0. && input.data[1] == 0. {
        values[0]
    }
    else if input.data[0] == 1. && input.data[1] == 0. {
        values[1]
    }
    else if input.data[0] == 0. && input.data[1] == 1. {
        values[2]
    }
    else {
        values[3]
    }
}

pub fn train_nets() {
    /*
    let inputs_neg: [Matrix<f32>; 2] = [Matrix::from_vector(1, 1, vec![0.]),
                                    Matrix::from_vector(1, 1, vec![1.])]; */


    let inputs: [Matrix<f32>; 4] = [Matrix::from_vector(1, 2, vec![0., 0.]),
                                        Matrix::from_vector(1, 2, vec![1., 0.]), 
                                        Matrix::from_vector(1, 2, vec![0., 1.]),
                                        Matrix::from_vector(1, 2, vec![1., 1.])];
//    inputs[3] = &input;
   
    
   // let input = 
  //  let mut network_neg = Network::new(Matrix::new(0, 0, 0.));
    let mut network = Network::new(Matrix::new(0, 0, 0.));

    network.add(ModuleType::new(Sigmoid, 2, 2));
    network.add(ModuleType::new(Sigmoid, 2, 1));
    network.add(ModuleType::new(Relu, 1, 1));

    let inthread1 = inputs.clone();
    let inthread2 = inputs.clone();
    let inthread3 = inputs.clone();
  //  let inthread4 = inputs_neg.clone();

    let netthread1 = network.clone();
    let netthread2 = network.clone();
    let netthread3 = network.clone();
  //  let netthread4 = network_neg.clone();

    let builder = thread::Builder::new();
    let th1 = builder.spawn(move || {
        train_xor(inthread1, netthread1);
        1
    }).unwrap();

    let builder = thread::Builder::new();
    let th2 = builder.spawn(move || {
        train_or(inthread2, netthread2);
        1
    }).unwrap();

    let builder = thread::Builder::new();

    let th3 = builder.spawn(move || {
        train_and(inthread3, netthread3);
        1
    }).unwrap();

 //   let builder = thread::Builder::new();

   /* let th4 = builder.spawn(move || {
        train_neg(inthread4, netthread4);
        1
    }).unwrap(); */


    let sum = th1.join().unwrap()+th2.join().unwrap()+th3.join().unwrap()/*+th4.join().unwrap()*/;

    println!("sum: {:?}", sum);


}
/*
pub fn train_neg(inputs: [Matrix<f32>; 2], net: Network<f32>) {
    WeightsStore::new().save(net.clone(), "./gates/negate/");
    let mut net = WeightsStore::new().construct("./gates/negate");
}*/
pub fn train_and(inputs: [Matrix<f32>; 4], net: Network<f32>) {

    WeightsStore::new().save(net.clone(), "./gates/and/");
    let mut net = WeightsStore::new().construct("./gates/and/");

    for _x in 0..EPOCHS {
        let idx = get_rand_idx(4);
        //println!("idx: {}", idx);
        let input = inputs[idx].clone();
        
        net.aiming = Matrix::new(1, 1, get_index(input.clone(), [0., 0., 0., 1.,]));
        for _x in 0..INPUT_DUR {
            net.backwards(input.clone());
        }

    }

    WeightsStore::new().save(net.clone(), "./gates/and/");


}
pub fn train_or(inputs: [Matrix<f32>; 4], net: Network<f32>) {
    WeightsStore::new().save(net.clone(), "./gates/or/");
    let mut net = WeightsStore::new().construct("./gates/or/");

    for _x in 0..EPOCHS {
        let idx = get_rand_idx(4);
        //println!("idx: {}", idx);
        let input = inputs[idx].clone();

        net.aiming = Matrix::new(1, 1, get_index(input.clone(), [0., 1., 1., 1.,]));
        //net.aiming = Matrix::from_vector(1, 1, aiming);
        for _x in 0..INPUT_DUR {
            net.backwards(input.clone());
        }

    }

    WeightsStore::new().save(net.clone(), "./gates/or/");

}
pub fn train_xor(inputs: [Matrix<f32>; 4], net: Network<f32>) {

    WeightsStore::new().save(net.clone(), "./gates/xor/");
    let mut net = WeightsStore::new().construct("./gates/xor/");

    for _x in 0..EPOCHS {
        let idx = get_rand_idx(4);
        //println!("idx: {}", idx);
        let input = inputs[idx].clone();
      //  println!("input: {:?}", input);
       // println!("aiming: {:?}", get_index(input.clone(), [0., 1., 1., 0.,]));
       // println!("");
        net.aiming = Matrix::new(1, 1, get_index(input.clone(), [0., 1., 1., 0.,]));
      //  net.aiming = Matrix::from_vector(1, 1, aiming);
        for _x in 0..INPUT_DUR {
            net.backwards(input.clone());
        }

    }

    WeightsStore::new().save(net.clone(), "./gates/xor/");

}
