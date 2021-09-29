## Introduction

This is an example of using [arkworks](https://github.com/arkworks-rs )' libraries to create [zksnark](https://zkp.science/).

Arkworks have many great work about zksnark, including [groth16](https://github.com/arkworks-rs/groth16), and [ripp](https://github.com/arkworks-rs/ripp ) which is aggregated groth16.

The prover **P** has a matrix multiplication equation,

X * W = Y

where X and Y are public inputs, W is the witness (secret information only known to **P**).

**P** could generate proof about the matrix multiplication computation.

The verifier **V** could verify the proof. 

If the result is true, then V could trust P.



## Description

*main.rs*: it uses groth16 library to generate and verify one proof.

*Cargo.toml*: define all dependencies and libraries used in the project.

*ripp/*: contains the source code copied from [ripp](https://github.com/arkworks-rs/ripp ).

*bin/batch_groth16.rs*: it uses ripp library to aggregate 1024 proofs.



## How to run

1. make sure you have **rust** environment

2. run the following command in arkworks_learn directory

   ```shell
   cargo run --bin arkworks_learn 
   cargo run --bin batch_groth16
   ```



