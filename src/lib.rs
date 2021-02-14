#![allow(non_snake_case)]
pub mod training;
pub mod gate;

#[cfg(test)]
mod tests {

  use combine::Matrix;
  use crate::gate::*;

  #[test]
  fn test() {
      //crate::training::train_nets();

      //let x: Matrix<f32> = Matrix::from_vector(1, 2, vec![1., 1.]);




      let input1: Matrix<f32> = Matrix::from_vector(1, 2, vec![1., 1.]);
      let input2: Matrix<f32> = Matrix::from_vector(1, 2, vec![1., 1.]);

      let input3: Matrix<f32> = Matrix::from_vector(1, 2, vec![1., 0.]);
      
      let gc = GateCreator::new();

      let and = gc.gate(GateLoader::load_and()); //.negate nand
      let xor = gc.gate(GateLoader::load_xor()); //.negate xnor
      let or = gc.gate(GateLoader::load_or()); //.negate nor

    
      let gate = xor.compute(xor.compute(input3)+(and.compute(and.compute(input1)+or.compute(input2).negate()))).negate();

    // let gate = xor.compute(input3).negate();
      println!("g: {:?}", gate);

      //let f = or.compute(input1).negate()+and.compute(input2);
      //println!("f: {:?}", f);

      /* 
      
      let forward = or.compute(x.clone());
      //forward.negate();
      println!("or forward: {:?}", forward.negate());

      let forward = xor.compute(x.clone());
      println!("xor forward: {:?}", forward);

      let forward = and.compute(x);
      println!("and forward: {:?}", forward);
      */
      
  }

}
