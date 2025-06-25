#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("69agw7oTBq2CzS8DBdcZKLPUQLtusrFyDrVg7BKjHj2s");

#[program]
pub mod registration_contract {
    use super::*;

    pub fn init_profile(ctx: Context<InitProfile>, name: String) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        let user = &mut ctx.accounts.user;

        profile.name = name;
        profile.user = user.key();
        profile.timestamp = Clock::get()?.unix_timestamp as i64;

        Ok(())
    }

    pub fn init_validator(ctx: Context<InitValidator>, id: u64, name: String) -> Result<()> {
        let validator = &mut ctx.accounts.validator;
        validator.id = id;
        validator.name = name;
        validator.is_active = true;
        validator.authority = ctx.accounts.authority.key();
        validator.profile = ctx.accounts.profile.key();
        validator.bump = ctx.bumps.validator;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitProfile<'info> {
    #[account(
        init,
        payer = user,
        space = 8 +  (4 + 32) + 32 + 8 + 1,  // discriminator +  name + pubkey + timestamp + bump
        seeds = [b"profile", user.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, ProfileInfo>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct InitValidator<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 8 + 4 + 32 + 1 + 32 + 32 + 1,
        seeds = [b"validator", authority.key().as_ref(), &id.to_le_bytes()],
        bump
    )]
    pub validator: Account<'info, ValidatorInfo>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub profile: Account<'info, ProfileInfo>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ProfileInfo {
    pub name: String,
    pub user: Pubkey,
    pub timestamp: i64,
}

#[account]
pub struct ValidatorInfo {
    pub id: u64,
    pub name: String,
    pub is_active: bool,
    pub authority: Pubkey,
    pub profile: Pubkey,
    pub bump: u8,
}