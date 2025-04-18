use anchor_lang::prelude::*;

declare_id!("HtVhuiF23oEMx1nMhqmLcL1eNKeQ8vz2UnbzhPE9ek1v");

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
