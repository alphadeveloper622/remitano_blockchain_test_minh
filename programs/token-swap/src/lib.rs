use anchor_lang::prelude::*;

declare_id!("emeVfnmJReFU9rcn1dFF2xBTYn7bsF3xTNq1u4LzwiJ");

#[program]
pub mod token_swap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
