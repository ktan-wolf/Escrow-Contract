use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod events;

pub use instructions::*;
pub use state::*;
pub use events::*;

declare_id!("HyJFViPNgqV3DZYWqy2UAWmoqAokCXdVKyRc8hAMVTiC");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make> , seed : u64, deposit_amt: u64 , receive_amt: u64) -> Result<()>{
        ctx.accounts.deposit(deposit_amt)?;
        ctx.accounts.init_escrow(seed, receive_amt, &ctx.bumps)?;

        emit!(MakeEvent{
            maker: ctx.accounts.maker.key(),
            mint_a: ctx.accounts.mint_a.key(),
            mint_b: ctx.accounts.mint_b.key(),
            deposit_amt,
            receive_amt
        });

        Ok(())
    }
}