# stellarzk-contracts

Soroban smart contracts wrapping Protocol X-Ray's BN254 and Poseidon host functions
into composable, reusable developer primitives.

## Contracts

| Contract | Purpose |
|----------|---------|
| `bn254_verifier` | Groth16 proof verifier using X-Ray BN254 host functions |
| `poseidon_hasher` | Poseidon hash utility contract using X-Ray host functions |
| `private_transfer` | End-to-end private token transfer using nullifiers and commitments |
