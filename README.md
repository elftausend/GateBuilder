# GateBuilder
Just a test library to experiment with my neural network framework.
You can compute AND (& NOR), OR (& NOR), XOR (& XNOR) via three neural networks with it.

The neural network framework isn't public, so you actually can't. 
Nonetheless here is an example:

```rust
use gate_builder::gate::*;
use combine::Matrix;

let input: Matrix<f32> = Matrix::from_vector(1, 2, vec![0., 1.,]);
let input1: Matrix<f32> = Matrix::from_vector(1, 2, vec![0., 0.,]);

let gb = GateBuilder::new();
let and = gb.gate(GateLoader::load_and()); //.negate NAND
let or = gb.gate(GateLoader::load_or()); //.negate NOR

let nor_and = or.compute(input).negate()+and.compute(input1);
println!("nor_and: {:?}", nor_and);
//output: Matrix { rows: 1, cols: 2, data: [0.0, 0.000043723012] }

let xor = gb.gate(GateLoader::load_xor()); //.negate XNOR
let xnor_of_nor_and = xor.compute(nor_and).negate();
println!("xnor_of_nor_and: {:?}", xnor_of_nor_and);
//output: GateOutput { output: 1.0 }
```
