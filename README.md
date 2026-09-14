# AstraCode Voting Protocol

A decentralized, double-vote-resistant polling protocol deployed on the Solana blockchain. This Anchor-based smart contract enforces strict cryptographic voting rules using Program Derived Addresses (PDAs) to map voter records directly to individual polls.

##  Devnet Deployment Details

The protocol is actively deployed and verifiable on the Solana Devnet.

* **Network:** Devnet
* **Program ID:** `CBfua9WfUgaoWyjyyPs4xQbDNPpzUPdwpaGvR3x1yHGt`
* **Deployment Transaction:** [`2SUMfLewLeyjRTvUXk8g2DNF62KAo8KBd8iuFLX1dU58L39TH5KJCqpoBE4ambSKGsw75uDNexB1EzihakXQ3iVk`](https://www.google.com/search?q=https://explorer.solana.com/tx/2SUMfLewLeyjRTvUXk8g2DNF62KAo8KBd8iuFLX1dU58L39TH5KJCqpoBE4ambSKGsw75uDNexB1EzihakXQ3iVk%3Fcluster%3Ddevnet)
* **Explorer Link:** [View Program on Solana Explorer](https://www.google.com/search?q=https://explorer.solana.com/address/CBfua9WfUgaoWyjyyPs4xQbDNPpzUPdwpaGvR3x1yHGt%3Fcluster%3Ddevnet)

##  On-Chain Architecture

The protocol utilizes two core PDAs to manage state and prevent Sybil attacks.

### 1. `Poll` Account

Stores the ongoing state and counter for a specific poll.

| Field | Type | Description |
| --- | --- | --- |
| `option_a` | `u64` | Vote tally for Option 1 |
| `option_b` | `u64` | Vote tally for Option 2 |
| `counter` | `u64` | The unique ID of the poll |

* **PDA Seed Derivation:** `[b"poll", poll_id.to_le_bytes()]`

### 2. `VoterRecord` Account

Created instantly upon a successful vote. Because the PDA seed requires the voter's public key, the Solana runtime fundamentally rejects any attempt by a user to vote twice on the same poll.

| Field | Type | Description |
| --- | --- | --- |
| `voter` | `Pubkey` | The wallet address of the voter |
| `poll_id` | `u64` | The ID of the poll they voted on |
| `bump` | `u8` | PDA bump seed |

* **PDA Seed Derivation:** `[b"voter_record", poll_id.to_le_bytes(), voter.key()]`

##  Client Integration (TypeScript)

To interact with the deployed Devnet contract via `@coral-xyz/anchor`:

### Initializing a Poll

```typescript
import { BN } from "@coral-xyz/anchor";

const pollId = new BN(1);

// Derive the Poll PDA
const [pollPda] = PublicKey.findProgramAddressSync(
  [Buffer.from("poll"), pollId.toArrayLike(Buffer, "le", 8)],
  program.programId
);

await program.methods
  .initializePoll(pollId)
  .accounts({
    poll: pollPda,
    payer: provider.wallet.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();

```

### Casting a Vote

```typescript
const voteOption = 1; // 1 for Option A, 2 for Option B

// Derive the Voter Record PDA to prevent double-voting
const [voterRecordPda] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("voter_record"), 
    pollId.toArrayLike(Buffer, "le", 8), 
    provider.wallet.publicKey.toBuffer()
  ],
  program.programId
);

await program.methods
  .castVote(voteOption)
  .accounts({
    poll: pollPda,
    voterRecord: voterRecordPda,
    voter: provider.wallet.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();

```

---
