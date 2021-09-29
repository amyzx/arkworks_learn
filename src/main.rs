// import libraries
use ark_r1cs_std::{fields::fp::FpVar, alloc::AllocVar};
use ark_r1cs_std::eq::EqGadget;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError}; 
use ark_bls12_381::{Bls12_381, Fr}; 
use ark_ff::{Zero}; 
use ark_groth16::*;
use ark_crypto_primitives::snark::*; // import Groth16 library
use ark_std::rand::{rngs::StdRng, SeedableRng};

// rename FpVar which is a variable in Fp256
pub type FqVar = FpVar<Fr>;
pub type Fq = Fr;

// Prove matrix multiplication correctness: x * w = y 
// 1. We define a struct which contains the data needed to prove the computation.
#[derive(Clone)]
struct MatmulCircuit {
    x: Vec<Vec<Fq>>,   // public input
    w: Vec<Vec<Fq>>,   // witness
    y: Vec<Vec<Fq>>,   // public input
}

// 2. We implement this trait and implement the details for generate_constraints function.
impl ConstraintSynthesizer<Fq> for MatmulCircuit { 
    fn generate_constraints(self, cs: ConstraintSystemRef<Fq>) -> Result<(), SynthesisError> {
        // get x's row length, x's col length, y's col length
        let m = self.x.len();
        let t = self.x[0].len();
        let n = self.y[0].len();

        for i in 0..m { 
            for j in 0..t {
                // x is public input
                let x_input = FpVar::<Fq>::new_input(
                    ark_relations::ns!(cs, "new input gadget"), || Ok(self.x[i][j])
                ).expect("create new input"); 
            }
        }

        for i in 0..t { 
            for j in 0..n {
                // w is the witness which should be hidden
                let w_witness = FpVar::<Fq>::new_witness(
                    ark_relations::ns!(cs, "new witness gadget"), || Ok(self.w[i][j])
                ).expect("create new witness"); 
            }
        }
 
        // matrix multiplication
        for j in 0..n {
            for i in 0..m {
                // create zero value
                let mut tmp_sum = FpVar::<Fq>::new_witness(
                    ark_relations::ns!(cs, "zero gadget"), || Ok(Fq::zero())
                ).expect("create zero ");

                for k in 0..t {
                    // calculate the sum of x row i * w col j  
                    tmp_sum += self.x[i][k] * self.w[k][j]; 
                }

                // y is public input
                let y_input = FpVar::<Fq>::new_input(
                    ark_relations::ns!(cs, "new input gadget"), || Ok(self.y[i][j])
                ).expect("create new input"); 

                y_input.enforce_equal(&tmp_sum).expect("enforce equal: x * w = y");
            }
        } 
        Ok(())
    }
}
   
// map i64 to a finite field Fp256
fn to_fq(x: i64) -> Fq {
    // get the positive value of x
    let val:u64 = i64::unsigned_abs(x); 
    // map integer to Fp256
    let mut fq: Fq = val.into();  
    if x< 0 { 
        // let modulus = ark_bls12_381::FrParameters::MODULUS;
        // println!("{:#?}", modu);
        // if x is negative, we should return the inverse value
        fq = - fq;   // neg_fq = modulus - fq
    }  
    fq
}


fn main() {  
    println!("===========================  matmul circuit test  ===========================  ");
    let mut rng = StdRng::seed_from_u64(0u64);
    let x = vec![vec![to_fq(-1); 4]; 5];   // 5*4 matrix   all elements are -1  
    let w = vec![vec![to_fq(2); 3]; 4];    // 4*3 matrix   all elements are 2
    let y = vec![vec![to_fq(-8); 3]; 5];   // 5*3 matrix   all elements are -8

    let circuit = MatmulCircuit {
        x : x.clone(),
        w : w.clone(),
        y : y.clone(), 
    };
  
    // statement: collect all public inputs
    let mut statement = Vec::new();
    for i in 0..5 {
        for j in 0..4 {
            statement.push(x[i][j].clone());
        } 
    }
    for i in 0..5 {
        for j in 0..3 {
            statement.push(y[i][j].clone()); // if we change y to x, we will get false when verify result.
        } 
    }
  
    let param = generate_random_parameters::<Bls12_381, _, _>(circuit.clone(), &mut rng).unwrap(); 
    println!("setup");

    let proof = create_random_proof(circuit.clone(), &param, &mut rng).unwrap();
    println!("prove");

    let pvk = prepare_verifying_key(&param.vk);  

    let result = verify_proof(&pvk, &proof, &statement).unwrap(); 
    println!("verify result is {:?}", result);

    
    // Optional: we could use Groth16 library to setup, prove and verify a circuit.
    
    // let params = Groth16::<Bls12_381>::setup(circuit.clone(), &mut rng).unwrap();
    // println!("set up");

    // let proof =  Groth16::<Bls12_381>::prove(&params.0, circuit.clone(), &mut rng).unwrap(); 
    // println!("prove");

    // let result = Groth16::<Bls12_381>::verify(&params.1, &statement, &proof).unwrap();

}