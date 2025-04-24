use anchor_lang::prelude::*;

declare_id!("1337wNtnZnGMvdKPBT1cUXfZnhvUgWv73fzUviMLj8x6");

#[program]
pub mod idl_example {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct User {
    pub name: String,
    pub age: u8,
}
