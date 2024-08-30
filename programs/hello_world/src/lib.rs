use anchor_lang::prelude::*;

declare_id!("22Hv3MiyfnMJkNoVKNXxueYtLhzJwh9EsuQmLzrG4NHP");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
