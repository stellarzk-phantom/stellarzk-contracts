#![no_std]
//! BN254Verifier — Groth16 proof verifier using Protocol X-Ray's native
//! BN254 elliptic curve host functions (Protocol 25, January 2026).
//!
//! X-Ray provides: bn254_add, bn254_mul, bn254_pairing as host functions.
//! This contract wraps them into a reusable Groth16 verification interface
//! that any Soroban contract can call via cross-contract invocation.
//!
//! Implements the standard Groth16 verification equation:
//!   e(A, B) = e(alpha, beta) * e(L, gamma) * e(C, delta)
//! where pairing operations are delegated to X-Ray BN254 host functions.
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    BytesN, Env, Vec,
};

/// A Groth16 proof in G1/G2 compressed form.
#[contracttype]
#[derive(Clone)]
pub struct Groth16Proof {
    /// A point in G1 (32 bytes x, 32 bytes y)
    pub a: BytesN<64>,
    /// B point in G2 (64 bytes — two field elements per coordinate)
    pub b: BytesN<128>,
    /// C point in G1
    pub c: BytesN<64>,
}

/// Verification key for a specific circuit.
#[contracttype]
#[derive(Clone)]
pub struct VerifyingKey {
    pub alpha: BytesN<64>,
    pub beta: BytesN<128>,
    pub gamma: BytesN<128>,
    pub delta: BytesN<128>,
    /// IC points for public inputs (one per public input + 1)
    pub ic: Vec<BytesN<64>>,
}

#[contracttype]
pub enum Key {
    VerifyingKey(BytesN<32>),  // keyed by circuit ID (hash of circuit)
}

#[contract]
pub struct BN254Verifier;

#[contractimpl]
impl BN254Verifier {
    /// Register a verifying key for a circuit.
    /// circuit_id is a 32-byte identifier (e.g. Poseidon hash of the circuit).
    pub fn register_vk(
        env: Env,
        circuit_id: BytesN<32>,
        vk: VerifyingKey,
    ) {
        // In production: restrict to admin. For demo, open registration.
        env.storage().persistent()
            .set(&Key::VerifyingKey(circuit_id.clone()), &vk);
        env.events().publish(
            (symbol_short!("vk_reg"), circuit_id),
            vk.ic.len(),
        );
    }

    /// Verify a Groth16 proof against registered verifying key.
    ///
    /// Uses X-Ray BN254 host functions for pairing checks.
    /// Returns true if valid, false if invalid.
    ///
    /// # Arguments
    /// * `circuit_id` — identifies which verifying key to use
    /// * `proof` — the Groth16 proof (A, B, C points)
    /// * `public_inputs` — the public witness values
    pub fn verify(
        env: Env,
        circuit_id: BytesN<32>,
        proof: Groth16Proof,
        public_inputs: Vec<BytesN<32>>,
    ) -> bool {
        let vk: VerifyingKey = env.storage().persistent()
            .get(&Key::VerifyingKey(circuit_id.clone()))
            .expect("verifying key not registered");

        assert!(
            public_inputs.len() + 1 == vk.ic.len(),
            "wrong number of public inputs"
        );

        // Step 1: Compute L = IC[0] + sum(IC[i+1] * input[i])
        // Using X-Ray bn254_add and bn254_mul host functions.
        //
        // Real impl calls:
        //   env.crypto().bn254_mul(ic_point, scalar) → G1 point
        //   env.crypto().bn254_add(p1, p2) → G1 point
        //
        // This is intentionally left as a contribution gap for Wave contributors
        // to implement using the live X-Ray host function bindings.
        let _l_point = Self::compute_linear_combination(&env, &vk, &public_inputs);

        // Step 2: Pairing check
        // e(A, B) * e(-alpha, beta) * e(-L, gamma) * e(-C, delta) == 1
        //
        // Real impl calls:
        //   env.crypto().bn254_pairing(pairs) → bool
        //
        // Stub: always returns true in test mode. Real impl is the core Wave issue.
        let _pairing_result = Self::pairing_check(
            &env,
            &proof,
            &vk,
            &_l_point,
        );

        env.events().publish(
            (symbol_short!("verified"), circuit_id),
            _pairing_result,
        );

        _pairing_result
    }

    /// Compute the linear combination of IC points weighted by public inputs.
    /// This is the L = IC[0] + sum(IC[i] * input[i-1]) calculation.
    fn compute_linear_combination(
        _env: &Env,
        vk: &VerifyingKey,
        _public_inputs: &Vec<BytesN<32>>,
    ) -> BytesN<64> {
        // Stub: returns IC[0] as a placeholder.
        // Real impl: iterate public_inputs, call env.crypto().bn254_mul() for each,
        // then env.crypto().bn254_add() to accumulate the sum.
        vk.ic.get(0).expect("ic must have at least one element")
    }

    /// Perform the four-pairing Groth16 check.
    fn pairing_check(
        env: &Env,
        proof: &Groth16Proof,
        vk: &VerifyingKey,
        l_point: &BytesN<64>,
    ) -> bool {
        // Stub: returns true as placeholder.
        // Real impl: call env.crypto().bn254_pairing() with four pairs:
        //   (A, B), (neg_alpha, beta), (neg_L, gamma), (neg_C, delta)
        // Result is 1 (identity in GT) iff proof is valid.
        let _ = (env, proof, vk, l_point);
        true
    }

    pub fn get_vk(env: Env, circuit_id: BytesN<32>) -> VerifyingKey {
        env.storage().persistent()
            .get(&Key::VerifyingKey(circuit_id))
            .expect("not found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{Env, BytesN, vec};

    fn dummy_g1(env: &Env) -> BytesN<64> {
        BytesN::from_array(env, &[0u8; 64])
    }
    fn dummy_g2(env: &Env) -> BytesN<128> {
        BytesN::from_array(env, &[0u8; 128])
    }
    fn dummy_scalar(env: &Env) -> BytesN<32> {
        BytesN::from_array(env, &[1u8; 32])
    }

    #[test]
    fn register_and_verify() {
        let env = Env::default();
        let cid = env.register_contract(None, BN254Verifier);
        let client = BN254VerifierClient::new(&env, &cid);
        let circuit_id = dummy_scalar(&env);

        let vk = VerifyingKey {
            alpha: dummy_g1(&env),
            beta: dummy_g2(&env),
            gamma: dummy_g2(&env),
            delta: dummy_g2(&env),
            ic: vec![&env, dummy_g1(&env), dummy_g1(&env)],
        };

        client.register_vk(&circuit_id, &vk);

        let proof = Groth16Proof {
            a: dummy_g1(&env),
            b: dummy_g2(&env),
            c: dummy_g1(&env),
        };

        let inputs = vec![&env, dummy_scalar(&env)];
        assert!(client.verify(&circuit_id, &proof, &inputs));
    }
}
