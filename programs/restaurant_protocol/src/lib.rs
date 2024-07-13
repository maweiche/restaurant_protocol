use anchor_lang::prelude::*;

declare_id!("99vazimgMSgqzg3zLBGXhZjSZVBRvThChw49VVp9U39T");

#[program]
pub mod restaurant_protocol {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
