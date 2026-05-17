#![no_std]
//! PoseidonHasher — wraps Protocol X-Ray's native Poseidon hash host function
//! into a reusable contract interface.
//!
//! Poseidon is a ZK-friendly hash function optimised for use inside
//! arithmetic circuits. X-Ray exposes it as a host function for efficient
//! on-chain hashing without expensive WASM computation.
//!
//! Common uses:
//! - Commitment schemes: commit(secret, nonce) = Poseidon(secret || nonce)
//! - Nullifier generation: nullifier = Poseidon(secret || nonce || spent_flag)
//! - Merkle tree nodes: node = Poseidon(left_child || right_child)
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    BytesN, Env, Vec,
};

#[contracttype]
pub enum HashMode {
    /// Standard Poseidon-128 (2 inputs)
    Poseidon128,
    /// Poseidon-256 (4 inputs)
    Poseidon256,
}

#[contract]
pub struct PoseidonHasher;

#[contractimpl]
impl PoseidonHasher {
    /// Hash up to 4 field elements using Protocol X-Ray Poseidon host function.
    ///
    /// Each input is a 32-byte field element (BN254 scalar field).
    /// Returns the 32-byte Poseidon hash output.
    ///
    /// Real impl: env.crypto().poseidon_hash(inputs) using X-Ray host function.
    /// Stub: XORs inputs together as placeholder for contributor implementation.
    pub fn hash(env: Env, inputs: Vec<BytesN<32>>) -> BytesN<32> {
        assert!(!inputs.is_empty() && inputs.len() <= 4,
            "Poseidon supports 1-4 inputs");

        // WAVE CONTRIBUTION GAP — Real implementation:
        // Use the X-Ray Poseidon host function binding:
        //   env.crypto().poseidon_hash(&inputs)
        //
        // The stub below XORs all inputs as a placeholder so the contract
        // compiles and tests pass. Replace with the actual host function call.
        let mut result = [0u8; 32];
        for input in inputs.iter() {
            let bytes = input.to_array();
            for (r, b) in result.iter_mut().zip(bytes.iter()) {
                *r ^= b;
            }
        }
        // Mix in a constant to differentiate from raw XOR
        result[0] ^= 0x50; // 'P' for Poseidon marker
        result[1] ^= 0x05;

        let hash = BytesN::from_array(&env, &result);
        env.events().publish((symbol_short!("hashed"), inputs.len()), hash.clone());
        hash
    }

    /// Generate a commitment: Poseidon(secret, nonce).
    /// Used in private transfer circuits for hiding token amounts.
    pub fn commit(env: Env, secret: BytesN<32>, nonce: BytesN<32>) -> BytesN<32> {
        Self::hash(env.clone(), soroban_sdk::vec![&env, secret, nonce])
            // note: env consumed above — this pattern works because hash() takes env by value
            // In real impl, restructure to avoid double-move
    }

    /// Generate a nullifier: Poseidon(secret, nonce, 1).
    /// Nullifiers are revealed when a commitment is spent, preventing double-spend.
    pub fn nullifier(env: Env, secret: BytesN<32>, nonce: BytesN<32>) -> BytesN<32> {
        let spent_flag = BytesN::from_array(&env, &{
            let mut a = [0u8; 32]; a[31] = 1; a
        });
        Self::hash(env.clone(), soroban_sdk::vec![&env, secret, nonce, spent_flag])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    fn scalar(env: &Env, val: u8) -> BytesN<32> {
        let mut a = [0u8; 32]; a[31] = val;
        BytesN::from_array(env, &a)
    }

    #[test]
    fn hash_two_inputs() {
        let env = Env::default();
        let cid = env.register_contract(None, PoseidonHasher);
        let client = PoseidonHasherClient::new(&env, &cid);
        let a = scalar(&env, 1);
        let b = scalar(&env, 2);
        let h = client.hash(&soroban_sdk::vec![&env, a, b]);
        assert_eq!(h.to_array().len(), 32);
    }

    #[test]
    fn different_inputs_different_outputs() {
        let env = Env::default();
        let cid = env.register_contract(None, PoseidonHasher);
        let client = PoseidonHasherClient::new(&env, &cid);
        let h1 = client.hash(&soroban_sdk::vec![&env, scalar(&env, 1), scalar(&env, 2)]);
        let h2 = client.hash(&soroban_sdk::vec![&env, scalar(&env, 3), scalar(&env, 4)]);
        assert_ne!(h1, h2);
    }

    #[test]
    #[should_panic(expected = "Poseidon supports 1-4 inputs")]
    fn too_many_inputs_panics() {
        let env = Env::default();
        let cid = env.register_contract(None, PoseidonHasher);
        let client = PoseidonHasherClient::new(&env, &cid);
        client.hash(&soroban_sdk::vec![
            &env,
            scalar(&env, 1), scalar(&env, 2),
            scalar(&env, 3), scalar(&env, 4), scalar(&env, 5)
        ]);
    }
}
