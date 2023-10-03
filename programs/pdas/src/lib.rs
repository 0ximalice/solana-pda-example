use anchor_lang::prelude::*;

declare_id!("A83dYxXd3yZ3bJoKdu7rgb63tvP7WDVgi1dxDhCWcwK");

#[program]
pub mod pdas {
    use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let (pda, nonce) = Pubkey::find_program_address(&[b"vault"], ctx.program_id);
        msg!("found PDA: {:?}, nonce: {}", pda, nonce);
        Ok(())
    }

    pub fn redeem(ctx: Context<Redemption>, bump_seed: u8, lamports: u64) -> Result<()> {
        invoke_signed(
            &system_instruction::transfer(
                ctx.accounts.vault.key,
                ctx.accounts.recipient.key,
                lamports,
            ),
            &[
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.recipient.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[&[b"vault", &[bump_seed]]],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
#[instruction(bump_seed: u8, lamports: u64)]
pub struct Redemption<'info> {
    /// CHECK: The vault account.
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    /// CHECK: just a recipient account.
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
