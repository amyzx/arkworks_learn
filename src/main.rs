use ark_r1cs_std::{fields::fp::FpVar, alloc::AllocVar};
use ark_r1cs_std::eq::EqGadget;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError}; 
use ark_bls12_381::{Bls12_381, Fr}; 
use ark_ff::{Zero}; 
use ark_groth16::*;
//use ark_crypto_primitives::snark::*;
use ark_std::rand::{rngs::StdRng, SeedableRng};

pub type FqVar = FpVar<Fr>;
pub type Fq = Fr;

// Prove: x * w = y 
#[derive(Clone)]
struct MatmulCircuit {
    x: Vec<Vec<Fq>>,   // public input
    w: Vec<Vec<Fq>>,   // witness
    y: Vec<Vec<Fq>>,   // public input
}

impl ConstraintSynthesizer<Fq> for MatmulCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fq>) -> Result<(), SynthesisError> {
        let m = self.x.len();
        let t = self.x[0].len();
        let n = self.y[0].len();
   
        // matrix multiplication
        for j in 0..n {
            for i in 0..m {
                let mut tmp_sum = FpVar::<Fq>::new_witness(
                    ark_relations::ns!(cs, "zero gadget"), || Ok(Fq::zero())
                ).expect("create zero ");
                for k in 0..t {
                    // calculate the sum of x row i * w col j 
                    let x_input = FpVar::<Fq>::new_input(
                        ark_relations::ns!(cs, "new input gadget"), || Ok(self.x[i][k])
                    ).expect("create new input");  

                    let w_witness = FpVar::<Fq>::new_witness(
                        ark_relations::ns!(cs, "new witness gadget"), || Ok(self.w[k][j])
                    ).expect("create new witness"); 

                    tmp_sum += x_input * w_witness; 
                }
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
    let val:u64 = i64::unsigned_abs(x); 
    let mut fq: Fq = val.into();  
    if x< 0 { 
        // let modulus = ark_bls12_381::FrParameters::MODULUS;
        // println!("{:#?}", modu);
        fq = - fq;   // neg_fq = modulus - fq
    }  
    fq
}


fn main() {  
    println!("===========================  matmul circuit test  ===========================  ");
    let mut rng = StdRng::seed_from_u64(0u64);
    let x = vec![vec![to_fq(-1); 4]; 5];   // 5*4 matrix     
    let w = vec![vec![to_fq(2); 3]; 4];    // 4*3 matrix
    let y = vec![vec![to_fq(-8); 3]; 5];   // 5*3 matrix

    let circuit = MatmulCircuit {
        x : x.clone(),
        w : w.clone(),
        y : y.clone(), 
    };

    let mut statement = Vec::new();
    for i in 0..5 {
        for j in 0..4 {
            statement.push(x[i][j].clone());
        } 
    }
    for i in 0..5 {
        for j in 0..3 {
            statement.push(y[i][j].clone());
        } 
    }

    // let params = Groth16::<Bls12_381>::setup(circuit.clone(), &mut rng).unwrap();
    // println!("set up");

    // let proof =  Groth16::<Bls12_381>::prove(&params.0, circuit.clone(), &mut rng).unwrap(); 
    // println!("prove");
 
    // let result = Groth16::<Bls12_381>::verify(&params.1, &statement, &proof).unwrap();
    
    let param = generate_random_parameters::<Bls12_381, _, _>(circuit.clone(), &mut rng).unwrap(); 
    println!("setup");

    let proof = create_random_proof(circuit.clone(), &param, &mut rng).unwrap();
    println!("prove");

    let pvk = prepare_verifying_key(&param.vk);  

    let result = verify_proof(&pvk, &proof, &statement).unwrap(); 
    println!("verify result is {:?}", result);

}