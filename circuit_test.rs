use ark_bls12_381::{Bls12_381, Fr};
use ark_groth16::{Groth16, Proof, ProvingKey, VerifyingKey, create_random_proof, verify_proof, prepare_verifying_key};
use ark_relations::r1cs::ConstraintSynthesizer;
use rand::thread_rng;
use crate::circuit::QubitTransactionCircuit;

#[test]
fn test_groth16_proof() {
    // Example values
    let secret_key = Fr::from(123u64);
    let current_balance = Fr::from(1000u64);
    let public_address = secret_key;
    let transfer_amount = Fr::from(100u64);
    let fee = Fr::from(10u64);

    let circuit = QubitTransactionCircuit {
        secret_key: Some(secret_key),
        current_balance: Some(current_balance),
        public_address: Some(public_address),
        transfer_amount: Some(transfer_amount),
        fee: Some(fee),
    };

    // Setup
    let mut rng = thread_rng();
    let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(circuit.clone(), &mut rng).unwrap();

    // Proof generation
    let proof = create_random_proof(circuit.clone(), &pk, &mut rng).unwrap();

    // Prepare public inputs
    let public_inputs = [public_address, transfer_amount, fee];
    let pvk = prepare_verifying_key(&vk);
    let verified = verify_proof(&pvk, &proof, &public_inputs).unwrap();
    assert!(verified, "Groth16 proof verification failed");
}
