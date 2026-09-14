use anchor_lang::prelude::*;

declare_id!("CBfua9WfUgaoWyjyyPs4xQbDNPpzUPdwpaGvR3x1yHGt");

#[program]
pub mod voting {
    use super::*;


    pub fn initialize_poll(ctx:Context<InitializePoll>,poll_id:u64)->Result<()>{
        let poll=&mut ctx.accounts.poll;
        poll.counter=poll_id;
        poll.optionA=0;
        poll.optionB=0;
        Ok(())
    }
   pub fn cast_vote(ctx:Context<CastVote>,vote_option:u8) -> Result<()> {
    let poll=&mut ctx.accounts.poll;
    let voter_record=&mut ctx.accounts.voter_record;
    voter_record.voter=ctx.accounts.voter.key();
    voter_record.poll_id=poll.counter;
    voter_record.bump=ctx.bumps.voter_record;


    match vote_option{
        1=>{
            poll.optionA=poll.optionA
                .checked_add(1)
                .ok_or(error!(VotingError::Overflow))?;
        }
        2 => {
            poll.optionB=poll.optionB
                .checked_add(1)
                .ok_or(error!(VotingError::Overflow))?;
        }
        _=> return Err(error!(VotingError::InvalidVoteOption)),
    }


    Ok(())
}
}

    #[error_code]
    pub enum VotingError{
        #[msg("Math Overflow error")]
        Overflow,
        #[msg("invalid vote option")]
        InvalidVoteOption,
    }

   



#[account]
pub struct Poll{
    pub optionA:u64,
    pub optionB:u64,
    pub counter:u64,
}
impl Poll{
    pub const LEN:usize=8+8+8+8;
}

#[account]
pub struct VoterRecord{
    pub voter:Pubkey,
    pub poll_id:u64,
    pub bump:u8,
}
impl VoterRecord{
    pub const LEN:usize=8+32+8+1;
}

#[derive(Accounts)]
#[instruction(poll_id:u64)]
pub struct InitializePoll<'info>{
    #[account(init,
    payer=payer,
    space=Poll::LEN,
    seeds=[b"poll", poll_id.to_le_bytes().as_ref()],
    bump
    )]

    pub poll:Account<'info, Poll>,
    pub system_program:Program<'info,System>,

    #[account(mut)]
    pub payer:Signer<'info>,
}



#[derive(Accounts)]
#[instruction(vote_option:u8)]
pub struct CastVote<'info>{
    #[account(
   mut
    )]
pub voter:Signer<'info>,



#[account(
   mut
    )]
    pub poll:Account<'info,Poll>,

    #[account(
        init,
        payer=voter,
        space = VoterRecord::LEN,
        seeds=[b"voter_record",poll.counter.to_le_bytes().as_ref(),voter.key().as_ref()],
        bump,
    )]
    pub voter_record:Account<'info, VoterRecord>,

    pub system_program:Program<'info, System>, 
}