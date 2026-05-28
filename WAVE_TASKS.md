# StellarZK — Drips Wave 5 Task Backlog

Welcome to the StellarZK Drips Wave task backlog! Enhance zero-knowledge privacy pipelines for Stellar smart contracts.

---

## 🟢 Trivial Tasks (100 Points)

### SZK-T01: Proof Generation Performance Telemetry Logger
* **Description:** Write utility helper inside the SDK to record proof-generation durations and benchmark constraint speeds.
* **Complexity:** Trivial (100 pts)
* **Status:** Open 🚀
* **Files to Edit:**
  * `stellarzk-sdk/src/utils/telemetry.ts`
  * `stellarzk-sdk/src/index.ts`

---

## 🟡 Medium Tasks (150 Points)

### SZK-M01: Verification History Log Modal
* **Description:** Build a paginated tabular panel in the UI listing historic SNARK proof verifications.
* **Complexity:** Medium (150 pts)
* **Status:** Open 🚀
* **Files to Edit:**
  * `stellarzk-demo/src/pages/ProofsPage.tsx`
  * `stellarzk-demo/src/components/ProofTable.tsx`
* **Acceptance Criteria:**
  * Implements smooth transitions, interactive details modal, and search filtration.

---

## 🔴 High Tasks (200 Points)

### SZK-H01: WebWorker Prover Integration for Browser Performance
* **Description:** Integrate the snarkjs proof generation execution loop inside a dedicated browser WebWorker, preventing main UI thread freezes.
* **Complexity:** High (200 pts)
* **Status:** Open 🚀
* **Files to Edit:**
  * `stellarzk-sdk/src/prover/WebWorkerProver.ts`
  * `stellarzk-sdk/src/prover/index.ts`
* **Acceptance Criteria:**
  * Prover works entirely off-thread, rendering responsive progress bars.
