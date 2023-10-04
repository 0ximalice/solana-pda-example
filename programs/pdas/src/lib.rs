use anchor_lang::prelude::*;

declare_id!("A83dYxXd3yZ3bJoKdu7rgb63tvP7WDVgi1dxDhCWcwK");

#[program]
pub mod pdas {
    use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let (pda, bump) = Pubkey::find_program_address(
            &[b"treasury", ctx.accounts.user.key.as_ref()],
            ctx.program_id,
        );
        msg!(
            "found PDA: {:?}, bump: {}, for {}",
            pda,
            bump,
            ctx.accounts.user.key.to_string()
        );
        Ok(())
    }

    pub fn redeem(ctx: Context<Redemption>, bump_seed: u8, lamports: u64) -> Result<()> {
        invoke_signed(
            &system_instruction::transfer(
                ctx.accounts.treasury.key,
                ctx.accounts.user.key,
                lamports,
            ),
            &[
                ctx.accounts.treasury.to_account_info(),
                ctx.accounts.user.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[&[b"treasury", ctx.accounts.user.key.as_ref(), &[bump_seed]]],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: user account
    #[account(mut)]
    pub user: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(bump_seed: u8, lamports: u64)]
pub struct Redemption<'info> {
    /// CHECK: the user treasury account
    #[account(mut)]
    pub treasury: AccountInfo<'info>,
    /// CHECK: just a user account
    #[account(mut)]
    pub user: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
