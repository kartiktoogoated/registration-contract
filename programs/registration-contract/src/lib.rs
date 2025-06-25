#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("69agw7oTBq2CzS8DBdcZKLPUQLtusrFyDrVg7BKjHj2s");

#[program]
pub mod registration_contract {
    use super::*;

    pub fn register(ctx: Context<Register>,id:u64, name: String) -> Result<()> {
        let registration = &mut ctx.accounts.registration;
        let user = &mut ctx.accounts.user;

        registration.id = id;
        registration.name = name;
        registration.user = user.key();
        registration.timestamp = Clock::get()?.unix_timestamp as u64;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Register<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 8 + (4 + 32) + 32 + 8 + 1,  // discriminator +  id + name + pubkey + timestamp + bump
        seeds = [b"registration", user.key().as_ref()],
        bump
    )]
    pub registration: Account<'info, Registration>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Registration {
    pub id: u64,
    pub name: String,
    pub user: Pubkey,
    pub timestamp: u64,
}