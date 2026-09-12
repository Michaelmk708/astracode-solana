use anchor_lang::prelude::*;

declare_id!("AB3BP17KxYd2Ab8ec1fKg3UVmsdFo7JxZEmtjcCLJkF9");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[Account]
pub struct Poll{
    pub optionA:u64,
    pub optionB:u64,
    pub counter:u64,
}
impl Poll{
    pub const LEN:usize=8+32+32+32;
}

#[Account]
pub struct VoterRecord{
    pub voter:Pubkey,
    pub poll_id:u64,
    pub bump:u8,
}
impl voterRecord{
    pub const LEN:usize=8+32+32+1;
}