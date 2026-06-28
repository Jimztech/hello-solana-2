use anchor_lang::prelude::*;

declare_id!("5KPdALcmebHEF2LhHx5vpZfMsM5wFPg3pHWK7rsCr4QW");

#[program]
pub mod hello_solanaa_2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
