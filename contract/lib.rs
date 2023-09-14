use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("BNDCEb5uXCuWDxJW9BGmbfvR1JBMAKckfhYrEKW2Bv1W");

#[program]
pub mod zk_infomation {
    use super::*;
    pub fn send_infomation(ctx: Context<SendInfomation>, topic: [u8; 32], content: [u8; 32], zk_proof: [u8; ZK_PROOF_SIZE],) -> ProgramResult {
        let infomation: &mut Account<Infomation> = &mut ctx.accounts.infomation;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();

        // if topic.chars().count() > 50 {
        //     return Err(ErrorCode::TopicTooLong.into())
        // }

        // if content.chars().count() > 280 {
        //     return Err(ErrorCode::ContentTooLong.into())
        // }

        infomation.author = *author.key;
        infomation.timestamp = clock.unix_timestamp;
        infomation.topic = topic;
        infomation.content = content;
        infomation.zk_proof = zk_proof;

        Ok(())
    }

    pub fn update_infomation(ctx: Context<UpdateInfomation>, topic: [u8; 32], content: [u8; 32],) -> ProgramResult {
        let infomation: &mut Account<Infomation> = &mut ctx.accounts.infomation;

        // if topic.chars().count() > 50 {
        //     return Err(ErrorCode::TopicTooLong.into())
        // }

        // if content.chars().count() > 280 {
        //     return Err(ErrorCode::ContentTooLong.into())
        // }

        infomation.topic = topic;
        infomation.content = content;

        Ok(())
    }

    pub fn delete_infomation(_ctx: Context<DeleteInfomation>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SendInfomation<'info> {
    #[account(init, payer = author, space = Infomation::LEN)]
    pub infomation: Account<'info, Infomation>,
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateInfomation<'info> {
    #[account(mut, has_one = author)]
    pub infomation: Account<'info, Infomation>,
    pub author: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteInfomation<'info> {
    #[account(mut, has_one = author, close = author)]
    pub infomation: Account<'info, Infomation>,
    pub author: Signer<'info>,
}

#[account]
pub struct Infomation {
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: [u8; 32],
    pub content: [u8; 32],
    pub zk_proof: [u8; ZK_PROOF_SIZE],  // Size should be based on the zk proof system

}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4; // Stores the size of the string.
const ZK_POORF_LENGTH: usize = 280 * 4; // 280 chars max
impl Infomation {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Author.
        + TIMESTAMP_LENGTH // Timestamp.
        + 32 // Topic.
        + 32 // Content
        + ZK_POORF_LENGTH ; 
}

#[error]
pub enum ErrorCode {
    #[msg("The provided topic should be 32 characters long maximum.")]
    TopicTooLong,
    #[msg("The provided content should be 32 characters long maximum.")]
    ContentTooLong,
}
