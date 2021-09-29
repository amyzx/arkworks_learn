## Introduction

This is an example of using [arkworks](https://github.com/arkworks-rs )' libraries to create [zksnark](https://zkp.science/).

Arkworks have many great work about zksnark, including [groth16](https://github.com/arkworks-rs/groth16), and [ripp](https://github.com/arkworks-rs/ripp ) which is aggregated groth16.

The prover **P** has a matrix multiplication equation,

X * W = Y

where X and Y are public inputs, W is the witness (secret information only known to **P**).

**P** could generate proof about the matrix multiplication computation.

The verifier **V** could verify the proof. 

If the result is true, then V could trust P.

## How to run

1. make sure you have **rust** environment

2. run the following command in terminal

   ```shell
   cargo run
   ```

   
