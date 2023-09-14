use anchor_lang::prelude::*;

declare_id!("YourProgramAddressHere");  // Update with your actual program's address.

const PROOF_SIZE: usize = 512;  // Replace with the actual size of your zk-proof.

#[program]
pub mod zk_proof_store {
    use super::*;

    pub fn store_proof(ctx: Context<StoreProof>, proof: [u8; PROOF_SIZE]) -> ProgramResult {
        // Simple validation example: Check the first byte is non-zero.
        // Replace with your actual validation logic if needed.
        if proof[0] == 0 {
            return Err(ErrorCode::InvalidProof.into());
        }

        let proof_record = &mut ctx.accounts.proof_record;
        proof_record.proof = proof;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StoreProof<'info> {
    #[account(init, payer = user, space = ProofRecord::LEN)]
    pub proof_record: Account<'info, ProofRecord>,
    pub user: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

#[account]
pub struct ProofRecord {
    pub proof: [u8; PROOF_SIZE],
}

impl ProofRecord {
    const LEN: usize = PROOF_SIZE;
}

#[error]
pub enum ErrorCode {
    #[msg("The provided proof is invalid.")]
    InvalidProof,
    // Add other error codes as needed.
}
