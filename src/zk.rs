use ark_groth16::{Groth16, PreparedVerifyingKey, ProvingKey, VerifyingKey};
use ark_snark::SNARK;
use ark_bls12_381::{Bls12_381, Fr};
use ark_relations::r1cs::ConstraintSynthesizer;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_ff::PrimeField;
use sha2::{Sha256, Digest};
use std::fs;
use std::path::Path;
use std::sync::OnceLock;
use crate::circuit::QubitTransactionCircuit;

// Global key storage - loaded once on first access
static PROVING_KEY: OnceLock<ProvingKey<Bls12_381>> = OnceLock::new();
static VERIFYING_KEY: OnceLock<VerifyingKey<Bls12_381>> = OnceLock::new();

/// Load ZK keys from disk (downloads if not present)
pub fn load_zk_keys() -> Result<(), Box<dyn std::error::Error>> {
    if PROVING_KEY.get().is_some() && VERIFYING_KEY.get().is_some() {
        return Ok(()); // Already loaded
    }

    let key_dir = dirs::home_dir()
        .ok_or("Could not find home directory")?
        .join(".qubit")
        .join("keys");

    fs::create_dir_all(&key_dir)?;

    let pk_path = key_dir.join("proving_key.bin");
    let vk_path = key_dir.join("verification_key.json");

    // Download keys if not present
    if !pk_path.exists() || !vk_path.exists() {
        println!("🔑 ZK keys not found. Downloading...");
        download_zk_keys()?;
    }

    // Load proving key
    let pk_file = fs::File::open(&pk_path)?;
    let proving_key = ProvingKey::deserialize_compressed(pk_file)?;
    PROVING_KEY.set(proving_key).map_err(|_| "Failed to set proving key")?;

    // Load verification key
    let vk_content: serde_json::Value = serde_json::from_reader(fs::File::open(&vk_path)?)?;
    let vk_hex = vk_content["verification_key_hex"]
        .as_str()
        .ok_or("Invalid verification key format")?;

    let vk_bytes = hex::decode(vk_hex)?;
    let verifying_key = VerifyingKey::deserialize_compressed(&vk_bytes[..])?;
    VERIFYING_KEY.set(verifying_key).map_err(|_| "Failed to set verification key")?;

    println!("✅ ZK keys loaded successfully");
    Ok(())
}

/// Download ZK keys from decentralized storage
fn download_zk_keys() -> Result<(), Box<dyn std::error::Error>> {
    use std::process::Command;

    println!("⬇️  Downloading ZK keys...");

    // Run the download script
    let script_path = Path::new("zk-setup/download-keys.sh");
    if !script_path.exists() {
        return Err("Download script not found. Please run setup first.".into());
    }

    let status = Command::new("bash")
        .arg(script_path)
        .status()?;

    if !status.success() {
        return Err("Key download failed".into());
    }

    Ok(())
}

/// Generate actual ZK-SNARK proof for a transaction
pub fn generate_transaction_proof(
    secret_key: &[u8; 32],
    current_balance: u64,
    transfer_amount: u64,
    fee: u64,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    load_zk_keys()?;

    let pk = PROVING_KEY.get().ok_or("Proving key not loaded")?;

    // Convert inputs to field elements
    let secret_fr = Fr::from_le_bytes_mod_order(secret_key);
    let balance_fr = Fr::from(current_balance);
    let amount_fr = Fr::from(transfer_amount);
    let fee_fr = Fr::from(fee);

    // Derive public address from secret key (simplified)
    let mut hasher = Sha256::new();
    hasher.update(secret_key);
    let address_bytes = hasher.finalize();
    let address_fr = Fr::from_le_bytes_mod_order(&address_bytes);

    // Create circuit instance
    let circuit = QubitTransactionCircuit {
        secret_key: Some(secret_fr),
        current_balance: Some(balance_fr),
        public_address: Some(address_fr),
        transfer_amount: Some(amount_fr),
        fee: Some(fee_fr),
    };

    // Generate proof
    let mut rng = rand::thread_rng();
    let proof = Groth16::<Bls12_381>::prove(pk, circuit, &mut rng)?;

    // Serialize proof
    let mut proof_bytes = Vec::new();
    proof.serialize_compressed(&mut proof_bytes)?;

    Ok(proof_bytes)
}

/// Verify ZK-SNARK proof for a transaction
pub fn verify_transaction_proof(
    proof_bytes: &[u8],
    public_address: &[u8; 32],
    transfer_amount: u64,
    fee: u64,
) -> Result<bool, Box<dyn std::error::Error>> {
    load_zk_keys()?;

    let vk = VERIFYING_KEY.get().ok_or("Verification key not loaded")?;

    // Deserialize proof
    let proof = ark_groth16::Proof::deserialize_compressed(&proof_bytes[..])?;

    // Prepare public inputs
    let address_fr = Fr::from_le_bytes_mod_order(public_address);
    let amount_fr = Fr::from(transfer_amount);
    let fee_fr = Fr::from(fee);

    let public_inputs = vec![address_fr, amount_fr, fee_fr];

    // Verify proof
    let valid = Groth16::<Bls12_381>::verify(vk, &public_inputs, &proof)?;

    Ok(valid)
}

/// Generate ZK proof for mining (simplified for performance)
pub fn generate_zk_pass(wallet_secret: &[u8; 32], parent_hash: [u8; 32]) -> Vec<u8> {
    // For mining, we use a lightweight proof generation
    // In production, this could use a separate mining circuit
    let mut proof_data = vec![0u8; 128];
    let mut hasher = Sha256::new();
    hasher.update(wallet_secret);
    hasher.update(parent_hash);
    hasher.update(b"mining_proof");
    let hash = hasher.finalize();
    proof_data[..32].copy_from_slice(&hash);

    // If ZK keys are available, generate a real proof
    if let Ok(real_proof) = generate_transaction_proof(wallet_secret, 0, 0, 0) {
        if real_proof.len() >= 128 {
            proof_data.copy_from_slice(&real_proof[..128]);
        }
    }

    proof_data
}

/// Verify mining proof
pub fn verify_zk_pass(miner_address: &[u8; 32], _parent: &[u8; 32], proof: &[u8]) -> bool {
    if proof.len() != 128 {
        return false;
    }

    if miner_address == &[0u8; 32] {
        return false;
    }

    // If ZK verification is available, use it
    if let Ok(valid) = verify_transaction_proof(proof, miner_address, 0, 0) {
        return valid;
    }

    // Fallback to hash-based verification for backwards compatibility
    let mut hasher = Sha256::new();
    hasher.update(miner_address);
    hasher.update(_parent);
    hasher.update(b"mining_proof");
    let expected_hash = hasher.finalize();

    proof[..32] == expected_hash[..32]
}